
pub fn parser(token:std::sync::mpsc::Receiver<std::string::String>,lanjut:std::sync::mpsc::Sender<std::string::String>){
    let mut buf = String::with_capacity(15);
    loop {
        buf = token.recv().expect("");
        if buf == "" {break}
        print!("{}",buf);
        lanjut.send("".to_string()).expect("");
    }
    
}
