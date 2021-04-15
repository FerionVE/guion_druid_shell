use std::collections::HashMap;

use guion::ctx::queue::{StdEnqueueable, StdOrder};
use guion::env::Env;
use guion::util::AsRefMut;

pub struct Queue<E>
where
    E: Env,
{
    pub queues: HashMap<StdOrder,Vec<(StdEnqueueable<E>,i64)>>,
    pub force_render: bool,
}

impl<E> guion::ctx::queue::Queue<StdEnqueueable<E>,StdOrder> for Queue<E> where E: Env + Sync, /*ECQueue<E>: AsRefMut<Self>*/ {
    fn push(&mut self, v: StdEnqueueable<E>, o: StdOrder, p: i64) {
        self.queues.entry(o).or_default().push((v,p))
    }
    fn send(&self, v: StdEnqueueable<E>, o: StdOrder, p: i64) {
        todo!()
    }
}

impl<E> AsRefMut<Self> for Queue<E> where E: Env {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
