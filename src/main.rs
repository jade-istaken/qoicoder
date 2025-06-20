use image::ImageReader;
use std::{fs::write};
use itertools::concat;

const MAGIC_BYTES: [u8; 4] = [113,111,105,102];
const END_BYTES: [u8;8] = [0,0,0,0,0,0,0,1];

#[derive(Debug, Clone, Copy, PartialEq)]
struct RGBAPixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}
struct RGBPixel {
    r: u8,
    g: u8,
    b: u8
}

fn main() {
    let img = ImageReader::open("testcard.png").unwrap().decode().unwrap();
    let img_bytes = concat([construct_header(img.width(), img.height()),convert_bytes(img.into_bytes()),END_BYTES.to_vec()]);
    let _ = std::fs::write("test.qoi", img_bytes);
}

fn construct_header(width: u32, height: u32) -> Vec<u8> {
    concat([MAGIC_BYTES.to_vec(),width.to_be_bytes().to_vec(),height.to_be_bytes().to_vec(),vec![4,1]])
}

// commenting out because rust doesn't have a tail-call recursion annotation
// fn convert_bytes(img_bytes: Vec<u8>) -> Vec<u8> {
//     if img_bytes.len() == 0 {
//         return vec![]
//     } else {
//         let current_pixel = qoi_op_rgba(img_bytes[0], img_bytes[1], img_bytes[2],img_bytes[3]);
//         convert_bytes(img_bytes[4..].to_vec())
//     }
// }

fn convert_bytes(img_bytes: Vec<u8>) -> Vec<u8> {
    let mut pixel_array: [RGBAPixel;64] = [RGBAPixel{r:0,g:0,b:0,a:0};64];
    let length = img_bytes.len();
    let mut processed_bytes: Vec<u8> = vec![];
    let mut index = 0;
    while index < length {
        let current_pixel = RGBAPixel{r:img_bytes[index],g:img_bytes[index+1],b:img_bytes[index+2],a:img_bytes[index+3]};
        let pixel_hash = calculate_hash(current_pixel);
        if pixel_array[pixel_hash] == current_pixel {
            processed_bytes.push(qoi_op_index(pixel_hash));
        } else {
            processed_bytes.append(&mut qoi_op_rgba(current_pixel));
            pixel_array[pixel_hash] = current_pixel;
        }
        index += 4;
    }
    processed_bytes
}

fn qoi_op_rgba(pixel: RGBAPixel) -> Vec<u8> {
    vec![255,pixel.r, pixel.g, pixel.b, pixel.a]
}

fn calculate_hash(pixel: RGBAPixel) -> usize{
    (pixel.r as usize * 3 + pixel.g as usize * 5 + pixel.b as usize * 7 + pixel.a as usize * 11) % 64
}

fn qoi_op_index(pixel_hash: usize) -> u8 {
    (0b00000000 + pixel_hash).try_into().unwrap()
}