extern crate byteorder;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use self::byteorder::{ByteOrder, BigEndian};


#[derive(Debug)]
struct Numbers {
    magic_number : u32,
    tsize : u32,
    rsize : u32,
    csize : u32,
    pictures : Vec<Vec<Vec<u8>>>,
}


#[derive(Debug)]
struct Labels {
    magic_number : u32,
    size : u32,
    labels : Vec<u8>,
}


#[derive(Debug, Clone)]
pub struct Picture {
    pixels : Vec<Vec<u8>>,
    value : u8,
}


/// when provided with two filenames of the MNIST Dataset 
/// Pictures and Labels respectively, returning a Vector of combined Pictures.
pub fn get_pictures(nname: &str, lname: &str)
    -> Result<Vec<Picture>, io::Error> {
    let nums = read_numbers(nname)?;
    let lab = read_labels(lname)?;
    Ok(nums.pictures.iter().zip(lab.labels)
        .map(|(p, l)| Picture { pixels: p.to_vec(), value: l })
        .collect()
        )
}


///! reading number-pictures from the following format:
///!
///! ```text
///! [offset] [type]          [value]          [description]
///! 0000     32 bit integer  0x00000803(2051) magic number
///! 0004     32 bit integer  10000            number of images
///! 0008     32 bit integer  28               number of rows
///! 0012     32 bit integer  28               number of columns
///! 0016     unsigned byte   ??               pixel
///! 0017     unsigned byte   ??               pixel
///! ........
///! xxxx     unsigned byte   ??               pixel
///! ```
///!
///! Pixels are organized row-wise. Pixel values are 0 to 255.
///! 0 means background (white), 255 means foreground (black).
fn read_numbers(filename : &str) -> Result<Numbers, io::Error> {

    let mut f = File::open(filename)?;
    let mut buf = [0; 4];
    f.read(&mut buf)?;
    let magic = BigEndian::read_u32(&buf);
    f.read(&mut buf)?;
    let totsize = BigEndian::read_u32(&buf);
    f.read(&mut buf)?;
    let rowsize = BigEndian::read_u32(&buf);
    f.read(&mut buf)?;
    let colsize = BigEndian::read_u32(&buf);

    let mut bytes = f.bytes();
    let mut pics = Vec::new();

    //println!("m: {}, s: {}, r: {}, c: {}", magic, totsize, rowsize, colsize);
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
    Ok(Numbers { magic_number: magic, tsize: totsize, rsize: rowsize,
                 csize: colsize, pictures: pics })
}


/// reading the associated labels (should-be-numbers)
/// from a file with the following format:
///
/// ```text
/// [offset] [type]          [value]          [description]
/// 0000     32 bit integer  0x00000801(2049) magic number (MSB first)
/// 0004     32 bit integer  10000            number of items
/// 0008     unsigned byte   ??               label
/// 0009     unsigned byte   ??               label
/// ........
/// xxxx     unsigned byte   ??               label
/// ```
///
/// The labels values are 0 to 9.
fn read_labels(filename : &str) -> Result<Labels, io::Error> {

    let mut f = File::open(filename)?;
    let mut buf = [0; 4];
    f.read(&mut buf)?;
    let magic = BigEndian::read_u32(&buf);
    f.read(&mut buf)?;
    let totsize = BigEndian::read_u32(&buf);

    let mut bytes = f.bytes();
    let mut labs = Vec::new();

    for _ in 0..totsize as usize {
        labs.push(bytes.next()
            .expect("no more values").expect("no more items"));
    }
    Ok(Labels { magic_number: magic, size: totsize, labels: labs })
}


