use guion::text::stor::TextStor;
use guion::text::stor::TextStorMut;
use guion::util::sma::SMA;
use guion::widget::as_widget::{AsWidget, AsWidgetMut};
use guion::widget::resolvable::{Resolvable, ResolvableMut};
use guion::widgets::button::Button;
use guion::widgets::label::Label;
use guion::widgets::textbox::TextBox;
use guion::widgets::textbox::state::Cursor;
use guion::widgets::util::LocalGlyphCache;
use guion::{const_std_id, constraint};
use guion::layout::Orientation;
use guion::widgets::pane::Pane;
use guion_druid_shell::app::ArcApp;
use guion_druid_shell::example::ctx::ExampleCtx;
use guion_druid_shell::example::env::ExampleEnv;

pub struct Model {
    current_cursor: Cursor,
    current_text: String,
    text_cache: LocalGlyphCache<ExampleEnv>,
    scroll: (u32,u32),
    cu: Option<u32>,
    history: Vec<(String,Cursor)>,
    current_entry: usize,
}

impl Model {
    fn restore(&mut self) {
        assert!(self.history.get(self.current_entry).is_some());
        if let Some((t,h)) = self.history.get(self.current_entry) {
            self.scroll = (0,0);
            self.cu = None;
            self.text_cache = None;
            self.current_text = t.clone();
            self.current_cursor = *h;
        }
    }
}

const_std_id!(RootE PaneID HPane ButtonUndo ButtonRedo LabelUndo LabelRedo TBox Doof);

impl AsWidget<ExampleEnv> for Model {
    fn as_ref(&self) -> Resolvable<ExampleEnv> {
        Resolvable::from_widget(
            Pane::new(
                PaneID(),
                Orientation::Vertical,
                (
                    Pane::new(
                        HPane(),
                        Orientation::Horizontal,
                        (
                            Button::immediate(
                                ButtonUndo(),
                                Label::immediate(LabelUndo(),"Undo")
                                    .with_size(constraint!(~0-|24)),
                            ),
                            Button::immediate(
                                ButtonRedo(),
                                Label::immediate(LabelRedo(),"Redo")
                                    .with_size(constraint!(~0-|24)),
                            ),
                        ),
                    ),
                    TextBox::immediate(TBox(),&self.current_text)
                        .with_states(self.scroll,self.current_cursor,self.cu),
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
        let sma = SMA::<ExampleEnv,_,_,_>::new(self);

        let sma_a = sma.fork();
        let sma_b = sma.fork();
        let sma_c = sma.fork();

        let sma_text = sma.fork_with_lens(|model| &mut model.current_text );
        let sma_scroll = sma.fork_with_lens(|model| &mut model.scroll );
        let sma_cursor= sma.fork_with_lens(|model| &mut model.current_cursor );
        let sma_cu = sma.fork_with_lens(|model| &mut model.cu );

        let text = sma_text.on_modification(move |_| {
            let model = &mut *sma_a.borrow_mut();

            model.history.truncate(model.current_entry+1);

            model.history.push((model.current_text.clone(),model.current_cursor));

            model.current_entry = model.history.len()-1;
        });

        let undo = move |_: &mut _| { // https://github.com/rust-lang/rust/issues/81511
            let model = &mut *sma_b.borrow_mut();

            model.current_entry = model.current_entry.saturating_sub(1);

            model.restore();
        };

        let redo = move |_: &mut _| { // https://github.com/rust-lang/rust/issues/81511
            let mut model = sma_c.borrow_mut();

            model.current_entry = model.history.len().saturating_sub(1).min( model.current_entry +1 );

            model.restore();
        };

        ResolvableMut::from_widget(
            Pane::new(
                PaneID(),
                Orientation::Vertical,
                (
                    Pane::new(
                        HPane(),
                        Orientation::Horizontal,
                        (
                            Button::immediate(
                                ButtonUndo(),
                                Label::immediate(LabelUndo(),"".immutable()),
                            )
                                .with_trigger_mut(undo),
                            Button::immediate(
                                ButtonRedo(),
                                Label::immediate(LabelRedo(),"".immutable()),
                            )
                                .with_trigger_mut(redo),
                        ),
                    ),
                    TextBox::immediate(TBox(),text)
                        .with_states(sma_scroll,sma_cursor,sma_cu),
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
            w.set_title("tur");
        },
        Model{
            current_cursor: Default::default(),
            current_text: Default::default(),
            text_cache: Default::default(),
            scroll: Default::default(),
            cu: Default::default(),
            history: vec![("".to_owned(),Default::default())],
            current_entry: 0,
        },
    );

    //while app.do_events() {}
    app.run();
}