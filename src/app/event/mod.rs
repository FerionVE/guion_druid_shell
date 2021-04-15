use druid_shell::{FileDialogToken, FileInfo, IdleToken, KeyEvent, MouseEvent, Scale, TimerToken, kurbo};

pub mod handle;

pub enum BaseEvent<'a> {
    Size(kurbo::Size),
    Scale(Scale),
    Command(u32),
    SaveAs{
        token: FileDialogToken,
        file: Option<FileInfo>,
    },
    OpenFile{
        token: FileDialogToken,
        file: Option<FileInfo>,
    },
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    Wheel(&'a MouseEvent),
    Zoom(f64),
    MouseMove(&'a MouseEvent),
    MouseDown(&'a MouseEvent),
    MouseUp(&'a MouseEvent),
    MouseLeave,
    Timer(TimerToken),
    GotFocus,
    LostFocus,
    RequestClose,
    Destroy,
    Idle(IdleToken),

    PreRender,
    PostRender,
}
