use crate::{errors::ZipError, map::compress::CompressionMap};
use crate::map::decompress::DecompressionMap;
use crate::tree::node_tree::Tree;
use crate::utils::*;
use serde::{Serialize, Deserialize};
use postcard::{to_stdvec, from_bytes};
use std::fs::File;
use tar::{Builder, Archive};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Zipper {
    map: DecompressionMap,
    bytes: Vec<u8>
}

impl Zipper {
    pub fn new() -> Self {
        Self { map: DecompressionMap::new(), bytes: vec![] }
    }

    pub fn dump(&self, filename: impl AsRef<str>) -> Result<(), ZipError> {
        let data = to_stdvec(self)?;
        write_bytes(filename.as_ref(), data)?;
        Ok(())
    }

    pub fn from_file(filename: impl AsRef<str>) -> Result<Self, ZipError> {
        let bytes = read_bytes(filename)?;
        let zipper: Zipper = from_bytes(&bytes)?;
        Ok(zipper)
    }

    pub fn compress(&mut self, filename: impl AsRef<str>, tar_filename: impl AsRef<str>, outfile: impl AsRef<str>) -> Result<(), ZipError> {
        let tar_file = File::create(&tar_filename.as_ref())?;
        let mut t = Builder::new(tar_file);
        t.append_dir_all(filename.as_ref(), filename.as_ref())?;
        t.finish()?;
        let map = CompressionMap::from(&Tree::from_file(&tar_filename)?);
        let mut result = vec![];
        for byte in read_bytes(&tar_filename)?.iter() {
            if let Some(mut boolvec) = map.get(byte) {
                result.append(&mut boolvec);
            }
        }
        self.bytes = vec_bool_to_vec_u8(result);
        self.map = DecompressionMap::from(&map);
        self.dump(outfile)?;
        Ok(())
    }

    pub fn decompress(filename: impl AsRef<str>, tar_filename: impl AsRef<str>, outfile: impl AsRef<str>) -> Result<(), ZipError> {
        let zipper = Zipper::from_file(filename.as_ref())?;
        let mut data = vec_u8_to_vec_bool(zipper.bytes);
        let tree = Tree::from(&zipper.map);
        let mut raw_data: Vec<u8>=vec![];
        while let Some(byte) = tree.search(&mut data) {
            raw_data.push(byte);
        }
        write_bytes(&tar_filename, raw_data)?;
        let tar_file = File::open(tar_filename.as_ref())?;
        let mut archive = Archive::new(tar_file);
        archive.unpack(outfile.as_ref())?;
        Ok(())
    }
}



