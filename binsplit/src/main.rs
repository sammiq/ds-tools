use byteorder::{LittleEndian, ReadBytesExt};
use lexopt::{Arg, Parser, ValueExt};
use std::io::Read;
use std::io::Seek;
struct Args {
    filename: Vec<String>,
    read_size: u64,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    let mut filename = Vec::new();
    let mut read_size = 0x1000; // default value

    let mut args = Parser::from_env();
    while let Some(arg) = args.next()? {
        match arg {
            Arg::Value(val) => {
                filename.push(val.string()?);
            }
            Arg::Long("read-size") => {
                read_size = args.value()?.parse()?;
            }
            Arg::Long("help") => {
                println!("Usage: binaplit [--read-size=<size>] <binfile>");
                std::process::exit(0);
            }
            _ => {}
        }
    }

    if filename.is_empty() {
        eprint!("Error: No input file specified.\n");
        std::process::exit(1);
    }

    Ok(Args {
        filename: filename,
        read_size,
    })
}

struct FileOffset {
    begin: u32,
    end: u32,
}

fn main() {
    let args = parse_args().expect("Failed to parse command line");
    for filename in args.filename {
        let mut file = std::fs::File::open(&filename).expect("Failed to open input file");

        let mut offsets = Vec::new();
        while file.stream_position().unwrap() < args.read_size {
            let begin = file.read_u32::<LittleEndian>().expect("Failed to read begin");
            let end = file.read_u32::<LittleEndian>().expect("Failed to read end");

            if begin == 0 || end == 0 || end <= begin {
                break;
            }

            offsets.push(FileOffset { begin, end });
        }
        for offset in offsets {
            println!("Reading item from 0x{:X} to 0x{:X}", offset.begin, offset.end);
            file.seek(std::io::SeekFrom::Start(offset.begin as u64))
                .expect("Failed to seek to file data");

            let length = (offset.end - offset.begin) as usize;
            let mut buffer = vec![0u8; length];
            file.read_exact(&mut buffer).expect("Failed to read data");

            let input_name = std::path::Path::new(&filename).file_stem().unwrap().to_string_lossy();
            let output_file_name = format!("{}_0x{:X}.{}", input_name, offset.begin, detect_file_suffix(&buffer));

            std::fs::write(&output_file_name, &buffer).expect("Failed to write output file");
            println!("Extracted file {}: {} bytes", output_file_name, length);
        }
    }
}

fn detect_file_suffix(file_data: &[u8]) -> &'static str {
    match file_data.get(0..4) {
        Some(b"MIG.") => "gim", //PSP Image
        Some(b"MThd") => "mid", //MIDI Audio
        Some(b"PPHD") => "phd", //PSP Audio
        Some(b"PSMF") => "pmf", //PSP Movie
        Some(b"VAGp") => "vag", //Playstation Audio
        _ => "bin",
    }
}
