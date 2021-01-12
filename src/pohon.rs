use std::io::Write;
pub fn tulis (_proyek:&usize,terima:std::sync::mpsc::Receiver<std::string::String>){
    let args: Vec<String> = std::env::args().collect();
    let mut file = std::fs::File::create(format!("{}\\parser",args[*_proyek])).expect("");
    let mut _buf = String::with_capacity(15);
    loop {
        _buf = terima.recv().expect("");
        if _buf == "" {break}
        file.write(_buf.as_bytes()).expect("");
    }
}