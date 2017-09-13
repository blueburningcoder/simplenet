extern crate byteorder;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use byteorder::{ByteOrder, LittleEndian};


pub fn test() {
    println!("working :)");
}


#[derive(Debug)]
pub struct Numbers {
    magic_number : u32,
    size : u32,
    pictures : Vec<Vec<Vec<u8>>>,
}


#[derive(Debug)]
pub struct Labels {
    magic_number : u32,
    size : u32,
    labels : Vec<u8>,
}


/// reading Numbers from the following format:
///
/// [offset] [type]          [value]          [description]
/// 0000     32 bit integer  0x00000803(2051) magic number
/// 0004     32 bit integer  10000            number of images
/// 0008     32 bit integer  28               number of rows
/// 0012     32 bit integer  28               number of columns
/// 0016     unsigned byte   ??               pixel
/// 0017     unsigned byte   ??               pixel
/// ........
/// xxxx     unsigned byte   ??               pixel
pub fn read_numbers(filename : &str) -> Result<Numbers, io::Error> {

    let mut f = File::open(filename)?;
    let mut buf = [0; 4];
    f.read(&mut buf)?;
    let magic = LittleEndian::read_u32(&buf);
    f.read(&mut buf)?;
    let totsize = LittleEndian::read_u32(&buf);
    f.read(&mut buf)?;
    let rowsize = LittleEndian::read_u32(&buf);
    f.read(&mut buf)?;
    let colsize = LittleEndian::read_u32(&buf);

    let mut bytes = f.bytes();
    let mut pics = Vec::new();

    println!("m: {}, s: {}, r: {}, c: {}", magic, totsize, rowsize, colsize);

    for i in 0..totsize as usize {
        pics.push(Vec::new());
        for j in 0..rowsize as usize {
            pics[i].push(Vec::new());
            for _ in 0..colsize as usize {
                pics[i][j].push(bytes.next()
                    .expect("no more values").expect("no more items"));
            }
        }
    }

    Ok(Numbers { magic_number: magic, size: totsize, pictures: pics })
}


pub fn read_labels(filename : &str) -> Result<Labels, io::Error> {

    let f = File::open(filename)?;

    for byte in f.bytes() {
        println!("{}", byte.unwrap());
    }

    Ok(Labels { magic_number: 0, size: 0, labels: Vec::new() })
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
