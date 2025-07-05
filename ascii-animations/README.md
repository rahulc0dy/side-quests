# ASCII Animation Processor

A Rust application that converts images and videos into ASCII art animations.

## Requirements

- Rust (1.86.0 or newer)
- FFmpeg (must be installed and available in PATH)

## Installation

```bash
cargo build --release
```

## Usage

### Creating and playing an ASCII animation from a video

```bash
cargo run -- video.mp4 --play
```

### Command-line options

- `<path>`: Path to the input file (image or video)
- `--animation`: Process as video animation (default: true)
- `--gray-scale, -g`: Use grayscale ASCII characters instead of colors
- `--width, -w <WIDTH>`: Width of the ASCII output (default: 20)
- `--height <HEIGHT>`: Height of the ASCII output (default: 20)
- `--output, -o <FILE>`: Output directory for the ASCII text frames
- `--save-intermediate <FILE>`: Save the resized image used for ASCII conversion
- `--fatness, -f <FACTOR>`: Adjust the width scaling factor
- `--fps <FPS>`: Frames per second for animation extraction (default: 24)
- `--play`: Play the animation after processing
- `--delay <MS>`: Milliseconds delay between frames during playback (default: 41)

## Examples

### Convert a video to ASCII animation and play it

```bash
cargo run -- animation.mp4 --play -w 100 --height 50 --fps 30 --delay 33
```

### Save animation frames as text files

```bash
cargo run -- animation.mp4 -o output_folder/frame
```

## File Organization

- `animations/`: Place your input videos here
- `frames/`: Temporary storage for extracted video frames (automatically created and always deleted after processing)
- `text/`: Storage for ASCII text frames (automatically created and always preserved)
