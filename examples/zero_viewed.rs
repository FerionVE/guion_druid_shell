#![feature(type_alias_impl_trait)]

use std::borrow::Cow;
use std::ops::Range;

use guion::env::Env;
use guion::error::ResolveResult;
use guion::text::stor::TextStorMut;
use guion::view::View;
use guion::widget::Widget;
use guion::widget::dyn_tunnel::WidgetDyn;
use guion::widgets::area::Area;
use guion::widgets::button::Button;
use guion::widgets::checkbox::CheckBox;
use guion::widgets::label::Label;
use guion::widgets::pbar::ProgressBar;
use guion::widgets::splitpane::SplitPane;
use guion::widgets::textbox::TextBox;
use guion::{const_std_id, constraint, mutor};
use guion::layout::Orientation;
use guion::widgets::pane::Pane;
use guion_druid_shell::app::ArcApp;
use guion_druid_shell::app::windows::Windows;
use guion_druid_shell::example::ctx::ExampleCtx;
use guion_druid_shell::example::env::ExampleEnv;
use guion_druid_shell::style::cursor::Cusror;

pub struct Model {
    button51_count: u32,
    button52_count: u32,
    button_a_count: u32,
    button_b_count: u32,
    check: bool,
    progress: f32,
    splitpane: f32,
    area_scroll: (i32,i32),

    tbtext: String,
    tbscroll: (u32,u32),
    tbcursor: Cusror,
}

// Define some persistent WidgetIDs
const_std_id!(RootPane TopLabel Area51 Pane51 Button51 Button51Label Button52 Button52Label ProgBar Check CheckLabel Split2 ButtonA ButtonALabel ButtonB ButtonBLabel TextBoxx);

// Immutable immediate view, rendering and layouting done here
impl<'z> View<'z,ExampleEnv> for Model {
    type Viewed<'v,MutorFn> = dyn WidgetDyn<ExampleEnv> + 'v where MutorFn: 'static, 'z: 'v;
    type Mutable<'k> = Model;

    fn view<'d,MutorFn,DispatchFn,R>(&'d self, dispatch: DispatchFn, mutor: MutorFn, root: <ExampleEnv as Env>::RootRef<'_>, ctx: &mut <ExampleEnv as Env>::Context<'_>) -> R
    where
        MutorFn: for<'s,'c,'cc> Fn(
            <ExampleEnv as Env>::RootMut<'s>,&'s (),
            &mut (dyn for<'is,'iss> FnMut(ResolveResult<&'is mut Self::Mutable<'iss>>,&'iss (),&'c mut <ExampleEnv as Env>::Context<'cc>)),
            &'c mut <ExampleEnv as Env>::Context<'cc>
        ) + Send + Sync + Clone + 'static,
        DispatchFn: guion::dispatchor::ViewDispatch<'z,Self,MutorFn,R,ExampleEnv>,
    {
        let b_bounds = constraint!(~40-|64);
        let pb_bounds = constraint!(~0-|32~48);
        let cb_bounds = constraint!(~0-|24);

        let widget = Pane::<ExampleEnv,_>::new(
            RootPane(),
            Orientation::Vertical,
            (
                Label::immediate(TopLabel(),"Label"),
                Area::new(
                    Area51(),
                    Pane::new(
                        Pane51(),
                        Orientation::Horizontal,
                        (
                            Button::immediate(
                                Button51(),
                                Label::immediate(Button51Label(),self.button51_count),
                            )
                                .with_size(b_bounds)
                                .with_trigger_mut(mutor!(mutor =>| |s,c| {s.button51_count += 1; s.progress=(s.progress+0.1)%1.0;}; )),
                            Button::immediate(
                                Button52(),
                                Label::immediate(Button52Label(),self.button52_count),
                            )
                                .with_size(b_bounds)
                                .with_trigger_mut(mutor!(mutor =>| |s,c| {s.button52_count += 1; s.progress=(s.progress+0.1)%1.0;}; )),
                        ),
                    ),
                )
                    .with_state(self.area_scroll)
                    .with_scroll_updater(mutor!(mutor =>| |s,c,v| s.area_scroll = v.offset; )),
                    //.with_scroll_atomstate(mutor!(ExampleEnv;mutor => |s,c| &mut s.area_scroll; )),
                ProgressBar::new(ProgBar(), Orientation::Horizontal)
                    .with_value(self.progress)
                    .with_size(pb_bounds),
                CheckBox::new(Check(), self.check)
                    .with_caption(Label::immediate(CheckLabel(),"CheckBox"))
                    .with_size(cb_bounds)
                    .with_update(mutor!(mutor =>| |s,c,v| s.check = v; )),
                    //.with_atomstate(mutor!(ExampleEnv;mutor => |s,c| &mut s.check; )),
                SplitPane::new(
                    Split2(), Orientation::Horizontal, self.splitpane,
                    (
                        Button::immediate(
                            ButtonA(),
                            Label::immediate(ButtonALabel(),self.button_a_count),
                        )
                            .with_size(b_bounds)
                            .with_trigger_mut(mutor!(mutor =>| |s,c| {s.button_a_count += 1; s.progress=(s.progress+0.1)%1.0;}; )),
                        Button::immediate(
                            ButtonB(),
                            Label::immediate(ButtonBLabel(),self.button_b_count),
                        )
                            .with_size(b_bounds)
                            .with_trigger_mut(mutor!(mutor =>| |s,c| {s.button_b_count += 1; s.progress=(s.progress+0.1)%1.0;}; )),
                    ),
                )
                    .with_update(mutor!(mutor =>| |s,c,v| s.splitpane = v; )),
                //TextBox::new(StdID::new()),
                //ImmediateLabel{text:"Immediate Label".to_owned(),id:StdID::new()} ,
                //ImmediateTextBox{text:"Immediate TextBox".to_owned(),id:StdID::new()},
                TextBox::immediate_test(
                    TextBoxx(),
                    &self.tbtext,
                    self.tbscroll,
                    self.tbcursor,
                    mutor!(mutor =>| |s,c,tbupd,curs| {
                        //let tbupd: Option<(Range<usize>,Cow<'static,str>)> = tbupd;
                        if let Some(tbupd) = &tbupd {
                            TextStorMut::<ExampleEnv>::replace(&mut s.tbtext,tbupd.0.clone(),tbupd.1.as_ref());
                        }
                        //let curs: Option<Cusror> = curs;
                        if let Some(curs) = curs {
                            s.tbcursor = curs;
                        }
                    }; ),
                    mutor!(mutor =>| |s,c,scroll| s.tbscroll = scroll; ),
                ),
            ),
        );

        dispatch.call(widget.erase(), root, ctx)
    }
}

fn main() {
    let app = ArcApp::<ExampleEnv>::new(ExampleCtx::new());

    app.add_window::<Model,_>(
        |window| {
            window.set_title("zero")
        },
        Model{
            button51_count: 0,
            button52_count: 0,
            button_a_count: 0,
            button_b_count: 0,
            check: false,
            progress: 0.5,
            splitpane: 0.5,
            area_scroll: (0,0),

            tbtext: Default::default(),
            tbscroll: Default::default(),
            tbcursor: Default::default(),
        },
    );

    //while app.do_events() {}
    app.run();
}

/// required to correctly infer closure type
fn trig<E,F>(f: F) -> F where E: Env, F: for<'a,'b> FnMut(&'a mut E::Context<'b>) {
    f
}
