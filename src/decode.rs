use crate::parsing::IHDR;
use std::io::Write;
use deflate::Compression;
use deflate::write::ZlibEncoder;


fn decrypt(idat: usize, iend: usize) -> String {
    let file_path = "out.png";
    let file_data: Vec<u8> = fs::read(file_path).unwrap();
    
}
