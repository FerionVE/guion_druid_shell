use guion::aliases::{ESize, EStyle, ERenderer};
use guion::ctx::Context;
use guion::env::Env;
use guion::path::WidgetPath;
use guion::queron::Queron;
use guion::root::{RootRef, RootMut};
use guion::util::AsRefMut;
use guion::util::bounds::Bounds;
use guion::widget::cache::{DynWidgetCache, WidgetCache};
use guion::widget::dyn_tunnel::WidgetDyn;
use guion::widget::stack::{WithCurrentWidget, WithCurrentBounds};
use guion::widget::{Widget};

use super::window::Window;

pub struct Windows<E> where E: Env {
    pub windows: Vec<Window<E>>,
    pub _id: E::WidgetID, //TODO not required when WidgetIdent is gone
}

// impl WidgetRoot

impl<E> Windows<E> where for<'a,'b> E: Env<RootRef<'a>=&'a Windows<E>,RootMut<'b>=&'b mut Windows<E>> {
    pub(crate) fn path_of_window(&self, window: usize, ctx: &mut E::Context<'_>) -> E::WidgetPath {
        //TODO isn't it stupid that we actually need to view it to get the path of the widget? It's time for a new path system
        self.windows[window].view(|w,_,_| w.in_parent_path(WidgetPath::empty()) ,window,self,ctx) //TODO empty default constructor for path
    }

    pub fn with_window_by_path<'s,'l:'s,F,R>(
        &'s self,
        i: E::WidgetPath,
        callback: F,
        ctx: &mut E::Context<'_>,
    ) -> R
    where 
        F: for<'w,'ww,'c,'cc> FnOnce(Result<&'w (dyn WidgetDyn<E>+'ww),()>,usize,&'c mut E::Context<'cc>) -> R,
        Self: 'l
    {
        for c in 0..self.childs() {
            if let Some(r) = self.with_child(
                c, 
                #[inline] |w,_| w.unwrap().resolved_by_path(&i),
                self.fork(), ctx,
            ) {
                return self.with_child(
                    c,
                    #[inline] |child,ctx| (callback)(child,c,ctx),
                    self.fork(), ctx,
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

impl<'g,E> RootRef<E> for &'g Windows<E> where for<'a,'b> E: Env<RootRef<'a>=&'a Windows<E>,RootMut<'b>=&'b mut Windows<E>> {
    fn fork<'s,'w:'s>(&'s self) -> <E as Env>::RootRef<'w> where Self: 'w {
        self
    }

    fn with_widget<'s,'l:'s,F,R>(
        &'s self,
        i: E::WidgetPath,
        callback: F,
        ctx: &mut E::Context<'_>,
    ) -> R
    where 
        F: for<'w,'ww,'c,'cc> FnOnce(Result<&'w (dyn WidgetDyn<E>+'ww),E::Error>,&'c mut E::Context<'cc>) -> R,
        Self: 'l
    {
        self.with_resolve(
            i,
            callback,
            self, ctx
        )
    }

    fn trace_bounds(&self, ctx: &mut <E as Env>::Context<'_>, i: <E as Env>::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,<E as Env>::Error> {
        todo!()
    }
}
impl<'g,E> RootMut<E> for &'g mut Windows<E> where for<'a,'b> E: Env<RootRef<'a>=&'a Windows<E>,RootMut<'b>=&'b mut Windows<E>> {
    fn fork<'s>(&'s mut self) -> <E as Env>::RootMut<'s> where Self: 's {
        self
    }
}

// impl<E> Widgets<E> for Windows<E> where for<'a,'b> E: Env<RootRef<'a>=&'a Windows<E>,RootMut<'b>=&'b mut Windows<E>> {
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

impl<E> Widget<E> for Windows<E> where for<'a,'b> E: Env<RootRef<'a>=&'a Self,RootMut<'b>=&'b mut Self> {
    type Cache = GlobalCache<E>;

    fn id(&self) -> E::WidgetID {
        self._id.clone()
    }

    fn _render<P>(
        &self,
        stack: &P,
        renderer: &mut ERenderer<'_,E>,
        force_render: bool,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) where P: Queron<E> + ?Sized {
        unimplemented!()
    }

    fn _event_direct<P,Evt>(
        &self,
        stack: &P,
        event: &Evt,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> guion::EventResp
    where
        P: Queron<E> + ?Sized, Evt: guion::event_new::Event<E> + ?Sized
    {
        let mut passed = false;

        for i in 0..self.childs() {
            self.windows[i].view(|child,root,ctx| {
                let stack = WithCurrentBounds {
                    inner: WithCurrentWidget {
                        inner: stack,
                        path: child.in_parent_path(WidgetPath::empty()),
                        id: child.id(),
                    },
                    bounds: Bounds::from_size(self.windows[i].dims),
                    viewport: Bounds::from_size(self.windows[i].dims),
                };

                cache.cache[i].reset_current();

                passed |= child.event_direct(&stack,event,&mut cache.cache[i], root,ctx);
            },i,root,ctx)
        }
        //eprintln!("e{}",passed);
        passed
    }

    fn _size<P>(
        &self,
        stack: &P,
        cache: &mut Self::Cache,
        root: E::RootRef<'_>,
        ctx: &mut E::Context<'_>
    ) -> ESize<E> where P: Queron<E> + ?Sized {
        unimplemented!()
    }

    fn childs(&self) -> usize {
        self.windows.len()
    }

    fn with_child<'s,F,R>(
        &'s self,
        i: usize,
        callback: F,
        root: E::RootRef<'s>,
        ctx: &mut E::Context<'_>
    ) -> R
    where
        F: for<'w,'ww,'c,'cc> FnOnce(Result<&'w (dyn WidgetDyn<E>+'ww),()>,&'c mut E::Context<'cc>) -> R
    {
        self.windows[i].view(|child,root,ctx| {
            (callback)(Ok(child),ctx)
        },i,root,ctx)
    }

    //TODO Widget::child_bounds isn't a thing in the new render/layout/caching concept
    fn child_bounds<P>(&self, stack: &P, b: &Bounds, force: bool, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<Vec<Bounds>,()> where P: Queron<E> + ?Sized {
        Ok(self.windows.iter()
            .map(|r| Bounds::from_size(todo!()) )
            .collect::<Vec<_>>())
    }

    fn focusable(&self) -> bool {
        false
    }
}

// impl<E> AsWidget<E> for Windows<E> where E: Env {
//     type Widget = Self;
//     type WidgetOwned = Self;

//     #[inline]
//     fn as_widget<'w>(&'w self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
//         WCow::Borrowed(self)
//     }
//     #[inline]
//     fn into_widget<'w>(self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
//         WCow::Owned(self)
//     }
//     #[inline]
//     fn box_into_widget<'w>(self: Box<Self>, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
//         WCow::Owned(*self)
//     }
//     #[inline]
//     fn as_widget_dyn<'w,'s>(&'w self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
//         WCow::Borrowed(self)
//     }
//     #[inline]
//     fn into_widget_dyn<'w,'s>(self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: Sized + 'w {
//         WCow::Owned(Box::new(self))
//     }
//     #[inline]
//     fn box_into_widget_dyn<'w,'s>(self: Box<Self>, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
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
    fn reset_current(&mut self) {
        for c in &mut self.cache {
            c.reset_current();
        }
    }
}

impl<E> Default for GlobalCache<E> where E: Env {
    fn default() -> Self {
        unimplemented!()
    }
}
