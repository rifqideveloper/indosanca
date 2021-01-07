
pub fn lexer(lanjut:std::sync::mpsc::Sender<std::string::String>,terima:std::sync::mpsc::Receiver<std::string::String>,ke_lex_f:std::sync::mpsc::Sender<std::string::String>){
    let mut _buf = String::with_capacity(15);
    let mut sintak = String::with_capacity(15);
    let mut _char :std::str::Split<&str>;
    loop {
        _buf = terima.recv().expect("");
        if _buf == "" {break}
        _char = _buf.split("");
        for i in _char {
            match i {
                "#" => { sintak.clear();break }
                "\t"|" " =>{continue}
                _ => {sintak.push_str(i)}
            }
            match sintak.as_str() {
                "cpu" => {
                    ke_lex_f.send(format!("('f0')<<('{}')\n",_buf.replace("cpu ", " ").trim())).expect("");
                    sintak.clear()
                }
                _ =>{}
            }
        }
        lanjut.send("".to_string()).expect("")
    }
    ke_lex_f.send("".to_string()).expect("")
}
