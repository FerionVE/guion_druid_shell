#![feature(type_alias_impl_trait)]
//! MVC-style 7gui example to increment counter

use guion::env::Env;
use guion::error::ResolveResult;
use guion::view::View;
use guion::widget::Widget;
use guion::widgets::button::Button;
use guion::widgets::label::Label;
use guion::{const_std_id, constraint, mutor, impl_view};
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
// impl<'o,MutFn> View<ExampleEnv,MutFn> for &'o Model where
//     MutFn: for<'a> Fn(&'a mut Windows<ExampleEnv>,&'a (),&mut ExampleCtx)->ResolveResult<&'a mut Model> + Clone + Send + Sync + 'static,
// {
//     type Viewed = impl Widget<ExampleEnv>;

//     fn view(self, mutor: MutFn, _: &Windows<ExampleEnv>, _: &mut ExampleCtx) -> Self::Viewed {
//         Pane::new(
//             PaneID(),
//             Orientation::Horizontal,
//             (
//                 Label::immediate(LabelID(),self.count)
//                     .with_size(constraint!(~0-@2.0|24)),
//                 Button::immediate(ButtonID(),Label::immediate(ButtonLabelID(),"Increment"))
//                     .with_trigger_mut(mutor!(mutor =>| |s,c| s.count += 1 )),
//             ),
//         )
//     }
// }

// Immutable immediate view, rendering and layouting done here
impl_view!(
    ExampleEnv;('o) for &'o Model : <'a> &'a mut Model {
        fn view(self, mutor: MutFn, _: &Windows<ExampleEnv>, _: &mut ExampleCtx) -> Self::Viewed {
            Pane::new(
                PaneID(),
                Orientation::Horizontal,
                (
                    Label::immediate(LabelID(),self.count)
                        .with_size(constraint!(~0-@2.0|24)),
                    Button::immediate(ButtonID(),Label::immediate(ButtonLabelID(),"Increment"))
                        .with_trigger_mut(mutor!(mutor =>| |s,c| s.count += 1 )),
                ),
            )
        }
    }
);

fn main() {
    let app = ArcApp::<ExampleEnv>::new(ExampleCtx::new());

    app.add_window::<Model,_>(
        |window| {
            window.set_title("7gui1")
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
