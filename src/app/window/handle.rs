use druid_shell::piet::CairoTextLayout;
use druid_shell::{WinHandler, WindowHandle};
use guion::aliases::*;
use guion::backend::Backend;
use guion::env::Env;
use guion::event::filter::StdFilter;
use guion::event::imp::StdVarSup;
use guion::render::widgets::RenderStdWidgets;
use guion::util::AsRefMut;

use crate::app::ArcApp;
use crate::app::event::BaseEvent;
use crate::app::windows::Windows;
use crate::render::Render;
use crate::style::cursor::IntoGuionDruidShellCursor;

pub struct WHandle<E> where E: Env {
    pub(crate) app: ArcApp<E>,
    pub(crate) handle: WindowHandle,
    pub(crate) self_id: usize, //TODO use stable id and HashMap
}

impl<E> WinHandler for WHandle<E> where
    E: Env,
    for<'a> ECQueue<'a,E>: AsRefMut<crate::ctx::queue::Queue<E>>,
    //for<'a> ECQueue<'a,E>: AsRefMut<crate::ctx::queue::Queue<E>>,
    EEvent<E>: StdVarSup<E>,
    EEKey<E>: From<crate::event::key::Key>,
    EEFilter<E>: From<StdFilter<E>>,
    for<'a>  E::Storage<'a>: AsRefMut<Windows<E>>,
    for<'a> Windows<E>: AsRefMut<E::Storage<'a>>,
    for<'a> E::Backend: Backend<E,Renderer<'a>=Render<'a,E>>,
    //for<'a> ERenderer<'a,E>: RenderStdWidgets<E>,
    for<'a> Render<'a,E>: RenderStdWidgets<E>,
    ETextLayout<E>: AsRefMut<CairoTextLayout>, //TODO use Piet trait variant
    ESCursor<E>: IntoGuionDruidShellCursor<E>,
{
    fn connect(&mut self, handle: &druid_shell::WindowHandle) {
        self.handle = handle.clone();
    }

    fn prepare_paint(&mut self) {
        self.app.do_event(self.self_id, BaseEvent::PreRender);
        self.app.render_pre(self.self_id);
    }

    fn paint(&mut self, piet: &mut druid_shell::piet::Piet, invalid: &druid_shell::Region) {
        self.app.render(self.self_id, piet, invalid);
        self.app.render_post(self.self_id);
        self.app.do_event(self.self_id, BaseEvent::PostRender);
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        todo!()
    }

    fn size(&mut self, size: druid_shell::kurbo::Size) {
        self.app.do_event(self.self_id, BaseEvent::Size(size));
    } // app::do_event and invalidate

    fn scale(&mut self, scale: druid_shell::Scale) {
        self.app.do_event(self.self_id, BaseEvent::Scale(scale));
    }

    fn rebuild_resources(&mut self) {
        todo!()
    }

    fn command(&mut self, id: u32) {
        todo!()
    }

    fn save_as(&mut self, token: druid_shell::FileDialogToken, file: Option<druid_shell::FileInfo>) {
        self.app.do_event(self.self_id, BaseEvent::SaveAs{token,file});
    }

    fn open_file(&mut self, token: druid_shell::FileDialogToken, file: Option<druid_shell::FileInfo>) {
        self.app.do_event(self.self_id, BaseEvent::OpenFile{token,file});
    }

    fn key_down(&mut self, event: druid_shell::KeyEvent) -> bool {
        self.app.do_event(self.self_id, BaseEvent::KeyDown(event))
    }

    fn key_up(&mut self, event: druid_shell::KeyEvent) {
        self.app.do_event(self.self_id, BaseEvent::KeyUp(event));
    }

    fn wheel(&mut self, event: &druid_shell::MouseEvent) {
        self.app.do_event(self.self_id, BaseEvent::Wheel(event));
    }

    fn zoom(&mut self, delta: f64) {
        self.app.do_event(self.self_id, BaseEvent::Zoom(delta));
    }

    fn mouse_move(&mut self, event: &druid_shell::MouseEvent) {
        self.app.do_event(self.self_id, BaseEvent::MouseMove(event));
    } // TODO is there a stupid fn to q u e r y mouse pos? or do we have to calc from the event?

    fn mouse_down(&mut self, event: &druid_shell::MouseEvent) {
        self.app.do_event(self.self_id, BaseEvent::MouseDown(event));
    }

    fn mouse_up(&mut self, event: &druid_shell::MouseEvent) {
        self.app.do_event(self.self_id, BaseEvent::MouseUp(event));
    }

    fn mouse_leave(&mut self) {
        self.app.do_event(self.self_id, BaseEvent::MouseLeave);
    }

    fn timer(&mut self, token: druid_shell::TimerToken) {
        self.app.do_event(self.self_id, BaseEvent::Timer(token));
    }

    fn got_focus(&mut self) {
        self.app.do_event(self.self_id, BaseEvent::GotFocus);
    }

    fn lost_focus(&mut self) {
        self.app.do_event(self.self_id, BaseEvent::LostFocus);
    }

    fn request_close(&mut self) {
        if !self.app.do_event(self.self_id, BaseEvent::RequestClose) {
            self.handle.close();
        }
    }

    fn destroy(&mut self) {
        if !self.app.do_event(self.self_id, BaseEvent::Destroy) {
            self.app.inner.lock().unwrap().ds_app.quit();
        }
    }

    fn idle(&mut self, token: druid_shell::IdleToken) {
        self.app.do_event(self.self_id, BaseEvent::Idle(token));
    }
}
