//! MVC-style 7gui example to increment counter

use guion::env::Env;
use guion::text::stor::TextStor;
use guion::widget::as_widget::{AsWidget, AsWidgetMut};
use guion::widget::resolvable::{Resolvable, ResolvableMut};
use guion::widgets::button::Button;
use guion::widgets::label::Label;
use guion::{const_std_id, constraint};
use guion::layout::Orientation;
use guion::widgets::pane::Pane;
use guion_druid_shell::app::ArcApp;
use guion_druid_shell::example::ctx::ExampleCtx;
use guion_druid_shell::example::env::ExampleEnv;

// Simple model
pub struct Model {
    count: u32,
}

// Define some persistent WidgetIDs
const_std_id!(RootE PaneID LabelID ButtonID ButtonLabelID);

// Immutable immediate view, rendering and layouting done here
impl AsWidget<ExampleEnv> for Model {
    fn as_ref(&self) -> Resolvable<ExampleEnv> {
        Resolvable::<'_,ExampleEnv>::from_widget(
            Pane::new(
                PaneID(),
                Orientation::Horizontal,
                (
                    Label::immediate(LabelID(),self.count)
                        .with_size(constraint!(~0-@2.0|24)),
                    Button::immediate(ButtonID(),Label::immediate(ButtonLabelID(),"Increment")),
                ),
            )
        )
    }
    fn into_ref<'w>(self) -> Resolvable<'w,ExampleEnv> where Self: 'w {
        unimplemented!()
    }
}
// Mutable immediate view for state mutation and controller, identical Widget tree to immutable view
impl AsWidgetMut<ExampleEnv> for Model {
    fn as_mut(&mut self) -> guion::widget::resolvable::ResolvableMut<ExampleEnv> {
        let count = self.count;
        
        // Closure to increment count
        let increment = trig(move |_| self.count += 1);

        ResolvableMut::<'_,ExampleEnv>::from_widget(
            Pane::new(
                PaneID(),
                Orientation::Horizontal,
                (
                    Label::immediate(LabelID(),count.immutable()),
                    Button::immediate(ButtonID(),Label::immediate(ButtonLabelID(),"Increment".to_owned()))
                        .with_trigger_mut(increment), // Pass closure to Button trigger
                ),
            )
        )
    }
    fn into_mut<'w>(self) -> guion::widget::resolvable::ResolvableMut<'w,ExampleEnv> where Self: 'w {
        unimplemented!()
    }
}

fn main() {
    let app = ArcApp::<ExampleEnv>::new(ExampleCtx::new());

    app.add_window(
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
