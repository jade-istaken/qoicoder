use image::ImageReader;
use std::fs::write;
use itertools::concat;

const END_BYTES: [u8;8]= [0,0,0,0,0,0,0,1];

fn main() {
    let img = ImageReader::open("test.png").unwrap().decode().unwrap();

    let img_bytes = concat([construct_header(img.width(), img.height()),END_BYTES.to_vec()]);
    println!("{:?}", img_bytes);
    let _ = std::fs::write("test.qoi", img_bytes);
}

fn construct_header(width: u32, height: u32) -> Vec<u8> {
    let magic_bytes: Vec<u8> = vec![113,111,105,102];
    concat([magic_bytes,width.to_be_bytes().to_vec(),height.to_be_bytes().to_vec(),vec![4,1]])
}