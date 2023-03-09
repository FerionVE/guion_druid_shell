use guion::ctx::Context;
use guion::ctx::clipboard::CtxClipboardAccess;
use guion::env::Env;
use guion::intercept::{InterceptBuilder, InterceptStateResolve};
use guion::intercept::standard::StdIntercept;
use guion::state::CtxStdState;
use guion::util::AsRefMut;
use guion::widget::id::WidgetID;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::num::NonZeroUsize;

use crate::ctx::queue::Queue;
use crate::ctx::state::DSState;

use super::env::ExampleEnv;

pub struct ExampleCtx<'cc> {
    pub handler: ExampleIntercept,
    pub ds_state: DSState,
    pub queue: Queue<ExampleEnv>,
    pub id_counter: usize,
    test: PhantomData<&'cc mut u32>,
}

impl<'cc> ExampleCtx<'cc> {
    pub fn new() -> Self {
        Self {
            handler: StdIntercept::<(),ExampleEnv>::new(()),
            ds_state: DSState::new(),
            queue: Queue{queues:HashMap::new(),force_render:true},
            id_counter: 255,
            test: PhantomData,
        }
    }
}

pub type ExampleIntercept = StdIntercept<(),ExampleEnv>;

impl<'cc> Context<'cc,ExampleEnv> for ExampleCtx<'cc> {
    type Intercept = ExampleIntercept;
    type Queue = Queue<ExampleEnv>;

    #[inline]
    fn queue_mut(&mut self) -> &mut Self::Queue {
        &mut self.queue
    }
    #[inline]
    fn queue(&self) -> &Self::Queue {
        &self.queue
    }

    fn lt_mut(&mut self) -> &mut <ExampleEnv as Env>::Context<'cc> where Self: 'cc {
        self
    }

    fn build_intercept(&mut self) -> <Self::Intercept as InterceptBuilder<ExampleEnv>>::Built where Self: Sized {
        <ExampleIntercept as InterceptBuilder<ExampleEnv>>::build::<ExampleInterceptResolve>(self)
    }

    fn retained_id(&mut self) -> WidgetID {
        self.id_counter += 1;
        WidgetID(NonZeroUsize::new(self.id_counter).unwrap())
    }

    fn tcell_owner(&self) -> &guion::qcell::TCellOwner<<ExampleEnv as Env>::CtxTCellOwner> {
        todo!()
    }

    fn tcell_owner_mut(&mut self) -> &mut guion::qcell::TCellOwner<<ExampleEnv as Env>::CtxTCellOwner> {
        todo!()
    }
}

impl AsRefMut<Self> for ExampleCtx<'_> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
impl AsRefMut<StdIntercept<(),ExampleEnv>> for ExampleCtx<'_> {
    #[inline]
    fn as_ref(&self) -> &ExampleIntercept {
        &self.handler
    }
    #[inline]
    fn as_mut(&mut self) -> &mut ExampleIntercept {
        &mut self.handler
    }
}
impl AsRefMut<DSState> for ExampleCtx<'_> {
    #[inline]
    fn as_ref(&self) -> &DSState {
        &self.ds_state
    }
    #[inline]
    fn as_mut(&mut self) -> &mut DSState {
        &mut self.ds_state
    }
}
impl<'cc> CtxStdState<'cc,ExampleEnv> for ExampleCtx<'cc> {
    type T = ExampleIntercept;
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
impl CtxClipboardAccess<ExampleEnv> for ExampleCtx<'_> {
    #[inline]
    fn clipboard_set_text(&mut self, v: &str) {
        self.ds_state.clipboard.as_mut().unwrap().put_string(v);
    }
    #[inline]
    fn clipboard_get_text(&mut self) -> Option<String> {
        self.ds_state.clipboard.as_mut().unwrap().get_string()
    }
}

// impl DynState<ExampleEnv> for ExampleCtx<'_> {
//     fn remote_state_or_default<T>(&self, id: StdID) -> T where T: Default + Clone + 'static {
//         self.handler.remote_state_or_default(id)
//     }
//     fn push_remote_state<T>(&mut self, id: StdID, v: T) where T: 'static {
//         self.handler.push_remote_state(id,v)
//     }
// }

fn akw22(a: &<ExampleEnv as Env>::Context<'_>) {
    let r = AsRefMut::<ExampleIntercept>::as_ref(a);
}

struct ExampleInterceptResolve;

impl<E> InterceptStateResolve<ExampleIntercept,E> for ExampleInterceptResolve where for<'a> E: Env<Context<'a>=ExampleCtx<'a>>, ExampleIntercept: InterceptBuilder<E> {
    #[inline]
    fn resolve_intercept_state<'a>(ctx_root: &'a mut E::Context<'_>) -> &'a mut ExampleIntercept {
        &mut ctx_root.handler
    }
}