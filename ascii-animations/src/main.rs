use clap::Parser;
use std::env::current_dir;
use std::fs::{self, create_dir_all, remove_dir_all};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

mod image_processor;
use crate::image_processor::resize_image;
use image_processor::ascii_representation_from_image;

#[derive(Parser)]
struct Cli {
    #[clap(name = "path", help = "Path to the input file (image or video)")]
    file_name: String,

    #[clap(long, default_value = "true", help = "Process a video file as an animation")]
    animation: bool,

    #[clap(short, long, help = "Use grayscale ASCII characters instead of colored output")]
    gray_scale: bool,

    #[clap(short, long, default_value = "20", help = "Width of the ASCII output")]
    width: u32,

    #[clap(long, default_value = "20", help = "Height of the ASCII output")]
    height: u32,

    #[clap(short, long, help = "Output directory for the ASCII text frames")]
    output: Option<String>,

    #[clap(long, help = "Save the resized image used for ASCII conversion")]
    save_intermediate: Option<String>,

    #[clap(short, long, help = "Adjust the width scaling factor")]
    fatness: Option<f32>,

    #[clap(
        long,
        default_value = "24",
        help = "Frames per second for animation extraction"
    )]
    fps: u32,

    #[clap(long, help = "Play the animation after processing")]
    play: bool,

    #[clap(
        long,
        default_value = "41",
        help = "Milliseconds delay between frames during playback"
    )]
    delay: u64,

    #[clap(long, help = "Keep intermediate frame images after processing")]
    keep_frames: bool,
}

fn extract_frames_from_video<P: AsRef<Path>>(
    video_path: P,
    output_dir: P,
    fps: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_pattern = output_dir.as_ref().join("frame_%04d.png");

    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(video_path.as_ref())
        .arg("-vf")
        .arg(format!("fps={}", fps))
        .arg("-pix_fmt")
        .arg("rgb24")
        .arg(output_pattern)
        .status()?;

    if !status.success() {
        return Err("ffmpeg command failed".into());
    }

    Ok(())
}

fn play_animation<P: AsRef<Path>>(
    frames_dir: P,
    delay_ms: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let frame_files = fs::read_dir(frames_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    let mut frame_files = frame_files;
    frame_files.sort();

    print!("\x1B[2J\x1B[1;1H");

    for frame_path in frame_files {
        print!("\x1B[2J\x1B[1;1H");

        let frame_content = fs::read_to_string(frame_path)?;
        print!("{}", frame_content);

        sleep(Duration::from_millis(delay_ms));
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let animations_path = current_dir()?.join("animations");
    let frames_path = current_dir()?.join("frames");
    let text_path = if let Some(output_dir) = &cli.output {
        PathBuf::from(output_dir)
    } else {
        current_dir()?.join("ascii_frames")
    };
        // Ensure directories exist
        create_dir_all(&animations_path)?;
        create_dir_all(&text_path)?;

        let mut frame_files: Vec<PathBuf>;

        if cli.animation {
            create_dir_all(&frames_path)?;
            extract_frames_from_video(animations_path.join(&cli.file_name), frames_path.clone(), cli.fps)?;

            frame_files = fs::read_dir(&frames_path)?
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .collect::<Vec<_>>();
        } else {
            frame_files = vec![animations_path.join(&cli.file_name)];
        }

        frame_files.sort();

        for (index, frame_path) in frame_files.iter().enumerate() {
            let image = image::open(frame_path)?;
            let resized_image = resize_image(&image, cli.width, cli.height, cli.fatness);

            if let Some(save_path) = &cli.save_intermediate {
                if index == 0 {
                    resized_image.save(save_path)?;
                }
            }

            let ascii_frame = ascii_representation_from_image(&resized_image, cli.gray_scale);
            let text_file_path = text_path.join(format!("frame_{:04}.txt", index));
            fs::write(&text_file_path, ascii_frame.as_bytes())?;
        }

        if cli.play {
            play_animation(&text_path, cli.delay)?;
        }

        if cli.animation && frames_path.exists() {
            remove_dir_all(frames_path)?;
            println!("Removed intermediate frame images");
        }

        println!("Animation processing complete!");
        println!("ASCII frames saved to: {}", text_path.display());

    Ok(())
}
