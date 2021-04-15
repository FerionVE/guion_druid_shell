use druid_shell::{Cursor, WindowHandle};
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
    ESCursor<E>: Into<StdCursor>,  //TODO Into<DruidCursor>
{
    pub(crate) fn inscope21<'s,'l:'s,R>(window_handle: WindowHandle, piet: &'s mut Piet<'l>, f: impl FnOnce(&mut Self)->R) -> R {
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
        Self::pietor(&mut self.piet)
    }
    unsafe fn pietor<'s,'l:'s>(p: &'s mut *mut Piet<'static>) -> &'s mut Piet<'l> {
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

pub(crate) fn bounds2rect(b: Bounds) -> Rect {
    Rect{
        x0: b.x() as f64,
        y0: b.y() as f64,
        x1: b.x1() as f64,
        y1: b.y1() as f64,
    }
}

pub(crate) fn offset2point(o: Offset) -> Point {
    Point{
        x: o.x as f64,
        y: o.y as f64,
    }
}

pub(crate) fn color2color(c: impl guion::style::color::Color) -> piet::Color {
    let [r,g,b,a] = c.into_rgba8();
    piet::Color::rgba8(r,g,b,a)
}

impl<E> AsRefMut<Self> for Render<E> where E: Env {
    fn as_ref(&self) -> &Self {
        self
    }

    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
