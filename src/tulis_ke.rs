use std::fs::File;
use std::io::Write;
pub fn js(data:std::sync::mpsc::Receiver<std::string::String>,proyek:&usize){
    let mut buf = String::with_capacity(15);
    let mut file = File::create(format!("{}\\target\\www\\index.js",std::env::args().collect::<Vec<String>>()[*proyek])).expect("");
    loop{
        buf.push_str(&data.recv().expect(""));
        match buf.as_str() {
            "" =>{break}
            _=>{file.write_all(buf.as_bytes()).expect("");buf.clear()}
        }
    }
}
pub fn html(proyek:&usize){
    let mut file = File::create(format!("{}\\target\\www\\index.html",std::env::args().collect::<Vec<String>>()[*proyek])).expect("");
    file.write_all(b"<!DOCTYPE HTML><html><body>").expect("");
    /*
    let mut extra = String::with_capacity(15);
    loop {}
    */
    file.write(b"<script src=\"index.js\"></script></body></html>").expect("");
}
