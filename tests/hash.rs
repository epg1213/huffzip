
use huffzip::huffman::Zipper;
use huffzip::errors::ZipError;
use huffzip::utils::*;
use std::path::Path;
use sha256::try_digest;
use std::fs::remove_file;

#[test]
fn test_hash_origin() -> Result<(), ZipError> {
    let infile = "testfile";
    let outfile = "testfile.hzip";
    write_bytes(infile, vec![1, 3, 3, 7])?;
    let hash = try_digest(Path::new(infile))?;
    Zipper::new().compress(infile, outfile)?;
    remove_file(infile)?;
    Zipper::decompress(outfile, infile)?;
    assert_eq!(hash, try_digest(Path::new(infile))?);
    remove_file(infile)?;
    remove_file(outfile)?;
    Ok(())
}

