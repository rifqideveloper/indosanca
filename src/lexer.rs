
pub fn lexer(lanjut:std::sync::mpsc::Sender<std::string::String>,terima:std::sync::mpsc::Receiver<std::string::String>,ke_lex_f:std::sync::mpsc::Sender<std::string::String>){
    let mut _buf = String::with_capacity(15);
    let mut sintak = String::with_capacity(15);
    let mut _char :std::str::Split<&str>;
    let _funsi = ["f0","cpu","f1","gpu"];
    let _kode = ["('c0')\n"];
    let mut _str_ = false;
    let mut _koment = false;
    print!("[lexer siap]\n");
    loop {
        _buf = terima.recv().expect("");
        if _buf == "" {break}
        _char = _buf.split("");
        for i in _char {
            if _str_ {if i == "\""{_str_ = false;ke_lex_f.send(format!("\t('str','{}')",sintak)).expect("")}else{sintak.push_str(i)}continue}
            else if _koment {if i == "\n"||i == "#"{_koment = false}continue}
            match i {
                "#" => { _koment = true ;continue}
                "\"" => {_str_ = true ; sintak.clear() ;continue}
                "\t"|" "|"\n" =>{continue}
                //"*" => {}
                //"&" => {}
                _ => {sintak.push_str(i)}
            }
            match sintak.as_str() {
                "cpu"   => {ke_lex_f.send(fungsi(&mut sintak,&_funsi[0].to_string(),&_funsi[1].to_string(),&_buf)).expect("");break}
                "gpu"   => {ke_lex_f.send(fungsi(&mut sintak,&_funsi[2].to_string(),&_funsi[3].to_string(),&_buf)).expect("");break}
                "cetak" => {sintak.clear();ke_lex_f.send(_kode[0].to_string()).expect("")}
                //"kons"  => {}
                //"din"   => {}
                //"heap"  => {}
                //"keluar" => {}
                //"kembali" => {}
                //"file" =>{}
                //"ini" => {}
                //"loop" => {}
                //"jika" => {}
                _ =>{}
            }
        }
        lanjut.send("".to_string()).expect("")
    }
    ke_lex_f.send("".to_string()).expect("");
    print!("[lexer selesai]\n")
}
fn fungsi(sintak:&mut String ,f:&String,fn_:&String,n:&String) -> String{
    sintak.clear();
    format!("('{}','{}')\n",&f,n.replace(fn_, " ").trim())
}

