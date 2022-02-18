use std::any::Any;
use std::sync::Arc;

use guion::env::Env;
use guion::error::ResolveResult;
use guion::util::bounds::Dims;
use guion::view::View;
use guion::widget::Widget;

pub mod handle;

pub struct Window<E> where E: Env {
    pub handle: Option<druid_shell::WindowHandle>,
    pub widget: Box<dyn WindowState<E>+'static>,
    pub dims: Dims,
}

pub trait WindowState<E>: Any where E: Env {
    fn view<'s>(&'s self, root: E::RootRef<'s>, ctx: &mut E::Context<'_>) -> Box<dyn Widget<E>+'s>;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct AWindowState<E,S> where
    E: Env,
    S: 'static,
    for<'a> &'a S: View<E,&'static (dyn for<'r> Fn(E::RootMut<'r>,&'r (),&mut E::Context<'_>)->ResolveResult<&'r mut S> + Send + Sync)>,
{
    state: S,
    mut_fn: &'static (dyn for<'r> Fn(E::RootMut<'r>,&'r (),&mut E::Context<'_>)->ResolveResult<&'r mut S> + Send + Sync),
}

impl<E,S> AWindowState<E,S> where
    E: Env,
    S: 'static,
    for<'a> &'a S: View<E,&'static (dyn for<'r> Fn(E::RootMut<'r>,&'r (),&mut E::Context<'_>)->ResolveResult<&'r mut S> + Send + Sync)>,
{
    pub fn new(state: S, reachor: impl for<'r> Fn(E::RootMut<'r>,&'r ())->&'r mut (dyn WindowState<E>) + Clone + Send + Sync + 'static) -> Box<dyn WindowState<E>> {
        fn funnel<E,F,S>(f: F) -> F where E: Env, F: for<'r> Fn(E::RootMut<'r>,&'r (),&mut E::Context<'_>)->ResolveResult<&'r mut S> + Clone + Send + Sync + 'static {
            f
        }
        let f = funnel::<E,_,_>(
            move |r,_,_| Ok( &mut reachor(r,&()).as_any_mut().downcast_mut::<Self>().unwrap().state )
        );
        Box::new(Self {
            state,
            mut_fn: Box::leak(Box::new(f)),
        })
    }
}

impl<E,S> WindowState<E> for AWindowState<E,S> where
    E: Env,
    S: 'static,
    for<'a> &'a S: View<E,&'static (dyn for<'r> Fn(E::RootMut<'r>,&'r (),&mut E::Context<'_>)->ResolveResult<&'r mut S> + Send + Sync)>,
{
    fn view<'s>(&'s self, root: <E as Env>::RootRef<'s>, ctx: &mut <E as Env>::Context<'_>) -> Box<dyn Widget<E>+'s> {
        (&self.state).view(self.mut_fn,root,ctx).boxed()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
