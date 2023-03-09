use druid_shell::piet::CairoTextLayout;
use druid_shell::{WinHandler, WindowHandle};
use guion::aliases::*;
use guion::backend::Backend;
use guion::env::Env;
use guion::event::imp::StdVarSup;
use guion::render::widgets::RenderStdWidgets;
use guion::util::AsRefMut;

use crate::app::{ArcApp, ModelRoot};
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
    for<'a,'b> E: Env<RootRef<'a>=&'a ModelRoot,RootMut<'b>=&'b mut ModelRoot>,
    for<'a> ECQueue<'a,E>: AsRefMut<crate::ctx::queue::Queue<E>>,
    //for<'a> ECQueue<'a,E>: AsRefMut<crate::ctx::queue::Queue<E>>,
    EEvent<E>: StdVarSup<E>,
    EEKey<E>: From<crate::event::key::Key>,
    for<'a> E::Backend: Backend<E,Renderer<'a>=Render<'a,E>>,
    //for<'a> ERenderer<'a,E>: RenderStdWidgets<E>,
    for<'a> Render<'a,E>: RenderStdWidgets<E>,
    ETextLayout<E>: AsRefMut<CairoTextLayout>, //TODO use Piet trait variant
    ESCursor<E>: IntoGuionDruidShellCursor<E>,
{
    fn connect(&mut self, handle: &druid_shell::WindowHandle) {
        self.handle = handle.clone();
        //self.handle.invalidate();
    }

    fn prepare_paint(&mut self) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::PreRender, ts);
        //self.handle.invalidate();
        self.app.render_pre(self.self_id);
    }

    fn paint(&mut self, piet: &mut druid_shell::piet::Piet, invalid: &druid_shell::Region) {
        let ts = now_time_stamp();
        self.app.render(self.self_id, piet, invalid);
        self.app.render_post(self.self_id);
        self.app.do_event(self.self_id, BaseEvent::PostRender, ts);
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        todo!()
    }

    fn size(&mut self, size: druid_shell::kurbo::Size) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::Size(size), ts);
    } // app::do_event and invalidate

    fn scale(&mut self, scale: druid_shell::Scale) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::Scale(scale), ts);
    }

    fn rebuild_resources(&mut self) {
        self.app.inner.lock().unwrap().caches = Default::default();
        //self.handle.invalidate();
    }

    fn command(&mut self, id: u32) {
        //self.handle.invalidate();
        todo!()
    }

    fn save_as(&mut self, token: druid_shell::FileDialogToken, file: Option<druid_shell::FileInfo>) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::SaveAs{token,file}, ts);
        //self.handle.invalidate();
    }

    fn open_file(&mut self, token: druid_shell::FileDialogToken, file: Option<druid_shell::FileInfo>) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::OpenFile{token,file}, ts);
        //self.handle.invalidate();
    }

    fn key_down(&mut self, event: druid_shell::KeyEvent) -> bool {
        let ts = now_time_stamp();
        let r = self.app.do_event(self.self_id, BaseEvent::KeyDown(event), ts);
        //self.handle.invalidate();
        r
    }

    fn key_up(&mut self, event: druid_shell::KeyEvent) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::KeyUp(event), ts);
        //self.handle.invalidate();
    }

    fn wheel(&mut self, event: &druid_shell::MouseEvent) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::Wheel(event), ts);
        //self.handle.invalidate();
    }

    fn zoom(&mut self, delta: f64) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::Zoom(delta), ts);
        //self.handle.invalidate();
    }

    fn mouse_move(&mut self, event: &druid_shell::MouseEvent) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::MouseMove(event), ts);
        //self.handle.invalidate();
    } // TODO is there a stupid fn to q u e r y mouse pos? or do we have to calc from the event?

    fn mouse_down(&mut self, event: &druid_shell::MouseEvent) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::MouseDown(event), ts);
        //self.handle.invalidate();
    }

    fn mouse_up(&mut self, event: &druid_shell::MouseEvent) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::MouseUp(event), ts);
        //self.handle.invalidate();
    }

    fn mouse_leave(&mut self) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::MouseLeave, ts);
        //self.handle.invalidate();
    }

    fn timer(&mut self, token: druid_shell::TimerToken) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::Timer(token), ts);
        //self.handle.invalidate();
    }

    fn got_focus(&mut self) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::GotFocus, ts);
        //self.handle.invalidate();
    }

    fn lost_focus(&mut self) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::LostFocus, ts);
        //self.handle.invalidate();
    }

    fn request_close(&mut self) {
        let ts = now_time_stamp();
        if true || !self.app.do_event(self.self_id, BaseEvent::RequestClose, ts) {
            self.handle.close();
        }
        //self.handle.invalidate();
    }

    fn destroy(&mut self) {
        let ts = now_time_stamp();
        if true || !self.app.do_event(self.self_id, BaseEvent::Destroy, ts) {
            self.app.inner.lock().unwrap().ds_app.quit();
        }
        //self.handle.invalidate();
    }

    fn idle(&mut self, token: druid_shell::IdleToken) {
        let ts = now_time_stamp();
        self.app.do_event(self.self_id, BaseEvent::Idle(token), ts);
        //self.handle.invalidate();
    }

    fn open_files(&mut self, token: druid_shell::FileDialogToken, files: Vec<druid_shell::FileInfo>) {}

    fn acquire_input_lock(
        &mut self,
        token: druid_shell::TextFieldToken,
        mutable: bool,
    ) -> Box<dyn druid_shell::text::InputHandler> {
        panic!("acquire_input_lock was called on a WinHandler that did not expect text input.")
    }

    fn release_input_lock(&mut self, token: druid_shell::TextFieldToken) {
        panic!("release_input_lock was called on a WinHandler that did not expect text input.")
    }
}

fn now_time_stamp() -> u64 {
    //TODO new time stamp format and retrieve method
    std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64
}
