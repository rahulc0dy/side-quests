use image::{DynamicImage, GenericImageView, Rgba};

pub const ASCII_LOOKUP: [&str; 16] = [
    " ",
    "~",
    "$",
    ">",
    "╶",
    "=",
    "<",
    "=",
    "^",
    "+",
    "$",
    "$",
    "~",
    "*",
    "@",
    "#",
];

pub fn resize_image(image: &DynamicImage, width: u32, height: u32, fatness: Option<f32>) -> DynamicImage {
    image.resize_exact(
        (width as f32 * fatness.unwrap_or(2.45)) as u32,
        height,
        image::imageops::FilterType::Nearest,
    )
}

pub fn ascii_representation_from_image(image: &DynamicImage, gray_scale: bool) -> String {
    let mut output_raster: Vec<Vec<String>> =
        vec![vec![String::new(); image.width() as usize]; image.height() as usize];

    let chars = ASCII_LOOKUP
        .iter()
        .map(|&s| s.chars().next().unwrap())
        .collect::<Vec<char>>();

    for (x, y, rgba) in image.pixels() {
        let [r, g, b, a] = rgba.0;

        if a as f32 / 255f32 < 0.25 {
            output_raster[y as usize][x as usize] = " ".to_string();
            continue;
        }

        let around: [[Rgba<u8>; 3]; 3] = [
            [
                get_pixel_checked(image, x.saturating_sub(1), y + 1)
                    .unwrap_or(Rgba::from([0, 0, 0, 0])),
                get_pixel_checked(image, x, y + 1).unwrap_or(Rgba::from([0, 0, 0, 0])),
                get_pixel_checked(image, x + 1, y + 1).unwrap_or(Rgba::from([0, 0, 0, 0])),
            ],
            [
                get_pixel_checked(image, x.saturating_sub(1), y)
                    .unwrap_or(Rgba::from([0, 0, 0, 0])),
                rgba,
                get_pixel_checked(image, x + 1, y).unwrap_or(Rgba::from([0, 0, 0, 0])),
            ],
            [
                get_pixel_checked(image, x.saturating_sub(1), y.saturating_sub(1))
                    .unwrap_or(Rgba::from([0, 0, 0, 0])),
                get_pixel_checked(image, x, y.saturating_sub(1))
                    .unwrap_or(Rgba::from([0, 0, 0, 0])),
                get_pixel_checked(image, x + 1, y.saturating_sub(1))
                    .unwrap_or(Rgba::from([0, 0, 0, 0])),
            ],
        ];

        let grid: Vec<Vec<u8>> = around
            .iter()
            .map(|row| {
                row.iter()
                    .map(|pixel| (rgba_distance_squared(*pixel, rgba) < 50) as u8)
                    .collect::<Vec<u8>>()
            })
            .collect();

        let subs = extract_2x2(&grid);

        let best = subs
            .iter()
            .max_by_key(|&&sub| sub.iter().flatten().sum::<u8>())
            .unwrap();

        let ind = marching_square_index(*best);
        let char = chars[ind as usize];

        let formatted = if !gray_scale {
            format!("\x1b[38;2;{r};{g};{b}m{}\x1b[0m", char.to_string())
        } else {
            format!("{}", char)
        };

        output_raster[y as usize][x as usize] = formatted;
    }

    let mut buff = String::new();

    for row in output_raster {
        for pixel in row {
            buff.push_str(&pixel);
        }
        buff.push('\n');
    }

    buff
}

fn rgba_distance_squared(a: Rgba<u8>, b: Rgba<u8>) -> u32 {
    a.0.iter()
        .zip(b.0.iter())
        .map(|(x, y)| (*x as i32 - *y as i32).pow(2) as u32)
        .sum()
}

fn get_pixel_checked(image: &DynamicImage, x: u32, y: u32) -> Option<Rgba<u8>> {
    if x >= image.width() || y >= image.height() {
        None
    } else {
        Some(image.get_pixel(x, y))
    }
}

fn extract_2x2(grid: &Vec<Vec<u8>>) -> Vec<[[u8; 2]; 2]> {
    vec![
        [[grid[0][0], grid[0][1]], [grid[1][0], grid[1][1]]],
        [[grid[0][1], grid[0][2]], [grid[1][1], grid[1][2]]],
        [[grid[1][0], grid[1][1]], [grid[2][0], grid[2][1]]],
        [[grid[1][1], grid[1][2]], [grid[2][1], grid[2][2]]],
    ]
}

fn marching_square_index(square: [[u8; 2]; 2]) -> u8 {
    (square[0][0] << 3) | (square[0][1] << 2) | (square[1][1] << 1) | (square[1][0] << 0)
}
