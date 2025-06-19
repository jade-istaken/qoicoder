use image::ImageReader;
use std::{convert, fs::write};
use itertools::concat;

const MAGIC_BYTES: [u8; 4] = [113,111,105,102];
const END_BYTES: [u8;8] = [0,0,0,0,0,0,0,1];

fn main() {
    let img = ImageReader::open("test.png").unwrap().decode().unwrap();

    let img_bytes = concat([construct_header(img.width(), img.height()),convert_bytes(img.into_bytes()),END_BYTES.to_vec()]);
    println!("{:?}", img_bytes);
    let _ = std::fs::write("test.qoi", img_bytes);
}

fn construct_header(width: u32, height: u32) -> Vec<u8> {
    concat([MAGIC_BYTES.to_vec(),width.to_be_bytes().to_vec(),height.to_be_bytes().to_vec(),vec![4,1]])
}

fn convert_bytes(img_bytes: Vec<u8>) -> Vec<u8> {
    if img_bytes.len() == 0 {
        return vec![]
    } else {
        concat(vec![qoi_op_rgba(img_bytes[0], img_bytes[1], img_bytes[2], img_bytes[3]),convert_bytes(img_bytes[4..].to_vec())])
    }
}

fn qoi_op_rgba(r:u8, g:u8, b:u8, a:u8) -> Vec<u8> {
    vec![255,r,g,b,a]
}