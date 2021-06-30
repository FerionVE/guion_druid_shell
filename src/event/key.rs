use druid_shell::MouseButton as DMouse;
use druid_shell::keyboard_types::Key as DKey;
use druid_shell::keyboard_types::Location as DLocation;
use guion::event::key::MatchKeyCode;
use guion::event::key::MatchScanCode;

#[derive(Clone,Debug,PartialEq)]
pub enum Key {
    Kbd(DKey,DLocation),
    Mouse(DMouse),
}

#[derive(Clone)]
pub enum KeyOrigin { //druid_shell doesn't know multiple kbd/mouses
    Kbd,
    Mouse,
}

impl guion::event::key::Key for Key {
    type Origin = KeyOrigin;

    fn origin(&self) -> Self::Origin {
        match self {
            Self::Kbd(_,_) => KeyOrigin::Kbd,
            Self::Mouse(_) => KeyOrigin::Mouse,
        }
    }
}

impl Key {
    fn char(&self, s: &str) -> bool {
        if let Self::Kbd(DKey::Character(c),l) = self {
            return c.to_lowercase() == s.to_lowercase() && *l != DLocation::Numpad
        }
        false
    }
    fn char_everywhere(&self, s: &str) -> bool {
        if let Self::Kbd(DKey::Character(c),l) = self {
            return c.to_lowercase() == s.to_lowercase()
        }
        false
    }
    fn char_kp(&self, s: &str) -> bool {
        if let Self::Kbd(DKey::Character(c),l) = self {
            return c.to_lowercase() == s.to_lowercase() && *l == DLocation::Numpad
        }
        false
    }
    fn char_case(&self, s: &str) -> bool {
        if let Self::Kbd(DKey::Character(c),l) = self {
            return c == s
        }
        false
    }
    fn kp(&self) -> bool {
        matches!(self, Self::Kbd(_,DLocation::Numpad))
    }
    fn key(&self, s: DKey) -> bool {
        todo!()
    }
    fn num(&self, n: usize) -> bool {
        match n {
            0 => self.char_everywhere("0"),
            1 => self.char_everywhere("1"),
            2 => self.char_everywhere("2"),
            3 => self.char_everywhere("3"),
            4 => self.char_everywhere("4"),
            5 => self.char_everywhere("5"),
            6 => self.char_everywhere("6"),
            7 => self.char_everywhere("7"),
            8 => self.char_everywhere("8"),
            9 => self.char_everywhere("9"),
            _ => panic!(),
        }
    }
    fn num_row(&self, n: usize) -> bool {
        match n {
            0 => self.char("0"),
            1 => self.char("1"),
            2 => self.char("2"),
            3 => self.char("3"),
            4 => self.char("4"),
            5 => self.char("5"),
            6 => self.char("6"),
            7 => self.char("7"),
            8 => self.char("8"),
            9 => self.char("9"),
            _ => panic!(),
        }
    }
    fn num_kp(&self, n: usize) -> bool {
        match n {
            0 => self.char_kp("0"),
            1 => self.char_kp("1"),
            2 => self.char_kp("2"),
            3 => self.char_kp("3"),
            4 => self.char_kp("4"),
            5 => self.char_kp("5"),
            6 => self.char_kp("6"),
            7 => self.char_kp("7"),
            8 => self.char_kp("8"),
            9 => self.char_kp("9"),
            _ => panic!(),
        }
    }
    fn left(&self) -> bool {
        matches!(self, Self::Kbd(_,DLocation::Left))
    }
    fn right(&self) -> bool {
        matches!(self, Self::Kbd(_,DLocation::Right))
    }
}

impl PartialEq<MatchKeyCode<'_>> for Key {
    fn eq(&self, other: &MatchKeyCode) -> bool {
        match self {
            Key::Mouse(k) => match other {
                MatchKeyCode::MouseLeft => *k == DMouse::Left,
                MatchKeyCode::MouseRight => *k == DMouse::Right,
                MatchKeyCode::MouseCenter => *k == DMouse::Middle,
                _ => false,
            },
            Key::Kbd(k,l) => match other {
                MatchKeyCode::CharCaseSensitive(c) => self.char_case(c),
                MatchKeyCode::CharCaseInsensitive(c) => self.char(c),
                MatchKeyCode::Number(n) => self.num(*n),
                MatchKeyCode::KbdBackspace => *k == DKey::Backspace,
                MatchKeyCode::KbdTab => *k == DKey::Tab,
                MatchKeyCode::KbdReturn => *k == DKey::Enter || self.char("\n"),
                MatchKeyCode::KbdEscape => *k == DKey::Escape,

                MatchKeyCode::KbdSpace => self.char(" "),
                MatchKeyCode::KbdExclaim => self.char("!"),
                MatchKeyCode::KbdQuotedbl => self.char("\""),
                MatchKeyCode::KbdHash => self.char("#"),
                MatchKeyCode::KbdDollar => self.char("$"),
                MatchKeyCode::KbdPercent => self.char("%"),
                MatchKeyCode::KbdAmpersand => self.char("&"),
                MatchKeyCode::KbdQuote => self.char("'"),
                MatchKeyCode::KbdLeftParen => self.char("("),
                MatchKeyCode::KbdRightParen => self.char(")"),
                MatchKeyCode::KbdAsterisk => self.char("*"),
                MatchKeyCode::KbdPlus => self.char("+"),
                MatchKeyCode::KbdComma => self.char(","),
                MatchKeyCode::KbdMinus => self.char("-"),
                MatchKeyCode::KbdPeriod => self.char("."),
                MatchKeyCode::KbdSlash => self.char("/"),
                MatchKeyCode::KbdNum0 => self.num_row(0),
                MatchKeyCode::KbdNum1 => self.num_row(1),
                MatchKeyCode::KbdNum2 => self.num_row(2),
                MatchKeyCode::KbdNum3 => self.num_row(3),
                MatchKeyCode::KbdNum4 => self.num_row(4),
                MatchKeyCode::KbdNum5 => self.num_row(5),
                MatchKeyCode::KbdNum6 => self.num_row(6),
                MatchKeyCode::KbdNum7 => self.num_row(7),
                MatchKeyCode::KbdNum8 => self.num_row(8),
                MatchKeyCode::KbdNum9 => self.num_row(9),
                MatchKeyCode::KbdColon => self.char(":"),
                MatchKeyCode::KbdSemicolon => self.char(";"),
                MatchKeyCode::KbdLess => self.char("<"),
                MatchKeyCode::KbdEquals => self.char("="),
                MatchKeyCode::KbdGreater => self.char(">"),
                MatchKeyCode::KbdQuestion => self.char("?"),
                MatchKeyCode::KbdAt => self.char("@"),
                MatchKeyCode::KbdLeftBracket => self.char("["),
                MatchKeyCode::KbdRightBracket => self.char("]"),
                MatchKeyCode::KbdBackslash => self.char("\\"),
                MatchKeyCode::KbdCaret => self.char("^"),
                MatchKeyCode::KbdUnderscore => self.char("_"),
                MatchKeyCode::KbdBackquote => self.char("`"),
                MatchKeyCode::KbdA => self.char("a"),
                MatchKeyCode::KbdB => self.char("b"),
                MatchKeyCode::KbdC => self.char("c"),
                MatchKeyCode::KbdD => self.char("d"),
                MatchKeyCode::KbdE => self.char("e"),
                MatchKeyCode::KbdF => self.char("f"),
                MatchKeyCode::KbdG => self.char("g"),
                MatchKeyCode::KbdH => self.char("h"),
                MatchKeyCode::KbdI => self.char("i"),
                MatchKeyCode::KbdJ => self.char("j"),
                MatchKeyCode::KbdK => self.char("k"),
                MatchKeyCode::KbdL => self.char("l"),
                MatchKeyCode::KbdM => self.char("m"),
                MatchKeyCode::KbdN => self.char("n"),
                MatchKeyCode::KbdO => self.char("o"),
                MatchKeyCode::KbdP => self.char("p"),
                MatchKeyCode::KbdQ => self.char("q"),
                MatchKeyCode::KbdR => self.char("r"),
                MatchKeyCode::KbdS => self.char("s"),
                MatchKeyCode::KbdT => self.char("t"),
                MatchKeyCode::KbdU => self.char("u"),
                MatchKeyCode::KbdV => self.char("v"),
                MatchKeyCode::KbdW => self.char("w"),
                MatchKeyCode::KbdX => self.char("x"),
                MatchKeyCode::KbdY => self.char("y"),
                MatchKeyCode::KbdZ => self.char("z"),

                MatchKeyCode::KbdDelete => *k == DKey::Delete,
                MatchKeyCode::KbdCapsLock => *k == DKey::CapsLock,
                MatchKeyCode::KbdF1 => *k == DKey::F1,
                MatchKeyCode::KbdF2 => *k == DKey::F2,
                MatchKeyCode::KbdF3 => *k == DKey::F3,
                MatchKeyCode::KbdF4 => *k == DKey::F4,
                MatchKeyCode::KbdF5 => *k == DKey::F5,
                MatchKeyCode::KbdF6 => *k == DKey::F6,
                MatchKeyCode::KbdF7 => *k == DKey::F7,
                MatchKeyCode::KbdF8 => *k == DKey::F8,
                MatchKeyCode::KbdF9 => *k == DKey::F8,
                MatchKeyCode::KbdF10 => *k == DKey::F10,
                MatchKeyCode::KbdF11 => *k == DKey::F11,
                MatchKeyCode::KbdF12 => *k == DKey::F12,
                MatchKeyCode::KbdPrintScreen => *k == DKey::Print,//TODO
                MatchKeyCode::KbdScrollLock => *k == DKey::ScrollLock,
                MatchKeyCode::KbdPause => *k == DKey::Pause,
                MatchKeyCode::KbdInsert => *k == DKey::Insert,
                MatchKeyCode::KbdHome => *k == DKey::Home,
                MatchKeyCode::KbdPageUp => *k == DKey::PageUp,
                MatchKeyCode::KbdEnd => *k == DKey::End,
                MatchKeyCode::KbdPageDown => *k == DKey::PageDown,
                MatchKeyCode::KbdRight => *k == DKey::ArrowRight,
                MatchKeyCode::KbdLeft => *k == DKey::ArrowLeft,
                MatchKeyCode::KbdDown => *k == DKey::ArrowDown,
                MatchKeyCode::KbdUp => *k == DKey::ArrowUp,
                MatchKeyCode::KbdNumLock => *k == DKey::NumLock,
                MatchKeyCode::KpDivide => self.char_kp("/"),
                MatchKeyCode::KpMultiply => self.char_kp("*"),
                MatchKeyCode::KpMinus => self.char_kp("-"),
                MatchKeyCode::KpPlus => self.char_kp("+"),
                MatchKeyCode::KpEnter => ( *k == DKey::Enter || self.char("\n") ) && self.kp(),
                MatchKeyCode::Kp1 => self.num_kp(0),
                MatchKeyCode::Kp2 => self.num_kp(1),
                MatchKeyCode::Kp3 => self.num_kp(2),
                MatchKeyCode::Kp4 => self.num_kp(3),
                MatchKeyCode::Kp5 => self.num_kp(4),
                MatchKeyCode::Kp6 => self.num_kp(5),
                MatchKeyCode::Kp7 => self.num_kp(6),
                MatchKeyCode::Kp8 => self.num_kp(7),
                MatchKeyCode::Kp9 => self.num_kp(8),
                MatchKeyCode::Kp0 => self.num_kp(9),
                MatchKeyCode::KpPeriod => self.char_kp("."),
                MatchKeyCode::KbdApplication => *k == DKey::ContextMenu,
                MatchKeyCode::KbdPower => *k == DKey::Power,
                MatchKeyCode::KpEquals => self.char_kp("="),
                MatchKeyCode::KbdF13 => false/*TODO*/,
                MatchKeyCode::KbdF14 => false/*TODO*/,
                MatchKeyCode::KbdF15 => false/*TODO*/,
                MatchKeyCode::KbdF16 => false/*TODO*/,
                MatchKeyCode::KbdF17 => false/*TODO*/,
                MatchKeyCode::KbdF18 => false/*TODO*/,
                MatchKeyCode::KbdF19 => false/*TODO*/,
                MatchKeyCode::KbdF20 => false/*TODO*/,
                MatchKeyCode::KbdF21 => false/*TODO*/,
                MatchKeyCode::KbdF22 => false/*TODO*/,
                MatchKeyCode::KbdF23 => false/*TODO*/,
                MatchKeyCode::KbdF24 => false/*TODO*/,
                MatchKeyCode::KbdExecute => *k == DKey::Execute,
                MatchKeyCode::KbdHelp => *k == DKey::Help,
                MatchKeyCode::KbdMenu => *k == DKey::ContextMenu,//TODO
                MatchKeyCode::KbdSelect => *k == DKey::Select,
                MatchKeyCode::KbdStop => *k == DKey::MediaStop,//TODO
                MatchKeyCode::KbdAgain => *k == DKey::Again,
                MatchKeyCode::KbdUndo => *k == DKey::Undo,
                MatchKeyCode::KbdCut => *k == DKey::Cut,
                MatchKeyCode::KbdCopy => *k == DKey::Copy,
                MatchKeyCode::KbdPaste => *k == DKey::Paste,
                MatchKeyCode::KbdFind => *k == DKey::Find,
                MatchKeyCode::KbdMute => *k == DKey::AudioVolumeMute,
                MatchKeyCode::KbdVolumeUp => *k == DKey::AudioVolumeUp,
                MatchKeyCode::KbdVolumeDown => *k == DKey::AudioVolumeDown,
                MatchKeyCode::KpComma => self.char_kp(","),
                MatchKeyCode::KpEqualsAS400 => todo!(),
                MatchKeyCode::KbdAltErase => todo!(),
                MatchKeyCode::KbdSysreq => todo!(),
                MatchKeyCode::KbdCancel => *k == DKey::Cancel,
                MatchKeyCode::KbdClear => *k == DKey::Clear,
                MatchKeyCode::KbdPrior => todo!(),
                MatchKeyCode::KbdReturn2 => todo!(),
                MatchKeyCode::KbdSeparator => todo!(),
                MatchKeyCode::KbdOut => todo!(),
                MatchKeyCode::KbdOper => todo!(),
                MatchKeyCode::KbdClearAgain => todo!(),
                MatchKeyCode::KbdCrSel => *k == DKey::CrSel,
                MatchKeyCode::KbdExSel => *k == DKey::ExSel,
                MatchKeyCode::Kp00 => todo!(),
                MatchKeyCode::Kp000 => todo!(),
                MatchKeyCode::KbdThousandsSeparator => todo!(),
                MatchKeyCode::KbdDecimalSeparator => todo!(),
                MatchKeyCode::KbdCurrencyUnit => todo!(),
                MatchKeyCode::KbdCurrencySubUnit => todo!(),
                MatchKeyCode::KpLeftParen => self.char_kp("("),
                MatchKeyCode::KpRightParen => self.char_kp(")"),
                MatchKeyCode::KpLeftBrace => self.char_kp("["),
                MatchKeyCode::KpRightBrace => self.char_kp("]"),
                MatchKeyCode::KpTab => ( *k == DKey::Tab || self.char("\t") ) && self.kp(),
                MatchKeyCode::KpBackspace => *k == DKey::Backspace && self.kp(),
                MatchKeyCode::KpA => self.char_kp("a"),
                MatchKeyCode::KpB => self.char_kp("b"),
                MatchKeyCode::KpC => self.char_kp("c"),
                MatchKeyCode::KpD => self.char_kp("d"),
                MatchKeyCode::KpE => self.char_kp("e"),
                MatchKeyCode::KpF => self.char_kp("f"),
                MatchKeyCode::KpXor => todo!(),
                MatchKeyCode::KpPower => todo!(),
                MatchKeyCode::KpPercent => self.char_kp("%"),
                MatchKeyCode::KpLess => self.char_kp("<"),
                MatchKeyCode::KpGreater => self.char_kp(">"),
                MatchKeyCode::KpAmpersand => self.char_kp("&"),
                MatchKeyCode::KpDblAmpersand => todo!(),
                MatchKeyCode::KpVerticalBar => self.char_kp("|"),
                MatchKeyCode::KpDblVerticalBar => todo!(),
                MatchKeyCode::KpColon => self.char_kp(":"),
                MatchKeyCode::KpHash => self.char_kp("#"),
                MatchKeyCode::KpSpace => self.char_kp(" "),
                MatchKeyCode::KpAt => self.char_kp("@"),
                MatchKeyCode::KpExclam => self.char_kp("!"),
                MatchKeyCode::KpMemStore => todo!(),
                MatchKeyCode::KpMemRecall => todo!(),
                MatchKeyCode::KpMemClear => todo!(),
                MatchKeyCode::KpMemAdd => todo!(),
                MatchKeyCode::KpMemSubtract => todo!(),
                MatchKeyCode::KpMemMultiply => todo!(),
                MatchKeyCode::KpMemDivide => todo!(),
                MatchKeyCode::KpPlusMinus => todo!(),
                MatchKeyCode::KpClear => todo!(),
                MatchKeyCode::KpClearEntry => todo!(),
                MatchKeyCode::KpBinary => todo!(),
                MatchKeyCode::KpOctal => todo!(),
                MatchKeyCode::KpDecimal => todo!(),
                MatchKeyCode::KpHexadecimal => todo!(),
                MatchKeyCode::KbdCtrl => *k == DKey::Control,
                MatchKeyCode::KbdShift => *k == DKey::Shift,
                MatchKeyCode::KbdAlt => *k == DKey::Alt,
                MatchKeyCode::KbdGui => todo!(),
                MatchKeyCode::KbdLCtrl => *k == DKey::Control && self.left(),
                MatchKeyCode::KbdLShift => *k == DKey::Shift && self.left(),
                MatchKeyCode::KbdLAlt => *k == DKey::Alt && self.left(),
                MatchKeyCode::KbdLGui => todo!(),
                MatchKeyCode::KbdRCtrl => *k == DKey::Control && self.right(),
                MatchKeyCode::KbdRShift => *k == DKey::Shift && self.right(),
                MatchKeyCode::KbdRAlt => *k == DKey::Alt && self.right(),
                MatchKeyCode::KbdRGui => todo!(),
                MatchKeyCode::KbdAltGr => *k == DKey::AltGraph,
                MatchKeyCode::KbdMode => todo!(),
                MatchKeyCode::KbdAudioNext => *k == DKey::MediaTrackNext,
                MatchKeyCode::KbdAudioPrev => *k == DKey::MediaTrackPrevious,
                MatchKeyCode::KbdAudioStop => *k == DKey::MediaStop,
                MatchKeyCode::KbdAudioPlay => *k == DKey::MediaPlay,
                MatchKeyCode::KbdAudioMute => *k == DKey::AudioVolumeMute,
                MatchKeyCode::KbdMediaSelect => todo!(),
                MatchKeyCode::KbdWww => todo!(),
                MatchKeyCode::KbdMail => todo!(),
                MatchKeyCode::KbdCalculator => todo!(),
                MatchKeyCode::KbdComputer => todo!(),
                MatchKeyCode::KbdAcSearch => todo!(),
                MatchKeyCode::KbdAcHome => todo!(),
                MatchKeyCode::KbdAcBack => todo!(),
                MatchKeyCode::KbdAcForward => todo!(),
                MatchKeyCode::KbdAcStop => todo!(),
                MatchKeyCode::KbdAcRefresh => todo!(),
                MatchKeyCode::KbdAcBookmarks => todo!(),
                MatchKeyCode::KbdBrightnessDown => *k == DKey::BrightnessUp,
                MatchKeyCode::KbdBrightnessUp => *k == DKey::BrightnessDown,
                MatchKeyCode::KbdDisplaySwitch => *k == DKey::DisplaySwap,
                MatchKeyCode::KbdKbdIllumToggle => todo!(),
                MatchKeyCode::KbdKbdIllumDown => todo!(),
                MatchKeyCode::KbdKbdIllumUp => todo!(),
                MatchKeyCode::KbdEject => *k == DKey::Eject,
                MatchKeyCode::KbdSleep => todo!(),
                _ => false,
            },
        }
        
    }
}

impl PartialEq<MatchScanCode<'_>> for Key {
    fn eq(&self, other: &MatchScanCode) -> bool {
        todo!()
    }
}
