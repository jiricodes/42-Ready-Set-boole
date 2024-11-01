#[derive(Default, Debug, Clone, Copy)]
pub enum Screen {
    #[default]
    Home,
    Ex00,
    Ex01,
    Ex02,
    Ex03,
    Ex04,
    Ex05,
    Ex06,
    Ex07,
    Ex08,
    Ex09,
    Ex10,
    Ex11,
}

impl Screen {
    pub const ALL: &'static [Self] = &[
        Self::Home,
        Self::Ex00,
        Self::Ex01,
        Self::Ex02,
        Self::Ex03,
        Self::Ex04,
        Self::Ex05,
        Self::Ex06,
        Self::Ex07,
        Self::Ex08,
        Self::Ex09,
        Self::Ex10,
        Self::Ex11,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Ex00 => "Ex00",
            Self::Ex01 => "Ex01",
            Self::Ex02 => "Ex02",
            Self::Ex03 => "Ex03",
            Self::Ex04 => "Ex04",
            Self::Ex05 => "Ex05",
            Self::Ex06 => "Ex06",
            Self::Ex07 => "Ex07",
            Self::Ex08 => "Ex08",
            Self::Ex09 => "Ex09",
            Self::Ex10 => "Ex10",
            Self::Ex11 => "Ex11",
        }
    }
}
