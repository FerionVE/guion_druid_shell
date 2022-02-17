use guion::aliases::{ESize, EStyle, ERenderer, WidgetRef};
use guion::ctx::Context;
use guion::env::Env;
use guion::event::compound::EventCompound;
use guion::path::WidgetPath;
use guion::root::{RootRef, RootMut};
use guion::util::AsRefMut;
use guion::util::bounds::Bounds;
use guion::widget::as_widget::{WCow, AsWidget, DynWCow};
use guion::widget::link::Link;
use guion::widget::{Widget};
use guion::widget::resolved::{Resolved};

use super::window::Window;

pub struct Windows<E> where E: Env {
    pub windows: Vec<Window<E>>,
    pub _id: E::WidgetID, //TODO not required when WidgetIdent is gone
}

// impl WidgetRoot

impl<E> Windows<E> where for<'a,'b> E: Env<RootRef<'a>=&'a Windows<E>,RootMut<'b>=&'b mut Windows<E>> {
    pub(crate) fn path_of_window(&self, window: usize, ctx: &mut E::Context<'_>) -> E::WidgetPath {
        (*self.windows[window].widget).view(self,ctx).in_parent_path(WidgetPath::empty()) //TODO empty default constructor for path
    }

    pub fn resolved(&self) -> Resolved<E> {
        Resolved{
            wref: WCow::Borrowed(self as &dyn Widget<E>),
            path: WidgetPath::empty(),
            direct_path: WidgetPath::empty(),
            root: self,
        }
    }
}

impl<'g,E> RootRef<E> for &'g Windows<E> where for<'a,'b> E: Env<RootRef<'a>=&'a Windows<E>,RootMut<'b>=&'b mut Windows<E>> {
    fn fork<'s,'w:'s>(&'s self) -> <E as Env>::RootRef<'w> where Self: 'w {
        self
    }

    fn widget<'s,'w:'s>(&'s self, i: E::WidgetPath, ctx: &mut E::Context<'_>) -> Result<Resolved<'w,E>,E::Error> where Self: 'w {
        let wref = self.resolve(i.clone(),self,ctx)?;

        Ok(Resolved {
            wref,
            path: i.clone(),
            direct_path: i,
            root: self,
        })
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

impl<E> Widget<E> for Windows<E> where E: Env {
    fn id(&self) -> E::WidgetID {
        self._id.clone()
    }

    fn _render(&self, _: Link<E>, _: &mut ERenderer<'_,E>) {
        unimplemented!()
    }

    fn _event_direct(&self, mut l: Link<E>, e: &EventCompound<E>) -> guion::EventResp {
        let mut passed = false;

        for i in 0..self.childs() {
            let mut l = l.for_child(i).expect("Dead Path Inside Pane");
            let sliced = e.with_bounds( Bounds::from_size(self.windows[i].dims) );
            if let Some(ee) = sliced.filter(&l) {
                passed |= l.event_direct(&ee);
            }
        }
        //eprintln!("e{}",passed);
        passed
    }

    fn _size(&self, _: Link<E>, _: &EStyle<E>) -> ESize<E> {
        unimplemented!()
    }

    fn childs(&self) -> usize {
        self.windows.len()
    }

    fn child<'s>(&'s self, i: usize, root: E::RootRef<'s>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> {
        self.windows.get(i)
            .map(|w| WCow::Owned( (*w.widget).view(root,ctx) ) )
            .ok_or(())
    }

    fn into_child<'s>(self: Box<Self>, i: usize, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'s,E>,()> where Self: 's {
        unimplemented!()
    }

    fn into_childs<'s>(self: Box<Self>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Vec<WidgetRef<'s,E>> where Self: 's {
        unimplemented!()
    }

    fn child_bounds(&self, l: Link<E>, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Vec<Bounds>,()> {
        Ok(self.windows.iter()
            .map(|r| Bounds::from_size(todo!()) )
            .collect::<Vec<_>>())
    }

    fn focusable(&self) -> bool {
        false
    }
}

impl<E> AsWidget<E> for Windows<E> where E: Env {
    type Widget = Self;
    type WidgetOwned = Self;

    #[inline]
    fn as_widget<'w>(&'w self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        WCow::Borrowed(self)
    }
    #[inline]
    fn into_widget<'w>(self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
        WCow::Owned(self)
    }
    #[inline]
    fn box_into_widget<'w>(self: Box<Self>, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        WCow::Owned(*self)
    }
    #[inline]
    fn as_widget_dyn<'w,'s>(&'w self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        WCow::Borrowed(self)
    }
    #[inline]
    fn into_widget_dyn<'w,'s>(self, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: Sized + 'w {
        WCow::Owned(Box::new(self))
    }
    #[inline]
    fn box_into_widget_dyn<'w,'s>(self: Box<Self>, _: <E as Env>::RootRef<'_>, _: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        WCow::Owned(self)
    }
}

impl<E> AsRefMut<Self> for Windows<E> where E: Env {
    fn as_ref(&self) -> &Self {
        self
    }

    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

