use druid_shell::{WindowHandle, piet};
use druid_shell::kurbo::{Rect, Point};
use druid_shell::piet::{Piet, RenderContext};
use guion::aliases::{EStyle, ESSelector, ESCursor, ERenderer};
use guion::backend::Backend;
use guion::env::Env;
use guion::util::AsRefMut;
use guion::util::bounds::{Bounds, Offset};

use crate::style::cursor::IntoGuionDruidShellCursor;

pub mod imp;

pub struct Render<'s,E> where E: Env {
    piet: PietRef<'s>,
    window_handle: &'s mut WindowHandle,
    pub next_cursor: &'s mut Option<ESCursor<E>>,
    //pub bounds: Bounds,
    //pub viewport: Bounds,
    //pub style: EStyle<E>,
    //pub selector: ESSelector<E>,
    pub force: bool,
}

impl<'s,E> Render<'s,E> where E: Env, ESCursor<E>: IntoGuionDruidShellCursor<E> {
    pub fn new<'b>(window_handle: &'s mut WindowHandle, piet: &'s mut Piet<'b>, next_cursor: &'s mut Option<ESCursor<E>>, dim: (u32,u32)) -> Self where 'b: 's {
        Self {
            piet: PietRef::new(piet),
            // bounds: Bounds::from_xywh(0,0,dim.0,dim.1),
            // viewport: Bounds::from_xywh(0,0,dim.0,dim.1),
            // style: Default::default(),
            // selector: Default::default(),
            next_cursor,
            force: false,
            window_handle,
        }
    }

    pub(crate) fn pre(&mut self) {
        let r = unsafe{self.piet.get()};
        r.save().unwrap();
    }
    pub(crate) fn post(&mut self) {
        let c = self.next_cursor.take().unwrap_or_default().into_druid_shell_cursor();
        self.window_handle.set_cursor(&c);
        let r = unsafe{self.piet.get()};
        r.restore().unwrap();
        while let Err(e) = r.status() {
            eprintln!("Render Error: {}",e);
        }
    }

    pub(crate) fn fork<'y,'z>(&'y mut self) -> Render<'z,E> where 's: 'z, 's: 'y, 'y: 'z {
        Render {
            piet: self.piet.fork(),
            window_handle: self.window_handle,
            // bounds: self.bounds.clone(),
            // viewport: self.viewport.clone(),
            // style: self.style.clone(),
            // selector: self.selector.clone(),
            next_cursor: self.next_cursor,
            force: self.force,
        }
    }
}

pub struct PietRef<'s> {
    piet: &'s mut Piet<'static>,
}

impl<'s> PietRef<'s> {
    pub fn new<'l>(piet: &'s mut Piet<'l>) -> Self where 'l: 's {
        Self{piet: unsafe{&mut *std::mem::transmute::<_,*mut Piet<'static>>(piet as &mut Piet<'l> as *mut Piet<'l>)}}
    }

    pub unsafe fn get(&mut self) -> &mut Piet<'static> {
        self.piet
    }

    pub fn fork<'z>(&'z mut self) -> PietRef<'z> where 's: 'z {
        PietRef {
            piet: self.piet,
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

pub(crate) fn rect2bounds(r: Rect) -> Bounds {
    Bounds::from_xyxy(r.x0 as i32, r.y0 as i32, r.x1 as i32, r.y1 as i32)
}

pub(crate) fn offset2point(o: Offset) -> Point {
    Point{
        x: o.x as f64,
        y: o.y as f64,
    }
}

pub(crate) fn color2color(c: impl guion::style::color::Color) -> piet::Color {
    let [r,g,b,a] = c.to_rgba8();
    piet::Color::rgba8(r,g,b,a)
}

impl<'s,E> AsRefMut<Render<'s,E>> for Render<'s,E> where E: Env {
    fn as_ref(&self) -> &Render<'s,E> {
        self
    }

    fn as_mut(&mut self) -> &mut Render<'s,E> {
        self
    }
}

pub fn kursize(w: f64, h: f64) -> druid_shell::kurbo::Size {
    druid_shell::kurbo::Size::new(w,h)
}
