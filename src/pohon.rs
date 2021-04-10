use std::io::Write;
pub fn tulis (_proyek:String,terima:std::sync::mpsc::Receiver<std::string::String>){
    let mut file = std::fs::File::create(format!("{}\\parsing\\parser",_proyek)).expect("");
    let mut _buf = String::with_capacity(15);
    loop {
        _buf = terima.recv().expect("");
        if _buf == "" {break}
        file.write(_buf.as_bytes()).expect("");
    }
    file.write("\n(!!!)".as_bytes()).expect("");
}