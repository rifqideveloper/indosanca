use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn baca(main_forder: String,kirim:std::sync::mpsc::Sender<std::string::String>,terima:std::sync::mpsc::Receiver<std::string::String>){
    let mut baris = String::with_capacity(15);
    for i in direktori_list(format!("{}\\kode",main_forder)){
        let mut file = BufReader::with_capacity(10, File::open(i).expect(""));
        while file.read_line(&mut baris).expect("") != 0 {
            kirim.send(baris.clone()).expect("");
            //print!("{}",baris);
            baris.clear();
            terima.recv().expect("");
        }
        baris.push_str("\n");
    }
    kirim.send("".to_string()).expect("");
}
fn direktori_list(path:String) -> Vec<String>{    
    let mut file : Vec<String> = Vec::with_capacity(2);
    for path in fs::read_dir(path).unwrap() 
    {
        file.push(path.unwrap().path().display().to_string())
    }
    file    
}