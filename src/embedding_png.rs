use std::fs;

pub fn create_out_file(idats: Vec<Vec<u8>>) -> bool {
    
    let file_path = "in.png";
    let mut file_data: Vec<u8> = fs::read(file_path).unwrap();
    
    let file_out_path = "out.png";
    let mut temp = vec!(); 
    for i in 0..file_data.len()-4 {
        if (file_data[i] == 73) {
            if (file_data[i+1] == 69 && file_data[i+2] == 78 && file_data[i+3] == 68){
                temp.append(&mut file_data[..i-1].to_vec());
                for idat in &idats {
                    temp.append(&mut vec!(73, 68, 65, 84));
                    temp.append(&mut idat.clone());
                }
                temp.append(&mut file_data[i..].to_vec());
                break;
            }
        }
   }
    match fs::write(file_out_path, temp) {
        Ok(_file) => return true,
        Err(error) => panic!("Problem writing to file: {error:?}"),
    }
}
