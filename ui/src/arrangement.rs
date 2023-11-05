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
    SpacedAligned(Dp, alignment::Horizontal),
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

    fn arrange(&self, density: Density, total_size: i32, sizes: Vec<i32>) -> Vec<i32> {
        match *self {
            Self::Start => place_left_or_top(sizes),
            Self::Center => place_center(total_size, sizes),
            Self::End => place_right_or_bottom(total_size, sizes),
            Self::SpacedBy(space) => spaced_align(space, None, density, total_size, sizes),
            Self::SpacedAligned(space, alignment) => spaced_align(
                space,
                Some(Box::new(move |space| alignment.align(0, space))),
                density,
                total_size,
                sizes,
            ),
            Self::Aligned(alignment) => spaced_align(
                0.dp(),
                Some(Box::new(move |space| alignment.align(0, space))),
                density,
                total_size,
                sizes,
            ),
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

    fn arrange(&self, density: Density, total_size: i32, sizes: Vec<i32>) -> Vec<i32> {
        match *self {
            Self::Top => place_left_or_top(sizes),
            Self::Center => place_center(total_size, sizes),
            Self::Bottom => place_right_or_bottom(total_size, sizes),
            Self::SpacedBy(space) => spaced_align(space, None, density, total_size, sizes),
            Self::SpacedAligned(space, alignment) => spaced_align(
                space,
                Some(Box::new(move |space| alignment.align(0, space))),
                density,
                total_size,
                sizes,
            ),
            Self::Aligned(alignment) => spaced_align(
                0.dp(),
                Some(Box::new(move |space| alignment.align(0, space))),
                density,
                total_size,
                sizes,
            ),
        }
    }
}

fn spaced_align(
    space: Dp,
    alignment: Option<Box<dyn FnOnce(i32) -> i32>>,
    density: Density,
    total_size: i32,
    sizes: Vec<i32>,
) -> Vec<i32> {
    if sizes.is_empty() {
        return sizes;
    }

    let space_px = density.round_to_pixels(space);

    let mut occupied = 0;
    let mut last_space = 0;

    let mut out = Vec::with_capacity(sizes.len());

    for (index, size) in sizes.iter().enumerate() {
        out[index] = occupied.min(total_size - size);
        last_space = space_px.min(total_size - out[index] - size);
        occupied = out[index] + size + last_space;
    }

    occupied -= last_space;

    if let Some(alignment) = alignment.filter(|_| occupied < total_size) {
        let group_position = alignment(total_size - occupied);

        out.iter_mut().for_each(|x| *x += group_position);
    }

    out
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
