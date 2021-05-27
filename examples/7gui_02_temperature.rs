use druid_shell::kurbo::Size;
use guion::text::stor::TextStor;
use guion::text::stor::TextStorMut;
use guion::util::sma::SMA;
use guion::widget::as_widget::{AsWidget, AsWidgetMut};
use guion::widget::resolvable::{Resolvable, ResolvableMut};
use guion::widgets::label::Label;
use guion::widgets::textbox::TextBox;
use guion::{const_std_id};
use guion::layout::Orientation;
use guion::widgets::pane::Pane;
use guion_druid_shell::app::ArcApp;
use guion_druid_shell::example::ctx::ExampleCtx;
use guion_druid_shell::example::env::ExampleEnv;

pub struct Model {
    celsius: String,
    fahrenheit: String,
}

const_std_id!(PaneID TextC LabelC LabelF TextF);

impl AsWidget<ExampleEnv> for Model {
    fn as_ref(&self) -> Resolvable<ExampleEnv> {
        Resolvable::from_widget(
            Pane::new(
                PaneID(),
                Orientation::Horizontal,
                (
                    TextBox::immediate(TextC(), &self.celsius),
                    Label::immediate(LabelC(), "Celsius = "),
                    TextBox::immediate(TextF(), &self.fahrenheit),
                    Label::immediate(LabelF(), "Fahrenheit"),
                ),
            )
        )
    }
    fn into_ref<'w>(self) -> Resolvable<'w,ExampleEnv> where Self: 'w {
        unimplemented!()
    }
}
impl AsWidgetMut<ExampleEnv> for Model {
    fn as_mut(&mut self) -> guion::widget::resolvable::ResolvableMut<ExampleEnv> {
        // Shared Mutable Access for tree and closures
        let sma = SMA::new(self);

        let sma_celsius = sma.fork_with_lens(|model| &mut model.celsius );
        let sma_fahrenheit = sma.fork_with_lens(|model| &mut model.fahrenheit );

        let sma_a = sma.fork();
        let sma_b = sma.fork();

        ResolvableMut::from_widget(
            Pane::new(
                PaneID(),
                Orientation::Horizontal,
                (
                    TextBox::immediate(
                        TextC(),
                        sma_celsius
                        .on_modification(move |_| {
                            let model = &mut *sma_a.borrow_mut();
                            if let Ok(v) = model.celsius.trim().parse::<i64>() {
                                model.fahrenheit = ((v*9/5)+32).to_string();
                            }
                        })
                    ),
                    Label::immediate(LabelC(), "".immutable()),
                    TextBox::immediate(
                        TextF(),
                        sma_fahrenheit
                        .on_modification(move |_| {
                            let model = &mut *sma_b.borrow_mut();
                            if let Ok(v) = model.fahrenheit.trim().parse::<i64>() {
                                model.celsius = ((v-32)*5/9).to_string();
                            }
                        })
                    ),
                    Label::immediate(LabelF(), "".immutable()),
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
        |w| {
            w.set_title("7gui2");
            w.set_size(Size::new(512.0, 32.0));
        },
        Model{
            celsius: "0".to_owned(),
            fahrenheit: "0".to_owned(),
        },
    );

    //while app.do_events() {}
    app.run();
}