use druid_shell::piet::{Piet, CairoTextLayout};
use guion::aliases::{ECQueue, ERenderer, ESCursor, ETextLayout};
use guion::backend::Backend;
use guion::env::Env;
use guion::newpath::{FixedIdx, PathFragment};
use guion::pathslice::{NewPathStack, PathStackBase};
use guion::render::{Render as _, WithTestStyle, TestStyleColorType, StdRenderProps};
use guion::render::widgets::RenderStdWidgets;
use guion::style::selectag::standard::StdSelectag;
use guion::style::standard::cursor::StdCursor;
use guion::util::AsRefMut;
use guion::ctx::Context;
use guion::root::RootRef;
use guion::widget::Widget;
use guion::widget::cache::WidgetCache;
use guion::widget::dyn_tunnel::WidgetDyn;
use guion::widget::stack::{WithCurrentBounds};

use crate::render::Render;
use crate::style::cursor::IntoGuionDruidShellCursor;
use crate::style::stupid_test_style;

use super::{ArcApp, ksize2dims, ModelRoot};
use super::windows::Windows;


impl<E> ArcApp<E> where
for<'a,'b> E: Env<RootRef<'a>=&'a ModelRoot,RootMut<'b>=&'b mut ModelRoot>,
    //for<'a> ERenderer<'a,E>: RenderStdWidgets<E>,
    for<'a> E::Backend: Backend<E,Renderer<'a>=Render<'a,E>>,
    for<'a> Render<'a,E>: RenderStdWidgets<E>,
    for<'a> ECQueue<'a,E>: AsRefMut<crate::ctx::queue::Queue<E>>,
    ETextLayout<E>: AsRefMut<CairoTextLayout>, //TODO use Piet trait variant
    ESCursor<E>: IntoGuionDruidShellCursor<E>,
{
    pub(crate) fn render_pre(&self, window_id: usize) {
        //todo!()
    }
    pub(crate) fn render(&self, window_id: usize, render: &mut Piet, invalid_region: &druid_shell::Region) {
        eprintln!("Render {:?}",invalid_region);

        let mut s = self.inner.lock().unwrap();
        let s = &mut *s;
        let mut window_handle = s.windows.windows[window_id].handle.as_ref().unwrap().clone();

        if !s.windows.windows[window_id].vali.render {return;}

        eprintln!("real render");

        let path = s.windows.path_of_window(window_id,&mut s.ctx);
        let dims = window_handle.get_size();

        let mut next_cursor = None;

        let mut render = Render::<'_,E>::new(
            &mut window_handle,
            render,
            &mut next_cursor,
            (dims.width as u32, dims.height as u32),
        );

        let test_style = stupid_test_style();
        let props = WithTestStyle((),test_style);
        let props = WithCurrentBounds {
            inner: props,
            bounds: guion::util::bounds::Bounds::from_xywh(0,0,dims.width as u32,dims.height as u32),
            viewport: guion::util::bounds::Bounds::from_xywh(0,0,dims.width as u32,dims.height as u32),
        };

        //TODO reset cursor
        //TODO restore renderer
        render.pre();

        // visual caching debug
        render.fill_rect(&(TestStyleColorType::Custom(guion::style::color::Color::from_rgba8([0,0,0,10])) + &props), &mut s.ctx);

        //process queued and render
        render.force = false; //TODO force from piet backend

        let force = render.force;

        let mut pathstack = PathStackBase::new_desktop();
        let mut pathstack = pathstack.path_stack();

        let root: E::RootRef<'_> = &s.models;
        s.windows.with_window_by_path_mut(
            path.as_slice(),
            #[inline] |widget, idx, ctx| {
                let widget = widget.expect("Lost Widget in render");

                //s.caches.cache[idx].reset_current();

                widget.render(
                    &mut pathstack.with(FixedIdx(idx as isize)),
                    StdRenderProps::new(&props),
                    &mut render,
                    force,
                    &mut s.caches.cache[idx],
                    root, ctx
                );
            },
            &mut s.ctx
        );

        s.windows.windows[window_id].vali.render = false;

        s.ctx.queue_mut().as_mut().force_render = false;

        //rl.r.update_cursor();

        //let sdl render it
        //rl.r.windows[widx].present();

        render.post();
    }
    pub(crate) fn render_post(&self, window_id: usize) {
        //todo!()
    }
}
