
pub fn parser(token:std::sync::mpsc::Receiver<std::string::String>,lanjut:std::sync::mpsc::Sender<std::string::String>,kirim:std::sync::mpsc::Sender<std::string::String>){
    let mut buf = String::with_capacity(15);
    loop {
        buf = token.recv().expect("");
        match buf.as_str(){
            "('c0')\n" =>{kirim.send("\n\tkonsole log".to_string()).expect("")}
            "" =>{break}
            _ =>{
                if buf.contains("('f0')('main')"){
                    let mut nama = buf.split("')").collect::<Vec<&str>>()[1].to_string();
                    nama.remove(0);
                    nama.remove(0);
                    kirim.send(format!("fn {}",nama)).expect("")
                }
                if buf.contains("\t('str'"){kirim.send(_str_(&buf)).expect("")}
            }
        }
        lanjut.send("".to_string()).expect("");
    }
    kirim.send("".to_string()).expect("");
}
fn _str_(buf:&String) -> String {
    let mut test = String::with_capacity(10);
    for i in buf.split("('str')"){
        test = i.to_string();
    }
    if test.contains("\n"){
        test.pop();
    }
    format!("\n\t\t\"{}\"",test)
}
