use std::any::Any;

use guion::env::Env;
use guion::path::WidgetPath;
use guion::text::stor::TextStor;
use guion::traitcast::*;
use guion::widget::Widget;

use self::env::ExampleEnv;

pub mod ctx;
pub mod env;
pub mod valid;

pub trait Nucular<E> {

}

/*guion::traitcast_for_from_widget!(Nucular<E>);

unsafe impl TraitcastImpl<'static,dyn Any> for dyn Widget<ExampleEnv> {
    type DestTypeID = dyn Any;
}*/

/*unsafe impl<E> Traitcast<dyn Nucular<E>,E> for dyn Widget<E> where E: Env {
    type DestTypeID = dyn Nucular<E>;
}*/
