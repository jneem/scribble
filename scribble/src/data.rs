use druid::{Color, Data, Lens, Point};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::Arc;

use scribble_curves::{time, Curve, LineStyle, SnippetData, SnippetId, SnippetsData, Time};

use crate::audio::{AudioSnippetData, AudioSnippetsData, AudioState};
use crate::widgets::ToggleButtonState;

#[derive(Clone, Data)]
pub struct CurveInProgressData {
    #[data(ignore)]
    inner: Arc<RefCell<Curve>>,

    #[data(ignore)]
    cur_style: LineStyle,

    // Data comparison is done using only the curve's length, since the length grows with
    // every modification.
    len: usize,
}

impl CurveInProgressData {
    pub fn new(color: Color, thickness: f64) -> CurveInProgressData {
        CurveInProgressData {
            inner: Arc::new(RefCell::new(Curve::new())),
            cur_style: LineStyle {
                color: color,
                thickness,
            },
            len: 0,
        }
    }

    pub fn move_to(&mut self, p: Point, time: Time) {
        self.inner
            .borrow_mut()
            .move_to(p, time, self.cur_style.clone());
        self.len += 1;
    }

    pub fn set_color(&mut self, c: Color) {
        self.cur_style.color = c;
    }

    pub fn line_to(&mut self, p: Point, time: Time) {
        self.inner.borrow_mut().line_to(p, time);
        self.len += 1;
    }

    // TODO: we don't need to consume self, so we could reuse the old curve's memory
    pub fn into_curve(self, distance_threshold: f64, angle_threshold: f64) -> Curve {
        self.inner
            .borrow()
            .smoothed(distance_threshold, angle_threshold)
    }
}

#[derive(Deserialize, Serialize)]
pub struct SaveFileData {
    pub version: u64,
    pub snippets: SnippetsData,
    pub audio_snippets: AudioSnippetsData,
}

/// This data contains the state of the drawing.
#[derive(Clone, Data, Lens)]
pub struct ScribbleState {
    pub new_snippet: Option<CurveInProgressData>,
    pub snippets: SnippetsData,
    pub audio_snippets: AudioSnippetsData,
    pub selected_snippet: Option<SnippetId>,

    pub mark: Option<Time>,
}

/// This data contains the state of the entire app.
#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub scribble: ScribbleState,
    pub action: CurrentAction,
    pub time: Time,

    // This is a bit of an odd one out, since it's specifically for input handling in the
    // drawing-pane widget. If there get to be more of these, maybe they should get split out.
    pub mouse_down: bool,

    pub line_thickness: f64,

    pub audio: Arc<RefCell<AudioState>>,

    pub palette: crate::widgets::PaletteData,

    pub encoding_status: Option<crate::encode::EncodingStatus>,

    #[data(ignore)]
    pub save_path: Option<PathBuf>,
}

impl Default for AppState {
    fn default() -> AppState {
        AppState {
            scribble: ScribbleState::default(),
            action: CurrentAction::Idle,
            time: time::ZERO,
            mouse_down: false,
            line_thickness: 5.0,
            audio: Arc::new(RefCell::new(AudioState::init())),
            palette: crate::widgets::PaletteData::default(),
            encoding_status: None,

            save_path: None,
        }
    }
}

impl Default for ScribbleState {
    fn default() -> ScribbleState {
        ScribbleState {
            new_snippet: None,
            snippets: SnippetsData::default(),
            audio_snippets: AudioSnippetsData::default(),
            selected_snippet: None,
            mark: None,
        }
    }
}

impl AppState {
    pub fn from_save_file(data: SaveFileData) -> AppState {
        AppState {
            scribble: ScribbleState::from_save_file(data),
            ..Default::default()
        }
    }

    pub fn start_recording(&mut self) {
        assert!(self.scribble.new_snippet.is_none());
        assert_eq!(self.action, CurrentAction::Idle);

        self.scribble.new_snippet = Some(CurveInProgressData::new(
            self.palette.selected_color().clone(),
            self.line_thickness,
        ));
        self.action = CurrentAction::WaitingToRecord;
    }

    /// Stops recording drawing, returning the snippet that we just finished recording (if it was
    /// non-empty).
    pub fn stop_recording(&mut self) -> Option<SnippetData> {
        assert!(
            self.action == CurrentAction::Recording
                || self.action == CurrentAction::WaitingToRecord
        );
        let new_snippet = self
            .scribble
            .new_snippet
            .take()
            .expect("Tried to stop recording, but we hadn't started!");
        self.action = CurrentAction::Idle;
        let new_curve = new_snippet.into_curve(1.0, std::f64::consts::PI / 4.0);
        if !new_curve.path.elements().is_empty() {
            Some(SnippetData::new(new_curve))
        } else {
            None
        }
    }

    pub fn start_playing(&mut self) {
        assert_eq!(self.action, CurrentAction::Idle);
        self.action = CurrentAction::Playing;
        self.audio
            .borrow_mut()
            .start_playing(self.scribble.audio_snippets.clone(), self.time, 1.0);
    }

    pub fn stop_playing(&mut self) {
        assert_eq!(self.action, CurrentAction::Playing);
        self.action = CurrentAction::Idle;
        self.audio.borrow_mut().stop_playing();
    }

    pub fn start_recording_audio(&mut self) {
        assert_eq!(self.action, CurrentAction::Idle);
        self.action = CurrentAction::RecordingAudio(self.time);
        self.audio.borrow_mut().start_recording();
    }

    /// Stops recording audio, returning the audio snippet that we just recorded.
    pub fn stop_recording_audio(&mut self) -> AudioSnippetData {
        if let CurrentAction::RecordingAudio(rec_start) = self.action {
            self.action = CurrentAction::Idle;
            let buf = self.audio.borrow_mut().stop_recording();
            dbg!(buf.len());
            AudioSnippetData::new(buf, rec_start)
        //self.audio_snippets = self.audio_snippets.with_new_snippet(buf, rec_start);
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
                self.audio.borrow_mut().start_playing(
                    self.scribble.audio_snippets.clone(),
                    self.time,
                    velocity,
                );
            }
            _ => {
                log::warn!("not scanning, because I'm busy doing {:?}", self.action);
            }
        }
    }

    pub fn stop_scanning(&mut self) {
        match self.action {
            CurrentAction::Scanning(_) => {
                self.audio.borrow_mut().stop_playing();
                self.action = CurrentAction::Idle;
            }
            _ => panic!("not scanning"),
        }
    }
}

impl ScribbleState {
    pub fn from_save_file(data: SaveFileData) -> ScribbleState {
        ScribbleState {
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

    pub fn curve_in_progress<'a>(&'a self) -> Option<impl std::ops::Deref<Target = Curve> + 'a> {
        self.new_snippet.as_ref().map(|s| s.inner.borrow())
    }
}

#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub enum CurrentAction {
    WaitingToRecord,
    Recording,
    Playing,

    /// The argument is the time at which audio capture started.
    RecordingAudio(Time),

    /// Fast-forward or reverse. The parameter is the speed factor, negative for reverse.
    Scanning(f64),
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
            WaitingToRecord => ToggledOn,
            Recording => ToggledOn,
            Idle => ToggledOff,
            Playing => Disabled,
            Scanning(_) => Disabled,
            RecordingAudio(_) => Disabled,
        }
    }

    pub fn play_toggle(&self) -> ToggleButtonState {
        use CurrentAction::*;
        use ToggleButtonState::*;
        match *self {
            WaitingToRecord => Disabled,
            Recording => Disabled,
            Scanning(_) => Disabled,
            Playing => ToggledOn,
            Idle => ToggledOff,
            RecordingAudio(_) => Disabled,
        }
    }

    pub fn rec_audio_toggle(&self) -> ToggleButtonState {
        use CurrentAction::*;
        use ToggleButtonState::*;
        match *self {
            WaitingToRecord => Disabled,
            Recording => Disabled,
            Scanning(_) => Disabled,
            Playing => Disabled,
            Idle => ToggledOff,
            RecordingAudio(_) => ToggledOn,
        }
    }

    pub fn is_idle(&self) -> bool {
        *self == CurrentAction::Idle
    }

    pub fn is_recording(&self) -> bool {
        *self == CurrentAction::Recording
    }

    pub fn is_waiting_to_record(&self) -> bool {
        *self == CurrentAction::WaitingToRecord
    }

    pub fn is_ticking(&self) -> bool {
        use CurrentAction::*;
        match *self {
            Recording | Playing | RecordingAudio(_) => true,
            _ => false,
        }
    }

    pub fn is_scanning(&self) -> bool {
        if let CurrentAction::Scanning(_) = *self {
            true
        } else {
            false
        }
    }
}