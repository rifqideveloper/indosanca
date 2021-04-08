use std::io::{BufRead, BufReader};
use std::fs::File;
pub fn baca(_proyek:&usize,kirim:std::sync::mpsc::Sender<std::string::String>,tunggu:std::sync::mpsc::Receiver<bool>){
    let mut file = BufReader::new(File::open(format!("{}\\parsing\\parser",std::env::args().collect::<Vec<String>>()[*_proyek])).expect(""));
    let mut baris = String::with_capacity(15);
    while file.read_line(&mut baris).expect("") != 0 {
        kirim.send(baris.clone()).expect("");
        baris.clear();
        tunggu.recv().expect("");
    }
    kirim.send("".to_string()).expect("")
}