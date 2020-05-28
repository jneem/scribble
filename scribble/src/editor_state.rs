use druid::kurbo::BezPath;
use druid::{Data, Lens, Point};
use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use scribble_curves::{
    time, Curve, Effect, Effects, FadeEffect, LineStyle, SegmentData, SnippetData, SnippetId,
    SnippetsData, Time,
};

use crate::audio::{AudioSnippetData, AudioSnippetId, AudioSnippetsData, AudioState};
use crate::save_state::SaveFileData;
use crate::undo::{UndoStack, UndoState};
use crate::widgets::ToggleButtonState;

/// While drawing, this stores one continuous poly-line (from pen-down to
/// pen-up). Because we expect lots of fast changes to this, it uses interior
/// mutability to avoid repeated allocations.
#[derive(Clone, Data, Default)]
pub struct SegmentInProgress {
    #[data(ignore)]
    points: Arc<RefCell<Vec<Point>>>,

    #[data(ignore)]
    times: Arc<RefCell<Vec<Time>>>,

    // Data comparison is done using the number of points, which grows with every modification.
    len: usize,
}

impl SegmentInProgress {
    pub fn add_point(&mut self, p: Point, t: Time) {
        self.points.borrow_mut().push(p);
        self.times.borrow_mut().push(t);
        self.len += 1;
    }

    /// Returns a simplified and smoothed version of this polyline.
    ///
    /// `distance_threshold` controls the simplification: higher values will result in
    /// a curve with fewer points. `angle_threshold` affects the presence of angles in
    /// the returned curve: higher values will result in more smooth parts and fewer
    /// angular parts.
    pub fn to_curve(&self, distance_threshold: f64, angle_threshold: f64) -> (BezPath, Vec<Time>) {
        let points = self.points.borrow();
        let times = self.times.borrow();
        let point_indices = scribble_curves::simplify::simplify(&points, distance_threshold);
        let times: Vec<Time> = point_indices.iter().map(|&i| times[i]).collect();
        let points: Vec<Point> = point_indices.iter().map(|&i| points[i]).collect();
        let path = scribble_curves::smooth::smooth(&points, 0.4, angle_threshold);
        (path, times)
    }
}

/// A snippet id, an audio snippet id, or neither.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Data)]
pub enum MaybeSnippetId {
    Draw(SnippetId),
    Audio(AudioSnippetId),
    None,
}

impl MaybeSnippetId {
    pub fn is_none(&self) -> bool {
        matches!(self, MaybeSnippetId::None)
    }

    pub fn as_draw(&self) -> Option<SnippetId> {
        if let MaybeSnippetId::Draw(id) = self {
            Some(*id)
        } else {
            None
        }
    }

    pub fn as_audio(&self) -> Option<AudioSnippetId> {
        if let MaybeSnippetId::Audio(id) = self {
            Some(*id)
        } else {
            None
        }
    }
}

impl From<SnippetId> for MaybeSnippetId {
    fn from(id: SnippetId) -> MaybeSnippetId {
        MaybeSnippetId::Draw(id)
    }
}

impl From<AudioSnippetId> for MaybeSnippetId {
    fn from(id: AudioSnippetId) -> MaybeSnippetId {
        MaybeSnippetId::Audio(id)
    }
}

impl Default for MaybeSnippetId {
    fn default() -> MaybeSnippetId {
        MaybeSnippetId::None
    }
}

/// This data contains the state of an editor window.
#[derive(Clone, Data, Lens)]
pub struct EditorState {
    pub new_segment: Option<SegmentInProgress>,
    pub new_curve: Option<Arc<Curve>>,

    pub snippets: SnippetsData,
    pub audio_snippets: AudioSnippetsData,
    pub selected_snippet: MaybeSnippetId,

    pub mark: Option<Time>,

    pub action: CurrentAction,
    pub recording_speed: RecordingSpeed,

    // TODO: there doesn't seem to be a lens(ignore) attribute?
    #[lens(name = "ignore_undo")]
    #[data(ignore)]
    pub undo: Arc<RefCell<UndoStack>>,

    #[lens(name = "time_lens")]
    time: Time,

    /// Here is how our time-keeping works: whenever something changes the
    /// current "speed" (e.g, starting to scan, draw command, etc.), we store the
    /// current wall clock time and the current logical time. Then on every
    /// frame, we use those stored values to update `time`. This is better than
    /// just incrementing `time` based on the inter-frame time, which is prone to
    /// drift.
    #[data(ignore)]
    time_snapshot: (Instant, Time),

    /// When true, the "fade out" toggle button is pressed down.
    pub fade_enabled: bool,

    pub line_thickness: f64,

    pub audio: Arc<RefCell<AudioState>>,

    pub palette: crate::widgets::PaletteData,

    pub encoding_status: Option<crate::encode::EncodingStatus>,

    #[data(ignore)]
    pub save_path: Option<PathBuf>,
}

impl Default for EditorState {
    fn default() -> EditorState {
        EditorState {
            new_segment: None,
            new_curve: None,
            snippets: SnippetsData::default(),
            audio_snippets: AudioSnippetsData::default(),
            selected_snippet: MaybeSnippetId::None,
            mark: None,

            action: CurrentAction::Idle,
            recording_speed: RecordingSpeed::Slow,
            undo: Arc::new(RefCell::new(UndoStack::new(UndoState::default()))),

            time_snapshot: (Instant::now(), time::ZERO),
            time: time::ZERO,
            fade_enabled: false,
            line_thickness: 0.004,
            audio: Arc::new(RefCell::new(AudioState::init())),
            palette: crate::widgets::PaletteData::default(),
            encoding_status: None,

            save_path: None,
        }
    }
}

impl EditorState {
    fn selected_effects(&self) -> Effects {
        let mut ret = Effects::default();
        if self.fade_enabled {
            ret.add(Effect::Fade(FadeEffect {
                pause: time::Diff::from_micros(250_000),
                fade: time::Diff::from_micros(250_000),
            }));
        }
        ret
    }

    /// Updates `self.time` according to the current wall clock time.
    pub fn update_time(&mut self) {
        self.time = self.accurate_time();
    }

    /// The current logical time.
    pub fn time(&self) -> Time {
        self.time
    }

    /// Our most accurate estimate for the current time.
    ///
    /// [`time`](AppData::time) returns the time at the last frame. This function checks
    /// the elapsed time since the last frame and interpolates the time based on that.
    pub fn accurate_time(&self) -> Time {
        let wall_micros_elapsed = Instant::now()
            .duration_since(self.time_snapshot.0)
            .as_micros();
        let logical_time_elapsed = time::Diff::from_micros(
            (wall_micros_elapsed as f64 * self.action.time_factor()) as i64,
        );
        self.time_snapshot.1 + logical_time_elapsed
    }

    // Remembers the current time, for calculating time changes later. This should probably be
    // called every time the action changes (TODO: we could make this less error-prone by
    // centralizing the action changes somewhere)
    fn take_time_snapshot(&mut self) {
        self.time_snapshot = (Instant::now(), self.time);
    }

    pub fn start_recording(&mut self, time_factor: f64) {
        assert!(self.new_curve.is_none());
        assert!(self.new_segment.is_none());
        assert_eq!(self.action, CurrentAction::Idle);

        self.action = CurrentAction::WaitingToRecord(time_factor);
        self.take_time_snapshot();
    }

    /// Puts us into the `WaitingToRecord` state, after first cleaning up any
    /// other states that need to be cleaned up. This is useful for handling
    /// mid-drawing undos.
    pub fn ensure_recording(&mut self) {
        match self.action {
            CurrentAction::Playing => self.stop_playing(),
            CurrentAction::Recording(_) => {
                // We don't want to call stop_recording(), because that will
                // clear out the snippet in progress. But we do need to reset
                // the audio.
                self.audio.borrow_mut().stop_playing();
            }
            CurrentAction::RecordingAudio(_) => {
                let _ = self.stop_recording_audio();
            }
            CurrentAction::Scanning(_) => self.stop_scanning(),
            _ => {}
        }
        self.new_segment = None;
        self.action = CurrentAction::WaitingToRecord(self.recording_speed.factor());
        self.take_time_snapshot();
    }

    pub fn start_actually_recording(&mut self) {
        if let CurrentAction::WaitingToRecord(time_factor) = self.action {
            self.action = CurrentAction::Recording(time_factor);
            self.take_time_snapshot();
            if time_factor > 0.0 {
                if let Err(e) = self.audio.borrow_mut().start_playing(
                    self.audio_snippets.clone(),
                    self.time,
                    time_factor,
                ) {
                    log::error!("failed to start playing audio: {}", e);
                }
            }
        } else {
            panic!("wasn't waiting to record");
        }
    }

    /// Takes the segment that is currently being drawn and adds it to the snippet in progress.
    pub fn add_segment_to_snippet(&mut self, seg: SegmentInProgress) {
        let effects = self.selected_effects();
        let style = LineStyle {
            color: self.palette.selected_color().clone(),
            thickness: self.line_thickness,
        };
        let seg_data = SegmentData { effects, style };
        let (path, times) = seg.to_curve(0.0005, std::f64::consts::PI / 4.0);

        // TODO(performance): this is quadratic for long snippets with lots of segments, because
        // we clone it every time the pen lifts.
        if let Some(curve) = self.new_curve.as_ref() {
            let mut curve_clone = curve.as_ref().clone();
            curve_clone.append_segment(path, times, seg_data);
            self.new_curve = Some(Arc::new(curve_clone));
        } else {
            let mut curve = Curve::new();
            curve.append_segment(path, times, seg_data);
            self.new_curve = Some(Arc::new(curve));
        }
    }

    /// Stops recording drawing, returning the snippet that we just finished recording (if it was
    /// non-empty).
    pub fn stop_recording(&mut self) -> Option<SnippetData> {
        assert!(
            matches!(self.action, CurrentAction::Recording(_) | CurrentAction::WaitingToRecord(_))
        );

        self.audio.borrow_mut().stop_playing();

        if let Some(seg) = self.new_segment.take() {
            // If there is an unfinished segment, we add it directly to the snippet without going
            // through a command, because we don't need the extra undo state.
            self.add_segment_to_snippet(seg);
        }
        self.action = CurrentAction::Idle;
        self.take_time_snapshot();
        self.new_curve
            .take()
            .map(|arc_curve| SnippetData::new(arc_curve.as_ref().clone()))
    }

    pub fn start_playing(&mut self) {
        assert_eq!(self.action, CurrentAction::Idle);
        self.action = CurrentAction::Playing;
        self.take_time_snapshot();
        if let Err(e) =
            self.audio
                .borrow_mut()
                .start_playing(self.audio_snippets.clone(), self.time, 1.0)
        {
            log::error!("failed to start playing audio: {}", e);
        }
    }

    pub fn stop_playing(&mut self) {
        assert_eq!(self.action, CurrentAction::Playing);
        self.action = CurrentAction::Idle;
        self.take_time_snapshot();
        self.audio.borrow_mut().stop_playing();
    }

    pub fn start_recording_audio(&mut self) {
        assert_eq!(self.action, CurrentAction::Idle);
        self.action = CurrentAction::RecordingAudio(self.time);
        self.take_time_snapshot();
        if let Err(e) = self.audio.borrow_mut().start_recording() {
            log::error!("failed to start recording audio: {}", e);
        }
    }

    /// Stops recording audio, returning the audio snippet that we just recorded.
    pub fn stop_recording_audio(&mut self) -> AudioSnippetData {
        if let CurrentAction::RecordingAudio(rec_start) = self.action {
            self.action = CurrentAction::Idle;
            self.take_time_snapshot();
            let buf = self.audio.borrow_mut().stop_recording();
            let mut ret = AudioSnippetData::new(buf, rec_start);

            // By default, we normalize to loudness -24. For some reason (possibly to do with
            // incorrectness in the lufs crate), this seems like a good value for avoiding
            // clipping.
            ret.set_multiplier(-24.0);
            ret
        } else {
            panic!("not recording");
        }
    }

    pub fn scan(&mut self, velocity: f64) {
        match self.action {
            CurrentAction::Scanning(cur_vel) if cur_vel != velocity => {
                self.action = CurrentAction::Scanning(velocity);
                // The audio player doesn't support changing direction midstream, and our UI should
                // never put us in that situation, because they have to lift one arrow key before
                // pressing the other.
                assert_eq!(velocity.signum(), cur_vel.signum());
                self.audio.borrow_mut().set_velocity(velocity);
            }
            CurrentAction::Idle => {
                self.action = CurrentAction::Scanning(velocity);
                if let Err(e) = self.audio.borrow_mut().start_playing(
                    self.audio_snippets.clone(),
                    self.time,
                    velocity,
                ) {
                    log::error!("failed to play audio: {}", e);
                }
            }
            _ => {
                log::warn!("not scanning, because I'm busy doing {:?}", self.action);
            }
        }
        self.take_time_snapshot();
    }

    pub fn stop_scanning(&mut self) {
        match self.action {
            CurrentAction::Scanning(_) => {
                self.audio.borrow_mut().stop_playing();
                self.action = CurrentAction::Idle;
                self.take_time_snapshot();
            }
            _ => panic!("not scanning"),
        }
    }

    pub fn warp_time_to(&mut self, time: Time) {
        self.time = time;
        self.take_time_snapshot();
    }

    pub fn add_to_cur_snippet(&mut self, p: Point, t: Time) {
        assert!(self.action.is_recording());

        if let Some(ref mut snip) = self.new_segment {
            snip.add_point(p, t);
        } else {
            let mut snip = SegmentInProgress::default();
            snip.add_point(p, t);
            self.new_segment = Some(snip);
        }
    }

    pub fn finish_cur_segment(&mut self) -> Option<SegmentInProgress> {
        assert!(self.action.is_recording());
        self.new_segment.take()
    }

    /// Returns the new snippet being drawn, converted to a [`Curve`] for your rendering convenience.
    pub fn new_snippet_as_curve(&self) -> Option<Curve> {
        if let Some(ref new_snippet) = self.new_segment {
            let mut ret = Curve::new();
            for (i, (p, t)) in new_snippet
                .points
                .borrow()
                .iter()
                .zip(new_snippet.times.borrow().iter())
                .enumerate()
            {
                if i == 0 {
                    let style = LineStyle {
                        color: self.palette.selected_color().clone(),
                        thickness: self.line_thickness,
                    };
                    let effects = self.selected_effects();
                    ret.move_to(*p, *t, style, effects);
                } else {
                    ret.line_to(*p, *t);
                }
            }
            Some(ret)
        } else {
            None
        }
    }

    pub fn from_save_file(data: SaveFileData) -> EditorState {
        EditorState {
            snippets: data.snippets,
            audio_snippets: data.audio_snippets,
            ..Default::default()
        }
    }

    pub fn to_save_file(&self) -> SaveFileData {
        SaveFileData {
            version: 0,
            snippets: self.snippets.clone(),
            audio_snippets: self.audio_snippets.clone(),
        }
    }

    fn undo_state(&self) -> UndoState {
        UndoState {
            new_curve: self.new_curve.clone(),
            snippets: self.snippets.clone(),
            audio_snippets: self.audio_snippets.clone(),
            selected_snippet: self.selected_snippet.clone(),
            mark: self.mark,
            time: self.time,
        }
    }

    pub fn push_undo_state(&mut self) {
        self.undo.borrow_mut().push(self.undo_state());
    }

    pub fn push_transient_undo_state(&mut self) {
        self.undo.borrow_mut().push_transient(self.undo_state());
    }

    fn restore_undo_state(&mut self, undo: UndoState) {
        let mid_recording = self.new_curve.is_some();

        self.new_curve = undo.new_curve;
        self.snippets = undo.snippets;
        self.audio_snippets = undo.audio_snippets;
        self.selected_snippet = undo.selected_snippet;
        self.mark = undo.mark;
        self.warp_time_to(undo.time);

        // This is a bit of a special-case hack. If there get to be more of
        // these, it might be worth storing some metadata in the undo state.
        //
        // In case the undo resets us to a mid-recording state, we ensure that
        // the state is waiting-to-record (i.e., recording but paused).
        if mid_recording {
            if let Some(new_curve) = self.new_curve.as_ref() {
                if let Some(&time) = new_curve.times.last() {
                    // This is even more of a special-case hack: the end of the
                    // last-drawn curve is likely to be after undo.time (because
                    // undo.time is the time of the beginning of the frame in
                    // which the last curve was drawn). Set the time to be the
                    // end of the last-drawn curve, otherwise they might try to
                    // draw the next segment before the last one finishes.
                    self.warp_time_to(time);
                }
            }
            self.ensure_recording();
        }
    }

    pub fn undo(&mut self) {
        let state = self.undo.borrow_mut().undo();
        if let Some(state) = state {
            self.restore_undo_state(state);
        }
    }

    pub fn redo(&mut self) {
        let state = self.undo.borrow_mut().redo();
        if let Some(state) = state {
            self.restore_undo_state(state);
        }
    }
}

#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub enum CurrentAction {
    /// They started an animation (e.g. by pressing the "video" button), but
    /// haven't actually started drawing yet. The time is not moving; we're
    /// waiting until they start drawing.
    WaitingToRecord(f64),

    /// They are drawing an animation, while the time is ticking.
    Recording(f64),

    /// They are watching the animation.
    Playing,

    /// The argument is the time at which audio capture started.
    RecordingAudio(Time),

    /// Fast-forward or reverse. The parameter is the speed factor, negative for reverse.
    Scanning(f64),

    /// They aren't doing anything.
    Idle,
}

impl Default for CurrentAction {
    fn default() -> CurrentAction {
        CurrentAction::Idle
    }
}

impl CurrentAction {
    pub fn rec_toggle(&self) -> ToggleButtonState {
        use CurrentAction::*;
        use ToggleButtonState::*;
        match *self {
            WaitingToRecord(_) => ToggledOn,
            Recording(_) => ToggledOn,
            Idle => ToggledOff,
            _ => Disabled,
        }
    }

    pub fn play_toggle(&self) -> ToggleButtonState {
        use CurrentAction::*;
        use ToggleButtonState::*;
        match *self {
            Playing => ToggledOn,
            Idle => ToggledOff,
            _ => Disabled,
        }
    }

    pub fn rec_audio_toggle(&self) -> ToggleButtonState {
        use CurrentAction::*;
        use ToggleButtonState::*;
        match *self {
            RecordingAudio(_) => ToggledOn,
            Idle => ToggledOff,
            _ => Disabled,
        }
    }

    pub fn is_idle(&self) -> bool {
        *self == CurrentAction::Idle
    }

    pub fn is_recording(&self) -> bool {
        matches!(*self, CurrentAction::Recording(_))
    }

    pub fn time_factor(&self) -> f64 {
        use CurrentAction::*;
        match *self {
            Playing => 1.0,
            RecordingAudio(_) => 1.0,
            Recording(x) => x,
            Scanning(x) => x,
            _ => 0.0,
        }
    }

    pub fn is_scanning(&self) -> bool {
        matches!(*self, CurrentAction::Scanning(_))
    }
}

#[derive(Clone, Copy, Data, PartialEq, Eq)]
pub enum RecordingSpeed {
    Paused,
    Slower,
    Slow,
    Normal,
}

impl RecordingSpeed {
    pub fn factor(&self) -> f64 {
        match self {
            RecordingSpeed::Paused => 0.0,
            RecordingSpeed::Slower => 1.0 / 8.0,
            RecordingSpeed::Slow => 1.0 / 3.0,
            RecordingSpeed::Normal => 1.0,
        }
    }
}
