use crate::skia_safe::*;
use std::fmt;
use std::str::CharIndices;

// FromStr but we own it so we can impl it on torin and skia_safe types.
pub trait Parse: Sized {
    type Err;

    fn parse(value: &str) -> Result<Self, Self::Err>;
}

pub trait ExtSplit {
    fn split_excluding_group(
        &self,
        delimiter: char,
        group_start: char,
        group_end: char,
    ) -> SplitExcludingGroup<'_>;
    fn split_ascii_whitespace_excluding_group(
        &self,
        group_start: char,
        group_end: char,
    ) -> SplitAsciiWhitespaceExcludingGroup<'_>;
}

impl ExtSplit for str {
    fn split_excluding_group(
        &self,
        delimiter: char,
        group_start: char,
        group_end: char,
    ) -> SplitExcludingGroup<'_> {
        SplitExcludingGroup {
            text: self,
            chars: self.char_indices(),
            delimiter,
            group_start,
            group_end,
            trailing_empty: true,
        }
    }

    fn split_ascii_whitespace_excluding_group(
        &self,
        group_start: char,
        group_end: char,
    ) -> SplitAsciiWhitespaceExcludingGroup<'_> {
        SplitAsciiWhitespaceExcludingGroup {
            text: self,
            chars: self.char_indices(),
            group_start,
            group_end,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SplitExcludingGroup<'a> {
    pub text: &'a str,
    pub chars: CharIndices<'a>,
    pub delimiter: char,
    pub group_start: char,
    pub group_end: char,
    pub trailing_empty: bool,
}

impl<'a> Iterator for SplitExcludingGroup<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let first = self.chars.next();

        let (start, mut prev) = match first {
            None => {
                if self.text.ends_with(self.delimiter) && self.trailing_empty {
                    self.trailing_empty = false;
                    return Some("");
                }
                return None;
            }
            Some((_, c)) if c == self.delimiter => return Some(""),
            Some(v) => v,
        };

        let mut in_group = false;
        let mut nesting = -1;

        loop {
            if prev == self.group_start {
                if nesting == -1 {
                    in_group = true;
                }
                nesting += 1;
            } else if prev == self.group_end {
                nesting -= 1;
                if nesting == -1 {
                    in_group = false;
                }
            }

            prev = match self.chars.next() {
                None => return Some(&self.text[start..]),
                Some((end, c)) if c == self.delimiter && !in_group => {
                    return Some(&self.text[start..end])
                }
                Some((_, c)) => c,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct SplitAsciiWhitespaceExcludingGroup<'a> {
    pub text: &'a str,
    pub chars: CharIndices<'a>,
    pub group_start: char,
    pub group_end: char,
}

impl<'a> Iterator for SplitAsciiWhitespaceExcludingGroup<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let first = self.chars.next();

        let (start, mut prev) = match first {
            None => return None,
            Some((_, c)) if c.is_ascii_whitespace() => return self.next(),
            Some(v) => v,
        };

        let mut in_group = false;
        let mut nesting = -1;

        loop {
            if prev == self.group_start {
                if nesting == -1 {
                    in_group = true;
                }
                nesting += 1;
            } else if prev == self.group_end {
                nesting -= 1;
                if nesting == -1 {
                    in_group = false;
                }
            }

            prev = match self.chars.next() {
                None => return Some(&self.text[start..]),
                Some((end, c)) if c.is_ascii_whitespace() && !in_group => {
                    return Some(&self.text[start..end])
                }
                Some((_, c)) => c,
            }
        }
    }
}
pub trait DisplayColor {
    fn fmt_rgb(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn fmt_hsl(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseColorError;

impl Parse for Color {
    type Err = ParseColorError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        match value {
            "red" => Ok(Color::RED),
            "green" => Ok(Color::GREEN),
            "blue" => Ok(Color::BLUE),
            "yellow" => Ok(Color::YELLOW),
            "black" => Ok(Color::BLACK),
            "gray" => Ok(Color::GRAY),
            "white" => Ok(Color::WHITE),
            "orange" => Ok(Color::from_rgb(255, 165, 0)),
            "transparent" => Ok(Color::TRANSPARENT),
            _ => {
                if value.starts_with("hsl(") {
                    parse_hsl(value)
                } else if value.starts_with("rgb(") {
                    parse_rgb(value)
                } else {
                    Err(ParseColorError)
                }
            }
        }
    }
}

impl DisplayColor for Color {
    fn fmt_rgb(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "rgb({}, {}, {}, {})",
            self.r(),
            self.g(),
            self.b(),
            self.a()
        )
    }

    fn fmt_hsl(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // HSV to HSL conversion
        let hsv = self.to_hsv();
        let l = hsv.v - (hsv.v * hsv.s / 2.0);
        let s = if l == 1.0 || l == 0.0 {
            0.0
        } else {
            (hsv.v - l) / f32::min(l, 1.0 - l)
        };

        write!(
            f,
            "hsl({}deg, {}%, {}%, {}%)",
            hsv.h,
            s * 100.0,
            l * 100.0,
            (self.a() as f32 / 128.0) * 100.0
        )
    }
}

fn parse_rgb(color: &str) -> Result<Color, ParseColorError> {
    if !color.ends_with(')') {
        return Err(ParseColorError);
    }

    let color = color.replacen("rgb(", "", 1).replacen(')', "", 1);

    let mut colors = color.split(',');

    let r = colors
        .next()
        .ok_or(ParseColorError)?
        .trim()
        .parse::<u8>()
        .map_err(|_| ParseColorError)?;
    let g = colors
        .next()
        .ok_or(ParseColorError)?
        .trim()
        .parse::<u8>()
        .map_err(|_| ParseColorError)?;
    let b = colors
        .next()
        .ok_or(ParseColorError)?
        .trim()
        .parse::<u8>()
        .map_err(|_| ParseColorError)?;
    let a: Option<&str> = colors.next();

    // There should not be more than 4 components.
    if colors.next().is_some() {
        return Err(ParseColorError);
    }

    if let Some(a) = a {
        let a = a.trim().parse::<u8>().map_err(|_| ParseColorError)?;
        Ok(Color::from_argb(a, r, g, b))
    } else {
        Ok(Color::from_rgb(r, g, b))
    }
}

fn parse_hsl(color: &str) -> Result<Color, ParseColorError> {
    if !color.ends_with(')') {
        return Err(ParseColorError);
    }

    let color = color.replacen("hsl(", "", 1).replacen(')', "", 1);
    let mut colors = color.split(',');

    // Get each color component as a string
    let h_str = colors.next().ok_or(ParseColorError)?.trim();
    let s_str = colors.next().ok_or(ParseColorError)?.trim();
    let l_str = colors.next().ok_or(ParseColorError)?.trim();
    let a_str: Option<&str> = colors.next();

    // Ensure correct units and lengths.
    if colors.next().is_some()
        || !h_str.ends_with("deg")
        || !s_str.ends_with('%')
        || !l_str.ends_with('%')
    {
        return Err(ParseColorError);
    }

    // S, L and A can end in percentage, otherwise its 0.0 - 1.0
    let h = h_str
        .replacen("deg", "", 1)
        .parse::<f32>()
        .map_err(|_| ParseColorError)?;
    let mut s = s_str
        .replacen('%', "", 1)
        .parse::<f32>()
        .map_err(|_| ParseColorError)?
        / 100.0;
    let mut l = l_str
        .replacen('%', "", 1)
        .parse::<f32>()
        .map_err(|_| ParseColorError)?
        / 100.0;

    // HSL to HSV Conversion
    l *= 2.0;
    s *= if l <= 1.0 { l } else { 2.0 - l };
    let v = (l + s) / 2.0;
    s = (2.0 * s) / (l + s);
    let hsv = HSV::from((h, s, v));

    // Handle alpha formatting and convert to ARGB
    if let Some(a_str) = a_str {
        if !s_str.ends_with('%') {
            return Err(ParseColorError);
        }

        let a = a_str
            .trim()
            .replace('%', "")
            .parse::<f32>()
            .map_err(|_| ParseColorError)?
            / 100.0;

        Ok(hsv.to_color((a * 255.0).round() as u8))
    } else {
        Ok(hsv.to_color(255))
    }
}
