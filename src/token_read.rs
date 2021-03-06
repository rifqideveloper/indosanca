use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn baca(proyek:&usize,kirim_ke_parser:std::sync::mpsc::Sender<std::string::String>,tunggu:std::sync::mpsc::Receiver<std::string::String>,tunggu_lexer:std::sync::mpsc::Receiver<bool>){
    let args: Vec<String> = std::env::args().collect();
    let file = format!("{}\\lexer",args[*proyek]);
    let mut buf = String::with_capacity(15);
    let mut jeda = tunggu_lexer.recv().expect(""); 
    let mut file = BufReader::with_capacity(15,File::open(file).expect(""));
    while file.read_line(&mut buf).expect("") != 0 {
        if jeda {
            jeda = tunggu_lexer.recv().expect("");
        }
        kirim_ke_parser.send(buf.clone()).expect("");
        buf.clear();
        tunggu.recv().expect("");
    }
    kirim_ke_parser.send("".to_string()).expect("");
}