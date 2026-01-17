
use huffzip::huffman::Zipper;
use huffzip::errors::ZipError;
use huffzip::utils::*;
use std::path::Path;
use sha256::try_digest;
use std::fs::{remove_file, remove_dir_all, create_dir};

#[test]
fn test_hash_origin() -> Result<(), ZipError> {
    let indir = "testdir";
    let infile = format!("{}/testfile", indir);
    let tmp_tar = "testtar";
    let outfile = "testfile.hzip";
    create_dir(indir)?;
    write_bytes(infile.as_str(), vec![1, 3, 3, 7])?;
    let hash = try_digest(Path::new(infile.as_str()))?;
    Zipper::new().compress(indir, tmp_tar, outfile)?;
    remove_file(infile.as_str())?;
    remove_dir_all(indir)?;
    Zipper::decompress(outfile, tmp_tar, ".")?;
    assert_eq!(hash, try_digest(Path::new(infile.as_str()))?);
    remove_file(infile)?;
    remove_dir_all(indir)?;
    remove_file(outfile)?;
    remove_file(tmp_tar)?;
    Ok(())
}

