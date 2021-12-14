use guion::text::stor::TextStor;
use guion::widget::as_widget::{AsWidget, AsWidgetMut};
use guion::widget::resolvable::{Resolvable, ResolvableMut};
use guion::widgets::textbox::TextBox;
use guion::const_std_id;
use guion_druid_shell::app::ArcApp;
use guion_druid_shell::example::ctx::ExampleCtx;
use guion_druid_shell::example::env::ExampleEnv;

pub struct Model {
    text: String,
}

const_std_id!(TBox);

impl AsWidget<ExampleEnv> for Model {
    fn as_ref(&self) -> Resolvable<ExampleEnv> {
        Resolvable::from_widget(
            TextBox::immediate(TBox(),&self.text)
        )
    }
    fn into_ref<'w>(self) -> Resolvable<'w,ExampleEnv> where Self: 'w {
        unimplemented!()
    }
}
impl AsWidgetMut<ExampleEnv> for Model {
    fn as_mut(&mut self) -> guion::widget::resolvable::ResolvableMut<ExampleEnv> {
        ResolvableMut::from_widget(
            TextBox::immediate(TBox(),(&self.text).immutable())
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
            w.set_title("tur");
        },
        Model{
            text: include_str!("fake_text.rs").to_owned(),
        },
    );

    //while app.do_events() {}
    app.run();
}
