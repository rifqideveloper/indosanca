use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn baca(proyek:String,kirim_ke_parser:std::sync::mpsc::Sender<std::string::String>,tunggu:std::sync::mpsc::Receiver<std::string::String>,tunggu_lexer:std::sync::mpsc::Receiver<bool>){
    let mut file = BufReader::with_capacity(15,File::open(format!("{}\\parsing\\lexer",proyek)).expect(""));
    let mut buf = String::with_capacity(15);
    let mut jeda = tunggu_lexer.recv().expect(""); 
    while file.read_line(&mut buf).expect("") != 0 {
        if jeda { jeda = tunggu_lexer.recv().expect("") }
        kirim_ke_parser.send(buf.clone()).expect("");
        buf.clear();
        tunggu.recv().expect("");
    }
    kirim_ke_parser.send("".to_string()).expect("");
}