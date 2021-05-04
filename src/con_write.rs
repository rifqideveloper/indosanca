use std::io::Write;
pub fn main(terima:std::sync::mpsc::Receiver<std::string::String>,_proyek:String,lanjut:std::sync::mpsc::Sender<bool>){
    let mut file = std::fs::File::create(format!("{}\\parsing\\lexer",_proyek)).expect("");
    let mut _buf = terima.recv().expect("");
    while _buf != "" {
        file.write(_buf.as_bytes()).expect("");
        lanjut.send(true).expect("");
        _buf = terima.recv().expect("")
    }
    lanjut.send(false).expect("")
}