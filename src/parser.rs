
pub fn parser(token:std::sync::mpsc::Receiver<std::string::String>,lanjut:std::sync::mpsc::Sender<std::string::String>,kirim:std::sync::mpsc::Sender<std::string::String>){
    #[warn(unused_assignments)]
    let mut buf = String::with_capacity(15);
    loop {
        buf = token.recv().expect("");
        match buf.as_str(){
            "('f0','main')\n" =>{kirim.send("fn main\n".to_string()).expect("")}
            "('c0')\n" =>{kirim.send("\tjs konsole log\n".to_string()).expect("")}
            "" =>{break}
            _ =>{
                if buf.contains("\t('str'"){kirim.send(format!("\t{}",buf)).expect("")}
            }
        }
        lanjut.send("".to_string()).expect("");
    }
    kirim.send("".to_string()).expect("");
}
