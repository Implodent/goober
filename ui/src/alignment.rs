use super::*;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Alignment {
    pub horizontal: Horizontal,
    pub vertical: Vertical,
}

impl Alignment {
    pub const TOP_START: Self = Self {
        horizontal: Horizontal::Start,
        vertical: Vertical::Top,
    };
    pub const TOP_CENTER: Self = Self {
        horizontal: Horizontal::Center,
        vertical: Vertical::Top,
    };
    pub const TOP_END: Self = Self {
        horizontal: Horizontal::End,
        vertical: Vertical::Top,
    };

    pub const CENTER_START: Self = Self {
        horizontal: Horizontal::Start,
        vertical: Vertical::Center,
    };
    pub const CENTER: Self = Self {
        horizontal: Horizontal::Center,
        vertical: Vertical::Center,
    };
    pub const CENTER_END: Self = Self {
        horizontal: Horizontal::End,
        vertical: Vertical::Center,
    };

    pub const BOTTOM_START: Self = Self {
        horizontal: Horizontal::Start,
        vertical: Vertical::Bottom,
    };
    pub const BOTTOM_CENTER: Self = Self {
        horizontal: Horizontal::Center,
        vertical: Vertical::Bottom,
    };
    pub const BOTTOM_END: Self = Self {
        horizontal: Horizontal::End,
        vertical: Vertical::Bottom,
    };
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub enum Horizontal {
    Start,
    #[default]
    Center,
    End,
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub enum Vertical {
    Top,
    #[default]
    Center,
    Bottom,
}

impl Into<JustifyItems> for Horizontal {
    fn into(self) -> JustifyItems {
        match self {
            Self::Start => AlignItems::Start,
            Self::Center => AlignItems::Center,
            Self::End => AlignItems::End,
        }
    }
}

impl Into<AlignItems> for Vertical {
    fn into(self) -> AlignItems {
        match self {
            Self::Top => AlignItems::Start,
            Self::Center => AlignItems::Center,
            Self::Bottom => AlignItems::End,
        }
    }
}
