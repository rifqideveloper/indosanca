pub fn file(kirim:std::sync::mpsc::Sender<std::string::String>,path:String){   
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let mut file = BufReader::new(File::open(path).expect("membuka file"));
    let mut buf = String::with_capacity(100);
    while file.read_line(&mut buf).expect("membuka file ") != 0 {
        kirim.send(buf.clone()).expect("");
        buf.clear()
    }
    kirim.send("".to_string()).expect("");
}