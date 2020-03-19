use druid::{
    Affine, BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Point, Rect, RenderContext, Size, TimerToken, UpdateCtx, Vec2, Widget,
};
use std::convert::TryInto;
use std::time::Instant;

use crate::data::ScribbleState;
use crate::FRAME_TIME;

// Width/height of the drawing in image coordinates.
const DRAWING_WIDTH: u64 = 1600;
const DRAWING_HEIGHT: u64 = 1200;

const ASPECT_RATIO: f64 = (DRAWING_WIDTH as f64) / (DRAWING_HEIGHT as f64);
const PAPER_COLOR: Color = Color::rgb8(0xff, 0xff, 0xff);
const PAPER_BDY_COLOR: Color = Color::rgb8(0x00, 0x00, 0x00);
const PAPER_BDY_THICKNESS: f64 = 1.0;

pub struct DrawingPane {
    paper_rect: Rect,
    timer_id: TimerToken,
}

impl DrawingPane {
    fn to_image_coords(&self) -> Affine {
        let top_left = Vec2::new(self.paper_rect.x0, self.paper_rect.y0);
        let size_ratio = (DRAWING_WIDTH as f64) / self.paper_rect.width();
        Affine::scale(size_ratio) * Affine::translate(-top_left)
    }

    fn from_image_coords(&self) -> Affine {
        let top_left = Vec2::new(self.paper_rect.x0, self.paper_rect.y0);
        let size_ratio = (DRAWING_WIDTH as f64) / self.paper_rect.width();
        Affine::translate(top_left) * Affine::scale(1.0 / size_ratio)
    }
}

impl Default for DrawingPane {
    fn default() -> DrawingPane {
        DrawingPane {
            paper_rect: Rect::ZERO,
            timer_id: TimerToken::INVALID,
        }
    }
}

impl Widget<ScribbleState> for DrawingPane {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, state: &mut ScribbleState, _env: &Env) {
        match event {
            Event::MouseMoved(ev) => {
                if state.mouse_down && state.action.is_recording() {
                    state
                        .new_snippet
                        .as_mut()
                        .unwrap()
                        .line_to(self.to_image_coords() * ev.pos);
                    ctx.request_paint();
                }
            }
            Event::MouseDown(ev) => {
                if ev.button.is_left() && state.action.is_recording() {
                    let snip = state
                        .new_snippet
                        .as_mut()
                        .expect("Recording, but no snippet!");
                    snip.move_to(self.to_image_coords() * ev.pos);

                    state.mouse_down = true;
                    ctx.request_paint();
                }
            }
            Event::MouseUp(ev) => {
                if ev.button.is_left() && state.action.is_recording() {
                    state.mouse_down = false;
                }
            }
            Event::WindowConnected => {
                ctx.request_paint();
                self.timer_id = ctx.request_timer(Instant::now() + FRAME_TIME);
            }
            Event::Timer(tok) => {
                if tok == &self.timer_id && !state.action.is_idle() {
                    let frame_time_micros: i64 = FRAME_TIME.as_micros().try_into().unwrap();
                    state.time_us += frame_time_micros;
                    ctx.request_paint();
                }

                self.timer_id = ctx.request_timer(Instant::now() + FRAME_TIME);
            }
            _ => {}
        }
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_state: &ScribbleState,
        _state: &ScribbleState,
        _env: &Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _: &LifeCycle,
        _state: &ScribbleState,
        _env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &ScribbleState,
        _env: &Env,
    ) -> Size {
        let size = bc.max();

        // Find the largest rectangle of the correct aspect ratio that will fit in the box.
        let paper_width = size.width.min(ASPECT_RATIO * size.height);
        let paper_height = paper_width / ASPECT_RATIO;
        dbg!(size);
        dbg!((paper_width, paper_height));
        self.paper_rect = Rect::from_origin_size(Point::ZERO, (paper_width, paper_height));
        self.paper_rect =
            self.paper_rect + size.to_vec2() / 2.0 - self.paper_rect.center().to_vec2();
        dbg!(self.paper_rect);
        self.paper_rect = self.paper_rect.inset(PAPER_BDY_THICKNESS).round();

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &ScribbleState, _env: &Env) {
        ctx.stroke(&self.paper_rect, &PAPER_BDY_COLOR, PAPER_BDY_THICKNESS);
        ctx.fill(&self.paper_rect, &PAPER_COLOR);

        ctx.with_save(|ctx| {
            ctx.transform(self.from_image_coords());
            if let Some(curve) = data.curve_in_progress() {
                ctx.stroke(&curve.path, &curve.color, curve.thickness);
            }

            for curve in &data.snippets.borrow().curves {
                ctx.stroke(
                    curve.path_until(data.time_us),
                    &curve.curve.color,
                    curve.curve.thickness,
                );
            }
            Ok(())
        })
        .unwrap();
    }
}
