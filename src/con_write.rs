use std::env;
use std::fs::File;
use std::io::Write;
pub fn tuliskan(terima:std::sync::mpsc::Receiver<std::string::String>,_proyek:&usize){
    let args: Vec<String> = env::args().collect();
    let mut file = File::create(format!("{}\\lexer",args[*_proyek])).expect("");
    let mut _buf = String::with_capacity(15);
    loop {
        _buf = terima.recv().expect("");
        if _buf == "" {break}
        file.write(_buf.as_bytes()).expect("");
    }
}