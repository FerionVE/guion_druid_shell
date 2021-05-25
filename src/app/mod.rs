use std::sync::{Arc, Mutex};

use druid_shell::piet::CairoTextLayout;
use druid_shell::{WindowBuilder, kurbo};
use guion::aliases::*;
use guion::env::Env;
use guion::event::filter::StdFilter;
use guion::event::imp::StdVarSup;
use guion::id::WidgetIDAlloc;
use guion::render::widgets::RenderStdWidgets;
use guion::style::standard::cursor::StdCursor;
use guion::util::AsRefMut;
use guion::util::bounds::Dims;
use guion::widget::as_widget::AsWidgetMut;

use crate::ctx::state::DSState;
use crate::render::Render;

use self::window::Window;
use self::window::handle::WHandle;
use self::windows::Windows;

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
    ctx: E::Context,
}

impl<E> ArcApp<E> where E: Env, E::Context: AsRefMut<DSState>, E::WidgetID: WidgetIDAlloc {
    pub fn new(mut ctx: E::Context) -> Self {
        let ds_app = druid_shell::Application::new().unwrap(); //TODO error handling in all crate
        let windows = Windows{windows: vec![],_id: WidgetIDAlloc::new_id()};
        ctx.as_mut().clipboard = Some(ds_app.clipboard());
        let app = App {
            ds_app,
            windows,
            ctx
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
    E: Env,
    E::Context: AsRefMut<DSState>,
    ECQueue<E>: AsRefMut<crate::ctx::queue::Queue<E>>,
    EEvent<E>: StdVarSup<E>,
    EEKey<E>: From<crate::event::key::Key>,
    EEFilter<E>: From<StdFilter<E>>,
    E::Storage: AsRefMut<Windows<E>>,
    Windows<E>: AsRefMut<E::Storage>,
    ERenderer<E>: AsRefMut<Render<E>> + RenderStdWidgets<E>,
    Render<E>: AsRefMut<ERenderer<E>> + RenderStdWidgets<E>,
    ETextLayout<E>: AsRefMut<CairoTextLayout>, //TODO use Piet trait variant
    ESCursor<E>: Into<StdCursor>,  //TODO Into<DruidCursor>
{
    pub fn add_window(&self, f: impl FnOnce(&mut WindowBuilder), widget: impl AsWidgetMut<E>+'static) {
        let app;
        let next_id;
        {
            let mut s = self.inner.lock().unwrap();
            app = s.ds_app.clone();
            next_id = s.windows.windows.len();
            s.windows.windows.push(Window{handle:None,widget:Box::new(widget),dims:Default::default()});
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