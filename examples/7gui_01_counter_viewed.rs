#![feature(type_alias_impl_trait)]
//! MVC-style 7gui example to increment counter

use guion::dispatchor::AsWidgetClosure;
use guion::env::Env;
use guion::error::ResolveResult;
use guion::view::View;
use guion::widget::Widget;
use guion::widget::as_widget::AsWidget;
use guion::widget::cache::DynWidgetCache;
use guion::widget::dyn_tunnel::WidgetDyn;
use guion::widgets::button::Button;
use guion::widgets::label::Label;
use guion::{const_std_id, constraint, mutor};
use guion::layout::Orientation;
use guion::widgets::pane::Pane;
use guion_druid_shell::app::ArcApp;
use guion_druid_shell::app::windows::Windows;
use guion_druid_shell::example::ctx::ExampleCtx;
use guion_druid_shell::example::env::ExampleEnv;

// Simple model
pub struct Model {
    count: u32,
}

// Define some persistent WidgetIDs
const_std_id!(RootE PaneID LabelID ButtonID ButtonLabelID);

// Immutable immediate view, rendering and layouting done here
impl View<ExampleEnv> for Model {
    type Viewed<'v,'z,MutorFn> = dyn WidgetDyn<ExampleEnv> + 'v where MutorFn: 'static, 'z: 'v, Self: 'z;
    type WidgetCache = DynWidgetCache<ExampleEnv>;
    type Mutable<'k> = Model;

    fn view<'d,MutorFn,DispatchFn,R>(&self, dispatch: DispatchFn, mutor: MutorFn, root: <ExampleEnv as Env>::RootRef<'_>, ctx: &mut <ExampleEnv as Env>::Context<'_>) -> R
    where
        MutorFn: for<'s,'c,'cc> Fn(
            <ExampleEnv as Env>::RootMut<'s>,&'s (),
            &mut (dyn for<'is,'iss> FnMut(ResolveResult<&'is mut Self::Mutable<'iss>>,&'iss (),&'c mut <ExampleEnv as Env>::Context<'cc>)),
            &'c mut <ExampleEnv as Env>::Context<'cc>
        ) + Send + Sync + Clone + 'static,
        DispatchFn: guion::dispatchor::ViewDispatch<'d,Self,MutorFn,R,ExampleEnv>,
    {
        let widget = Pane::<ExampleEnv,_>::new(
            PaneID(),
            Orientation::Horizontal,
            (
                Label::immediate(LabelID(),self.count)
                    .with_size(constraint!(~0-@2.0|24)),
                Button::immediate(ButtonID(),Label::immediate(ButtonLabelID(),"Increment"))
                    .with_trigger_mut(mutor!(mutor =>| |s,c| s.count += 1; )),
            ),
        );

        dispatch.call(widget.erase(), root, ctx)
    }
}

fn main() {
    let app = ArcApp::<ExampleEnv>::new(ExampleCtx::new());

    app.add_window::<Model,_>(
        |window| {
            window.set_title("7GUIs 01 Counter")
        },
        Model{count:0},
    );

    //while app.do_events() {}
    app.run();
}

/// required to correctly infer closure type
fn trig<E,F>(f: F) -> F where E: Env, F: for<'a,'b> FnMut(&'a mut E::Context<'b>) {
    f
}
