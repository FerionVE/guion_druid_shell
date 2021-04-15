use druid_shell::MouseButton;
use druid_shell::keyboard_types::Key as DKey;

#[derive(Clone,Debug,PartialEq)]
pub enum Key {
    Kbd(DKey),
    Mouse(MouseButton),
    KbdSpace,
    KbdA,
    KbdX,
    KbdC,
    KbdV,
}

#[derive(Clone)]
pub enum KeyOrigin { //druid_shell doesn't know multiple kbd/mouses
    Kbd,
    Mouse,
}

impl Key {
    pub(crate) fn kbd(k: &DKey) -> Self {
        match k {
            DKey::Character(s) if s == " " => Self::KbdSpace,
            DKey::Character(s) if s == "\n" => Self::Kbd(DKey::Enter),
            DKey::Character(s) if s == "a" || s == "A" => Self::KbdA,
            DKey::Character(s) if s == "x" || s == "X" => Self::KbdX,
            DKey::Character(s) if s == "c" || s == "C" => Self::KbdC,
            DKey::Character(s) if s == "v" || s == "V" => Self::KbdA,
            _ => Self::Kbd(k.clone()),
        }
    }
}

impl guion::event::key::Key for Key {
    type Origin = KeyOrigin;

    const MOUSE_LEFT: Self = Self::Mouse(MouseButton::Left);
    const ENTER: Self = Self::Kbd(DKey::Enter);
    const SPACE: Self = Self::KbdSpace;
    const TAB: Self = Self::Kbd(DKey::Tab);
    const SHIFT: Self = Self::Kbd(DKey::Shift);
    const CTRL: Self = Self::Kbd(DKey::Control);
    const BACKSPACE: Self = Self::Kbd(DKey::Backspace);
    const LEFT: Self = Self::Kbd(DKey::ArrowLeft);
    const RIGHT: Self = Self::Kbd(DKey::ArrowRight);
    const UP: Self = Self::Kbd(DKey::ArrowUp);
    const DOWN: Self = Self::Kbd(DKey::ArrowDown);
    const A: Self = Self::KbdA;
    const X: Self = Self::KbdX;
    const C: Self = Self::KbdC;
    const V: Self = Self::KbdV; //TODO fix guion char key case oofing

    fn origin(&self) -> Self::Origin {
        match self {
            Self::Kbd(_) => KeyOrigin::Kbd,
            Self::Mouse(_) => KeyOrigin::Mouse,
            Key::KbdSpace => KeyOrigin::Kbd,
            Key::KbdA => KeyOrigin::Kbd,
            Key::KbdX => KeyOrigin::Kbd,
            Key::KbdC => KeyOrigin::Kbd,
            Key::KbdV => KeyOrigin::Kbd,
        }
    }
}
