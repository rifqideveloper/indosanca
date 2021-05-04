pub fn main(proyek:&String,kirim:std::sync::mpsc::Sender<std::string::String>){
    use std::fs;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let mut baris = String::with_capacity(15);
    
    let mut file = BufReader::with_capacity(10, File::open(format!("{}\\parsing\\main",proyek)).expect(""));
    while file.read_line(&mut baris).expect("") != 0 {
        kirim.send(baris.clone()).expect("");
        baris.clear();
        
    }
    kirim.send("".to_string()).expect("");
}