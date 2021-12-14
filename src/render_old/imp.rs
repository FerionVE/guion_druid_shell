use druid_shell::piet::{CairoTextLayout, RenderContext};
use guion::aliases::*;
use guion::env::Env;
use guion::render::widgets::RenderStdWidgets;
use guion::style::Style;
use guion::util::AsRefMut;
use guion::render::Render as GRender;
use guion::util::bounds::{Bounds, Offset};

use super::*;

macro_rules! unsafe_piet {
    ($s:ident) => {unsafe{
        Render::<E>::piet_ref(&mut $s.piet)
    }}
}

impl<E> GRender<E> for Render<E> where
    E: Env,
    for<'a> ERenderer<'a,E>: AsRefMut<Self>,
    ESCursor<E>: Into<StdCursor>,  //TODO Into<DruidCursor>
{
    #[inline]
    fn _style(&self) -> &EStyle<E> {
        &self.live_style
    }
    #[inline]
    fn _selector(&self) -> &ESSelector<E> {
        &self.live_selector
    }
    #[inline]
    fn _bounds(&self) -> &Bounds {
        &self.live_bounds
    }
    #[inline]
    fn _viewport(&self) -> &Bounds {
        &self.live_viewport
    }
    #[inline(never)]
    fn _set_style(&mut self, v: &EStyle<E>) {
        self.live_style = v.clone();
    }
    #[inline(never)]
    fn _set_selector(&mut self, v: &ESSelector<E>) {
        self.live_selector = v.clone();
    }
    #[inline(never)]
    fn _set_bounds(&mut self, v: &Bounds) {
        self.live_bounds = v.clone();
    }
    #[inline(never)]
    fn _set_viewport(&mut self, v: &Bounds) {
        self.live_viewport = v.clone();
        //let r = &mut self.windows[self.current];
        //r.set_viewport(to_rect(&self.live_viewport));
        let r = unsafe{self.piet()};
        r.restore().unwrap();
        r.save().unwrap();
        r.clip(bounds2rect(v.clone()));
    }
}

impl<E> RenderStdWidgets<E> for Render<E> where
    E: Env + Sync,
    ERenderer<E>: AsRefMut<Self>,
    ETextLayout<E>: AsRefMut<CairoTextLayout>, //TODO use Piet trait variant
    ESCursor<E>: Into<StdCursor>,  //TODO Into<DruidCursor>
{
    #[inline]
    fn fill_rect(&mut self, c: &mut E::Context) {
        let r = unsafe_piet!(self);
        let rect = bounds2rect(self.live_bounds);
        let color = self.live_style.color(&self.live_selector,c);

        let brush = r.solid_brush(color2color(color));
        r.fill(rect,&brush);
    }
    #[inline]
    fn fill_border_inner(&mut self, c: &mut E::Context) {
        let r = unsafe_piet!(self);
        let rect = bounds2rect(self.live_bounds);
        let color = self.live_style.color(&self.live_selector,c);
        let thickness = self.live_style.border(&self.live_selector,c).top;

        if thickness == 0 {return;}

        let brush = r.solid_brush(color2color(color));
        r.stroke(rect,&brush,thickness as f64); //TODO guion thickness goes to inside, how does piet?
    }
    /*#[inline]
    fn render_text(&mut self, b: &Bounds, text: &str, align: (f32,f32), style: &EStyle<E>, variant: &EStyle<E>, ctx: &mut E::Context) {
        let (glyphs,bounds) = 
            glyphs_of_str(&ctx.as_ref().font,Scale::uniform(24.0), std::i32::MAX as u32, text);
        
        let b = b.inner_aligned_f((bounds.x,bounds.y),align);

        if b.not_empty() {
            //self.c.set_draw_color(SDLColor::RGBA(255, 0, 0, 255));
            //self.c.fill_rect(to_rect(&b)).expect("SDL Render Failure @ fill_rect");
            //self.c.set_blend_mode(BlendMode::Blend);
            let color = style.color(variant);
            self.render_glyphs(b, Offset::default(), color.into().v, glyphs.into_iter()).expect("TTOOF");
        }
    }*/
    #[inline]
    fn render_preprocessed_text(&mut self, text: &ETextLayout<E>, inner_offset: Offset, c: &mut E::Context) {
        let r = unsafe_piet!(self);
        let rect = bounds2rect(self.live_bounds);
        let color = self.live_style.color(&self.live_selector,c);

        r.with_save(|r| {
            r.clip(rect);
            r.draw_text(text.as_ref(), (rect.origin() - offset2point(inner_offset)).to_point() );
            Ok(())
        }).unwrap();
    }
    #[inline]
    fn set_cursor(&mut self, c: &mut E::Context) {
        let cursor = self.live_style.cursor(&self.live_selector,c);
        self.set_cursor_specific(&cursor,c);
    }

    fn set_cursor_specific(&mut self, cursor: &ESCursor<E>, _: &mut E::Context) {
        self.next_cursor = Some(cursor.clone());
    }
}
