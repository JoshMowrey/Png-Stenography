use std::fs;

pub fn create_out_file(idats: Vec<Vec<u8>>) -> bool {
    
    let file_path = "in.png";
    let mut file_data: Vec<u8> = fs::read(file_path).unwrap();
    
    let file_out_path = "out.png";
    

    for idat in &idats {
        file_data.append(&mut vec!(73, 68, 65, 84));
        file_data.append(&mut idat.clone());
    }
    file_data.append(&mut vec!(73, 69, 78, 68));
    match fs::write(file_out_path, file_data) {
        Ok(_file) => return true,
        Err(error) => panic!("Problem writing to file: {error:?}"),
    }
}
