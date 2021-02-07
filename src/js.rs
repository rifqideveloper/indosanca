
pub fn konvesi(data:std::sync::mpsc::Receiver<std::string::String>,lanjut:std::sync::mpsc::Sender<bool>,kirim:std::sync::mpsc::Sender<std::string::String>){
    let mut buf = String::with_capacity(15);
    let mut _main_ = false;
    let mut arg = false;
    loop {
        buf = data.recv().expect("");
        if _main_ {
            if arg {
                if buf.contains("\t\t"){
                    buf.remove(0);
                    buf.remove(0);
                    buf.pop();
                    kirim.send(format!("{});",buf.to_string())).expect("");
                    arg = false;
                }
            }
            else if buf == "\tkonsole log\n" {
                arg = true;
                kirim.send("console.log(".to_string()).expect("");
            }
        }
        match buf.as_str() {
            "" => {break}
            "fn main\n" => {
                kirim.send("const main=function(){".to_string()).expect("");
                _main_ = true
            }
            "(!!!)" =>{
                if _main_ {
                    _main_ = false;
                    kirim.send("}();".to_string()).expect("");
                }
            }
            _ =>{}
        }
        lanjut.send(true).expect("");
    }
    kirim.send("".to_string()).expect("")
}