use huffzip::huffman::Zipper;
use huffzip::errors::ZipError;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    infile: String,
    #[arg(short, long)]
    outfile: String,
    #[arg(short, long, default_value_t = false)]
    decompress: bool
}

fn main() -> Result<(), ZipError> {
    let args = Args::parse();
    if args.decompress {
        Zipper::decompress(args.infile, args.outfile)?;
    } else {
        Zipper::new().compress(args.infile, args.outfile)?;
    }
    Ok(())
}

