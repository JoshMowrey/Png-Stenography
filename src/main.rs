#![allow(non_snake_case)]
#![allow(unused_parens)]
//use std::env;
use std::fs;

fn main() {
    let FilePath = "in.png";

    let mut data: Vec<u8> = fs::read(FilePath).unwrap();
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
    println!("Number of Idat Chunks:  {}", num_of_idat);
    data.extend(b"Hello World");
    let _ = fs::write("out.png", data.clone()).unwrap();
    println!("{}", idat_start);
    println!("{}{}{}{}", data[idat_start] as char, data[idat_start + 1] as char, data[idat_start + 2] as char, 
             data[idat_start + 3] as char);
}

fn find_tRNS() -> String {
    let types: Vec[u8] = [];
    for (i, iter) in data.clone().into_iter().enumerate() {
        if (iter == 73) {
            if (data[i+1] == 68 && data[i+2] == 65 && data[i+3] == 84){
                types.push(data[i+13]);
            }
        }
    }
    let current_tRNS: Vec[u8] = [];
    for (i, iter) in data.clone().into_iter().enumerate() {
        if (iter == 116) {
            if (data[i+1] == 82 && data[i+2] == 78 && data[i+3] == 83){
                for j in range(i+4, i+15) {
                    current_tRNS.push(data[j]);
                }
            }
        }
    }
    for (i, iter) 
}

fn make_IHDR(String: s){
    let mut Buffer: Vec<u8> = [73, 72, 68, 82];
    
}

