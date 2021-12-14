use std::any::Any;

use guion::env::Env;
use guion::path::WidgetPath;
use guion::text::stor::TextStor;
use guion::util::traitcast::Traitcast;
use guion::widget::Widget;

use self::env::ExampleEnv;

pub mod ctx;
pub mod env;
pub mod valid;

pub trait Nucular<E> {

}

guion::traitcast_for!(Nucular<E>);

unsafe impl Traitcast<dyn Any,ExampleEnv> for dyn Widget<ExampleEnv> {
    type DestTypeID = dyn Any;
}

/*unsafe impl<E> Traitcast<dyn Nucular<E>,E> for dyn Widget<E> where E: Env {
    type DestTypeID = dyn Nucular<E>;
}*/
