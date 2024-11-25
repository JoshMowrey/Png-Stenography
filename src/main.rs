#![allow(unused_parens)]
#![allow(dead_code)]
use std::fs;

struct IHDR {
    header: [u8; 4],
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}


fn main() {
    let file_path = "in.png";

    let mut data: Vec<u8> = fs::read(file_path).unwrap();
    let mut idat_start = 0;

    for (i, iter) in data.clone().into_iter().enumerate() {
        if (iter == 80) {
            if (data[i+1] == 76 && data[i+2] == 84 && data[i+3] == 69){
                idat_start = i; 
                break;
            }
        }

    }
    let mut num_of_idat = 0;
    for (i, iter) in data.clone().into_iter().enumerate() {
        if (iter == 73) {
            if (data[i+1] == 68 && data[i+2] == 65 && data[i+3] == 84){
                num_of_idat += 1;
            }
        }

    }
    println!("Print tRNS:  ");
    print_ihdr(ihdr_data());
    println!("Number of Idat Chunks:  {}", num_of_idat);
    data.extend(b"Hello World");
    let _ = fs::write("out.png", data.clone()).unwrap();
    println!("{}", idat_start);
    println!("{}{}{}{}", data[idat_start] as char, data[idat_start + 1] as char, data[idat_start + 2] as char, 
             data[idat_start + 3] as char);
}

fn ihdr_data() -> Vec<u8> {
    
    let file_path = "in.png";

    let file_data: Vec<u8> = fs::read(file_path).unwrap();
    let mut ihdr_data: Vec<u8> = vec![];
    for (i, iter) in file_data.clone().into_iter().enumerate() {
        if (iter == 73) {
            if (file_data[i+1] == 72 && file_data[i+2] == 
                68 && file_data[i+3] == 82) {

                for j in i+4..i+17 {
                    ihdr_data.push(file_data[j]);
                }
            }
        }
    }
    println!("IHDR len: {}", ihdr_data.len());
    return ihdr_data;
}

fn make_ihdr(){
        
}

fn print_vecu8(data: Vec<u8>){
    for iter in data {
        print!(" {},", iter);
    }
}

fn print_ihdr(data: Vec<u8>) -> IHDR { 
    println!("Image Width: ");
    for i in 0..4 {
        print!("{} ", data[i]);
    }
    println!("\nImage Depth: ");
    for i in 4..8 {
        print!("{} ", data[i]);
    }
    println!("\nBit Depth: {}\nColor Type: {}\nCompression Type: {}\nFilter Method: {}\nInterlace Method {}", 
             data[8], data[9], data[10], data[11], data[12]);
    let mut width_pre_transmute: [u8; 4] = [0; 4];
    width_pre_transmute.clone_from_slice(&data[..4]);
    let height_pre_transmute: [u8; 4] = [0; 4];
    width_pre_transmute.clone_from_slice(&data[4..4]);
    let width = unsafe {
        std::mem::transmute::<[u8; 4], u32>(width_pre_transmute)
    };
    let height = unsafe {
        std::mem::transmute::<[u8; 4], u32>(height_pre_transmute)
    };
    let ihdr = IHDR {
        header: [73, 72, 68, 82],
        width: width,
        height: height,
        bit_depth: data[8],
        color_type: data[9],
        compression_method: data[10],
        filter_method: data[11],
        interlace_method: data[12],
    };
    return ihdr;

}
