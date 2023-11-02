use crate::unit::*;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Alignment {
    pub horizontal: Horizontal,
    pub vertical: Vertical,
}

impl Alignment {
    pub fn align(&self, size: ISize, space: ISize) -> IPoint {
        let center_x = (space.width - size.width) as f32 / 2f32;
        let center_y = (space.height - size.height) as f32 / 2f32;

        let x = center_x * (1f32 + self.horizontal.bias());
        let y = center_y * (1f32 + self.vertical.bias());

        IPoint::new(x.round() as i32, y.round() as i32)
    }

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
    Custom(f32),
}

impl Horizontal {
    pub fn bias(&self) -> f32 {
        match self {
            Self::Start => -1f32,
            Self::Center => 0f32,
            Self::End => 1f32,
            Self::Custom(bias) => *bias,
        }
    }

    pub fn align(&self, size: i32, space: i32) -> i32 {
        let center = (space - size) as f32 / 2f32;

        (center * (1f32 + self.bias())).round() as i32
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
    pub fn bias(&self) -> f32 {
        match self {
            Self::Top => -1f32,
            Self::Center => 0f32,
            Self::Bottom => 1f32,
            Self::Custom(bias) => *bias,
        }
    }

    pub fn align(&self, size: i32, space: i32) -> i32 {
        let center = (space - size) as f32 / 2f32;

        (center * (1f32 + self.bias())).round() as i32
    }
}
