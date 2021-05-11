pub fn tulis(_proyek:&String,f:&str,terima:std::sync::mpsc::Receiver<std::string::String>){
    use std::io::Write;
    let mut file = std::fs::File::create(format!("{}\\parsing\\{}",_proyek,f)).expect("");
    let mut _buf = String::with_capacity(40);
    loop {
        _buf = terima.recv().expect("");
        if _buf == "" {
            file.write(_buf.as_bytes()).expect("");
            break
        }
        file.write(_buf.as_bytes()).expect("");
    }
}