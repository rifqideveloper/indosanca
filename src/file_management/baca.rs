pub fn file(
    selesai:std::sync::mpsc::Receiver<()>,
    buf:&mut String,
    kirim:std::sync::mpsc::Sender<std::string::String>,
    path:String
){   
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    selesai.recv().unwrap();
    let mut file = BufReader::new(File::open(path).expect("membuka file"));
    while file.read_line(buf).expect("membuka file ") != 0 {
        kirim.send(buf.clone()).expect("");
        buf.clear()
    }
    kirim.send("".to_string()).expect("");
    
}