use std::io::Read;
use std::process;
use std::fs;
use crate::parsing;
use crate::create_idat;
use inflate::DeflateDecoder;


pub fn decrypt(idat: usize, iend: usize) -> String {
    let file_path = "out.png";
    let file_data: Vec<u8> = fs::read(file_path).unwrap();
        
    let ihdr = parsing::parse_ihdr();
    let to_be_writen: String = "Hello World".to_string();
    let idats = create_idat::create_idat(to_be_writen, &ihdr);
    let test = file_data[idat+4..iend-1].to_vec();
    for i in 0..10 {
        if (test[i] != idats[0][i]) {
            print!("test: {}\nIdats: {}\nIndex: {}\n\n\n", test[i], idats[0][i], i);
            
        }
    }
    let mut decoder = DeflateDecoder::from_zlib(&*test);
    let mut output = Vec::new();
    let status = decoder.read_to_end(&mut output).unwrap_or_else(|err| { eprintln!("Problem parsing arguments: {err}"); process::exit(1); });
    let mut finalvec = vec!();
    for i in 0..output.len() {
        if (output[i] == 69) {
            if (output.len() - 3 > i) {
                if (output[i + 1] == 78 && output[i + 2] == 68) {
                    break;
                }
            }
        }
        if ((i + 1) % 4 != 0) {
            finalvec.push(output[i]);
        }
    }
    return String::from_utf8(finalvec).unwrap();
}
