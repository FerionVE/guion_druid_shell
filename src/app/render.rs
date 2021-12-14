use druid_shell::piet::Piet;
use guion::aliases::{ECQueue, ERenderer, ESCursor};
use guion::env::Env;
use guion::render::widgets::RenderStdWidgets;
use guion::style::selectag::standard::StdSelectag;
use guion::style::standard::cursor::StdCursor;
use guion::util::AsRefMut;
use guion::ctx::Context;
use guion::widget::root::Widgets;

use crate::render::Render;

use super::{ArcApp, ksize2dims};
use super::windows::Windows;


impl<E> ArcApp<E> where
    E: Env,
    for<'a> ERenderer<'a,E>: AsRefMut<Render<'a,E>> + RenderStdWidgets<E>,
    for<'a> Render<'a,E>: AsRefMut<ERenderer<'a,E>>,
    for<'a> E::Storage<'a>: AsRefMut<Windows<E>>,
    for<'a> Windows<E>: AsRefMut<E::Storage<'a>>,
    ECQueue<E>: AsRefMut<crate::ctx::queue::Queue<E>>,
    ESCursor<E>: Into<StdCursor>,  //TODO Into<DruidCursor>
{
    pub(crate) fn render_pre(&self, window_id: usize) {
        //todo!()
    }
    pub(crate) fn render(&self, window_id: usize, render: &mut Piet, bounds: &druid_shell::Region) {
        let mut s = self.inner.lock().unwrap();
        let s = &mut *s;
        let window_handle = s.windows.windows[window_id].handle.as_ref().unwrap().clone();
        /*Render::<E>::scoped(window_handle.clone(),render, |r| {
            //TODO reset cursor
            //TODO restore renderer
            r.pre();
            //s.windows.child(window_id).unwrap()
            let path = s.windows.path_of_window(window_id);
            let dims = window_handle.get_size();
            s.windows.windows[window_id].dims = ksize2dims(dims);

            //build the RenderLink
            let mut rl: RenderLink<E> = RenderLink::simple(
                r.as_mut(),
                (dims.width as u32, dims.height as u32),
                &mut s.ctx,
            );
            //fill background
            rl.with(StdSelectag::ObjBackground)
                .fill_rect(&mut s.ctx);
            //process queued and render
            rl.force |= s.ctx.queue().as_ref().force_render;

            let w = s.windows.widget(path).expect("Lost Widget in render");
            let w = s.ctx.link(w);
            rl.render_widget(w);

            s.ctx.queue_mut().as_mut().force_render = false;

            //rl.r.update_cursor();

            //let sdl render it
            //rl.r.windows[widx].present();

            drop(rl);

            r.post();
            //TODO restore renderer
        })*/
    }
    pub(crate) fn render_post(&self, window_id: usize) {
        //todo!()
    }
}
