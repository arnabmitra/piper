use clap::Parser;
use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

const CHUNK_SIZE: usize = 16 * 1024;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input files
    #[arg(short = 'i', long = "infile", default_value = "")]
    infile: String,

    /// Output files
    #[arg(short = 'o', long = "outfile", default_value = "")]
    outfile: String,

    /// Whether the number of bytes read should be silent
    #[arg(short = 's', long = "silent", default_value_t = false)]
    silent: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let infile = args.infile;
    let outfile = args.outfile;
    let silent = args.silent;
    eprintln!("foo bar");
    eprintln!("{}",infile);
    eprintln!("{}",outfile);
    eprintln!("{}",silent);
    let mut total_bytes = 0;
    let silent = !env::var("SILENT").unwrap_or_default().is_empty();
    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        };
    }



    Ok(())
}
