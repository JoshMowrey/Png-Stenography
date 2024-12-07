use crate::parsing::IHDR;
use std::io::Write;
use deflate::Compression;
use deflate::write::ZlibEncoder;


pub fn create_idat(s: String, ihdr: &IHDR) -> (Vec<Vec<u8>> /*Vector of Idat Buffers*/) {
    if (ihdr.bit_depth != 8) {
        return (vec!());
    }
    let mut intermediate_vec: Vec<u8> = s.into_bytes();
    let len = (ihdr.width * ihdr.height).try_into().unwrap();
    let mut return_vector: Vec<Vec<u8>> = vec!();
    let mut int_vec_len = intermediate_vec.len();
    {
        let mut i = 3;
        while (i < intermediate_vec.len()){
            intermediate_vec.insert(i, 0);
            i += 4;
        }
        
        intermediate_vec.push(69); 
        intermediate_vec.push(78); 
        intermediate_vec.push(68); 
        i += 3;
        while (i%len != 0) {
            if (i%3 == 0) {
                intermediate_vec.push(0);
                i+=1;
            }
            else {
                intermediate_vec.push(rand::random::<u8>()); 
                i+=1;
            }
        }
    }
    loop {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Default);
        if (intermediate_vec.len() > len){
            if (encoder.write(&(intermediate_vec[..len])).unwrap() != len && int_vec_len > len) {
                return vec!();
            }
            
        }
        else {
            if (encoder.write(&(intermediate_vec)).unwrap() != len && int_vec_len > len) {
                return vec!();
            }
        }
        let mut checksum = encoder.checksum().to_ne_bytes().to_vec();
        let mut intermediate_idat_buffer: Vec<u8> = vec!(8, 0);
        intermediate_idat_buffer.append(&mut encoder.finish().unwrap());
        intermediate_idat_buffer.append(&mut checksum);
        return_vector.push(intermediate_idat_buffer);
        if (int_vec_len <= len) {
            break;
        }
        int_vec_len -= len;
    }
    
    return (return_vector);
}
