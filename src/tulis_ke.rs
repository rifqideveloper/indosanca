use std::fs::File;
use std::io::Write;
pub fn js(data:std::sync::mpsc::Receiver<std::string::String>,proyek:&usize){
    let mut buf = String::with_capacity(15);
    let arg : Vec<String>= std::env::args().collect();
    let mut file = File::create(format!("{}\\target\\www\\index.js",arg[*proyek])).expect("");
    loop{
        buf = data.recv().expect("").clone();
        if buf == "" {break}else {
            file.write_all(buf.as_bytes()).expect("");
        }
    }
}
pub fn html(proyek:&usize){
    let arg : Vec<String>= std::env::args().collect();
    let mut file = File::create(format!("{}\\target\\www\\index.html",arg[*proyek])).expect("");
    file.write_all(b"<!DOCTYPE HTML><html><body>").expect("");
    /*
    let mut extra = String::with_capacity(15);
    loop {}
    */
    file.write(b"<script src=\"index.js\"></script></body></html>").expect("");
}
