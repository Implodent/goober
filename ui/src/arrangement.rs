use super::*;

type Dp = LengthPercentage;

pub trait Horizontal {
    fn spacing(&self) -> Dp;

    fn justify(&self) -> JustifyItems;
}

pub trait Vertical {
    fn spacing(&self) -> Dp;

    fn align(&self) -> AlignItems;
}

pub trait Arrangement: Vertical + Horizontal {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BuiltinHorizontal {
    Start,
    Center,
    End,
    SpacedBy(Dp),
    SpacedAligned(Dp, alignment::Horizontal),
    Aligned(alignment::Horizontal),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BuiltinVertical {
    Top,
    Center,
    Bottom,
    SpacedBy(Dp),
    SpacedAligned(Dp, alignment::Vertical),
    Aligned(alignment::Vertical),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Builtin {
    Center,
    SpaceEvenly,
    SpaceBetween,
    SpaceAround,
    SpacedBy(Dp),
}

impl Horizontal for BuiltinHorizontal {
    fn spacing(&self) -> Dp {
        match self {
            Self::SpacedBy(dp) | Self::SpacedAligned(dp, _) => *dp,
            _ => Dp::ZERO,
        }
    }
    fn justify(&self) -> JustifyItems {
        match *self {
            Self::Aligned(align) | Self::SpacedAligned(_, align) => align.into(),
            Self::Start => JustifyItems::Start,
            Self::Center => JustifyItems::Center,
            Self::End => JustifyItems::End,
            Self::SpacedBy(_) => JustifyItems::Start,
        }
    }
}

impl Vertical for BuiltinVertical {
    fn spacing(&self) -> Dp {
        match self {
            Self::SpacedBy(dp) | Self::SpacedAligned(dp, _) => *dp,
            _ => Dp::ZERO,
        }
    }

    fn align(&self) -> AlignItems {
        match *self {
            Self::Aligned(align) | Self::SpacedAligned(_, align) => align.into(),
            Self::Top => AlignItems::Start,
            Self::Center => AlignItems::Center,
            Self::Bottom => AlignItems::End,
            Self::SpacedBy(_) => AlignItems::Start,
        }
    }
}
