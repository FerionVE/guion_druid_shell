#![feature(type_alias_impl_trait)]

use guion::env::Env;
use guion::view::{View, DynViewDispatch};
use guion::view::mut_target::{MStatic, MuTarget, DynAtomStateMutTarget};
use guion::view::mutor_trait::{MutorToBuilderDyn, MutorToBuilderExt};
use guion::view::view_widget::view_widget_cb_if;
use guion::widget::Widget;
use guion::widget::as_widgets::fixed_idx::WidgetsFixedIdx;
use guion::widget::cache::DynWidgetCache;
use guion::widgets::area::{Area, ScrollUpdate};
use guion::widgets::button::Button;
use guion::widgets::checkbox::CheckBox;
use guion::widgets::label::Label;
use guion::widgets::pbar::ProgressBar;
use guion::widgets::splitpane::SplitPane;
use guion::widgets::textbox::TextBox;
use guion::constraint;
use guion::layout::Orientation;
use guion::widgets::pane::Pane;
use guion_druid_shell::app::ArcApp;
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

// Immutable immediate view, rendering and layouting done here
impl View<ExampleEnv> for Model {
    type WidgetCache = DynWidgetCache<ExampleEnv>;
    type Mutarget = MStatic<Model>;

    fn view<R>(&self, dispatch: DynViewDispatch<'_,R,ExampleEnv>, mutor: &(dyn MutorToBuilderDyn<(),Self::Mutarget,ExampleEnv>+'_), root: <ExampleEnv as Env>::RootRef<'_>, ctx: &mut <ExampleEnv as Env>::Context<'_>) -> R
    {
        let button_constraints = constraint!(~40-|64);
        let progressbar_constraints = constraint!(~0-|32~48);
        let checkbox_constraints = constraint!(~0-|24);

        let widget = Pane::<ExampleEnv,_>::new(
            Orientation::Vertical,
            WidgetsFixedIdx((
                Label::of_text("Label"),
                Area::new(
                    view_widget_cb_if(
                        || &self.submodel,
                        mutor,(),|s,_,callback,_,ctx|
                            (callback)(Ok(&mut SubModelMut{sub: &mut s.submodel, progress: &mut s.progress}),&(),ctx)
                    ),
                )
                    .with_state(self.area_scroll)
                    .with_scroll_updater(mutor.mutor_end_if((), |s,_,v: ScrollUpdate,_| s.area_scroll = v.offset ))
                    .with_scroll_atomstate(mutor.for_view_cb_if::<DynAtomStateMutTarget<(i32,i32)>,_,_>((), |s,_,cb,_,ctx| (cb)(Ok(&mut s.area_scroll),&(),ctx) ))
                    .with_scroll_updater_if(mutor,(), |s,_,v,_| s.area_scroll = v.offset )
                    .with_scroll_atomstate_if(mutor, (), |s,_,_| &mut s.area_scroll )
                    ,
                ProgressBar::new(Orientation::Horizontal)
                    .with_value(self.progress)
                    .with_size(progressbar_constraints),
                CheckBox::new(self.check)
                    .with_caption(Label::of_text("CheckBox"))
                    .with_size(checkbox_constraints)
                    .with_update_if(mutor, (), |s,_,v,_| s.check = v )
                    .with_atomstate_if(mutor, (), |s,_,_| &mut s.check )
                    ,
                SplitPane::new(
                    Orientation::Horizontal, self.splitpane,
                    (
                        Button::of_text(
                            Label::of_text(self.button_a_count),
                        )
                            .with_size(button_constraints)
                            .with_trigger_mut_if(mutor,(), |s,_,_,_| {s.button_a_count += 1; s.progress=(s.progress+0.1)%1.0;} ),
                        Button::of_text(
                            Label::of_text(self.button_b_count),
                        )
                            .with_size(button_constraints)
                            .with_trigger_mut_if(mutor,(), |s,_,_,_| {s.button_b_count += 1; s.progress=(s.progress+0.1)%1.0;} ),
                    ),
                )
                    .with_update_if(mutor, (), |s,_,v,_| s.splitpane = v )
                    .with_atomstate_if(mutor, (), |s,_,_| &mut s.splitpane )
                    ,
                TextBox::of_text(&self.tbtext)
                    .with_cursor(self.tbcursor)
                    .with_scroll(self.tbscroll)
                    .with_update_if(mutor, (), |s,_,update,_| {
                        update.apply_to_text(&mut s.tbtext);
                        update.apply_to_scroll(&mut s.tbscroll);
                        update.apply_to_cursor(&mut s.tbcursor);
                    })
            )),
        );

        (dispatch)(widget.erase(), root, ctx)
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

// Immutable immediate view, rendering and layouting done here
impl View<ExampleEnv> for SubModel {
    type WidgetCache = DynWidgetCache<ExampleEnv>;
    type Mutarget = SubModelMutTarget;

    fn view<R>(&self, dispatch: DynViewDispatch<'_,R,ExampleEnv>, mutor: &(dyn MutorToBuilderDyn<(),Self::Mutarget,ExampleEnv>+'_), root: <ExampleEnv as Env>::RootRef<'_>, ctx: &mut <ExampleEnv as Env>::Context<'_>) -> R
    {
        let button_constraints = constraint!(~40-|64);

        let widget = Pane::<ExampleEnv,_>::new(
            Orientation::Horizontal,
            WidgetsFixedIdx((
                Button::of_text(
                    Label::of_text(self.button51_count),
                )
                    .with_size(button_constraints)
                    .with_trigger_mut(mutor.mutor_end_if((), |s,_,_,_| {s.sub.button51_count += 1; *s.progress=((*s.progress)+0.1)%1.0;} )),
                Button::of_text(
                    Label::of_text(self.button52_count),
                )
                    .with_size(button_constraints)
                    .with_trigger_mut(mutor.mutor_end_if((), |s,_,_,_| {s.sub.button52_count += 1; *s.progress=((*s.progress)+0.1)%1.0;} )),
            )),
        );

        
        (dispatch)(widget.erase(), root, ctx)
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
