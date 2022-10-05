#![feature(type_alias_impl_trait)]

use std::borrow::Cow;
use std::ops::{Range, Sub};

use guion::aliases::ETCurSel;
use guion::env::Env;
use guion::error::ResolveResult;
use guion::text::stor::TextStorMut;
use guion::view::View;
use guion::view::mut_target::{MStatic, MuTarget, DynAtomStateMutTarget};
use guion::view::mutor_trait::MutorTo;
use guion::view::view_widget::{view_widget_cb_if, view_widget_cb_if_dyn};
use guion::widget::Widget;
use guion::widget::cache::DynWidgetCache;
use guion::widget::dyn_tunnel::WidgetDyn;
use guion::widgets::area::{Area, ScrollUpdate};
use guion::widgets::button::Button;
use guion::widgets::checkbox::CheckBox;
use guion::widgets::label::Label;
use guion::widgets::pbar::ProgressBar;
use guion::widgets::splitpane::SplitPane;
use guion::widgets::textbox::TextBox;
use guion::widgets::util::state::AtomStateMut;
use guion::{const_std_id, constraint};
use guion::layout::Orientation;
use guion::widgets::pane::Pane;
use guion_druid_shell::app::ArcApp;
use guion_druid_shell::app::windows::Windows;
use guion_druid_shell::example::ctx::ExampleCtx;
use guion_druid_shell::example::env::ExampleEnv;
use guion_druid_shell::style::cursor::Cusror;

pub struct Model {
    submodel: SubModel,
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
impl View<ExampleEnv> for Model {
    type Viewed<'v,'z,MutorFn> = dyn WidgetDyn<ExampleEnv> + 'v where MutorFn: 'static, 'z: 'v, Self: 'z;
    type WidgetCache = DynWidgetCache<ExampleEnv>;
    type Mutarget = MStatic<Model>;

    fn view<'d,MutorFn,DispatchFn,R>(&self, dispatch: DispatchFn, mutor: MutorFn, root: <ExampleEnv as Env>::RootRef<'_>, ctx: &mut <ExampleEnv as Env>::Context<'_>) -> R
    where
        MutorFn: MutorTo<(),Self::Mutarget,ExampleEnv>,
        DispatchFn: guion::dispatchor::ViewDispatch<'d,Self,MutorFn,R,ExampleEnv>,
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
                    view_widget_cb_if(
                        || &self.submodel,
                        (),mutor.clone(),|s,_,callback,_,ctx|
                            (callback)(Ok(&mut SubModelMut{sub: &mut s.submodel, progress: &mut s.progress}),&(),ctx)
                    ),
                )
                    .with_state(self.area_scroll)
                    .with_scroll_updater(mutor.mutor_end_if((), |s,_,v: ScrollUpdate,_| s.area_scroll = v.offset ))
                    .with_scroll_atomstate(mutor.for_view_cb_if::<DynAtomStateMutTarget<(i32,i32)>,_,_>((), |s,_,cb,_,ctx| (cb)(Ok(&mut s.area_scroll),&(),ctx) ))
                    ,
                ProgressBar::new(ProgBar(), Orientation::Horizontal)
                    .with_value(self.progress)
                    .with_size(pb_bounds),
                CheckBox::new(Check(), self.check)
                    .with_caption(Label::immediate(CheckLabel(),"CheckBox"))
                    .with_size(cb_bounds)
                    .with_update(mutor.mutor_end_if((), |s,_,v,_| s.check = v ))
                    .with_atomstate(mutor.for_view_cb_if::<DynAtomStateMutTarget<bool>,_,_>((), |s,_,cb,_,ctx| (cb)(Ok(&mut s.check),&(),ctx) ))
                    ,
                SplitPane::new(
                    Split2(), Orientation::Horizontal, self.splitpane,
                    (
                        Button::immediate(
                            ButtonA(),
                            Label::immediate(ButtonALabel(),self.button_a_count),
                        )
                            .with_size(b_bounds)
                            .with_trigger_mut(mutor.mutor_end_if((), |s,_,_,_| {s.button_a_count += 1; s.progress=(s.progress+0.1)%1.0;} )),
                        Button::immediate(
                            ButtonB(),
                            Label::immediate(ButtonBLabel(),self.button_b_count),
                        )
                            .with_size(b_bounds)
                            .with_trigger_mut(mutor.mutor_end_if((), |s,_,_,_| {s.button_b_count += 1; s.progress=(s.progress+0.1)%1.0;} )),
                    ),
                )
                    .with_update(mutor.mutor_end_if((), |s,_,v,_| s.splitpane = v ))
                    .with_atomstate(mutor.for_view_cb_if::<DynAtomStateMutTarget<f32>,_,_>((), |s,_,cb,_,ctx| (cb)(Ok(&mut s.splitpane),&(),ctx) ))
                    ,
                TextBox::immediate_test(
                    TextBoxx(),
                    &self.tbtext,
                    self.tbscroll,
                    self.tbcursor,
                    mutor.mutor_end_if((), |s,_,(tbupd,curs): (Option<(Range<usize>,Cow<'static,str>)>,Option<ETCurSel<ExampleEnv>>),_| {
                        if let Some(tbupd) = &tbupd {
                            TextStorMut::<ExampleEnv>::replace(&mut s.tbtext,tbupd.0.clone(),tbupd.1.as_ref());
                        }
                        if let Some(curs) = curs {
                            s.tbcursor = curs;
                        }
                    }),
                    mutor.mutor_end_if((), |s,_,scroll,_| s.tbscroll = scroll ),
                ),
            ),
        );

        dispatch.call(widget.erase(), root, ctx)
    }
}

pub struct SubModel {
    button51_count: u32,
    button52_count: u32,
}

pub struct SubModelMut<'a> {
    sub: &'a mut SubModel,
    progress: &'a mut f32,
}

pub struct SubModelMutTarget;

impl<E> MuTarget<E> for SubModelMutTarget {
    type Mutable<'k> = SubModelMut<'k>;
}

impl View<ExampleEnv> for SubModel {
    type Viewed<'v,'z,MutorFn> = dyn WidgetDyn<ExampleEnv> + 'v where MutorFn: 'static, 'z: 'v, Self: 'z;
    type WidgetCache = DynWidgetCache<ExampleEnv>;
    type Mutarget = SubModelMutTarget;

    fn view<'d,MutorFn,DispatchFn,R>(&self, dispatch: DispatchFn, mutor: MutorFn, root: <ExampleEnv as Env>::RootRef<'_>, ctx: &mut <ExampleEnv as Env>::Context<'_>) -> R
    where
        MutorFn: MutorTo<(),ExampleEnv,Target=Self::Mutarget>,
        DispatchFn: guion::dispatchor::ViewDispatch<'d,Self,MutorFn,R,ExampleEnv>,
    {
        let b_bounds = constraint!(~40-|64);
        let pb_bounds = constraint!(~0-|32~48);
        let cb_bounds = constraint!(~0-|24);

        let widget = Pane::<ExampleEnv,_>::new(
            Pane51(),
            Orientation::Horizontal,
            (
                Button::immediate(
                    Button51(),
                    Label::immediate(Button51Label(),self.button51_count),
                )
                    .with_size(b_bounds)
                    .with_trigger_mut(mutor.mutor_end_if((), |s,_,_,_| {s.sub.button51_count += 1; *s.progress=((*s.progress)+0.1)%1.0;} )),
                Button::immediate(
                    Button52(),
                    Label::immediate(Button52Label(),self.button52_count),
                )
                    .with_size(b_bounds)
                    .with_trigger_mut(mutor.mutor_end_if((), |s,_,_,_| {s.sub.button52_count += 1; *s.progress=((*s.progress)+0.1)%1.0;} )),
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
            submodel: SubModel {
                button51_count: 0,
                button52_count: 0,
            },
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
