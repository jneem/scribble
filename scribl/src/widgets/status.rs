use druid::piet::{FontFamily, PietText, PietTextLayout, Text, TextLayout, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::widget::{Align, Either, Flex, Label, ProgressBar, WidgetExt};
use druid::{lens, Color, Point};
use druid::{Data, LensExt};
use std::borrow::Cow;
use std::path::Path;

use scribl_curves::Time;

use crate::editor_state::{AsyncOpsStatus, EditorState, FinishedStatus};

const LINE_HEIGHT_FACTOR: f64 = 1.2;
const X_PADDING: f64 = 5.0;

// We have two possible status widgets: one is just a label; the other is a label + progress bar.
#[derive(Clone, Data, Debug)]
enum StatusType {
    Label(String),
    Progress(String, f64),
}

fn status_type(status: &AsyncOpsStatus) -> StatusType {
    fn f_name(p: &Path) -> Cow<str> {
        p.file_name()
            .map(|s| s.to_string_lossy())
            .unwrap_or("".into())
    }
    // We prioritize "in progress" messages.
    if let Some(x) = status.in_progress.encoding {
        StatusType::Progress("Encoding: ".to_owned(), x.0 as f64 / x.1 as f64)
    } else if let Some(path) = &status.in_progress.saving {
        StatusType::Label(format!("Saving {}...", f_name(path)))
    } else if let Some(path) = &status.in_progress.loading {
        StatusType::Label(format!("Loading {}...", f_name(path)))
    } else if let Some(finished) = &status.last_finished {
        match finished {
            FinishedStatus::Saved { path, time: _ } => {
                StatusType::Label(format!("Saved {}", f_name(path)))
            }
            FinishedStatus::Loaded { path, time: _ } => {
                StatusType::Label(format!("Loaded {}", f_name(path)))
            }
            FinishedStatus::Encoded { path, time: _ } => {
                StatusType::Label(format!("Encoded {}", f_name(path)))
            }
            FinishedStatus::Error(s) => StatusType::Label(format!("Error: {}", s)),
        }
    } else {
        StatusType::Label(String::new())
    }
}

pub fn make_status_bar() -> impl Widget<EditorState> {
    let time_label = Clock.lens(EditorState::time_lens);

    // TODO: label requests layout every time the string changes, which isn't necessary here.
    // Make a fixed-size label that doesn't re-layout itself.
    let label = Label::dynamic(
        |data: &AsyncOpsStatus, _env: &Env| match status_type(data) {
            StatusType::Label(s) => s.to_owned(),
            StatusType::Progress(_, _) => String::new(),
        },
    );

    let label_progress =
        Label::dynamic(
            |data: &AsyncOpsStatus, _env: &Env| match status_type(data) {
                StatusType::Progress(s, _) => s.to_owned(),
                StatusType::Label(_) => String::new(),
            },
        );

    let progress = ProgressBar::new().lens(lens::Id.map(
        |s| {
            if let StatusType::Progress(_, x) = status_type(s) {
                x
            } else {
                0.0
            }
        },
        |_, _| {},
    ));

    let status_label = Either::new(
        |data: &AsyncOpsStatus, _env| matches!(status_type(data), StatusType::Progress(_, _)),
        Flex::row()
            .with_child(label_progress)
            .with_child(progress)
            .with_flex_spacer(1.0),
        label,
    )
    .fix_width(250.0); // TODO: can we make this depend on the text width?

    let row = Flex::row()
        .with_child(time_label)
        .with_flex_spacer(1.0)
        .with_child(status_label.lens(EditorState::status));
    Align::centered(row)
}

// This is basically a Label, but with a fixed width: Label calls `request_layout` every time its
// text changes, which is too much for this purpose.
struct Clock;

fn get_layout(time: Time, t: &mut PietText, env: &Env) -> PietTextLayout {
    let font_size = env.get(druid::theme::TEXT_SIZE_NORMAL);
    let usecs = time.as_micros();
    let mins = usecs / 60_000_000;
    let secs = (usecs / 1_000_000) % 60;
    let cents = (usecs / 10_000) % 100;
    t.new_text_layout(format!("{:02}:{:02}.{:02}", mins, secs, cents))
        .font(FontFamily::MONOSPACE, font_size)
        .text_color(Color::WHITE)
        .build()
        .unwrap()
}

impl Widget<Time> for Clock {
    fn event(&mut self, _: &mut EventCtx, _: &Event, _: &mut Time, _: &Env) {}

    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &Time, _: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, _: &Time, _: &Time, _: &Env) {
        ctx.request_paint();
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _: &Time, env: &Env) -> Size {
        let font_size = env.get(druid::theme::TEXT_SIZE_NORMAL);
        let layout = get_layout(Time::ZERO, &mut ctx.text(), env);
        bc.constrain((
            layout.size().width + 2.0 * X_PADDING,
            font_size * LINE_HEIGHT_FACTOR,
        ))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Time, env: &Env) {
        let layout = get_layout(*data, &mut ctx.text(), env);
        let origin = Point::new(X_PADDING, 0.0);
        ctx.draw_text(&layout, origin);
    }
}
