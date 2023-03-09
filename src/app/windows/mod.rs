use std::marker::PhantomData;
use std::ops::Range;

use guion::invalidation::Invalidation;
use guion::traitcast::WQueryResponder;
use guion::util::tabulate::{TabulateResponse, TabulateOrigin, TabulateDirection};
use guion::{EventResp, event_new};
use guion::aliases::{ESize, EStyle, ERenderer};
use guion::ctx::Context;
use guion::env::Env;
use guion::newpath::{PathResolvus, SimplePathResolvus, FixedIdx, PathResolvusDyn, PathStack, SimplePathStack, PathFragment};
use guion::queron::Queron;
use guion::root::{RootRef, RootMut};
use guion::util::AsRefMut;
use guion::util::bounds::Bounds;
use guion::widget::cache::{DynWidgetCache, WidgetCache};
use guion::widget::dyn_tunnel::WidgetDyn;
use guion::widget::stack::{WithCurrentBounds};
use guion::widget::{Widget, WidgetChildDynResult, WidgetChildResolveDynResult, WidgetChildDynResultMut, WidgetChildResolveDynResultMut};

use super::ModelRoot;
use super::window::Window;

pub struct Windows<E> where E: Env {
    pub windows: Vec<Window<E>>,
    pub vali: Invalidation,
}

// impl WidgetRoot

impl<E> Windows<E> where for<'a,'b> E: Env<RootRef<'a>=&'a ModelRoot,RootMut<'b>=&'b mut ModelRoot> {
    pub(crate) fn path_of_window(&self, window: usize, ctx: &mut E::Context<'_>) -> impl PathResolvus<E> {
        SimplePathResolvus {
            inner: (),
            value: FixedIdx(window as isize),
            _p: PhantomData,
        }
    }

    pub fn with_window_by_path<'s,'l:'s,F,R>(
        &'s self,
        i: &(dyn PathResolvusDyn<E>+'_),
        mut callback: F,
        ctx: &mut E::Context<'_>,
    ) -> R
    where 
        F: for<'w,'ww,'c,'cc> FnMut(Result<&'w (dyn WidgetDyn<E>+'ww),()>,usize,&'c mut E::Context<'cc>) -> R,
        Self: 'l
    {
        if let Some(idx) = i.try_fragment::<FixedIdx>() {
            if let Some(v) = self.windows.get(idx.0 as usize) {
                (callback)(
                    Ok(v.widget.erase()),
                    idx.0 as usize,
                    ctx,
                );
            }
        }

        (callback)(Err(()),usize::MAX,ctx)
    }

    pub fn with_window_by_path_mut<'s,'l:'s,F,R>(
        &'s mut self,
        i: &(dyn PathResolvusDyn<E>+'_),
        mut callback: F,
        ctx: &mut E::Context<'_>,
    ) -> R
    where 
        F: for<'w,'ww,'c,'cc> FnMut(Result<&'w mut (dyn WidgetDyn<E>+'ww),()>,usize,&'c mut E::Context<'cc>) -> R,
        Self: 'l
    {
        if let Some(idx) = i.try_fragment::<FixedIdx>() {
            if let Some(v) = self.windows.get_mut(idx.0 as usize) {
                return (callback)(
                    Ok(v.widget.erase_mut()),
                    idx.0 as usize,
                    ctx,
                );
            }
        }

        (callback)(Err(()),usize::MAX,ctx)
    }

    // pub fn resolved(&self) -> Resolved<E> {
    //     Resolved{
    //         wref: WCow::Borrowed(self as &dyn Widget<E>),
    //         path: WidgetPath::empty(),
    //         direct_path: WidgetPath::empty(),
    //         root: self,
    //     }
    // }
}

// impl<'g,E> RootRef<E> for &'g Windows<E> where for<'a,'b> E: Env<RootRef<'a>=&'a ModelRoot,RootMut<'b>=&'b mut ModelRoot> {
//     fn fork<'s,'w:'s>(&'s self) -> E::RootRef<'w> where Self: 'w {
//         self
//     }

//     fn with_widget<'s,'l:'s,F,R>(
//         &'s self,
//         i: &(dyn PathResolvusDyn<E>+'_),
//         callback: F,
//         ctx: &mut E::Context<'_>,
//     ) -> R
//     where 
//         F: for<'w,'ww,'c,'cc> FnMut(Result<&'w (dyn WidgetDyn<E>+'ww),E::Error>,&'c mut E::Context<'cc>) -> R,
//         Self: 'l
//     {
//         let child = self.resolve_child_dyn(
//             i
//         );

//         let child = match child {
//             Some(r) => Ok(r.widget),
//             None => Err(todo!()),
//         };

//         (callback)(child, ctx)
//     }

//     fn trace_bounds(&self, ctx: &mut E::Context<'_>, i: &(dyn PathResolvusDyn<E>+'_), b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,E::Error> {
//         todo!()
//     }
// }
// impl<'g,E> RootMut<E> for &'g mut Windows<E> where for<'a,'b> E: Env<RootRef<'a>=&'a ModelRoot,RootMut<'b>=&'b mut ModelRoot> {
//     fn fork_mut<'s>(&'s mut self) -> E::RootMut<'s> where Self: 's {
//         self
//     }
// }

// impl<E> Widgets<E> for Windows<E> where for<'a,'b> E: Env<RootRef<'a>=&'a ModelRoot,RootMut<'b>=&'b mut ModelRoot> {
//     fn widget(&self, i: E::WidgetPath) -> Result<Resolved<E>,E::Error> {
//         resolve_in_root(
//             self,
//             i.clone(),
//             i,
//             self.as_ref()
//         )
//     }

//     fn widget_mut(&mut self, i: E::WidgetPath) -> Result<ResolvedMut<E>,E::Error> {
//         resolve_in_root_mut(
//             self.as_mut(),
//             |s| AsRefMut::<Self>::as_mut(s) as &mut dyn WidgetMut<_>,
//             i.clone(),
//             i
//         )
//     }

//     fn trace_bounds(&self, ctx: &mut E::Context<'_>, i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,E::Error> {
//         let l = ctx.link(Resolved{
//             wref: AWidget::Ref(self.base()),
//             path: WidgetPath::empty(),
//             direct_path: WidgetPath::empty(),
//             stor: self.as_ref(),
//         });
//         Widget::trace_bounds(self,l,i,b,e,force)
//     }

//     fn lt_ref<'l,'r,'s>(&'r self) -> &'r E::Storage<'s> where 's: 'r, 'l: 'r, 'l: 's, Self: 'l {
//         self.as_ref()
//     }
//     fn lt_mut<'l,'r,'s>(&'r mut self) -> &'r mut E::Storage<'s> where 's: 'r, 'l: 'r, 'l: 's, Self: 'l {
//         self.as_mut()
//     }
// }

impl<E> Widget<E> for Windows<E> where for<'a,'b> E: Env<RootRef<'a>=&'a ModelRoot,RootMut<'b>=&'b mut ModelRoot> {
    type Cache = GlobalCache<E>;

    #[inline]
    fn id(&self) -> guion::widget::id::WidgetID {
        todo!()
    }

    fn _render<P,Ph>(
        &mut self,
        path: &Ph,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        unimplemented!()
    }

    fn _event_direct<P,Ph,Evt>(
        &mut self,
        path: &Ph,
        stack: &P,
        event: &Evt, // TODO what if e.g. bounds change, if it's validated by parents then it's not signaled here
        route_to_widget: Option<&(dyn PathResolvusDyn<E>+'_)>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized, Evt: event_new::Event<E> + ?Sized
    {
        let mut passed = Invalidation::valid();

        let route_to_window = match route_to_widget {
            // Event sent to this widget itself
            Some(v) if v.inner().is_none() => None,
            // Event sent with path to specific child
            Some(v) => if let Some(v) = v.try_fragment::<FixedIdx>() {
                if v.0 < self.windows.len() as isize {
                    Some(v.0)
                } else {
                    panic!("Misrouted Window")
                }
            } else {
                panic!("Misrouted Window")
            },
            // Event sent without path filter
            None => None,
        };

        for i in self.childs() {
            if let Some(v) = route_to_window {
                if v != i {
                    continue;
                }
            }

            let stack = WithCurrentBounds {
                inner: stack,
                bounds: Bounds::from_size(self.windows[i as usize].dims),
                viewport: Bounds::from_size(self.windows[i as usize].dims),
            };

            //cache.cache[i].reset_current();

            let vali = self.windows[i as usize].widget.event_direct(
                &FixedIdx(i).push_on_stack(path),
                &stack,
                event,
                route_to_widget.and_then(PathResolvus::inner),
                root,ctx
            );

            self.windows[i as usize].vali |= vali;

            self.vali |= vali;
        }
        //eprintln!("e{}",passed);
        passed
    }

    fn _size<P,Ph>(
        &mut self,
        path: &Ph,
        stack: &P,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized {
        unimplemented!()
    }

    // fn with_child<'s,F,R>(
    //     &'s self,
    //     i: usize,
    //     mut callback: F,
    //     root: E::RootRef<'s>,
    //     ctx: &mut E::Context<'_>
    // ) -> R
    // where
    //     F: for<'w,'ww,'c,'cc> FnMut(Result<&'w (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> R
    // {
    //     self.windows[i].view(|child,root,ctx| {
    //         (callback)(Ok(child),ctx)
    //     },i,root,ctx)
    // }

    //TODO Widget::child_bounds isn't a thing in the new render/layout/caching concept
    // fn child_bounds<P>(&self, stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where P: Queron<E> + ?Sized {
    //     Ok(self.windows.iter()
    //         .map(|r| Bounds::from_size(todo!()) )
    //         .collect::<Vec<_>>())
    // }

    // fn with_resolve_child<'s,F,R>(
    //     &'s self,
    //     sub_path: &(dyn PathResolvusDyn<E>+'_),
    //     mut callback: F,
    //     root: E::RootRef<'s>,
    //     ctx: &mut E::Context<'_>
    // ) -> R
    // where
    //     F: for<'w,'c,'cc> FnMut(Result<WidgetWithResolveChildDyn<'w,E>,E::Error>,&'c mut E::Context<'cc>) -> R
    // {
    //     if let Some(idx) = sub_path.try_fragment::<FixedIdx>() {
    //         if let Some(v) = self.windows.get(idx.0) {
    //             return v.view(#[inline] |child,root,ctx| {
    //                 (callback)(
    //                     Ok(WidgetWithResolveChildDyn {
    //                         idx: 0,
    //                         sub_path: sub_path.inner().unwrap(),
    //                         widget: child.erase(),
    //                     }),
    //                     ctx,
    //                 )
    //             },idx.0,root,ctx);
    //         }
    //     }
    //     (callback)(Err(todo!()),ctx)
    // }

    fn update<Ph>(
        &mut self,
        path: &Ph,
        route: guion::widget_decl::route::UpdateRoute<'_,E>,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Invalidation where Ph: PathStack<E> + ?Sized {
        if let Some(resolve) = route.resolving() {
            if let Some(r2) = resolve.try_fragment::<FixedIdx>() {
                if let Some(akw) = self.windows.get_mut(r2.0 as usize) {
                    let vali = akw.widget.update(&r2.push_on_stack(path), route.for_child_1(), root, ctx);
                    akw.vali |= vali;
                    self.vali |= vali;
                    return vali;
                }
            }
            return Invalidation::new();
        }
        
        let mut vali = Invalidation::valid();

        for (idx,w) in self.windows.iter_mut().enumerate() {
            let v = w.widget.update(&FixedIdx(idx as isize).push_on_stack(path), route.for_child_1(), root, ctx);
            w.vali |= v;
            vali |= v;
        }

        vali
    }

    fn childs(&self) -> Range<isize> {
        0 .. self.windows.len() as isize
    }

    fn child_dyn(&self, idx: isize) -> Option<WidgetChildDynResult<'_,E>> {
        self.windows.get(idx as usize).map(|w| WidgetChildDynResult {
            idx,
            widget_id: w.widget.id(),
            widget: &w.widget,
        })
    }

    fn child_dyn_mut(&mut self, idx: isize) -> Option<WidgetChildDynResultMut<'_,E>> {
        self.windows.get_mut(idx as usize).map(|w| WidgetChildDynResultMut {
            idx,
            widget_id: w.widget.id(),
            widget: &mut w.widget,
        })
    }

    fn childs_dyn<'a,F>(&'a self, range: Range<isize>, mut callback: F) where F: FnMut(WidgetChildDynResult<'a,E>) {
        todo!()
    }

    fn childs_dyn_mut<'a,F>(&'a mut self, range: Range<isize>, mut callback: F) where F: FnMut(WidgetChildDynResultMut<'a,E>) {
        todo!()
    }

    fn resolve_child_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResult<'a,'b,E>> {
        if let Some(akw) = path.try_fragment::<FixedIdx>() {
            if let Some(w) = self.windows.get(akw.0 as usize) {
                return Some(WidgetChildResolveDynResult {
                    idx: akw.0,
                    sub_path: path.inner().unwrap(),
                    widget_id: w.widget.id(),
                    widget: &w.widget,
                });
            }
        }
        None
    }

    fn resolve_child_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<WidgetChildResolveDynResultMut<'a,'b,E>> {
        if let Some(akw) = path.try_fragment::<FixedIdx>() {
            if let Some(w) = self.windows.get_mut(akw.0 as usize) {
                return Some(WidgetChildResolveDynResultMut {
                    idx: akw.0,
                    sub_path: path.inner().unwrap(),
                    widget_id: w.widget.id(),
                    widget: &mut w.widget,
                });
            }
        }
        None
    }

    fn send_mutation<Ph>(
        &mut self,
        path: &Ph,
        resolve: &(dyn PathResolvusDyn<E>+'_),
        args: &dyn std::any::Any,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>,
    ) where Ph: PathStack<E> + ?Sized {
        if let Some(akw) = resolve.try_fragment::<FixedIdx>() {
            if let Some(w) = self.windows.get_mut(akw.0 as usize) {
                w.widget.send_mutation(&akw.push_on_stack(path), resolve.inner().unwrap(), args, root, ctx);
            }
        }
    }

    fn focusable(&self) -> bool {
        false
    }

    fn _call_tabulate_on_child_idx<P,Ph>(
        &self,
        idx: isize,
        path: &Ph,
        stack: &P,
        op: TabulateOrigin<E>,
        dir: TabulateDirection,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> Result<TabulateResponse<E>,E::Error>
    where 
        Ph: PathStack<E> + ?Sized, P: Queron<E> + ?Sized
    {
        if let Some(v) = self.windows.get(idx as usize) {
            return v.widget._tabulate(&FixedIdx(idx).push_on_stack(path),stack,op.clone(),dir,root,ctx);
        }
        Err(todo!())
    }

    fn invalidate_recursive(&mut self, vali: Invalidation) {
        todo!()
    }

    fn respond_query<'a>(&'a self, _: WQueryResponder<'_,'a,E>) {}

    fn respond_query_mut<'a>(&'a mut self, responder: WQueryResponder<'_,'a,E>) {}
}

// impl<E> AsWidget<E> for Windows<E> where E: Env {
//     type Widget = Self;
//     type WidgetOwned = Self;

//     #[inline]
//     fn as_widget<'w>(&'w self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
//         WCow::Borrowed(self)
//     }
//     #[inline]
//     fn into_widget<'w>(self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
//         WCow::Owned(self)
//     }
//     #[inline]
//     fn box_into_widget<'w>(self: Box<Self>, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
//         WCow::Owned(*self)
//     }
//     #[inline]
//     fn as_widget_dyn<'w,'s>(&'w self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
//         WCow::Borrowed(self)
//     }
//     #[inline]
//     fn into_widget_dyn<'w,'s>(self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> DynWCow<'w,E> where Self: Sized + 'w {
//         WCow::Owned(Box::new(self))
//     }
//     #[inline]
//     fn box_into_widget_dyn<'w,'s>(self: Box<Self>, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
//         WCow::Owned(self)
//     }
// }

impl<E> AsRefMut<Self> for Windows<E> where E: Env {
    fn as_ref(&self) -> &Self {
        self
    }

    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

pub struct GlobalCache<E> where E: Env {
    pub cache: Vec<DynWidgetCache<E>>,
}

impl<E> WidgetCache<E> for GlobalCache<E> where E: Env {
    
}

impl<E> Default for GlobalCache<E> where E: Env {
    fn default() -> Self {
        unimplemented!()
    }
}
