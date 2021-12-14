use druid_shell::piet::CairoTextLayout;
use guion::aliases::{ERenderer, ETextLayout, ESCursor};
use guion::env::Env;
use guion::render::Render as GRender;
use guion::render::widgets::RenderStdWidgets;
use guion::util::AsRefMut;

use crate::style::selector::StdCursor;

use super::Render;

impl<'s,E> GRender<E> for Render<'s,E> where E: Env, ERenderer<'s,E>: AsRefMut<Self> {
    fn force(&self) -> bool {
        todo!()
    }

    fn validate_widgets(&mut self) -> bool {
        todo!()
    }

    fn lt<'r>(self) -> ERenderer<'r,E> where Self: 'r {
        todo!()
    }

    fn with_force<'r>(&'r mut self, force: bool) -> ERenderer<'r,E> where Self: 'r {
        todo!()
    }

    fn enforced<'r>(&'r mut self) -> ERenderer<'r,E> where Self: 'r {
        todo!()
    }

    fn inside_border_specific<'r>(&'r mut self, s: &guion::util::border::Border) -> ERenderer<'r,E> where Self: 'r {
        todo!()
    }

    fn slice<'r>(&'r mut self, s: &guion::util::bounds::Bounds) -> ERenderer<'r,E> where Self: 'r {
        todo!()
    }

    fn slice_abs<'r>(&'r mut self, s: &guion::util::bounds::Bounds) -> ERenderer<'r,E> where Self: 'r {
        todo!()
    }

    fn inner_centered<'r>(&'r mut self, size: guion::util::bounds::Dims) -> ERenderer<'r,E> where Self: 'r {
        todo!()
    }

    fn inner_aligned<'r>(&'r mut self, size: guion::util::bounds::Dims, align: (f32,f32)) -> ERenderer<'r,E> where Self: 'r {
        todo!()
    }

    fn with_style<'r>(&'r mut self, style: &guion::aliases::EStyle<E>) -> ERenderer<'r,E> {
        todo!()
    }

    fn with_style_selector<'r>(&'r mut self, style_selector: &guion::aliases::ESSelector<E>) -> ERenderer<'r,E> {
        todo!()
    }

    fn with_bounds<'r>(&'r mut self, bounds: guion::util::bounds::Bounds) -> ERenderer<'r,E> {
        todo!()
    }

    fn with_viewport<'r>(&'r mut self, viewport: guion::util::bounds::Bounds) -> ERenderer<'r,E> {
        todo!()
    }

    fn bounds(&self) -> &guion::util::bounds::Bounds {
        todo!()
    }

    fn viewport(&self) -> &guion::util::bounds::Bounds {
        todo!()
    }

    fn style(&self) -> &guion::aliases::EStyle<E> {
        todo!()
    }

    fn selector(&self) -> &guion::aliases::ESSelector<E> {
        todo!()
    }

    fn render_widget(&mut self, w: guion::widget::link::Link<E>) {
        todo!()
    }

    fn fork_with<'r>(&'r mut self, bounds: Option<guion::util::bounds::Bounds>, viewport: Option<guion::util::bounds::Bounds>, style: Option<guion::aliases::EStyle<E>>, selector: Option<guion::aliases::ESSelector<E>>) -> ERenderer<'r,E> {
        todo!()
    }
}

impl<'r,E> RenderStdWidgets<E> for Render<'r,E> where
    E: Env + Sync,
    ERenderer<'r,E>: AsRefMut<Self>,
    ETextLayout<E>: AsRefMut<CairoTextLayout>, //TODO use Piet trait variant
    ESCursor<E>: Into<StdCursor>,  //TODO Into<DruidCursor>
{

}
