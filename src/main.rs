use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::ops::Deref;
use std::str::FromStr;
use clap::Parser;
use image::{GenericImage, GenericImageView, ImageBuffer};
use image::io::Reader;
use crate::PosIntError::{ParseError, ZeroError};

/// Represents errors converting a string to a positive (non-zero, non-negative) integer
#[derive(Debug)]
enum PosIntError {

    /// Error parsing a string to a non-negative integer
    ParseError(ParseIntError),

    /// The integer is zero.
    ZeroError

}

impl Error for PosIntError {}

impl fmt::Display for PosIntError {

    /// Formats this error as a string.
    ///
    /// # Arguments
    ///
    /// * `f` - formatter
    ///
    /// # Errors
    ///
    /// Returns a formatting error if this error could not be written to the formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError(err) => write!(f, "{}", err),
            ZeroError => write!(f, "Integer cannot be zero")
        }
    }

}

impl From<ParseIntError> for PosIntError {

    /// Converts a [`ParseIntError`] from the standard library to the more general [`PosIntError`].
    ///
    /// # Arguments
    ///
    /// * `err` - error to convert
    fn from(err: ParseIntError) -> Self {
        ParseError(err)
    }

}

/// Parses a string as a positive (non-negative, non-zero) 32-bit integer.
///
/// # Arguments
///
/// * `str` - string to parse as a positive integer
///
/// # Errors
///
/// The function returns a [`ParseError`] if the integer cannot be
/// parsed into an unsigned 32-bit integer or a [`ZeroError`] if the
/// integer is zero.
fn parse_positive_int(str: &str) -> Result<u32, PosIntError> {
    let result = u32::from_str(str)?;

    match result == 0 {
        true => Err(ZeroError),
        false => Ok(result)
    }
}

/// Contains program arguments parsed from the command line
#[derive(Parser)]
#[clap(author, version, about)]
struct FrameTranslationGenerator {

    /// Path to input image
    #[arg(value_parser, short = 'i', long = "input")]
    input: std::path::PathBuf,

    /// Horizontal distance to translate per frame
    #[arg(long = "dx")]
    delta_x: f32,

    /// vertical distance to translate per frame
    #[arg(long = "dy")]
    delta_y: f32,

    /// Width of a frame in the image
    #[arg(short = 'x', long = "x-start")]
    x: u32,

    /// Height of a frame in the image
    #[arg(short = 'y', long = "y-start")]
    y: u32,

    /// Width of a frame in the image
    #[arg(value_parser = parse_positive_int, long = "fw")]
    frame_width: u32,

    /// Height of a frame in the image
    #[arg(value_parser = parse_positive_int, long = "fh")]
    frame_height: u32,

    /// Height of a frame in the image
    #[arg(value_parser = parse_positive_int, short = 'm', long = "max-frames", default_value = "4294967295")]
    max_frames: u32,

    /// Path to location whose contents will be overwritten with output
    #[arg(value_parser, short = 'o', long = "output")]
    output: std::path::PathBuf,

}

/// Runs the CLI.
///
/// # Panics
///
/// Panics if the provided command-line arguments are invalid (see [`FrameTranslationGenerator`]), if
/// the frame width or frame height are larger than the image dimensions, if the
/// input location cannot be read from, or if the output location cannot be written to.
fn main() {
    let args = FrameTranslationGenerator::parse();

    let mut reader = Reader::open(args.input).expect("Image not found");
    reader.no_limits();
    let source = reader.with_guessed_format()
        .expect("IO error while trying to guess image format")
        .decode()
        .expect("Unable to decode image");
    if args.x + args.frame_width > source.width() || args.y + args.frame_height > source.height() {
        panic!("First frame outside source image");
    }

    let mut cur_x = args.x as f32;
    let mut cur_y = args.y as f32;
    let mut rounded_x = args.x;
    let mut rounded_y = args.y;
    let mut frames = Vec::new();

    loop {
        if frames.len() == args.max_frames as usize {
            break;
        }

        if cur_x < 0f32 || cur_y < 0f32 {
            break;
        }

        if rounded_x + args.frame_width > source.width() || rounded_y + args.frame_height > source.height() {
            break;
        }

        frames.push(source.view(rounded_x, rounded_y, args.frame_width, args.frame_height));

        cur_x += args.delta_x;
        cur_y += args.delta_y;
        rounded_x = cur_x.round() as u32;
        rounded_y = cur_y.round() as u32;
    }

    let mut dest = ImageBuffer::new(
        args.frame_width,
        args.frame_height * frames.len() as u32
    );

    for (index, frame) in frames.iter().enumerate() {
        dest.copy_from(
            frame.deref(),
            0,
            args.frame_height * index as u32
        ).expect("Unable to copy frame");
    }

    println!("Saving {} frames", frames.len());
    dest.save(args.output).expect("Unable to save image");
}
