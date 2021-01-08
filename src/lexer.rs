
pub fn lexer(lanjut:std::sync::mpsc::Sender<std::string::String>,terima:std::sync::mpsc::Receiver<std::string::String>,ke_lex_f:std::sync::mpsc::Sender<std::string::String>){
    let mut _buf = String::with_capacity(15);
    let mut sintak = String::with_capacity(15);
    let mut _char :std::str::Split<&str>;
    let mut test2 = false;
    loop {
        _buf = terima.recv().expect("");
        if _buf == "" {break}
        _char = _buf.split("");
        for i in _char {
            if test2 {
                if i == "\""{
                    test2 = false;
                    ke_lex_f.send(format!("<<('str')<<('{}')",sintak)).expect("")
                }else {
                    sintak.push_str(i)
                }
                continue
            }
            match i {
                "#" => { sintak.clear();break }
                "\"" => {test2 = true ; sintak.clear() ;continue}
                "\t"|" " =>{continue}
                _ => {sintak.push_str(i)}
            }
            match sintak.as_str() {
                "cpu"   => {ke_lex_f.send(fungsi(&mut sintak,"f0".to_string(),"cpu".to_string(),&_buf)).expect("");break}
                "gpu"   => {ke_lex_f.send(fungsi(&mut sintak,"f1".to_string(),"gpu".to_string(),&_buf)).expect("");break}
                "cetak" => {
                    sintak.clear();
                    ke_lex_f.send("('c0')".to_string()).expect("")
                }
                //"kons"  => {}
                //"din"   => {}
                //"heap"  => {}
                _ =>{}
            }
        }
        lanjut.send("".to_string()).expect("")
    }
    ke_lex_f.send("".to_string()).expect("")
}
fn fungsi(sintak:&mut String ,f:String,fn_:String,n:&String) -> String{
    sintak.clear();
    format!("('{}')<<('{}')",f,n.replace(&fn_, " ").trim())
}

