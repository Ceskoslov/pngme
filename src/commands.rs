use std::fs;

use crate::Result;
use crate::args::{DecodeArgs, EncodeArgs, PngMeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;

use crate::png::Png;

pub fn run(command: PngMeArgs) -> Result<()> {
    match command {
        PngMeArgs::Encode(args) => encode(args),
        PngMeArgs::Decode(args) => decode(args),
        PngMeArgs::Remove(args) => remove(args),
        PngMeArgs::Print(args) => print_chunks(args),
    }
}

fn encode(args: EncodeArgs) -> Result<()> {
    let bytes = fs::read(&args.file_path)?;
    let mut png = Png::try_from(bytes.as_ref()).map_err(|e| e.to_string())?;

    let chunk_type = args.chunk_type;
    let data = args.message.into_bytes();
    let chunk = Chunk::new(chunk_type, data);
    png.append_chunk(chunk);

    let output_path = args.output_file.unwrap_or(args.file_path);
    fs::write(output_path, png.as_bytes())?;
    println!("Message encoded successfully.");
    Ok(())
}

fn decode(args: DecodeArgs) -> Result<()> {
    let bytes = fs::read(&args.file_path)?;
    let png = Png::try_from(bytes.as_ref()).map_err(|e| e.to_string())?;

    let chunk_type = args.chunk_type.to_string();
    match png.chunk_by_type(&chunk_type) {
        Some(chunk) => {
            let message = chunk.data_as_string()?;
            println!("Hidden message: {}", message);
        }
        None => {
            println!("No chunk with type '{}' found.", chunk_type);
        }
    }
    Ok(())
}

fn remove(args: RemoveArgs) -> Result<()> {
    let bytes = fs::read(&args.file_path)?;
    let mut png = Png::try_from(bytes.as_ref()).map_err(|e| e.to_string())?;

    let chunk_type = args.chunk_type.to_string();
    png.remove_first_chunk(&chunk_type)
        .map_err(|e| e.to_string())?;

    let output_path = args.output_file.unwrap_or(args.file_path);
    fs::write(output_path, png.as_bytes())?;
    println!("Chunk '{}' removed successfully.", chunk_type);
    Ok(())
}

fn print_chunks(args: PrintArgs) -> Result<()> {
    let bytes = fs::read(&args.file_path)?;
    let png = Png::try_from(bytes.as_ref()).map_err(|e| e.to_string())?;

    println!("PNG file: {}", args.file_path.display());
    println!("Number of chunks: {}", png.chunks().len());
    println!();
    for (i, chunk) in png.chunks().iter().enumerate() {
        println!(
            "Chunk #{}: type={}, length={}, crc={}",
            i + 1,
            chunk.chunk_type(),
            chunk.length(),
            chunk.crc()
        );
        if let Ok(s) = chunk.data_as_string() {
            if s.chars().all(|c| !c.is_control()) {
                println!("         data (text): {}", s);
            }
        }
    }
    Ok(())
}
