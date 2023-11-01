use crate::unit::*;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Alignment {
    pub horizontal: Horizontal,
    pub vertical: Vertical,
}

impl Alignment {
    pub const fn from_bias(x: f32, y: f32) -> Self {
        Self {
            horizontal: Horizontal::from_bias(x),
            vertical: Vertical::from_bias(y),
        }
    }

    pub const fn align(&self, size: ISize, space: ISize) -> IPoint {
        let center_x = (space.width - size.width) as f32 / 2f32;
        let center_y = (space.height - size.height) as f32 / 2f32;

        let x = center_x * (1 + self);
    }

    pub const TOP_START: Self = Self::from_bias(-1f32, -1f32);
    pub const TOP_CENTER: Self = Self::from_bias(0f32, -1f32);
    pub const TOP_END: Self = Self::from_bias(1f32, -1f32);

    pub const CENTER_START: Self = Self::from_bias(-1f32, 0f32);
    pub const CENTER: Self = Self::from_bias(0f32, 0f32);
    pub const CENTER_END: Self = Self::from_bias(1f32, 0f32);

    pub const BOTTOM_START: Self = Self::from_bias(-1f32, 1f32);
    pub const BOTTOM_CENTER: Self = Self::from_bias(0f32, 1f32);
    pub const BOTTOM_END: Self = Self::from_bias(1f32, 1f32);
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub enum Horizontal {
    Start,
    #[default]
    Center,
    End,
    Custom(f32),
}

impl Horizontal {
    pub const fn from_bias(bias: f32) -> Self {
        match bias {
            -1f32 => Self::Start,
            0f32 => Self::Center,
            1f32 => Self::End,
            _ => Self::Custom(bias),
        }
    }

    pub const fn bias(&self) -> f32 {
        match self {
            Self::Start => -1f32,
            Self::Center => 0f32,
            Self::End => 1f32,
            Self::Custom(bias) => bias,
        }
    }

    pub const fn align(&self, size: i32, space: i32) -> i32 {
        let center = (space - size) as f32 / 2f32;

        (center * (1 + self.bias()))
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub enum Vertical {
    Top,
    #[default]
    Center,
    Bottom,
    Custom(f32),
}

impl Vertical {
    pub const fn from_bias(bias: f32) -> Self {
        match bias {
            -1f32 => Self::Top,
            0f32 => Self::Center,
            1f32 => Self::Bottom,
            _ => Self::Custom(bias),
        }
    }

    pub const fn bias(&self) -> f32 {
        match self {
            Self::Top => -1f32,
            Self::Center => 0f32,
            Self::Bottom => 1f32,
            Self::Custom(bias) => bias,
        }
    }

    pub const fn align(&self, size: i32, space: i32) -> i32 {
        let center = (space - size) as f32 / 2f32;

        (center * (1 + self.bias()))
    }
}
