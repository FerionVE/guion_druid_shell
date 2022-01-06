use druid_shell::WindowHandle;
use druid_shell::kurbo::{Point, Rect};
use druid_shell::piet::{self, Piet, RenderContext};
use guion::aliases::{ESCursor, ESSelector, EStyle};
use guion::env::Env;
use guion::style::standard::cursor::StdCursor;
use guion::util::AsRefMut;
use guion::util::bounds::{Bounds, Offset};

use crate::style::cursor::cursor2cursor;

pub mod imp;

pub struct Render<E> where E: Env {
    piet: *mut Piet<'static>,

    pub live_bounds: Bounds,
    pub live_viewport: Bounds,
    pub live_style: EStyle<E>,
    pub live_selector: ESSelector<E>,
    pub next_cursor: Option<ESCursor<E>>,

    window_handle: WindowHandle,
}

impl<E> Render<E> where
    E: Env,
    ESCursor<E>: IntoGuionDruidShellCursor<E>,
{
    pub(crate) fn scoped<'s,'l:'s,R>(window_handle: WindowHandle, piet: &'s mut Piet<'l>, f: impl FnOnce(&mut Self)->R) -> R {
        let p = piet as *mut Piet;
        let p = unsafe{std::mem::transmute::<*mut Piet<'l>,*mut Piet<'static>>(p)};
        let mut s = Self {
            piet:p,
            live_bounds: Default::default(),
            live_viewport: Default::default(),
            live_style: Default::default(),
            live_selector: Default::default(),
            next_cursor: None,
            window_handle,
        };
        f(&mut s)
    }
    /// The contravariance of the internal Piet lifetime is violated so additional care is required in calling piet fns
    unsafe fn piet<'s,'l:'s>(&'s mut self) -> &'s mut Piet<'l> {
        Self::piet_ref(&mut self.piet)
    }
    unsafe fn piet_ref<'s,'l:'s>(p: &'s mut *mut Piet<'static>) -> &'s mut Piet<'l> {
        let p = std::mem::transmute::<*mut Piet<'static>,*mut Piet<'l>>(*p);
        &mut *p
    }
    
    pub(crate) fn pre(&mut self) {
        let r = unsafe{self.piet()};
        r.save().unwrap();
    }
    pub(crate) fn post(&mut self) {
        let c = cursor2cursor(self.next_cursor.take().unwrap_or_default());
        self.window_handle.set_cursor(&c);
        let r = unsafe{self.piet()};
        r.restore().unwrap();
        while let Err(e) = r.status() {
            eprintln!("Render Error: {}",e);
        }
    }
}
