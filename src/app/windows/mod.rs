use guion::aliases::{ESize, EStyle};
use guion::ctx::Context;
use guion::env::Env;
use guion::event::compound::EventCompound;
use guion::id::standard::StdID;
use guion::path::WidgetPath;
use guion::render::link::RenderLink;
use guion::util::AsRefMut;
use guion::util::bounds::Bounds;
use guion::util::error::GuionError;
use guion::widget::link::Link;
use guion::widget::resolvable::ResolvableMut;
use guion::widget::{WBaseMut, Widget, WidgetMut};
use guion::widget::resolved::{Resolved, ResolvedMut};
use guion::widget::root::{Widgets, resolve_in_root, resolve_in_root_mut};

use super::window::Window;

pub struct Windows<E> where E: Env {
    pub windows: Vec<Window<E>>,
    pub _id: E::WidgetID, //TODO not required when WidgetIdent is gone
}

// impl WidgetRoot

impl<E> Windows<E> where E: Env, E::Storage: AsRefMut<Self>, Self: AsRefMut<E::Storage> {
    pub(crate) fn path_of_window(&self, window: usize) -> E::WidgetPath {
        (*self.windows[window].widget).as_ref().in_parent_path(WidgetPath::empty(),true) //TODO empty default constructor for path
    }

    pub fn resolved(&self) -> Resolved<E> {
        Resolved{
            wref: Box::new(self as &dyn Widget<E>),
            path: WidgetPath::empty(),
            direct_path: WidgetPath::empty(),
            stor: self.as_ref(),
        }
    }
    pub fn resolved_mut(&mut self) -> ResolvedMut<E> {
        ResolvedMut{
            wref: Box::new(self as &mut dyn WidgetMut<E>),
            path: WidgetPath::empty(),
            direct_path: WidgetPath::empty(),
        }
    }
}

impl<E> Widgets<E> for Windows<E> where E: Env, E::Storage: AsRefMut<Self>, Self: AsRefMut<E::Storage> {
    fn widget(&self, i: E::WidgetPath) -> Result<Resolved<E>,E::Error> {
        resolve_in_root(
            self,
            i.clone(),
            i,
            self.as_ref()
        )
    }

    fn widget_mut(&mut self, i: E::WidgetPath) -> Result<ResolvedMut<E>,E::Error> {
        resolve_in_root_mut(
            self.as_mut(),
            |s| AsRefMut::<Self>::as_mut(s) as &mut dyn WidgetMut<_>,
            i.clone(),
            i
        )
    }

    fn trace_bounds(&self, ctx: &mut E::Context, i: E::WidgetPath, b: &Bounds, e: &EStyle<E>, force: bool) -> Result<Bounds,E::Error> {
        let l = ctx.link(Resolved{
            wref: Box::new(self.base()),
            path: WidgetPath::empty(),
            direct_path: WidgetPath::empty(),
            stor: self.as_ref(),
        });
        Widget::trace_bounds(self,l,i,b,e,force)
    }
}

impl<E> Widget<E> for Windows<E> where E: Env {
    fn id(&self) -> E::WidgetID {
        self._id.clone()
    }

    fn _render(&self, l: Link<E>, r: &mut RenderLink<E>) {
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

    fn _size(&self, l: Link<E>, e: &EStyle<E>) -> ESize<E> {
        unimplemented!()
    }

    fn childs(&self) -> usize {
        self.windows.len()
    }

    fn child<'s>(&'s self, i: usize) -> Result<guion::widget::resolvable::Resolvable<'s,E>,()> {
        self.windows.get(i)
            .map(|w| (*w.widget).as_ref() )
            .ok_or(())
    }

    fn into_child<'s>(self: Box<Self>, i: usize) -> Result<guion::widget::resolvable::Resolvable<'s,E>,()> where Self: 's {
        unimplemented!()
    }

    fn into_childs<'w>(self: Box<Self>) -> Vec<guion::widget::resolvable::Resolvable<'w,E>> where Self: 'w {
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

impl<E> WidgetMut<E> for Windows<E> where E: Env {
    fn child_mut(&mut self, i: usize) -> Result<ResolvableMut<E>,()> {
        self.windows.get_mut(i)
            .map(|w| (*w.widget).as_mut() )
            .ok_or(())
    }
    fn into_child_mut<'w>(self: Box<Self>, i: usize) -> Result<ResolvableMut<'w,E>,()> where Self: 'w {
        unimplemented!()
    }
    fn childs_mut(&mut self) -> Vec<ResolvableMut<E>> {
        unimplemented!()
    }
    fn into_childs_mut<'w>(self: Box<Self>) -> Vec<ResolvableMut<'w,E>> where Self: 'w {
        unimplemented!()
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
