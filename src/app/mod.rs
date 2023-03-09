use std::any::Any;
use std::sync::{Arc, Mutex};

use druid_shell::piet::CairoTextLayout;
use druid_shell::{WindowBuilder, kurbo};
use guion::aliases::*;
use guion::backend::Backend;
use guion::env::Env;
use guion::error::ResolveResult;
use guion::event::imp::StdVarSup;
use guion::invalidation::Invalidation;
use guion::newpath::{FixedIdx, PathFragment};
use guion::render::widgets::RenderStdWidgets;
use guion::root::{RootRef, RootMut};
use guion::util::AsRefMut;
use guion::util::bounds::Dims;
use guion::widget::Widget;
use guion::widget::cache::DynWidgetCache;
use guion::widget::declared::WidgetDeclarative;
use guion::widget_decl::WidgetDeclCallback;
use guion::widget_decl::mut_target::MStatic;
use guion::widget_decl::mutor_trait::MutorToBuilder;
use guion::widget_decl::mutor_trait::{MutorToBuilderDyn, MutorForTarget};

use crate::ctx::state::DSState;
use crate::render::Render;
use crate::style::cursor::IntoGuionDruidShellCursor;

use self::window::Window;
use self::window::handle::WHandle;
use self::windows::{Windows, GlobalCache};

pub mod windows;
pub mod window;
pub mod event;
pub mod render;
pub mod queue;

#[derive(Clone)]
pub struct ArcApp<E> where E: Env {
    inner: Arc<Mutex<App<E>>>, // druid_shell has owned Window(Handler), so we have to pull up this shit TODO parking_lot
}

pub struct App<E> where E: Env {
    ds_app: druid_shell::Application,
    windows: windows::Windows<E>,
    models: ModelRoot,
    caches: GlobalCache<E>, //TODO assert sync with windows
    ctx: E::Context<'static>,
}

impl<E> ArcApp<E> where E: Env, for<'a> E::Context<'a>: AsRefMut<DSState> {
    pub fn new(mut ctx: E::Context<'static>) -> Self {
        let ds_app = druid_shell::Application::new().unwrap(); //TODO error handling in all crate
        let windows = Windows{windows: vec![], vali: Invalidation::new()};
        ctx.as_mut().clipboard = Some(ds_app.clipboard());
        let app = App {
            ds_app,
            windows,
            models: ModelRoot { models: vec![] },
            ctx,
            caches: GlobalCache { cache: Vec::new() },
        };
        ArcApp{inner: Arc::new(Mutex::new(app))}
    }

    pub fn run(&self) {
        //TODO panic if called twice
        let app = self.inner.lock().unwrap().ds_app.clone();
        app.run(None);
    }
}

impl<E> ArcApp<E> where
    for<'a,'b> E: Env<RootRef<'a>=&'a ModelRoot,RootMut<'b>=&'b mut ModelRoot>,
    for<'a> E::Context<'a>: AsRefMut<DSState>,
    for<'a> ECQueue<'a,E>: AsRefMut<crate::ctx::queue::Queue<E>>,
    EEvent<E>: StdVarSup<E>,
    EEKey<E>: From<crate::event::key::Key>,
    for<'a> E::Backend: Backend<E,Renderer<'a>=Render<'a,E>>,
    //for<'a> ERenderer<'a,E>: AsRefMut<Render<'a,E>> + RenderStdWidgets<E>,
    for<'a> Render<'a,E>: RenderStdWidgets<E>,
    ETextLayout<E>: AsRefMut<CairoTextLayout>, //TODO use Piet trait variant
    ESCursor<E>: IntoGuionDruidShellCursor<E>,
{
    pub fn add_window_decl<W,M,WF,WB>(&self, f: WB, model: M, mut decl_fn: WF) where
        W: Widget<E> + 'static,
        WB: FnOnce(&mut WindowBuilder),
        M: Send + Sync + 'static,
        WF: FnMut(
            &M,
            &(dyn MutorToBuilderDyn<(),MStatic<M>,E>+'_),
            WidgetDeclCallback<'_,W,E>,
            E::RootRef<'_>,
            &mut E::Context<'_>,
        ) + 'static,
    {
        let app;
        let next_id;
        {
            let mut s = self.inner.lock().unwrap();
            app = s.ds_app.clone();
            next_id = s.windows.windows.len();
            
            let s = &mut *s;

            s.models.models.push(Box::new(model));

            let widget = WidgetDeclarative::<(),W,_,E>::new(
                move |root,model_ref,cb,ctx| {
                    let mutor = MutorForTarget::<MStatic<M>,(),_,_>::new(move |root: &mut ModelRoot,callback,_,ctx| {
                        let state = root.models[next_id].downcast_mut::<M>().expect("TODO");
        
                        (callback)(
                            Ok(state),
                            ctx
                        )
                    });

                    let model = root.models[next_id].downcast_ref::<M>().expect("TODO");

                    decl_fn(model, mutor.erase(), cb, root, ctx)
                },
                (),
                &s.models,
                &FixedIdx(next_id as isize).push_on_stack(&()),
                &mut s.ctx,
            );

            s.windows.windows.push(Window{
                handle: None,
                widget: Box::new(widget),
                vali: Invalidation::new(),
                dims: Default::default(),
            });
            s.caches.cache.push(Default::default());
        }

        let handler = WHandle {
            app: self.clone(),
            handle: Default::default(),
            self_id: next_id,  
        };

        let mut builder = WindowBuilder::new(app);
        f(&mut builder);
        builder.set_handler(Box::new(handler));
        let window = builder.build().unwrap();
        window.show();

        let mut s = self.inner.lock().unwrap();
        s.windows.windows[next_id].dims = ksize2dims(window.get_size());
        s.windows.windows[next_id].handle = Some(window);
    }
}

impl<E> App<E> where E: Env {
    
}

pub(crate) fn ksize2dims(k: kurbo::Size) -> Dims {
    Dims {
        w: k.width as u32,
        h: k.height as u32,
    }
}
pub(crate) fn dims2ksize(k: Dims) -> kurbo::Size {
    kurbo::Size {
        width: k.w as f64,
        height: k.h as f64,
    }
}

pub struct ModelRoot {
    models: Vec<Box<dyn Any>>,
}

impl<E> RootRef<E> for &ModelRoot where for<'a,'b> E: Env<RootRef<'a>=&'a ModelRoot,RootMut<'b>=&'b mut ModelRoot> {
    fn fork<'s,'w:'s>(&'s self) -> E::RootRef<'w> where Self: 'w {
        &**self
    }

    // fn with_widget<'s,'l:'s,F,R>(
    //     &'s self,
    //     i: &(dyn guion::newpath::PathResolvusDyn<E>+'_),
    //     callback: F,
    //     ctx: &mut E::Context<'_>,
    // ) -> R
    // where 
    //     F: for<'w,'ww,'c,'cc> FnMut(Result<&'w (dyn guion::widget::dyn_tunnel::WidgetDyn<E>+'ww),E::Error>,&'c mut E::Context<'cc>) -> R,
    //     Self: 'l {
    //     panic!("this doesn't work anymore")
    // }

    // fn trace_bounds(&self, ctx: &mut E::Context<'_>, i: &(dyn guion::newpath::PathResolvusDyn<E>+'_), b: &guion::util::bounds::Bounds, e: &EStyle<E>, force: bool) -> Result<guion::util::bounds::Bounds,E::Error> {
    //     panic!("this doesn't work anymore")
    // }
}
impl<E> RootMut<E> for &mut ModelRoot where for<'a,'b> E: Env<RootRef<'a>=&'a ModelRoot,RootMut<'b>=&'b mut ModelRoot> {
    fn fork_mut<'s>(&'s mut self) -> E::RootMut<'s> where Self: 's {
        self
    }
}
