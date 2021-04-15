use super::*;
use druid_shell::Cursor;
use guion::style::standard::cursor::StdCursor;

//TODO move, custom IntoDruidCursor trait
pub(crate) fn cursor2cursor(c: impl Into<StdCursor>) -> Cursor {
    match c.into() {
        StdCursor::Default => Cursor::Arrow,
        StdCursor::Arrow => Cursor::Arrow,
        StdCursor::IBeam => Cursor::IBeam,
        StdCursor::Wait => Cursor::Arrow, //TODO F U Z
        StdCursor::Crosshair => Cursor::Crosshair,
        StdCursor::WaitArrow => Cursor::Arrow, //TODO F U Z
        StdCursor::SizeNWSE => Cursor::Arrow, //TODO F U Z
        StdCursor::SizeNESW => Cursor::Arrow, //TODO F U Z
        StdCursor::SizeWE => Cursor::ResizeLeftRight,
        StdCursor::SizeNS => Cursor::ResizeUpDown,
        StdCursor::SizeAll => Cursor::Arrow, //TODO F U Z
        StdCursor::No => Cursor::NotAllowed,
        StdCursor::Hand => Cursor::Pointer,
        _ => Cursor::Arrow,
    }
}
