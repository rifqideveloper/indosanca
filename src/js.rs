
pub fn konvesi(data:std::sync::mpsc::Receiver<std::string::String>,lanjut:std::sync::mpsc::Sender<bool>,kirim:std::sync::mpsc::Sender<std::string::String>){
    let mut buf = String::with_capacity(15);
    let mut _main_ = false;
    let mut fn_main = String::with_capacity(15);
    let mut arg = false;
    loop {
        buf = data.recv().expect("");
        if _main_ {
            if arg {
                if buf.contains("\t\t"){
                    buf.remove(0);
                    buf.remove(0);
                    fn_main.push_str(format!("{});",buf).as_str())
                }
            }
            else if buf == "\tjs konsole log\n" {
                arg = true;
                fn_main.push_str("console.log(")
            }
        }
        match buf.as_str() {
            "" => {break}
            "fn main\n" => {_main_ = true}
            _ =>{}
        }
        lanjut.send(true).expect("");
    }
    kirim.send(fn_main).expect("");
    kirim.send("".to_string()).expect("")
}