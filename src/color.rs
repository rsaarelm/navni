use std::ops::{BitAndAssign, BitOrAssign, Mul};

use seq_macro::seq;
use serde::{Deserialize, Serialize};

/// 32-bit full-color color type.
#[derive(
    Copy, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Rgba { r, g, b, a }
    }

    /// Squared color distance using redmean method.
    ///
    /// Method from <https://en.wikipedia.org/wiki/Color_difference#sRGB>
    pub fn square_dist(&self, other: &Self) -> f32 {
        fn f(x: u8) -> f32 {
            (x as f32) / 255.0
        }

        let (dr, dg, db, da) = (
            f(self.r) - f(other.r),
            f(self.g) - f(other.g),
            f(self.b) - f(other.b),
            f(self.a) - f(other.a),
        );
        let rm = (f(self.r) + f(other.r)) / 2.0;

        (2.0 + rm) * dr * dr + 4.0 * dg * dg + (3.0 - rm) * db * db + da * da
    }

    pub fn greyscale(&self) -> u8 {
        ((0.2126 * self.r as f32)
            + (0.7152 * self.g as f32)
            + (0.0722 * self.b as f32)) as u8
    }

    #[inline(always)]
    pub fn is_transparent(&self) -> bool {
        self.a == 0
    }

    // SVG named colors.
    pub const ALICE_BLUE: Rgba = Rgba::new(0xf0, 0xf8, 0xff, 0xff);
    pub const ANTIQUE_WHITE: Rgba = Rgba::new(0xfa, 0xeb, 0xd7, 0xff);
    pub const AQUA: Rgba = Rgba::new(0x00, 0xff, 0xff, 0xff);
    pub const AQUAMARINE: Rgba = Rgba::new(0x7f, 0xff, 0xd4, 0xff);
    pub const AZURE: Rgba = Rgba::new(0xf0, 0xff, 0xff, 0xff);
    pub const BEIGE: Rgba = Rgba::new(0xf5, 0xf5, 0xdc, 0xff);
    pub const BISQUE: Rgba = Rgba::new(0xff, 0xe4, 0xc4, 0xff);
    pub const BLACK: Rgba = Rgba::new(0x00, 0x00, 0x00, 0xff);
    pub const BLANCHED_ALMOND: Rgba = Rgba::new(0xff, 0xeb, 0xcd, 0xff);
    pub const BLUE: Rgba = Rgba::new(0x00, 0x00, 0xff, 0xff);
    pub const BLUE_VIOLET: Rgba = Rgba::new(0x8a, 0x2b, 0xe2, 0xff);
    pub const BROWN: Rgba = Rgba::new(0xa5, 0x2a, 0x2a, 0xff);
    pub const BURLYWOOD: Rgba = Rgba::new(0xde, 0xb8, 0x87, 0xff);
    pub const CADET_BLUE: Rgba = Rgba::new(0x5f, 0x9e, 0xa0, 0xff);
    pub const CHARTREUSE: Rgba = Rgba::new(0x7f, 0xff, 0x00, 0xff);
    pub const CHOCOLATE: Rgba = Rgba::new(0xd2, 0x69, 0x1e, 0xff);
    pub const CORAL: Rgba = Rgba::new(0xff, 0x7f, 0x50, 0xff);
    pub const CORNFLOWER_BLUE: Rgba = Rgba::new(0x64, 0x95, 0xed, 0xff);
    pub const CORNSILK: Rgba = Rgba::new(0xff, 0xf8, 0xdc, 0xff);
    pub const CRIMSON: Rgba = Rgba::new(0xdc, 0x14, 0x3c, 0xff);
    pub const CYAN: Rgba = Rgba::new(0x00, 0xff, 0xff, 0xff);
    pub const DARK_BLUE: Rgba = Rgba::new(0x00, 0x00, 0x8b, 0xff);
    pub const DARK_CYAN: Rgba = Rgba::new(0x00, 0x8b, 0x8b, 0xff);
    pub const DARK_GOLDENROD: Rgba = Rgba::new(0xb8, 0x86, 0x0b, 0xff);
    pub const DARK_GRAY: Rgba = Rgba::new(0xa9, 0xa9, 0xa9, 0xff);
    pub const DARK_GREEN: Rgba = Rgba::new(0x00, 0x64, 0x00, 0xff);
    pub const DARK_GREY: Rgba = Rgba::new(0xa9, 0xa9, 0xa9, 0xff);
    pub const DARK_KHAKI: Rgba = Rgba::new(0xbd, 0xb7, 0x6b, 0xff);
    pub const DARK_MAGENTA: Rgba = Rgba::new(0x8b, 0x00, 0x8b, 0xff);
    pub const DARK_OLIVEGREEN: Rgba = Rgba::new(0x55, 0x6b, 0x2f, 0xff);
    pub const DARK_ORANGE: Rgba = Rgba::new(0xff, 0x8c, 0x00, 0xff);
    pub const DARK_ORCHID: Rgba = Rgba::new(0x99, 0x32, 0xcc, 0xff);
    pub const DARK_RED: Rgba = Rgba::new(0x8b, 0x00, 0x00, 0xff);
    pub const DARK_SALMON: Rgba = Rgba::new(0xe9, 0x96, 0x7a, 0xff);
    pub const DARK_SEA_GREEN: Rgba = Rgba::new(0x8f, 0xbc, 0x8f, 0xff);
    pub const DARK_SLATE_BLUE: Rgba = Rgba::new(0x48, 0x3d, 0x8b, 0xff);
    pub const DARK_SLATE_GRAY: Rgba = Rgba::new(0x2f, 0x4f, 0x4f, 0xff);
    pub const DARK_TURQUOISE: Rgba = Rgba::new(0x00, 0xce, 0xd1, 0xff);
    pub const DARK_VIOLET: Rgba = Rgba::new(0x94, 0x00, 0xd3, 0xff);
    pub const DEEP_PINK: Rgba = Rgba::new(0xff, 0x14, 0x93, 0xff);
    pub const DEEP_SKYBLUE: Rgba = Rgba::new(0x00, 0xbf, 0xff, 0xff);
    pub const DIM_GRAY: Rgba = Rgba::new(0x69, 0x69, 0x69, 0xff);
    pub const DIM_GREY: Rgba = Rgba::new(0x69, 0x69, 0x69, 0xff);
    pub const DODGER_BLUE: Rgba = Rgba::new(0x1e, 0x90, 0xff, 0xff);
    pub const FIREBRICK: Rgba = Rgba::new(0xb2, 0x22, 0x22, 0xff);
    pub const FLORAL_WHITE: Rgba = Rgba::new(0xff, 0xfa, 0xf0, 0xff);
    pub const FOREST_GREEN: Rgba = Rgba::new(0x22, 0x8b, 0x22, 0xff);
    pub const FUCHSIA: Rgba = Rgba::new(0xff, 0x00, 0xff, 0xff);
    pub const GAINSBORO: Rgba = Rgba::new(0xdc, 0xdc, 0xdc, 0xff);
    pub const GHOST_WHITE: Rgba = Rgba::new(0xf8, 0xf8, 0xff, 0xff);
    pub const GOLD: Rgba = Rgba::new(0xff, 0xd7, 0x00, 0xff);
    pub const GOLDENROD: Rgba = Rgba::new(0xda, 0xa5, 0x20, 0xff);
    pub const GRAY: Rgba = Rgba::new(0x80, 0x80, 0x80, 0xff);
    pub const GREEN: Rgba = Rgba::new(0x00, 0x80, 0x00, 0xff);
    pub const GREEN_YELLOW: Rgba = Rgba::new(0xad, 0xff, 0x2f, 0xff);
    pub const GREY: Rgba = Rgba::new(0x80, 0x80, 0x80, 0xff);
    pub const HONEYDEW: Rgba = Rgba::new(0xf0, 0xff, 0xf0, 0xff);
    pub const HOT_PINK: Rgba = Rgba::new(0xff, 0x69, 0xb4, 0xff);
    pub const INDIAN_RED: Rgba = Rgba::new(0xcd, 0x5c, 0x5c, 0xff);
    pub const INDIGO: Rgba = Rgba::new(0x4b, 0x00, 0x82, 0xff);
    pub const IVORY: Rgba = Rgba::new(0xff, 0xff, 0xf0, 0xff);
    pub const KHAKI: Rgba = Rgba::new(0xf0, 0xe6, 0x8c, 0xff);
    pub const LAVENDER: Rgba = Rgba::new(0xe6, 0xe6, 0xfa, 0xff);
    pub const LAVENDER_BLUSH: Rgba = Rgba::new(0xff, 0xf0, 0xf5, 0xff);
    pub const LAWNGREEN: Rgba = Rgba::new(0x7c, 0xfc, 0x00, 0xff);
    pub const LEMON_CHIFFON: Rgba = Rgba::new(0xff, 0xfa, 0xcd, 0xff);
    pub const LIGHT_BLUE: Rgba = Rgba::new(0xad, 0xd8, 0xe6, 0xff);
    pub const LIGHT_CORAL: Rgba = Rgba::new(0xf0, 0x80, 0x80, 0xff);
    pub const LIGHT_CYAN: Rgba = Rgba::new(0xe0, 0xff, 0xff, 0xff);
    pub const LIGHT_GOLDENRODYELLOW: Rgba = Rgba::new(0xfa, 0xfa, 0xd2, 0xff);
    pub const LIGHT_GRAY: Rgba = Rgba::new(0xd3, 0xd3, 0xd3, 0xff);
    pub const LIGHT_GREEN: Rgba = Rgba::new(0x90, 0xee, 0x90, 0xff);
    pub const LIGHT_GREY: Rgba = Rgba::new(0xd3, 0xd3, 0xd3, 0xff);
    pub const LIGHT_PINK: Rgba = Rgba::new(0xff, 0xb6, 0xc1, 0xff);
    pub const LIGHT_SALMON: Rgba = Rgba::new(0xff, 0xa0, 0x7a, 0xff);
    pub const LIGHT_SEAGREEN: Rgba = Rgba::new(0x20, 0xb2, 0xaa, 0xff);
    pub const LIGHT_SKYBLUE: Rgba = Rgba::new(0x87, 0xce, 0xfa, 0xff);
    pub const LIGHT_SLATEGRAY: Rgba = Rgba::new(0x77, 0x88, 0x99, 0xff);
    pub const LIGHT_SLATEGREY: Rgba = Rgba::new(0x77, 0x88, 0x99, 0xff);
    pub const LIGHT_STEELBLUE: Rgba = Rgba::new(0xb0, 0xc4, 0xde, 0xff);
    pub const LIGHT_YELLOW: Rgba = Rgba::new(0xff, 0xff, 0xe0, 0xff);
    pub const LIME: Rgba = Rgba::new(0x00, 0xff, 0x00, 0xff);
    pub const LIME_GREEN: Rgba = Rgba::new(0x32, 0xcd, 0x32, 0xff);
    pub const LINEN: Rgba = Rgba::new(0xfa, 0xf0, 0xe6, 0xff);
    pub const MAGENTA: Rgba = Rgba::new(0xff, 0x00, 0xff, 0xff);
    pub const MAROON: Rgba = Rgba::new(0x80, 0x00, 0x00, 0xff);
    pub const MEDIUM_AQUAMARINE: Rgba = Rgba::new(0x66, 0xcd, 0xaa, 0xff);
    pub const MEDIUM_BLUE: Rgba = Rgba::new(0x00, 0x00, 0xcd, 0xff);
    pub const MEDIUM_ORCHID: Rgba = Rgba::new(0xba, 0x55, 0xd3, 0xff);
    pub const MEDIUM_PURPLE: Rgba = Rgba::new(0x93, 0x70, 0xdb, 0xff);
    pub const MEDIUM_SEAGREEN: Rgba = Rgba::new(0x3c, 0xb3, 0x71, 0xff);
    pub const MEDIUM_SLATEBLUE: Rgba = Rgba::new(0x7b, 0x68, 0xee, 0xff);
    pub const MEDIUM_SPRINGGREEN: Rgba = Rgba::new(0x00, 0xfa, 0x9a, 0xff);
    pub const MEDIUM_TURQUOISE: Rgba = Rgba::new(0x48, 0xd1, 0xcc, 0xff);
    pub const MEDIUM_VIOLETRED: Rgba = Rgba::new(0xc7, 0x15, 0x85, 0xff);
    pub const MIDNIGHT_BLUE: Rgba = Rgba::new(0x19, 0x19, 0x70, 0xff);
    pub const MINT_CREAM: Rgba = Rgba::new(0xf5, 0xff, 0xfa, 0xff);
    pub const MISTY_ROSE: Rgba = Rgba::new(0xff, 0xe4, 0xe1, 0xff);
    pub const MOCCASIN: Rgba = Rgba::new(0xff, 0xe4, 0xb5, 0xff);
    pub const NAVAJO_WHITE: Rgba = Rgba::new(0xff, 0xde, 0xad, 0xff);
    pub const NAVY: Rgba = Rgba::new(0x00, 0x00, 0x80, 0xff);
    pub const OLD_LACE: Rgba = Rgba::new(0xfd, 0xf5, 0xe6, 0xff);
    pub const OLIVE: Rgba = Rgba::new(0x80, 0x80, 0x00, 0xff);
    pub const OLIVE_DRAB: Rgba = Rgba::new(0x6b, 0x8e, 0x23, 0xff);
    pub const ORANGE: Rgba = Rgba::new(0xff, 0xa5, 0x00, 0xff);
    pub const ORANGE_RED: Rgba = Rgba::new(0xff, 0x45, 0x00, 0xff);
    pub const ORCHID: Rgba = Rgba::new(0xda, 0x70, 0xd6, 0xff);
    pub const PALE_GOLDENROD: Rgba = Rgba::new(0xee, 0xe8, 0xaa, 0xff);
    pub const PALE_GREEN: Rgba = Rgba::new(0x98, 0xfb, 0x98, 0xff);
    pub const PALE_TURQUOISE: Rgba = Rgba::new(0xaf, 0xee, 0xee, 0xff);
    pub const PALE_VIOLETRED: Rgba = Rgba::new(0xdb, 0x70, 0x93, 0xff);
    pub const PAPAYA_WHIP: Rgba = Rgba::new(0xff, 0xef, 0xd5, 0xff);
    pub const PEACHPUFF: Rgba = Rgba::new(0xff, 0xda, 0xb9, 0xff);
    pub const PERU: Rgba = Rgba::new(0xcd, 0x85, 0x3f, 0xff);
    pub const PINK: Rgba = Rgba::new(0xff, 0xc0, 0xcb, 0xff);
    pub const PLUM: Rgba = Rgba::new(0xdd, 0xa0, 0xdd, 0xff);
    pub const POWDER_BLUE: Rgba = Rgba::new(0xb0, 0xe0, 0xe6, 0xff);
    pub const PURPLE: Rgba = Rgba::new(0x80, 0x00, 0x80, 0xff);
    pub const RED: Rgba = Rgba::new(0xff, 0x00, 0x00, 0xff);
    pub const ROSY_BROWN: Rgba = Rgba::new(0xbc, 0x8f, 0x8f, 0xff);
    pub const ROYAL_BLUE: Rgba = Rgba::new(0x41, 0x69, 0xe1, 0xff);
    pub const SADDLE_BROWN: Rgba = Rgba::new(0x8b, 0x45, 0x13, 0xff);
    pub const SALMON: Rgba = Rgba::new(0xfa, 0x80, 0x72, 0xff);
    pub const SANDY_BROWN: Rgba = Rgba::new(0xf4, 0xa4, 0x60, 0xff);
    pub const SEA_GREEN: Rgba = Rgba::new(0x2e, 0x8b, 0x57, 0xff);
    pub const SEASHELL: Rgba = Rgba::new(0xff, 0xf5, 0xee, 0xff);
    pub const SIENNA: Rgba = Rgba::new(0xa0, 0x52, 0x2d, 0xff);
    pub const SILVER: Rgba = Rgba::new(0xc0, 0xc0, 0xc0, 0xff);
    pub const SKY_BLUE: Rgba = Rgba::new(0x87, 0xce, 0xeb, 0xff);
    pub const SLATE_BLUE: Rgba = Rgba::new(0x6a, 0x5a, 0xcd, 0xff);
    pub const SLATE_GRAY: Rgba = Rgba::new(0x70, 0x80, 0x90, 0xff);
    pub const SLATE_GREY: Rgba = Rgba::new(0x70, 0x80, 0x90, 0xff);
    pub const SNOW: Rgba = Rgba::new(0xff, 0xfa, 0xfa, 0xff);
    pub const SPRING_GREEN: Rgba = Rgba::new(0x00, 0xff, 0x7f, 0xff);
    pub const STEEL_BLUE: Rgba = Rgba::new(0x46, 0x82, 0xb4, 0xff);
    pub const TAN: Rgba = Rgba::new(0xd2, 0xb4, 0x8c, 0xff);
    pub const TEAL: Rgba = Rgba::new(0x00, 0x80, 0x80, 0xff);
    pub const THISTLE: Rgba = Rgba::new(0xd8, 0xbf, 0xd8, 0xff);
    pub const TOMATO: Rgba = Rgba::new(0xff, 0x63, 0x47, 0xff);
    pub const TURQUOISE: Rgba = Rgba::new(0x40, 0xe0, 0xd0, 0xff);
    pub const VIOLET: Rgba = Rgba::new(0xee, 0x82, 0xee, 0xff);
    pub const WHEAT: Rgba = Rgba::new(0xf5, 0xde, 0xb3, 0xff);
    pub const WHITE: Rgba = Rgba::new(0xff, 0xff, 0xff, 0xff);
    pub const WHITE_SMOKE: Rgba = Rgba::new(0xf5, 0xf5, 0xf5, 0xff);
    pub const YELLOW: Rgba = Rgba::new(0xff, 0xff, 0x00, 0xff);
    pub const YELLOW_GREEN: Rgba = Rgba::new(0x9a, 0xcd, 0x32, 0xff);
}

impl From<image::Rgba<u8>> for Rgba {
    fn from(value: image::Rgba<u8>) -> Self {
        Rgba::new(value[0], value[1], value[2], value[3])
    }
}

impl Into<image::Rgba<u8>> for Rgba {
    fn into(self) -> image::Rgba<u8> {
        image::Rgba([self.r, self.g, self.b, self.a])
    }
}

impl From<X256Color> for Rgba {
    fn from(value: X256Color) -> Self {
        x256_to_rgba(value.0)
    }
}

impl BitOrAssign for Rgba {
    fn bitor_assign(&mut self, rhs: Self) {
        // Hopefully the compiler is smart enough to produce optimal code for
        // this. If not, we could unsafe-cast both to u64 and do a single op.
        self.r |= rhs.r;
        self.g |= rhs.g;
        self.b |= rhs.b;
        self.a |= rhs.a;
    }
}

impl BitAndAssign for Rgba {
    fn bitand_assign(&mut self, rhs: Self) {
        self.r &= rhs.r;
        self.g &= rhs.g;
        self.b &= rhs.b;
        self.a &= rhs.a;
    }
}

impl Mul for Rgba {
    type Output = Rgba;

    fn mul(self, rhs: Self) -> Self::Output {
        Rgba {
            r: (((self.r as u16) * (rhs.r as u16)) / 255) as u8,
            g: (((self.g as u16) * (rhs.g as u16)) / 255) as u8,
            b: (((self.b as u16) * (rhs.b as u16)) / 255) as u8,
            a: (((self.a as u16) * (rhs.a as u16)) / 255) as u8,
        }
    }
}

/// 256 color Xterm palette color.
///
/// See <https://www.ditig.com/256-colors-cheat-sheet>.
#[derive(
    Copy,
    Clone,
    Default,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct X256Color(pub u8);

impl X256Color {
    // System color names

    // Might be black or white, depends on user's terminal theme.
    pub const BACKGROUND: X256Color = X256Color(0x0);
    pub const MAROON: X256Color = X256Color(0x1);
    pub const GREEN: X256Color = X256Color(0x2);
    pub const BROWN: X256Color = X256Color(0x3);
    pub const NAVY: X256Color = X256Color(0x4);
    pub const PURPLE: X256Color = X256Color(0x5);
    pub const TEAL: X256Color = X256Color(0x6);
    // Might be white or black, depends on user's terminal theme.
    pub const FOREGROUND: X256Color = X256Color(0x7);
    pub const GRAY: X256Color = X256Color(0x8);
    pub const RED: X256Color = X256Color(0x9);
    pub const LIME: X256Color = X256Color(0xa);
    pub const YELLOW: X256Color = X256Color(0xb);
    pub const BLUE: X256Color = X256Color(0xc);
    pub const FUCHSIA: X256Color = X256Color(0xd);
    pub const AQUA: X256Color = X256Color(0xe);
    pub const BOLD_FOREGROUND: X256Color = X256Color(0xf);

    seq!(N in 0..=255 {
    pub const PALETTE: [Rgba; 256] =
    [
        #(x256_to_rgba(N),)*
    ];
    });
}

impl From<Rgba> for X256Color {
    fn from(col: Rgba) -> Self {
        // Snap 256-value component to closest x256 6-value component.
        fn c(x: u8) -> u8 {
            (((x as i32) - 35) / 40).max(0) as u8
        }

        // 6x6x6 chromatic color
        let c1 = 16 + c(col.r) * 36 + c(col.g) * 6 + c(col.b);
        // greyscale slide color
        let c2 =
            232 + (((col.greyscale() as i32 - 3) / 10).max(0).min(23) as u8);

        X256Color(
            if x256_to_rgba(c1).square_dist(&col)
                < x256_to_rgba(c2).square_dist(&col)
            {
                c1
            } else {
                c2
            },
        )
    }
}

const fn x256_to_rgba(c: u8) -> Rgba {
    match c {
        // EGA-like default system color palette.
        0x0 => Rgba::new(0x00, 0x00, 0x00, 0xff), // background
        0x1 => Rgba::new(0xaa, 0x00, 0x00, 0xff), // maroon
        0x2 => Rgba::new(0x00, 0xaa, 0x00, 0xff), // green
        0x3 => Rgba::new(0xaa, 0x55, 0x00, 0xff), // brown
        0x4 => Rgba::new(0x22, 0x22, 0xcc, 0xff), // navy
        0x5 => Rgba::new(0xaa, 0x00, 0xaa, 0xff), // purple
        0x6 => Rgba::new(0x00, 0xaa, 0xaa, 0xff), // teal
        0x7 => Rgba::new(0xaa, 0xaa, 0xaa, 0xff), // foreground
        0x8 => Rgba::new(0x55, 0x55, 0x55, 0xff), // gray
        0x9 => Rgba::new(0xff, 0x55, 0x55, 0xff), // red
        0xa => Rgba::new(0x55, 0xff, 0x55, 0xff), // lime
        0xb => Rgba::new(0xff, 0xff, 0x55, 0xff), // yellow
        0xc => Rgba::new(0x55, 0x55, 0xff, 0xff), // blue
        0xd => Rgba::new(0xff, 0x55, 0xff, 0xff), // fuchsia
        0xe => Rgba::new(0x55, 0xff, 0xff, 0xff), // aqua
        0xf => Rgba::new(0xff, 0xff, 0xff, 0xff), // bold foreground

        x if x < 232 => {
            const fn c(i: u8) -> u8 {
                i * 40 + if i > 0 { 55 } else { 0 }
            }
            let x = x - 0x10;
            Rgba::new(c(x / 36), c((x / 6) % 6), c(x % 6), 0xff)
        }
        x => {
            let x = x - 232;
            let c = 8 + 10 * x;
            Rgba::new(c, c, c, 0xff)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_mul() {
        assert_eq!(Rgba::BLACK * Rgba::BLACK, Rgba::BLACK);
        assert_eq!(Rgba::WHITE * Rgba::WHITE, Rgba::WHITE);
        assert_eq!(Rgba::WHITE * Rgba::RED, Rgba::RED);
        assert_eq!(Rgba::BLACK * Rgba::RED, Rgba::BLACK);
    }
}
