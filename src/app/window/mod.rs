use guion::env::Env;
use guion::util::bounds::Dims;
use guion::widget::as_widget::AsWidgetMut;

pub mod handle;

pub struct Window<E> where E: Env {
    pub handle: Option<druid_shell::WindowHandle>,
    pub widget: Box<dyn AsWidgetMut<E>+'static>,
    pub dims: Dims,
}
