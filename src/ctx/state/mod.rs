use druid_shell::Clipboard;


/// place this inside your Context and implement AsRefMut<DSState>
pub struct DSState {
    pub(crate) clipboard: Option<Clipboard>,
}

impl DSState {
    pub(crate) fn new() -> Self {
        Self {
            clipboard: None,
        }
    }
}