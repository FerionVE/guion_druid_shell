use guion::aliases::ECQueue;
use guion::ctx::{Context, queue::*};
use guion::env::Env;
use guion::util::AsRefMut;

use super::App;
use super::windows::Windows;

impl<E> App<E> where
    for<'a,'b> E: Env<RootRef<'a>=&'a Windows<E>,RootMut<'b>=&'b mut Windows<E>>,
    for<'a> ECQueue<'a,E>: AsRefMut<crate::ctx::queue::Queue<E>>,
{
    #[allow(unreachable_code)]
    pub(crate) fn do_queued(&mut self, pass: StdOrder) {
        let stor = self.windows.as_mut();
        let c = &mut self.ctx;

        if let Some(mut queue) = c.queue_mut().as_mut().queues.remove(&pass) { //c.queue_mut().as_mut().queues.remove(&pass)
            queue.sort_by_key(|(_,p)| *p );
    
            for (e,_) in queue {
                match e {
                    StdEnqueueable::InvalidateWidget { path } => {
                        //TODO fix validationions
                        //invalidate::<E>(stor, path.clone()).expect("Lost Widget in invalidate");
                    },
                    StdEnqueueable::ValidateWidgetRender { path } => {
                        //validate::<E>(stor, path.clone()).expect("Lost Widget in invalidate");
                    },
                    StdEnqueueable::ValidateWidgetSize { path, size } => todo!(),
                    StdEnqueueable::Render { force } => {
                        //todo!().force_render |= force;
                        //TODO Path and invalidate Window (here and not through force_render)
                    },
                    StdEnqueueable::Event { event, ts } => todo!(),
                    // StdEnqueueable::MutateWidget { path, f } => {
                    //     let w = stor.widget_mut(path.clone()).expect("TODO");
                    //     f(w.wref,c,path);
                    // },
                    // StdEnqueueable::MutateWidgetClosure { path, f } => {
                    //     let w = stor.widget_mut(path.clone()).expect("TODO");
                    //     f(w.wref,c,path);
                    // },
                    StdEnqueueable::MutateRoot { f } => {
                        f(stor,&(),c)
                    },
                    StdEnqueueable::MutateRootClosure { f } => {
                        f(stor,&(),c)
                    },
                    StdEnqueueable::AccessWidget { path, f } => todo!(),
                    StdEnqueueable::AccessWidgetClosure { path, f } => todo!(),
                    StdEnqueueable::AccessRoot { f } => todo!(),
                    StdEnqueueable::AccessRootClosure { f } => todo!(),
                    StdEnqueueable::MutMessage { path, msg } => {
                        todo!()
                        // let mut w = stor.widget_mut(path.clone()).expect("TODO");
                        // w.message(msg)
                    },
                }
            }
        }
    }
}
