use std::any::Any;

use guion::env::Env;
use guion::event_new::downcast_map::EventDowncastMap;
use guion::newpath::{PathResolvusDyn, PathStack};
use guion::queron::Queron;
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

pub struct StupidEventDowncastMap;

impl<E> EventDowncastMap<E> for StupidEventDowncastMap where E: Env {
    fn event_downcast_map<W,Ph,S,Evt>(
        widget: &W,
        path: &Ph,
        stack: &S,
        event: &Evt,
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        cache: &mut W::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) -> guion::EventResp
    where
        W: Widget<E> + ?Sized, Ph: PathStack<E> + ?Sized, S: Queron<E> + ?Sized, Evt: guion::event_new::Event<E> + ?Sized
    {
        use guion::event_new::variants::StdVariant;
        use guion::event::standard::variants::*;

        guion::event_downcast_map_tryion!(
            widget, path, stack, event, route_to_widget, cache, root, ctx;
            StdVariant<RootEvent<E>,E>;
            StdVariant<MouseMove,E>;
            StdVariant<MouseEnter,E>;
            StdVariant<MouseLeave,E>;
            StdVariant<Focus,E>;
            StdVariant<Unfocus,E>
        );
        widget.event_direct(path, stack, event, route_to_widget, cache, root, ctx)
    }
}
