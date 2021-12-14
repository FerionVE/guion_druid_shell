use guion::ctx::Context;
use guion::ctx::clipboard::CtxClipboardAccess;
use guion::env::Env;
use guion::handler::standard::StdHandler;
use guion::id::standard::StdID;
use guion::state::CtxStdState;
use guion::state::dyn_state::DynState;
use guion::util::AsRefMut;
use std::collections::HashMap;

use crate::ctx::queue::Queue;
use crate::ctx::state::DSState;

use super::env::ExampleEnv;

pub struct ExampleCtx {
    pub handler: ExampleHandler,
    pub ds_state: DSState,
    pub queue: Queue<ExampleEnv>,
}

impl ExampleCtx {
    pub fn new() -> Self {
        Self {
            handler: StdHandler::<(),ExampleEnv>::new(()),
            ds_state: DSState::new(),
            queue: Queue{queues:HashMap::new(),force_render:true},
        }
    }
}

pub type ExampleHandler = StdHandler<(),ExampleEnv>;

impl Context<ExampleEnv> for ExampleCtx {
    type Handler = ExampleHandler;
    type Queue = Queue<ExampleEnv>;

    #[inline]
    fn queue_mut(&mut self) -> &mut Self::Queue {
        &mut self.queue
    }
    #[inline]
    fn queue(&self) -> &Self::Queue {
        &self.queue
    }

    fn lt_mut<'s>(&mut self) -> &mut <ExampleEnv as Env>::Context {
        unsafe{std::mem::transmute(self)}
    }
}

impl AsRefMut<Self> for ExampleCtx {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
impl AsRefMut<StdHandler<(),ExampleEnv>> for ExampleCtx {
    #[inline]
    fn as_ref(&self) -> &ExampleHandler {
        &self.handler
    }
    #[inline]
    fn as_mut(&mut self) -> &mut ExampleHandler {
        &mut self.handler
    }
}
impl AsRefMut<DSState> for ExampleCtx {
    #[inline]
    fn as_ref(&self) -> &DSState {
        &self.ds_state
    }
    #[inline]
    fn as_mut(&mut self) -> &mut DSState {
        &mut self.ds_state
    }
}
impl CtxStdState<ExampleEnv> for ExampleCtx {
    type T = ExampleHandler;
    #[inline]
    fn state_mut(&mut self) -> &mut Self::T {
        &mut self.handler
    }
    #[inline]
    fn state(&self) -> &Self::T {
        &self.handler
    }
}

/*impl Deref for ExampleCtx {
    type Target = Core<ExampleEnv>;
    
    #[inline]
    fn deref(&self) -> &Self::Target {
        AsRefMut::as_ref(self)
    }
}
impl DerefMut for ExampleCtx {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        AsRefMut::as_mut(self)
    }
}*/

//TODO move to handler of different
impl<'a> CtxClipboardAccess<ExampleEnv> for ExampleCtx {
    #[inline]
    fn clipboard_set_text(&mut self, v: &str) {
        self.ds_state.clipboard.as_mut().unwrap().put_string(v);
    }
    #[inline]
    fn clipboard_get_text(&mut self) -> Option<String> {
        self.ds_state.clipboard.as_mut().unwrap().get_string()
    }
}

impl<'a> DynState<ExampleEnv> for ExampleCtx {
    fn remote_state_or_default<T>(&self, id: StdID) -> T where T: Default + Clone + 'static {
        self.handler.remote_state_or_default(id)
    }
    fn push_remote_state<T>(&mut self, id: StdID, v: T) where T: 'static {
        self.handler.push_remote_state(id,v)
    }
}

fn akw22(a: &<ExampleEnv as Env>::Context) {
    let r = AsRefMut::<ExampleHandler>::as_ref(a);
}
