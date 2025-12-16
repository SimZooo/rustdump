use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// File to dump
    pub file: String,
    /// Amount of lines displayed per-page
    #[arg(short, long)]
    pub line_length: Option<usize>,
}

struct OutputRow {
    offset: u64,
    bytes: Vec<u8>,
}

impl OutputRow {
    fn new() -> Self {
        OutputRow {
            offset: 0,
            bytes: Vec::new(),
        }
    }
}

impl Display for OutputRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08X} ", self.offset)?;
        for byte in &self.bytes {
            write!(f, "{:02X} ", byte)?;
        }
        for byte in &self.bytes {
            let ch = *byte as char;
            let ch = if ch.is_ascii_graphic() { ch } else { '.' };
            write!(f, "{} ", ch)?;
        }
        Ok(())
    }
}

fn main() {
    let args = Args::parse();
    let path = args.file;

    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = vec![];
    let n = match reader.read_to_end(&mut buf) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            std::process::exit(1);
        }
    };

    println!("Read {} bytes from file {}", n, path);
    let bytes = buf.bytes();
    let mut output: Vec<OutputRow> = Vec::new();
    let mut curr_row = OutputRow::new();
    for byte in bytes {
        match byte {
            Ok(byte) => {
                if curr_row.bytes.len() >= 16 {
                    let old_offset = curr_row.offset;
                    output.push(curr_row);
                    curr_row = OutputRow::new();
                    curr_row.offset = old_offset + 16;
                }

                curr_row.bytes.push(byte);
            }
            Err(e) => {
                eprintln!("{e}")
            }
        }
    }

    let mut out = String::new();
    let mut last_row: Option<OutputRow> = None;
    for row in output {
        // "Concatenate" rows with identical bytes
        if let Some(last_row_some) = last_row {
            if last_row_some.bytes == row.bytes {
                if !out.ends_with("*\n") {
                    out += "*\n";
                }
                last_row = Some(row);
                continue;
            }
        }
        out += format!("{}\n", row).as_str();

        last_row = Some(row);
    }

    if let Some(line_length) = args.line_length {
        let out_split = out.split("\n").collect::<Vec<&str>>();
        let mut offset = 0;
        loop {
            let out = (offset..offset + line_length)
                .filter_map(|i| {
                    if i < out_split.len() {
                        Some(out_split[i].to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>();
            if offset >= out_split.len() {
                break;
            }
            offset += line_length;
            println!("{}", out.join("\n"));
            println!(
                "Press Enter to continue ({}/{})...",
                (offset / line_length) as usize,
                (out_split.len() / line_length) as usize
            );
            let mut t = String::new();
            let _ = io::stdin().read_line(&mut t);
        }

        return;
    }

    println!("{}", out);
}
