use std::{fs::File, io::{Read, Write}};
use std::collections::HashMap;
use counter::Counter;
use crate::errors::ZipError;

pub fn read_bytes(filename: impl AsRef<str>) -> Result<Vec<u8>, ZipError> {
    let mut f = File::open(filename.as_ref())?;
    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn write_bytes(filename: impl AsRef<str>, bytes: Vec<u8>) -> Result<(), ZipError> {
    let mut f = File::create(filename.as_ref())?;
    f.write(&bytes)?;
    Ok(())
}

pub fn count_bytes(filename: impl AsRef<str>) -> Result<HashMap::<u8, u64>, ZipError> {
    let file_bytes = read_bytes(filename)?;
    let bytes_counts = file_bytes.iter().cloned().collect::<Counter<_, u64>>();
    Ok(bytes_counts.into_map())
}

pub fn vec_bool_to_vec_u8(mut origin: Vec<bool>) -> Vec<u8> {
    let mut destination = Vec::new();
    origin.insert(0, true); // end padding
    while origin.len()%8 != 0 {
        origin.insert(0, false);
    } // padding
    while let Some(elem) = origin.pop() {
        let mut new_u8 = 0;
        if elem {new_u8+=1;}
        if origin.pop().unwrap() {new_u8+=2;}
        if origin.pop().unwrap() {new_u8+=4;}
        if origin.pop().unwrap() {new_u8+=8;}
        if origin.pop().unwrap() {new_u8+=16;}
        if origin.pop().unwrap() {new_u8+=32;}
        if origin.pop().unwrap() {new_u8+=64;}
        if origin.pop().unwrap() {new_u8+=128;}
        destination.push(new_u8);
    }
    destination
}

pub fn vec_u8_to_vec_bool(mut origin: Vec<u8>) -> Vec<bool> {
    let mut destination = Vec::new();
    while let Some(mut elem) = origin.pop() {
        if elem>=128 {
            destination.push(true);
            elem-=128;
        } else {
            destination.push(false);
        }
        if elem>=64 {
            destination.push(true);
            elem-=64;
        } else {
            destination.push(false);
        }
        if elem>=32 {
            destination.push(true);
            elem-=32;
        } else {
            destination.push(false);
        }
        if elem>=16 {
            destination.push(true);
            elem-=16;
        } else {
            destination.push(false);
        }
        if elem>=8 {
            destination.push(true);
            elem-=8;
        } else {
            destination.push(false);
        }
        if elem>=4 {
            destination.push(true);
            elem-=4;
        } else {
            destination.push(false);
        }
        if elem>=2 {
            destination.push(true);
            elem-=2;
        } else {
            destination.push(false);
        }
        if elem>=1 {
            destination.push(true);
        } else {
            destination.push(false);
        }
    }
    while !destination.remove(0) {}
    destination
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn test_read_write() -> Result<(), ZipError> {
        write_bytes("testfile.txt", vec![1])?;
        assert_eq!(read_bytes("testfile.txt")?, vec![1]);
        remove_file("testfile.txt")?;
        Ok(())
    }

    #[test]
    fn test_count_bytes() -> Result<(), ZipError> {
        write_bytes("testfilecount.txt", vec![1, 2, 1])?;
        let mut result = HashMap::new();
        result.insert(1, 2);
        result.insert(2, 1);
        assert_eq!(count_bytes("testfilecount.txt")?, result);
        remove_file("testfilecount.txt")?;
        Ok(())
    }

    #[test]
    fn test_vec_u8_into() {
        assert_eq!(vec_u8_to_vec_bool(vec![6]), vec![true, false]);
    }

    #[test]
    fn test_vec_bool_into() {
        assert_eq!(vec_bool_to_vec_u8(vec![true, false]), vec![6]);
    }
}

