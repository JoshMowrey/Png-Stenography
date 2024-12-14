use std::fs;

pub struct IHDR {
    pub header: [u8; 4],
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
}

pub struct IDAT {
    pub header: [u8; 4],
    pub width: Vec<u8>,
}

pub fn parse_ihdr() -> IHDR {
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
    //let mut width_pre: [u8; 4] = [0; 4];
    //width_pre.clone_from_slice(&ihdr_data[..4]);
    //let height_pre: [u8; 4] = [ihdr_data[4],ihdr_data[5],ihdr_data[6],ihdr_data[7]];
    //println!("{} {} {} {}", ihdr_data[0], ihdr_data[1], ihdr_data[2], ihdr_data[3]);
    //let bin_width = to_bin(ihdr_data[..4].to_vec());
    //println!("\nbin_width: {}", bin_width);
    let ihdr = IHDR {
        header: [73, 72, 68, 82],
        width: u32::from_ne_bytes([ihdr_data[3], ihdr_data[2], ihdr_data[1], ihdr_data[0]]),
        height: u32::from_ne_bytes([ihdr_data[7], ihdr_data[6], ihdr_data[5], ihdr_data[4]]),
        bit_depth: ihdr_data[8],
        color_type: ihdr_data[9],
        compression_method: ihdr_data[10],
        filter_method: ihdr_data[11],
        interlace_method: ihdr_data[12],
    };
    return ihdr;
}

pub fn print_ihdr(ihdr: &IHDR) {
    println!("Image Width: {}", ihdr.width);
    println!("Image Height: {}", ihdr.height);
    println!("Bit Depth: {}\nColor Type: {}\nCompression Type: {}\nFilter Method: {}\nInterlace Method {}", ihdr.bit_depth, ihdr.color_type, ihdr.compression_method, ihdr.filter_method, ihdr.interlace_method);
}

fn to_bin(v: Vec<u8>) -> String {
    let mut s = String::from("");
    for i in v {
        let x = i;
        let temp = (&format!("{x:032b}"));
        s.push_str(temp);
    }
    return s;
}

pub fn last_idat(file_path: String) -> (usize, usize) {
    let file_data: Vec<u8> = fs::read(file_path).unwrap();
    
    let mut last_idat: usize = 0;
    for (i, iter) in file_data.clone().into_iter().enumerate() {
        if (iter == 73) {
            if (file_data[i+1] == 68 && file_data[i+2] == 
                65 && file_data[i+3] == 84) {
                last_idat = i;
                //change to find last idat and end of last idat and return as a tupple
            }
        }
        if (iter == 73) {
            if (file_data[i+1] == 69 && file_data[i+2] == 78 && file_data[i+3] == 68) {
                return (last_idat, i);
            }
        }
    }
    return (0, 0);
    

}

pub fn get_input() -> String {
    let file_path = "in.txt";

    let file_data: String = String::from_utf8(fs::read(file_path).unwrap()).unwrap();
    
    return file_data;
}
