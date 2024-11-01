use iced::Theme;

use crate::screen::Screen;

#[derive(Clone, Debug)]
pub enum Message {
    Screen(Screen),
    IntInputA(String),
    IntInputB(String),
    DebugToggle(bool),
    InputStringChange(String),
    EvalBoolRpn,
    EvalVarRpn,
    ThemeSelected(Theme),
}

impl From<Screen> for Message {
    fn from(value: Screen) -> Self {
        Self::Screen(value)
    }
}
