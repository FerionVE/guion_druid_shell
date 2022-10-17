use std::sync::{Arc, Mutex};

use druid_shell::piet::CairoTextLayout;
use druid_shell::{WindowBuilder, kurbo};
use guion::aliases::*;
use guion::backend::Backend;
use guion::env::Env;
use guion::error::ResolveResult;
use guion::event::imp::StdVarSup;
use guion::render::widgets::RenderStdWidgets;
use guion::util::AsRefMut;
use guion::util::bounds::Dims;
use guion::view::View;
use guion::widget::cache::DynWidgetCache;

use crate::ctx::state::DSState;
use crate::render::Render;
use crate::style::cursor::IntoGuionDruidShellCursor;

use self::window::{Window, ViewDyn3};
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
    caches: GlobalCache<E>, //TODO assert sync with windows
    ctx: E::Context<'static>,
}

impl<E> ArcApp<E> where E: Env, for<'a> E::Context<'a>: AsRefMut<DSState> {
    pub fn new(mut ctx: E::Context<'static>) -> Self {
        let ds_app = druid_shell::Application::new().unwrap(); //TODO error handling in all crate
        let windows = Windows{windows: vec![]};
        ctx.as_mut().clipboard = Some(ds_app.clipboard());
        let app = App {
            ds_app,
            windows,
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
    for<'a,'b> E: Env<RootRef<'a>=&'a Windows<E>,RootMut<'b>=&'b mut Windows<E>>,
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
    pub fn add_window<W,M>(&self, f: M, widget: W) where
        W: 'static,
        M: FnOnce(&mut WindowBuilder),
        W: ViewDyn3<E>,
    {
        let app;
        let next_id;
        {
            let mut s = self.inner.lock().unwrap();
            app = s.ds_app.clone();
            next_id = s.windows.windows.len();
            s.windows.windows.push(Window{
                handle: None,
                widget: Box::new(widget),
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