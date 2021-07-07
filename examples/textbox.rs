extern crate guion_druid_shell;

use guion::{id::standard::StdID, widgets::{textbox::{TextBox}}};
use guion_druid_shell::app::{ArcApp};
use guion_druid_shell::example::ctx::ExampleCtx;
use guion_druid_shell::example::env::{ExampleEnv};

fn main() {
    let app = ArcApp::<ExampleEnv>::new(ExampleCtx::new());

    let g = TextBox::new(StdID::new())
        .with_text("".to_owned());

    app.add_window(
        |w| {
            w.set_title("TextBoxion")
        },
        g,
    );

    app.run();
}
