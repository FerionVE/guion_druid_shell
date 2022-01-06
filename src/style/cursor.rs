use super::*;
use druid_shell::Cursor;
use guion::style::standard::cursor::StdCursor;

pub trait IntoGuionDruidShellCursor<E> {
    fn into_druid_shell_cursor(self) -> Cursor;
}

impl<T,E> IntoGuionDruidShellCursor<E> for T where T: Into<StdCursor> {
    fn into_druid_shell_cursor(self) -> Cursor {
        match Into::<StdCursor>::into(self) {
            StdCursor::Default => Cursor::Arrow,
            StdCursor::Arrow => Cursor::Arrow,
            StdCursor::IBeam => Cursor::IBeam,
            StdCursor::Wait => Cursor::Arrow, //TODO
            StdCursor::Crosshair => Cursor::Crosshair,
            StdCursor::WaitArrow => Cursor::Arrow, //TODO
            StdCursor::SizeNWSE => Cursor::Arrow, //TODO
            StdCursor::SizeNESW => Cursor::Arrow, //TODO
            StdCursor::SizeWE => Cursor::ResizeLeftRight,
            StdCursor::SizeNS => Cursor::ResizeUpDown,
            StdCursor::SizeAll => Cursor::Arrow, //TODO
            StdCursor::No => Cursor::NotAllowed,
            StdCursor::Hand => Cursor::Pointer,
            _ => Cursor::Arrow,
        }
    }
}
