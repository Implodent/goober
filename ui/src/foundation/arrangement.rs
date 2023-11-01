use super::*;

pub trait Horizontal {
    fn spacing(&self) -> Dp;

    fn arrange(&self, density: Density, total_size: i32, sizes: Vec<i32>) -> Vec<i32>;
}

pub trait Vertical {
    fn spacing(&self) -> Dp;

    fn arrange(&self, density: Density, total_size: i32, sizes: Vec<i32>) -> Vec<i32>;
}

pub trait Arrangement: Vertical + Horizontal {}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum BuiltinHorizontal {
    Start,
    Center,
    End,
    SpacedBy(Dp, alignment::Horizontal),
    Aligned(alignment::Horizontal),
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum BuiltinVertical {
    Top,
    Bottom,
    SpacedBy(Dp, alignment::Vertical),
    Aligned(alignment::Vertical),
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
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
            Self::SpacedBy(dp, _) => *dp,
            _ => Dp::ZERO,
        }
    }

    fn arrange(&self, density: Density, total_size: i32, sizes: Vec<i32>) -> Vec<i32> {
        match self {
            Self::Start => place_left_or_top(sizes),
            Self::Center => place_center(total_size, sizes),
            Self::End => place_right_or_bottom(total_size, sizes),
            _ => todo!(),
        }
    }
}

fn place_left_or_top(sizes: Vec<i32>) -> Vec<i32> {
    let mut current = 0;

    sizes
        .into_iter()
        .map(|size| {
            let ret = current;
            current += size;
            ret
        })
        .collect()
}

fn place_center(total_size: i32, sizes: Vec<i32>) -> Vec<i32> {
    let consumed_size = sizes.iter().sum::<i32>();
    let mut current = (total_size - consumed_size) as f32 / 2f32;

    sizes
        .into_iter()
        .map(|size| {
            let ret = current.round() as i32;
            current += size as f32;
            ret
        })
        .collect()
}

fn place_right_or_bottom(total_size: i32, sizes: Vec<i32>) -> Vec<i32> {
    let consumed_size = sizes.iter().sum::<i32>();
    let mut current = total_size - consumed_size;

    sizes
        .into_iter()
        .map(|size| {
            let ret = current;
            current += size;
            ret
        })
        .collect()
}
