use std::any::Any;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

use guion::env::Env;
use guion::error::ResolveResult;
use guion::invalidation::Invalidation;
use guion::util::bounds::{Dims, Bounds};
use guion::widget_decl::mut_target::MuTarget;
use guion::widget_decl::mutor_trait::*;
use guion::widget::Widget;
use guion::widget::cache::DynWidgetCache;
use guion::widget::dyn_tunnel::WidgetDyn;

use super::windows::Windows;

pub mod handle;

pub struct Window<E> where E: Env {
    pub handle: Option<druid_shell::WindowHandle>,
    pub widget: Box<dyn WidgetDyn<E>+'static>,
    pub vali: Invalidation,
    pub dims: Dims,
}

#[doc(hidden)]
pub struct ProtectedReturn(std::marker::PhantomData<()>);

impl<E> Window<E> where for<'a,'b> E: Env<RootMut<'b>=&'b mut Windows<E>> {
    // pub fn view<DispatchFn,R>(
    //     &self,
    //     mut dispatch: DispatchFn,
    //     window_id: usize,
    //     root: E::RootRef<'_>, ctx: &mut E::Context<'_>
    // ) -> R
    // where
    //     DispatchFn: for<'w,'ww,'r,'c,'cc> FnMut(&'w (dyn WidgetDyn<E>+'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
    //     Self: Sized,
    // {
    //     let mut callback_return: Option<R> = None;
    //     self.widget.view_dyn(
    //         &mut |widget,root,ctx| {
    //             let r = (dispatch)(widget,root,ctx);
    //             callback_return = Some(r);
    //             ProtectedReturn(PhantomData)
    //         },
    //         window_id,
    //         root,
    //         ctx
    //     );
    //     callback_return.unwrap()
    // }
}

// pub trait ViewDyn3<E>: 'static where for<'a,'b> E: Env<RootMut<'b>=&'b mut Windows<E>> {
//     fn view_dyn(
//         &self,
//         dispatch: &mut (dyn for<'w,'ww,'r,'c,'cc> FnMut(&'w (dyn WidgetDyn<E>+'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> ProtectedReturn + '_),
//         window_id: usize,
//         root: E::RootRef<'_>, ctx: &mut E::Context<'_>
//     ) -> ProtectedReturn;

//     fn as_any_mut(&mut self) -> &mut dyn Any;
// }

// impl<T,E> ViewDyn3<E> for T where for<'k> T: View<E> + 'static, for<'a> T::Mutarget: MuTarget<E,Mutable<'a>=Self>, for<'a,'b> E: Env<RootMut<'b>=&'b mut Windows<E>> {
//     #[inline]
//     fn view_dyn(
//         &self,
//         dispatch: &mut (dyn for<'w,'ww,'r,'c,'cc> FnMut(&'w (dyn WidgetDyn<E>+'ww),E::RootRef<'r>,&'c mut E::Context<'cc>) -> ProtectedReturn + '_),
//         window_id: usize,
//         root: E::RootRef<'_>, ctx: &mut E::Context<'_>
//     ) -> ProtectedReturn {
//         let mut g = box_view_cb(#[inline] move |widget,root,ctx|
//             (dispatch)(widget.erase(), root, ctx)
//         );
//         View::view( //TODO binding E::RootRef to &Windows triggers the horror compiler bug here
//             self,
//             &mut g,
//             MutorForTarget::<T::Mutarget,(),_,_>::new(move |root,callback,_,ctx| {
//                 let window = &mut root.windows[window_id];
//                 let state: &mut Self = window.widget.as_any_mut().downcast_mut::<Self>().expect("TODO");

//                 (callback)(
//                     Ok(state),
//                     ctx
//                 )
//             }).erase(),
//             root,
//             ctx
//         )
//     }

//     fn as_any_mut(&mut self) -> &mut dyn Any {
//         self
//     }
// }
