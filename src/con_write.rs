use std::fs::File;
use std::io::Write;
pub fn tuliskan(terima:std::sync::mpsc::Receiver<std::string::String>,_proyek:String,lanjut:std::sync::mpsc::Sender<bool>){
    let mut file = File::create(format!("{}\\parsing\\lexer",_proyek)).expect("");
    let mut _buf = terima.recv().expect("");
    print!("[con write siap]\n");
    while _buf != "" {
        file.write(_buf.as_bytes()).expect("");
        lanjut.send(true).expect("");
        _buf = terima.recv().expect("")
    }
    print!("[con write selesai]\n");
    lanjut.send(false).expect("")
}