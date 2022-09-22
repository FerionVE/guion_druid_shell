use druid_shell::kurbo;
use guion::aliases::*;
use guion::ctx::Context;
use guion::ctx::queue::StdOrder;
use guion::env::Env;
use guion::event::imp::StdVarSup;
use guion::event::variant::Variant;
use guion::event_new::variants::StdVariant;
use guion::handler::Handler;
use guion::path::WidgetPath;
use guion::render::WithTestStyle;
use guion::util::AsRefMut;
use guion::event::Event as GEvent;
use guion::event::standard::variants::RootEvent;
use guion::util::bounds::{Bounds, Offset};
use guion::widget::stack::WithCurrentWidget;

use crate::app::{App, ArcApp};
use crate::app::windows::Windows;
use crate::event::key::Key;
use crate::style::font::ksize2dims;
use crate::style::{stupid_test_style_variants, stupid_test_style};

use super::BaseEvent;

impl<E> ArcApp<E> where
    for<'a,'b> E: Env<RootRef<'a>=&'a Windows<E>,RootMut<'b>=&'b mut Windows<E>>,
    for<'a> ECQueue<'a,E>: AsRefMut<crate::ctx::queue::Queue<E>>,
    EEvent<E>: StdVarSup<E>,
    EEKey<E>: From<Key>,
{
    pub(crate) fn do_event(&self, window_id: usize, event: BaseEvent, ts: u64) -> bool {
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
                let e = StdVariant::new(RootEvent::WindowResize{
                    size: ksize2dims(size),
                },ts);
                handled |= s.send_legacy_root_event(window_id,e);
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
                let e = StdVariant::new(RootEvent::KbdDown{
                    key: Key::Kbd(key.key.clone(),key.location).into()
                },ts);
                eprintln!("KeyDown: {:?}",e);
                handled |= s.send_legacy_root_event(window_id,e);
                if let druid_shell::keyboard_types::Key::Character(c) = key.key {
                    //TODO do we have to timer-simulate ongoing keypress?
                    let e = StdVariant::new(RootEvent::TextInput{
                        text: c,
                    },ts);
                    eprintln!("KeyDownTI: {:?}",e);
                    handled |= s.send_legacy_root_event(window_id,e);
                }
            }
            BaseEvent::KeyUp(key) => {
                let e = StdVariant::new(RootEvent::KbdUp{
                    key: Key::Kbd(key.key,key.location).into()
                },ts);
                eprintln!("KeyUp: {:?}",e);
                handled = s.send_legacy_root_event(window_id,e);
            }
            BaseEvent::Wheel(m) => {
                let e = StdVariant::new(RootEvent::MouseScroll{
                    x: m.wheel_delta.x as i32,
                    y: m.wheel_delta.y as i32,
                },ts);
                eprintln!("Wheel: {:?}",e);
                handled |= s.send_legacy_root_event(window_id,e); //TODO event didn't have bounds filter, but maybe it needs?
            }
            BaseEvent::Zoom(_) => {
                //handled = todo!();
            }
            BaseEvent::MouseMove(m) => {
                let pos = kpoint2offset(m.pos);
                let e = StdVariant::new(RootEvent::MouseMove{
                    pos
                },ts).with_filter_point(pos); // TODO StdHandler currently doesn't keep the filter
                eprintln!("MouseMove: {:?}",e);
                handled |= s.send_legacy_root_event(window_id,e);
            }
            BaseEvent::MouseDown(m) => {
                let e = StdVariant::new(RootEvent::MouseDown{
                    key: Key::Mouse(m.button).into()
                },ts);
                eprintln!("MouseDown: {:?}",e);
                handled |= s.send_legacy_root_event(window_id,e); //TODO technically it had pos! in old guion, but StdHandler currently doesn't keep the filter. Maybe change this
            }
            BaseEvent::MouseUp(m) => {
                let e = StdVariant::new(RootEvent::MouseUp{
                    key: Key::Mouse(m.button).into()
                },ts);
                eprintln!("MouseUp: {:?}",e);
                handled |= s.send_legacy_root_event(window_id,e); //TODO technically it had pos! in old guion, but StdHandler currently doesn't keep the filter. Maybe change this
            }
            BaseEvent::MouseLeave => {
                let e = StdVariant::new(RootEvent::MouseLeaveWindow{},ts);
                handled |= s.send_legacy_root_event(window_id,e);
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
    for<'a,'b> E: Env<RootRef<'a>=&'a Windows<E>,RootMut<'b>=&'b mut Windows<E>>,
    for<'a> ECQueue<'a,E>: AsRefMut<crate::ctx::queue::Queue<E>>,
    EEvent<E>: StdVarSup<E>,
    EEKey<E>: From<Key>,
{
    // fn send_event(&mut self, window_id: usize, e: EEvent<E>) -> bool {
    //     let e = EventCompound{
    //         event: e,
    //         bounds: Bounds::default(),
    //         ts: 0, //TODO ts
    //         filter: StdFilter{
    //             filter_path: self.windows.path_of_window(window_id,&mut self.ctx),
    //             filter_bounds: true,
    //         }.into(),
    //         style: Default::default(),
    //         flag: true,
    //     };
    //     let mut link = self.ctx.link(self.windows.resolved());
    //     link._event_root(&e)
    // }

    fn send_legacy_root_event<V>(&mut self, window_id: usize, e: StdVariant<V,E>) -> bool where V: Variant<E> + Clone {
        let test_style = stupid_test_style_variants::<E>();
        let test_style = stupid_test_style(&test_style);
        let props = WithTestStyle((),test_style);
        
        // TODO where do we inject inital window bounds?
        let props = WithCurrentWidget{
            inner: props,
            path: WidgetPath::empty(),
            id: self.windows._id.clone(),
        };

        let e = e.with_filter_path(self.windows.path_of_window(window_id,&mut self.ctx));

        let ghandler = self.ctx.build_handler();

        ghandler._event_root(
            &self.windows,
            &props,
            &e,
            &self.windows,
            &mut self.ctx,
        )
    }
}

pub(crate) fn kpoint2offset(k: kurbo::Point) -> Offset {
    Offset {
        x: k.x as i32,
        y: k.y as i32,
    }
}
