use druid::widget::prelude::*;
use druid::{Data, Insets, Rect, Vec2, WidgetPod};

/// A re-implementation of druid's container, supporting drop-shadows (but no borders because we
/// don't use them).
pub struct SunkenContainer<T, W> {
    inner: WidgetPod<T, W>,
}

impl<T: Data, W: Widget<T>> SunkenContainer<T, W> {
    pub fn new(child: W) -> SunkenContainer<T, W> {
        SunkenContainer {
            inner: WidgetPod::new(child),
        }
    }

    pub fn child(&self) -> &W {
        self.inner.widget()
    }

    pub fn child_mut(&mut self) -> &mut W {
        self.inner.widget_mut()
    }
}

impl<T: Data, W: Widget<T>> Widget<T> for SunkenContainer<T, W> {
    fn event(&mut self, ctx: &mut EventCtx, ev: &Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, ev, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, ev: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, ev, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let radius = env.get(crate::DROP_SHADOW_RADIUS);
        let offset = env.get(crate::DROP_SHADOW_OFFSET);
        let child_size = self.inner.layout(ctx, bc, data, env);
        self.inner
            .set_layout_rect(ctx, data, env, child_size.to_rect());
        let child_insets = self.inner.paint_insets();

        let insets = Insets::new(
            (radius - offset.x).max(child_insets.x0),
            radius - offset.y.max(child_insets.y0),
            radius + offset.x.max(child_insets.x1),
            radius + offset.y.max(child_insets.y1),
        );
        ctx.set_paint_insets(insets.nonnegative());
        child_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let radius = env.get(crate::DROP_SHADOW_RADIUS);
        let offset = env.get(crate::DROP_SHADOW_OFFSET).to_vec2();
        let color = env.get(crate::DROP_SHADOW_COLOR);
        self.inner.paint(ctx, data, env);

        let size = ctx.size();
        ctx.with_save(|ctx| {
            ctx.clip(size.to_rect());

            // Wide rectangles just above and below me.
            let top_rect =
                Rect::from_origin_size((0.0, -2.0 * radius), Size::new(size.width, 2.0 * radius))
                    + offset;
            let bottom_rect = top_rect + Vec2::new(0.0, 2.0 * radius + size.height);
            ctx.blurred_rect(top_rect, radius, &color);
            ctx.blurred_rect(bottom_rect, radius, &color);
        });
    }
}