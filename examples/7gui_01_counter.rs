#![feature(type_alias_impl_trait)]
//! MVC-style 7gui example to increment counter

use guion::env::Env;
use guion::view::{View, DynViewDispatch};
use guion::view::mut_target::MStatic;
use guion::view::mutor_trait::{MutorToBuilderDyn};
use guion::widget::Widget;
use guion::widget::as_widgets::fixed_idx::WidgetsFixedIdx;
use guion::widget::cache::DynWidgetCache;
use guion::widgets::button::Button;
use guion::widgets::label::Label;
use guion::constraint;
use guion::layout::Orientation;
use guion::widgets::pane::Pane;
use guion_druid_shell::app::ArcApp;
use guion_druid_shell::example::ctx::ExampleCtx;
use guion_druid_shell::example::env::ExampleEnv;

// Simple model
pub struct Model {
    count: u32,
}

// Immutable immediate view, rendering and layouting done here
impl View<ExampleEnv> for Model {
    type WidgetCache = DynWidgetCache<ExampleEnv>;
    type Mutarget = MStatic<Model>;

    fn view<R>(&self, dispatch: DynViewDispatch<'_,R,ExampleEnv>, mutor: &(dyn MutorToBuilderDyn<(),Self::Mutarget,ExampleEnv>+'_), root: <ExampleEnv as Env>::RootRef<'_>, ctx: &mut <ExampleEnv as Env>::Context<'_>) -> R
    {
        let widget = Pane::<ExampleEnv,_>::new(
            Orientation::Horizontal,
            WidgetsFixedIdx((
                Label::of_text(self.count)
                    .with_size(constraint!(~0-@2.0|24)),
                Button::of_text(Label::of_text("Increment"))
                    //.with_trigger_mut(mutor.mutor_end_if((), |s,_,_,_| s.count += 1 ))
                    .with_trigger_mut_if(mutor, (), |s,_,_,_| s.count += 1 ),
            )),
        );

        (dispatch)(widget.erase(), root, ctx)
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
