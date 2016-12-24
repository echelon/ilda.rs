// Copyright (c) 2016 Brandon Thomas <bt@brand.io, echelon@gmail.com>

use data::ColorPalette;

/// Return the default RGB values for a given color index.
/// This is used when not color palette header is supplied.
pub fn default_color_index(index: i8) -> ColorPalette {
  match index {
    0 => ColorPalette { r: 255, g: 0, b: 0 }, // Red
    1 => ColorPalette { r: 255, g: 16, b: 0 },
    2 => ColorPalette { r: 255, g: 32, b: 0 },
    3 => ColorPalette { r: 255, g: 48, b: 0 },
    4 => ColorPalette { r: 255, g: 64, b: 0 },
    5 => ColorPalette { r: 255, g: 80, b: 0 },
    6 => ColorPalette { r: 255, g: 96, b: 0 },
    7 => ColorPalette { r: 255, g: 112, b: 0 },
    8 => ColorPalette { r: 255, g: 128, b: 0 },
    9 => ColorPalette { r: 255, g: 144, b: 0 },
    10 => ColorPalette { r: 255, g: 160, b: 0 },
    11 => ColorPalette { r: 255, g: 176, b: 0 },
    12 => ColorPalette { r: 255, g: 192, b: 0 },
    13 => ColorPalette { r: 255, g: 208, b: 0 },
    14 => ColorPalette { r: 255, g: 224, b: 0 },
    15 => ColorPalette { r: 255, g: 240, b: 0 },
    16 => ColorPalette { r: 255, g: 255, b: 0 }, // Yellow
    17 => ColorPalette { r: 224, g: 255, b: 0 },
    18 => ColorPalette { r: 192, g: 255, b: 0 },
    19 => ColorPalette { r: 160, g: 255, b: 0 },
    20 => ColorPalette { r: 128, g: 255, b: 0 },
    21 => ColorPalette { r: 96, g: 255, b: 0 },
    22 => ColorPalette { r: 64, g: 255, b: 0 },
    23 => ColorPalette { r: 32, g: 255, b: 0 },
    24 => ColorPalette { r: 0, g: 255, b: 0 }, // Green
    25 => ColorPalette { r: 0, g: 255, b: 36 },
    26 => ColorPalette { r: 0, g: 255, b: 73 },
    27 => ColorPalette { r: 0, g: 255, b: 109 },
    28 => ColorPalette { r: 0, g: 255, b: 146 },
    29 => ColorPalette { r: 0, g: 255, b: 182 },
    30 => ColorPalette { r: 0, g: 255, b: 219 },
    31 => ColorPalette { r: 0, g: 255, b: 255 }, // Cyan
    32 => ColorPalette { r: 0, g: 227, b: 255 },
    33 => ColorPalette { r: 0, g: 198, b: 255 },
    34 => ColorPalette { r: 0, g: 170, b: 255 },
    35 => ColorPalette { r: 0, g: 142, b: 255 },
    36 => ColorPalette { r: 0, g: 113, b: 255 },
    37 => ColorPalette { r: 0, g: 85, b: 255 },
    38 => ColorPalette { r: 0, g: 56, b: 255 },
    39 => ColorPalette { r: 0, g: 28, b: 255 },
    40 => ColorPalette { r: 0, g: 0, b: 255 }, // Blue
    41 => ColorPalette { r: 32, g: 0, b: 255 },
    42 => ColorPalette { r: 64, g: 0, b: 255 },
    43 => ColorPalette { r: 96, g: 0, b: 255 },
    44 => ColorPalette { r: 128, g: 0, b: 255 },
    45 => ColorPalette { r: 160, g: 0, b: 255 },
    46 => ColorPalette { r: 192, g: 0, b: 255 },
    47 => ColorPalette { r: 224, g: 0, b: 255 },
    48 => ColorPalette { r: 255, g: 0, b: 255 }, // Magenta
    49 => ColorPalette { r: 255, g: 32, b: 255 },
    50 => ColorPalette { r: 255, g: 64, b: 255 },
    51 => ColorPalette { r: 255, g: 96, b: 255 },
    52 => ColorPalette { r: 255, g: 128, b: 255 },
    53 => ColorPalette { r: 255, g: 160, b: 255 },
    54 => ColorPalette { r: 255, g: 192, b: 255 },
    55 => ColorPalette { r: 255, g: 224, b: 255 },
    56 => ColorPalette { r: 255, g: 255, b: 255 }, // White
    57 => ColorPalette { r: 255, g: 224, b: 224},
    58 => ColorPalette { r: 255, g: 192, b: 192},
    59 => ColorPalette { r: 255, g: 160, b: 160},
    60 => ColorPalette { r: 255, g: 128, b: 128},
    61 => ColorPalette { r: 255, g: 96, b: 96},
    62 => ColorPalette { r: 255, g: 64, b: 64},
    63 => ColorPalette { r: 255, g: 32, b: 32},
    _ => ColorPalette { r: 255, g: 255, b: 255 }, // Unknown; white.
  }
}

#[cfg(test)]
mod tests {
  use data::ColorPalette;
  use super::default_color_index;

  #[test]
  fn test_default_color_index() {
    // Known colors
    assert_eq!(ColorPalette { r: 255, g: 0, b: 0 }, default_color_index(0));
    assert_eq!(ColorPalette { r: 255, g: 255, b: 0 }, default_color_index(16));
    assert_eq!(ColorPalette { r: 0, g: 255, b: 0 }, default_color_index(24));
    assert_eq!(ColorPalette { r: 0, g: 255, b: 255 }, default_color_index(31));
    assert_eq!(ColorPalette { r: 0, g: 0, b: 255 }, default_color_index(40));
    assert_eq!(ColorPalette { r: 255, g: 0, b: 255 }, default_color_index(48));
    assert_eq!(ColorPalette { r: 255, g: 255, b: 255 },
        default_color_index(56));

    // Unknown colors
    assert_eq!(ColorPalette { r: 255, g: 255, b: 255 },
        default_color_index(64));
    assert_eq!(ColorPalette { r: 255, g: 255, b: 255 },
        default_color_index(100));
    assert_eq!(ColorPalette { r: 255, g: 255, b: 255 },
        default_color_index(-5));
  }
}
