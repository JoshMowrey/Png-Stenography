#![allow(unused_parens)]
#![allow(dead_code)]
use std::env;
pub mod parsing;
pub mod create_idat;
pub mod embedding_png;
pub mod decode;

fn main() {
    if (env::args().len() < 2) {
        encrypt();
        return;
    }
    else if (env::args().collect::<Vec<_>>()[1] == "e") {
        encrypt();
        return;
    }
    else {
        decrypt();
    }
}

fn print_vecu8(data: Vec<u8>){
    for iter in data {
        print!("{} ", iter);
    }
    print!("\n");
}

fn encrypt() {
    let ihdr = parsing::parse_ihdr();
    parsing::print_ihdr(&ihdr);
    let to_be_writen: String = parsing::get_input();
    let idats = create_idat::create_idat(to_be_writen, &ihdr);
    let idat_iend = parsing::last_idat("in.png".to_string());
    println!("Last Idat: {}\nIend: {}", idat_iend.0, idat_iend.1);
    println!("Idats: {}\n Idat Length: {}", idats.len(), idats[0].len());
    embedding_png::create_out_file(idats);
}

fn decrypt() {
   let idat_iend = parsing::last_idat("out.png".to_string());
   println!("{}", decode::decrypt(idat_iend.0, idat_iend.1));
}
