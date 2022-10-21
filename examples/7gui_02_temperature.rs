use druid_shell::kurbo::Size;
use guion::env::Env;
use guion::view::DynViewDispatch;
use guion::view::View;
use guion::view::mut_target::MStatic;
use guion::view::mutor_trait::MutorToBuilderDyn;
use guion::widget::Widget;
use guion::widget::as_widgets::fixed_idx::WidgetsFixedIdx;
use guion::widget::cache::DynWidgetCache;
use guion::widgets::label::Label;
use guion::widgets::textbox::TextBox;
use guion::layout::Orientation;
use guion::widgets::pane::Pane;
use guion::widgets::textbox::state::TextBoxMeta;
use guion_druid_shell::app::ArcApp;
use guion_druid_shell::example::ctx::ExampleCtx;
use guion_druid_shell::example::env::ExampleEnv;

pub struct Model {
    celsius: String,
    celsius_meta: TextBoxMeta<ExampleEnv>,
    fahrenheit: String,
    fahrenheit_meta: TextBoxMeta<ExampleEnv>,
}

impl View<ExampleEnv> for Model {
    type WidgetCache = DynWidgetCache<ExampleEnv>;
    type Mutarget = MStatic<Self>;

    fn view<R>(&self, dispatch: DynViewDispatch<'_,R,ExampleEnv>, mutor: &(dyn MutorToBuilderDyn<(),Self::Mutarget,ExampleEnv>+'_), root: <ExampleEnv as Env>::RootRef<'_>, ctx: &mut <ExampleEnv as Env>::Context<'_>) -> R {
        let widget = Pane::<ExampleEnv,_>::new(
            Orientation::Horizontal,
            WidgetsFixedIdx((
                TextBox::of_text(&self.celsius)
                    .with_meta(&self.celsius_meta)
                    .with_update_if(mutor, (), |s,_,update,_|{
                        update.apply_to_text(&mut s.celsius);
                        update.apply_to_meta(&mut s.celsius_meta);
                        if let Ok(v) = s.celsius.trim().parse::<i64>() {
                            s.fahrenheit = ((v*9/5)+32).to_string();
                        }
                    }),
                Label::of_text("Celsius = "),
                TextBox::of_text(&self.fahrenheit)
                    .with_meta(&self.fahrenheit_meta)
                    .with_update_if(mutor, (), |s,_,update,_|{
                        update.apply_to_text(&mut s.fahrenheit);
                        update.apply_to_meta(&mut s.fahrenheit_meta);
                        if let Ok(v) = s.fahrenheit.trim().parse::<i64>() {
                            s.celsius = ((v-32)*5/9).to_string();
                        }
                    }),
                Label::of_text("Fahrenheit"),
            )),
        );

        (dispatch)(widget.erase(), root, ctx)
    }
}
fn main() {
    let app = ArcApp::<ExampleEnv>::new(ExampleCtx::new());

    app.add_window(
        |w| {
            w.set_title("7GUIs 02 Temperature");
            w.set_size(Size::new(512.0, 32.0));
        },
        Model{
            celsius: "0".to_owned(),
            celsius_meta: Default::default(),
            fahrenheit: "0".to_owned(),
            fahrenheit_meta: Default::default(),
        },
    );

    //while app.do_events() {}
    app.run();
}
