use crate::chunk_type::ChunkType;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pngme", about = "Hide secret messages inside PNG files")]
pub struct Cli {
    #[structopt(subcommand)]
    pub command: PngMeArgs,
}

#[derive(Debug, StructOpt)]
pub enum PngMeArgs {
    /// Encode a secret message into a PNG file
    Encode(EncodeArgs),
    /// Decode a secret message from a PNG file
    Decode(DecodeArgs),
    /// Remove a chunk from a PNG file
    Remove(RemoveArgs),
    /// Print all chunks of a PNG file
    Print(PrintArgs),
}

#[derive(Debug, StructOpt)]
pub struct EncodeArgs {
    /// Path to the input PNG file
    pub file_path: PathBuf,
    /// The 4-byte chunk type (e.g. "ruSt")
    pub chunk_type: ChunkType,
    /// The secret message to encode
    pub message: String,
    /// Optional output file path (defaults to overwriting input)
    pub output_file: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct DecodeArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,
    /// The 4-byte chunk type to decode
    pub chunk_type: ChunkType,
}

#[derive(Debug, StructOpt)]
pub struct RemoveArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,
    /// The 4-byte chunk type to remove
    pub chunk_type: ChunkType,
    /// Optional output file path (defaults to overwriting input)
    pub output_file: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct PrintArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,
}
