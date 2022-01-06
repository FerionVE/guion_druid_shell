use druid_shell::kurbo;
use guion::aliases::*;
use guion::ctx::Context;
use guion::ctx::queue::StdOrder;
use guion::env::Env;
use guion::event::imp::StdVarSup;
use guion::util::AsRefMut;
use guion::event::Event as GEvent;
use guion::event::standard::variants::RootEvent;
use guion::event::filter::StdFilter;
use guion::event::compound::EventCompound;
use guion::util::bounds::{Bounds, Offset};

use crate::app::{App, ArcApp};
use crate::app::windows::Windows;
use crate::event::key::Key;
use crate::style::font::ksize2dims;

use super::BaseEvent;

impl<E> ArcApp<E> where
    E: Env,
    for<'a> ECQueue<'a,E>: AsRefMut<crate::ctx::queue::Queue<E>>,
    EEvent<E>: StdVarSup<E>,
    EEKey<E>: From<Key>,
    EEFilter<E>: From<StdFilter<E>>,
    for<'a> E::Storage<'a>: AsRefMut<Windows<E>>,
    for<'a> Windows<E>: AsRefMut<E::Storage<'a>>,
{
    pub(crate) fn do_event(&self, window_id: usize, event: BaseEvent) -> bool {
        let mut s = self.inner.lock().unwrap();
        let s = &mut *s;

        for w in &mut s.windows.windows {
            if let Some(handle) = &w.handle {
                w.dims = ksize2dims(handle.get_size());
            }
        }

        let stor = s.windows.as_mut();
        let c = &mut s.ctx;
        // do events and call pre/post queued
        match &event {
            BaseEvent::PreRender => {
                s.do_queued(StdOrder::PreRender);
                return true;
            }
            BaseEvent::PostRender => {
                s.do_queued(StdOrder::RenderValidation);
                s.do_queued(StdOrder::PostCurrent);
                s.do_queued(StdOrder::PostRender);
                return true;
            }
            _ => {},
        };

        s.do_queued(StdOrder::PreEvents);
        s.do_queued(StdOrder::PreEvent);

        let mut handled= false;
        match event {
            BaseEvent::Size(size) => {
                let e: EEvent<E> = GEvent::from(RootEvent::WindowResize{
                    size: ksize2dims(size),
                });
                handled |= s.send_event(window_id,e);
            }
            BaseEvent::Scale(_) => {
                //handled = todo!();
            }
            BaseEvent::Command(_) => {
                //handled = todo!();
            }
            BaseEvent::SaveAs { token, file } => {
                //handled = todo!();
            }
            BaseEvent::OpenFile { token, file } => {
                //handled = todo!();
            }
            BaseEvent::KeyDown(key) => {
                let e: EEvent<E> = GEvent::from(RootEvent::KbdDown{
                    key: Key::Kbd(key.key.clone(),key.location).into()
                });
                eprintln!("KeyDown: {:?}",e);
                handled |= s.send_event(window_id,e);
                if let druid_shell::keyboard_types::Key::Character(c) = key.key {
                    //TODO do we have to timer-simulate ongoing keypress?
                    let e: EEvent<E> = GEvent::from(RootEvent::TextInput{
                        text: c,
                    });
                    eprintln!("KeyDownTI: {:?}",e);
                    handled |= s.send_event(window_id,e);
                }
            }
            BaseEvent::KeyUp(key) => {
                let e: EEvent<E> = GEvent::from(RootEvent::KbdUp{
                    key: Key::Kbd(key.key,key.location).into()
                });
                eprintln!("KeyUp: {:?}",e);
                handled = s.send_event(window_id,e);
            }
            BaseEvent::Wheel(m) => {
                let e: EEvent<E> = GEvent::from(RootEvent::MouseScroll{
                    x: m.wheel_delta.x as i32,
                    y: m.wheel_delta.y as i32,
                });
                eprintln!("Wheel: {:?}",e);
                handled |= s.send_event(window_id,e);
            }
            BaseEvent::Zoom(_) => {
                //handled = todo!();
            }
            BaseEvent::MouseMove(m) => {
                let e: EEvent<E> = GEvent::from(RootEvent::MouseMove{
                    pos: kpoint2offset(m.pos)
                });
                eprintln!("MouseMove: {:?}",e);
                handled |= s.send_event(window_id,e);
            }
            BaseEvent::MouseDown(m) => {
                let e: EEvent<E> = GEvent::from(RootEvent::MouseDown{
                    key: Key::Mouse(m.button).into()
                });
                eprintln!("MouseDown: {:?}",e);
                handled |= s.send_event(window_id,e);
            }
            BaseEvent::MouseUp(m) => {
                let e: EEvent<E> = GEvent::from(RootEvent::MouseUp{
                    key: Key::Mouse(m.button).into()
                });
                eprintln!("MouseUp: {:?}",e);
                handled |= s.send_event(window_id,e);
            }
            BaseEvent::MouseLeave => {
                let e: EEvent<E> = GEvent::from(RootEvent::MouseLeaveWindow{});
                handled |= s.send_event(window_id,e);
            }
            BaseEvent::Timer(_) => {
                //handled = todo!();
            }
            BaseEvent::GotFocus => {
                //handled = todo!();
            }
            BaseEvent::LostFocus => {
                //handled = todo!();
            }
            BaseEvent::RequestClose => {
                //handled = todo!();
            }
            BaseEvent::Destroy => {
                //handled = todo!();
            }
            BaseEvent::Idle(_) => {
                //handled = todo!();
            }
            
            BaseEvent::PreRender => unreachable!(),
            BaseEvent::PostRender => unreachable!(),
        }

        s.do_queued(StdOrder::PostCurrent);
        s.do_queued(StdOrder::PostEvent);
        s.do_queued(StdOrder::PostEvents);
        
        if let Some(handle) = &s.windows.windows[window_id].handle {
            handle.invalidate();
        }

        handled
    }
}

impl<E> App<E> where
    E: Env,
    for<'a> ECQueue<'a,E>: AsRefMut<crate::ctx::queue::Queue<E>>,
    EEvent<E>: StdVarSup<E>,
    EEKey<E>: From<Key>,
    EEFilter<E>: From<StdFilter<E>>,
    for<'a> E::Storage<'a>: AsRefMut<Windows<E>>,
    for<'a> Windows<E>: AsRefMut<E::Storage<'a>>,
{
    fn send_event(&mut self, window_id: usize, e: EEvent<E>) -> bool {
        let e = EventCompound{
            event: e,
            bounds: Bounds::default(),
            ts: 0, //TODO ts
            filter: StdFilter{
                filter_path: self.windows.path_of_window(window_id),
                filter_bounds: true,
            }.into(),
            style: Default::default(),
            flag: true,
        };
        let mut link = self.ctx.link(self.windows.resolved());
        link._event_root(&e)
    }
}

pub(crate) fn kpoint2offset(k: kurbo::Point) -> Offset {
    Offset {
        x: k.x as i32,
        y: k.y as i32,
    }
}
