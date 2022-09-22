use druid_shell::piet::{CairoTextLayout, RenderContext};
use guion::aliases::{ERenderer, ETextLayout, ESCursor};
use guion::backend::Backend;
use guion::env::Env;
use guion::queron::Queron;
use guion::queron::query::Query;
use guion::render::{Render as GRender, QueryTestStyleCurrent};
use guion::render::widgets::RenderStdWidgets;
use guion::style::{Style,selector::StyleSelector};
use guion::util::AsRefMut;
use guion::widget::stack::QueryCurrentBounds;

use crate::style::cursor::IntoGuionDruidShellCursor;

use super::{Render, bounds2rect, color2color, offset2point};

impl<'s,E> GRender<E> for Render<'s,E> where
    E: Env,
    for<'a> E::Backend: Backend<E,Renderer<'a>=Render<'a,E>>,
    ESCursor<E>: IntoGuionDruidShellCursor<E>,
{
    // fn force(&self) -> bool {
    //     self.force
    // }

    // fn validate_widgets(&mut self) -> bool {
    //     todo!()//true
    // }

    // fn lt<'r>(self) -> ERenderer<'r,E> where Self: 'r {
    //     self
    // }

    // fn with_force<'r>(&'r mut self, force: bool) -> ERenderer<'r,E> where Self: 'r {
    //     Render{
    //         force,
    //         ..self.fork()
    //     }
    // }

    // fn inside_border_specific<'r>(&'r mut self, s: &guion::util::border::Border) -> ERenderer<'r,E> where Self: 'r {
    //     Render{
    //         bounds: self.bounds.inside_border(s),
    //         ..self.fork()
    //     }
    // }

    // fn slice<'r>(&'r mut self, s: &guion::util::bounds::Bounds) -> ERenderer<'r,E> where Self: 'r {
    //     Render{
    //         bounds: self.bounds.slice(s),
    //         ..self.fork()
    //     }
    // }

    // fn slice_abs<'r>(&'r mut self, s: &guion::util::bounds::Bounds) -> ERenderer<'r,E> where Self: 'r {
    //     Render{
    //         bounds: self.bounds & s,
    //         ..self.fork()
    //     }
    // }

    // fn inner_centered<'r>(&'r mut self, size: guion::util::bounds::Dims) -> ERenderer<'r,E> where Self: 'r {
    //     Render{
    //         bounds: self.bounds.inner_centered(size),
    //         ..self.fork()
    //     }
    // }

    // fn inner_aligned<'r>(&'r mut self, size: guion::util::bounds::Dims, align: (f32,f32)) -> ERenderer<'r,E> where Self: 'r {
    //     Render{
    //         bounds: self.bounds.inner_aligned(size,align),
    //         ..self.fork()
    //     }
    // }

    // fn with_style<'r>(&'r mut self, style: &guion::aliases::EStyle<E>) -> ERenderer<'r,E> {
    //     Render{
    //         style: self.style.and(style),
    //         ..self.fork()
    //     }
    // }

    // fn with_style_selector<'r>(&'r mut self, style_selector: &guion::aliases::ESSelector<E>) -> ERenderer<'r,E> {
    //     Render{
    //         selector: self.selector.and(style_selector),
    //         ..self.fork()
    //     }
    // }

    // fn with_bounds<'r>(&'r mut self, bounds: guion::util::bounds::Bounds) -> ERenderer<'r,E> {
    //     Render{
    //         bounds,
    //         ..self.fork()
    //     }
    // }

    // fn with_viewport<'r>(&'r mut self, viewport: guion::util::bounds::Bounds) -> ERenderer<'r,E> {
    //     Render{
    //         viewport,
    //         ..self.fork()
    //     }
    // }

    // fn bounds(&self) -> &guion::util::bounds::Bounds {
    //     &self.bounds
    // }

    // fn viewport(&self) -> &guion::util::bounds::Bounds {
    //     &self.viewport
    // }

    // fn style(&self) -> &guion::aliases::EStyle<E> {
    //     &self.style
    // }

    // fn selector(&self) -> &guion::aliases::ESSelector<E> {
    //     &self.selector
    // }

    // fn render_widget(&mut self, mut w: guion::widget::link::Link<E>) {
    //     w.render(self)
    // }

    // fn fork_with<'r>(&'r mut self, bounds: Option<guion::util::bounds::Bounds>, viewport: Option<guion::util::bounds::Bounds>, style: Option<guion::aliases::EStyle<E>>, selector: Option<guion::aliases::ESSelector<E>>) -> ERenderer<'r,E> {
    //     Render{
    //         bounds: bounds.unwrap_or(self.bounds.clone()),
    //         viewport: viewport.unwrap_or(self.viewport.clone()),
    //         style: style.unwrap_or(self.style.clone()),
    //         selector: selector.unwrap_or(self.selector.clone()),
    //         ..self.fork()
    //     }
    // }
}

impl<'r,E> RenderStdWidgets<E> for Render<'r,E> where
    E: Env + Sync,
    //for<'a> ERenderer<'a,E>: AsRefMut<Render<'a,E>>,
    ETextLayout<E>: AsRefMut<CairoTextLayout>, //TODO use Piet trait variant
    ESCursor<E>: IntoGuionDruidShellCursor<E>,
    //for<'a> ERenderer<'a,E>: RenderStdWidgets<E>+'a,
    for<'a> E::Backend: Backend<E,Renderer<'a>=Render<'a,E>>,
{
    fn fill_rect<Q>(&mut self, props: &Q, c: &mut E::Context<'_>) where Q: Queron<E> + ?Sized {
        let bounds = QueryCurrentBounds.query_in(props).unwrap();
        let style = QueryTestStyleCurrent.query_in(props).unwrap();

        let r = unsafe{self.piet.get()};
        let rect = bounds2rect(*bounds.bounds);
        let color = style.current_color;

        let brush = r.solid_brush(color2color(color)); //TODO IntoPietColor trait
        r.fill(rect,&brush); //TODO clip to viewport
    }

    fn fill_border_inner<Q>(&mut self, props: &Q, c: &mut E::Context<'_>) where Q: Queron<E> + ?Sized {
        let bounds = QueryCurrentBounds.query_in(props).unwrap();
        let style = QueryTestStyleCurrent.query_in(props).unwrap();

        let r = unsafe{self.piet.get()};
        let rect = bounds2rect(*bounds.bounds);
        let color = style.current_color;
        let thickness = style.current_border.top;

        if thickness == 0 {return;}

        let brush = r.solid_brush(color2color(color));
        r.stroke(rect,&brush,thickness as f64); //TODO guion thickness goes to inside, how does piet?
    }

    /*#[inline]
    fn render_text(&mut self, b: &Bounds, text: &str, align: (f32,f32), style: &EStyle<E>, variant: &EStyle<E>, ctx: &mut E::Context<'_>) {
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

    fn render_preprocessed_text<Q>(&mut self, text: &ETextLayout<E>, inner_offset: guion::util::bounds::Offset, props: &Q, c: &mut E::Context<'_>) where Q: Queron<E> + ?Sized {
        let bounds = QueryCurrentBounds.query_in(props).unwrap();
        let style = QueryTestStyleCurrent.query_in(props).unwrap();

        let r = unsafe{self.piet.get()};
        let rect = bounds2rect(*bounds.bounds);
        let color = style.text_color;

        r.with_save(|r| {
            r.clip(rect);
            r.draw_text(text.as_ref(), (rect.origin() - offset2point(inner_offset)).to_point() );
            Ok(())
        }).unwrap();
    }

    fn set_cursor_specific(&mut self, cursor: &ESCursor<E>, c: &mut E::Context<'_>) {
        *self.next_cursor = Some(cursor.clone());
    }
}
