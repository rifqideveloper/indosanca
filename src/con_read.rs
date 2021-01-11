use std::fs;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn baca(proyek:&usize,kirim:std::sync::mpsc::Sender<std::string::String>,terima:std::sync::mpsc::Receiver<std::string::String>){
    let main_forder : Vec<String> = env::args().collect();
    let main_forder = format!("{}\\kode",main_forder[*proyek]);
    let mut baris = String::with_capacity(15);
    print!("[con read siap]\n");
    for i in direktori_list(format!("{}",main_forder)){
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
    print!("[con read selesai]\n");
}
fn direktori_list(path:String) -> Vec<String>{    
    let mut file : Vec<String> = Vec::with_capacity(2);
    for path in fs::read_dir(path).unwrap() 
    {
        file.push(path.unwrap().path().display().to_string())
    }
    file    
}