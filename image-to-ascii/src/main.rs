use clap::Parser;
use image::{GenericImageView, Rgba};

/// Lookup table for converting binary patterns to ASCII characters.
/// Each index corresponds to a 4-bit pattern representing a 2x2 grid arrangement.
/// The comments show the binary pattern each character represents.
const ASCII_LOOKUP: [&str; 16] = [
    " ", // 0000 - Empty space
    "~", // 0001 - Bottom-left corner
    "$", // 0010 - Bottom-right corner
    ">", // 0011 - Bottom edge
    "╶", // 0100 - Top-left corner
    "=", // 0101 - Left edge
    "<", // 0110 - Diagonal (top-left to bottom-right)
    "=", // 0111 - Heavy horizontal
    "^", // 1000 - Top-right corner
    "+", // 1001 - Diagonal (top-right to bottom-left)
    "$", // 1010 - Right edge
    "$", // 1011 - Heavy right side
    "~", // 1100 - Top edge
    "*", // 1101 - Heavy intersection
    "@", // 1110 - Heavy bottom
    "#", // 1111 - Full block
];

/// Command-line interface configuration structure.
/// Uses clap for parsing command-line arguments.
#[derive(Parser)]
struct Cli {
    /// Path to the input image file that will be converted to ASCII art
    #[clap(name = "path")]
    input_path: String,

    /// When enabled, outputs the ASCII art in grayscale without color
    #[clap(short, long)]
    gray_scale: bool,

    /// Width of the output ASCII art in characters
    /// Defaults to 20 characters
    #[clap(short, long)]
    #[clap(long, default_value = "20")]
    width: u32,

    /// Height of the output ASCII art in characters
    /// Defaults to 20 characters
    #[clap(long, default_value = "20")]
    height: u32,

    /// Optional path to save the ASCII art output to a file
    /// If not provided, output will be printed to stdout
    #[clap(short, long)]
    output: Option<String>,

    /// Optional path to save the intermediate processed image
    /// This is the resized and prepared image before ASCII conversion
    #[clap(
        long,
        help = "The program will save the image produced for the asciifying to a file"
    )]
    save_intermediate: Option<String>,

    /// Optional scaling factor for character width compensation
    /// Helps adjust for the fact that ASCII characters are typically taller than wide
    /// Default value is 2.45 if not specified
    #[clap(short, long)]
    fattness: Option<f32>,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command line arguments
    let cli = Cli::parse();

    // Load the image from the input file path
    let image = image::open(cli.input_path)?;

    // Resize the image by the fatness factor
    let image = image.resize_exact(
        (cli.width as f32 * cli.fattness.unwrap_or(2.45)) as u32,
        cli.height,
        image::imageops::FilterType::Nearest,
    );

    // Save to intermediate path if specified
    if let Some(path) = cli.save_intermediate {
        image.save(path)?;
    }

    // 2d vector to store the output
    let mut output_raster: Vec<Vec<String>> =
        vec![vec![String::new(); image.width() as usize]; image.height() as usize];

    // ASCII lookup table into a vector of characters
    let chars = ASCII_LOOKUP
        .iter()
        .map(|&s| s.chars().next().unwrap())
        .collect::<Vec<char>>();

    for (x, y, rgba) in image.pixels() {
        let [r, g, b, a] = rgba.0;

        // Skip Transparent pixels. Replace by space in the output
        if a as f32 / 255f32 < 0.25 {
            output_raster[y as usize][x as usize] = " ".to_string();
            continue;
        }

        // Analyze the neighborhood of the pixel
        // Creates a 3x3 grid of pixels around the current pixel
        let around: [[Rgba<u8>; 3]; 3] = [
            [
                get_pixel_checked(&image, x.saturating_sub(1), y + 1)
                    .unwrap_or(Rgba::from([0, 0, 0, 0])),
                get_pixel_checked(&image, x, y + 1).unwrap_or(Rgba::from([0, 0, 0, 0])),
                get_pixel_checked(&image, x + 1, y + 1).unwrap_or(Rgba::from([0, 0, 0, 0])),
            ],
            [
                get_pixel_checked(&image, x.saturating_sub(1), y)
                    .unwrap_or(Rgba::from([0, 0, 0, 0])),
                rgba,
                get_pixel_checked(&image, x + 1, y).unwrap_or(Rgba::from([0, 0, 0, 0])),
            ],
            [
                get_pixel_checked(&image, x.saturating_sub(1), y.saturating_sub(1))
                    .unwrap_or(Rgba::from([0, 0, 0, 0])),
                get_pixel_checked(&image, x, y.saturating_sub(1))
                    .unwrap_or(Rgba::from([0, 0, 0, 0])),
                get_pixel_checked(&image, x + 1, y.saturating_sub(1))
                    .unwrap_or(Rgba::from([0, 0, 0, 0])),
            ],
        ];

        // Convert grid to binary values based on rgba similarity
        let grid: Vec<Vec<u8>> = around
            .iter()
            .map(|row| {
                row.iter()
                    .map(|pixel| (rgba_distance_squared(*pixel, rgba) < 50) as u8)
                    .collect::<Vec<u8>>()
            })
            .collect();

        // Extract all 2x2 grids from the 3x3 grid
        let subs = extract_2x2(&grid);

        // Find the subgrid with the most active pixels
        let best = subs
            .iter()
            .max_by_key(|&&sub| sub.iter().flatten().sum::<u8>())
            .unwrap();

        // Convert the pattern to an index
        let ind = marching_square_index(*best);
        // Get the ASCII value from the lookup table
        let char = chars[ind as usize];

        // Convert to grayscale or color (ANSI color codes) based on CLI arguments
        let formatted = if !cli.gray_scale {
            format!("\x1b[38;2;{r};{g};{b}m{}\x1b[0m", char.to_string())
        } else {
            format!("{}", char)
        };

        // Store the formatted character into the output buffer
        output_raster[y as usize][x as usize] = formatted;
    }

    // Output string
    let mut buff = String::new();

    // Combine all characters into a single string
    for row in output_raster {
        for pixel in row {
            buff.push_str(&pixel);
        }
        // Add new line between rows
        buff.push('\n');
    }

    // Specifies the output location based on the cli arguments
    if let Some(output) = cli.output {
        std::fs::write(&output, buff.as_bytes())?;
        println!("Output written to {}", output);
    } else {
        println!("{}", buff);
    }

    // Return success
    Ok(())
}

/// Calculates the squared Euclidean distance between two RGBA colors.
///
/// This function computes the sum of squared differences between corresponding
/// color components of two RGBA values. The result can be used to determine
/// how similar two colors are - smaller values indicate more similar colors.
///
/// # Arguments
/// * `a` - First RGBA color value
/// * `b` - Second RGBA color value
///
/// # Returns
/// * `u32` - The squared distance between the colors
fn rgba_distance_squared(a: Rgba<u8>, b: Rgba<u8>) -> u32 {
    a.0.iter()
        .zip(b.0.iter())
        .map(|(x, y)| (*x as i32 - *y as i32).pow(2) as u32)
        .sum()
}

/// Safely retrieves a pixel from an image at the specified coordinates.
///
/// # Arguments
/// * `image` - Reference to the source image
/// * `x` - X coordinate of the pixel
/// * `y` - Y coordinate of the pixel
///
/// # Returns
/// * `Some(Rgba<u8>)` if the coordinates are within the image bounds
/// * `None` if the coordinates are outside the image bounds
fn get_pixel_checked(image: &image::DynamicImage, x: u32, y: u32) -> Option<Rgba<u8>> {
    if x >= image.width() || y >= image.height() {
        None
    } else {
        Some(image.get_pixel(x, y))
    }
}

/// Extracts all possible 2x2 subgrids from a 3x3 binary grid.
///
/// Takes a 3x3 grid represented as a vector of vectors and returns
/// four 2x2 arrays representing the top-left, top-right, bottom-left,
/// and bottom-right quadrants of the input grid.
///
/// # Arguments
/// * `grid` - Reference to a 3x3 grid of binary values (0s and 1s)
///
/// # Returns
/// * Vector containing four 2x2 arrays representing each quadrant
fn extract_2x2(grid: &Vec<Vec<u8>>) -> Vec<[[u8; 2]; 2]> {
    vec![
        [[grid[0][0], grid[0][1]], [grid[1][0], grid[1][1]]], // top-left
        [[grid[0][1], grid[0][2]], [grid[1][1], grid[1][2]]], // top-right
        [[grid[1][0], grid[1][1]], [grid[2][0], grid[2][1]]], // bottom-left
        [[grid[1][1], grid[1][2]], [grid[2][1], grid[2][2]]], // bottom-right
    ]
}

/// Converts a 2x2 binary grid into a marching squares index.
///
/// Implements the marching squares algorithm by converting a 2x2 grid
/// of binary values into a single byte where each bit represents one corner.
/// The resulting value (0-15) can be used to look up the appropriate
/// ASCII character for that pattern.
///
/// # Arguments
/// * `square` - 2x2 array of binary values (0s and 1s)
///
/// # Returns
/// * `u8` - Index value (0-15) representing the pattern
///
/// # Bit layout
/// * bit 3: top-left     (square[0][0])
/// * bit 2: top-right    (square[0][1])
/// * bit 1: bottom-right (square[1][1])
/// * bit 0: bottom-left  (square[1][0])
fn marching_square_index(square: [[u8; 2]; 2]) -> u8 {
    (square[0][0] << 3) | (square[0][1] << 2) | (square[1][1] << 1) | (square[1][0] << 0)
}