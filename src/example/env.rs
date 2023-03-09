use druid_shell::piet::Piet;
use guion::backend::Backend;
use guion::env::Env;
use guion::event::standard::dyn_evt::DynEvent;
use guion::layout::size::StdGonstraints;
use guion::util::AsRefMut;
use guion::util::error::GuionError;
use std::any::Any;
use std::fmt::Debug;

use crate::app::ModelRoot;
use crate::app::windows::Windows;
use crate::event::key::Key;
use crate::render::Render;
use crate::style::Style;
use crate::style::font::Glyphs;

use super::StupidEventDowncastMap;
use super::ctx::ExampleCtx;
use super::valid::ExampleValidState;

#[derive(Clone,PartialEq,Default,Copy)]
pub struct ExampleEnv;
#[derive(Clone,PartialEq,Default,Copy)]
pub struct ExampleBackend;

impl Env for ExampleEnv {
    type Backend = ExampleBackend;
    type Context<'a> = ExampleCtx<'a>;
    type RootRef<'a> = &'a ModelRoot;
    type RootMut<'a> = &'a mut ModelRoot;
    type ValidState = ExampleValidState;
    type Message = Box<dyn Any>;
    type Error = GuionError<ExampleEnv>;
    type Phantom = std::convert::Infallible;
    type EventDowncastMap = 
        // ()
        StupidEventDowncastMap
    ;
    type WidgetRoot = Windows<Self>;
    type CtxTCellOwner = (); //TODO
}

impl Backend<ExampleEnv> for ExampleBackend {
    type Renderer<'a> = Render<'a,ExampleEnv>;
    type Event = DynEvent<ExampleEnv,Key,ExampleDest>; //TODO ditch Consuming
    type Style = Style;
    type Size = StdGonstraints;
    type TextLayout = Glyphs;
}

//TODO move this to guion
#[derive(Clone)]
pub struct ExampleDest {
    pub v: usize,
}

impl guion::event::Destination for ExampleDest {
    const ROOT: Self = Self{v: 0};
    const FOCUSED: Self = Self{v: 1};
    const HOVERED: Self = Self{v: 2};
    const INVALID: Self = Self{v: std::usize::MAX};
}

//guion::impl_env_stds!(ExampleEnv);
//guion::impl_remote_state!(u8,ExampleEnv);
//guion::impl_as_widget_for_path!(ExampleEnv;StandardPath);

/*impl AsWidget<ExampleEnv> for <ExampleEnv as Env>::WidgetPath {
    #[inline]
    fn as_ref(&self) -> guion::widget::resolvable::Resolvable<ExampleEnv> {
        guion::widget::resolvable::Resolvable::Path(self.clone().into())
    }
    #[inline]
    fn into_ref<'w>(self) -> guion::widget::resolvable::Resolvable<'w,ExampleEnv> where Self: 'w {
        guion::widget::resolvable::Resolvable::Path(self.clone().into())
    }
}
impl AsWidgetMut<ExampleEnv> for <ExampleEnv as Env>::WidgetPath {
    #[inline]
    fn as_mut(&mut self) -> guion::widget::resolvable::ResolvableMut<ExampleEnv> {
        guion::widget::resolvable::ResolvableMut::Path(self.clone().into())
    }
    #[inline]
    fn into_mut<'w>(self) -> guion::widget::resolvable::ResolvableMut<'w,ExampleEnv> where Self: 'w {
        guion::widget::resolvable::ResolvableMut::Path(self.clone().into())
    }
}*/

impl Debug for ExampleEnv {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
