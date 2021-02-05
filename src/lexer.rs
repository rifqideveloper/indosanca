
pub fn lexer(lanjut:std::sync::mpsc::Sender<std::string::String>,terima:std::sync::mpsc::Receiver<std::string::String>,ke_lex_f:std::sync::mpsc::Sender<std::string::String>){
    let mut _buf = String::with_capacity(15);
    let mut sintak = String::with_capacity(15);
    let _funsi = ["f0","cpu","f1","gpu"];
    let _kode = ["('c0')\n"];
    let mut _str_ = false;
    let mut _koment = false;
    let mut duplikat = (false,0u32,String::with_capacity(50));
    
    print!("[lexer siap]\n");
    loop {
        _buf = terima.recv().expect("");
        if _buf == "" {break}
        for i in _buf.split("") {
            if _str_ {
                if i == "\""{
                    if !duplikat.0 {
                        ke_lex_f.send(format!("\n\t('str'){}",sintak)).expect("");
                    }else {
                        duplikat.2.push_str(&format!("\n\t('str'){}",sintak))
                    }
                    sintak.clear();
                    _str_ = false;
                } else{
                    sintak.push_str(i);
                }
                continue
            }
            match i {
                " " | "\t"=>{}
                "\""=>{
                    _str_ = true;
                    continue
                }
                _ =>{sintak.push_str(i)}
            }
            match sintak.as_str(){
                "cpu"=>{
                    if !duplikat.0 {
                        ke_lex_f.send(fungsi(&mut sintak,&_funsi[0].to_string(),&_funsi[1].to_string(),&_buf)).expect("");
                    }else {
                        duplikat.2.push_str(&fungsi(&mut sintak,&_funsi[0].to_string(),&_funsi[1].to_string(),&_buf))
                    }
                }
                "cetak" =>{
                    sintak.clear();
                    if !duplikat.0 {
                        ke_lex_f.send("\n('c0')".to_string()).expect("");
                    }else {
                        duplikat.2.push_str(&"\n('c0')".to_string())
                    }
                    continue;
                }
                "<duplikat"=>{
                    print!("duplikat pembuka\n");
                    duplikat.0 = true;
                    duplikat.1 = _buf.split("=").collect::<Vec<&str>>()[1].trim().parse::<u32>().unwrap();
                    continue
                }
                "duplikat>"=>{
                    print!("duplikat tertutup\n");
                    duplikat.0 = false;
                    ke_lex_f.send(ulangi(&duplikat.2, &duplikat.1)).expect("");
                    duplikat.2.clear();
                    sintak.clear();
                    continue
                }
                _ =>{}
            }

        }
        //print!("{}",sintak);
        sintak.clear();
        lanjut.send("".to_string()).expect("")
    }
    ke_lex_f.send("".to_string()).expect("");

    print!("[lexer selesai]\n")
}
fn fungsi(sintak:&mut String ,f:&String,fn_:&String,n:&String) -> String{
    sintak.clear();
    format!("('{}','{}')",&f,n.replace(fn_, " ").trim())
}
fn ulangi(token:&String,jumlah_ulangi:&u32) -> String{
    let mut ret = String::with_capacity(15);
    for _i in 0..*jumlah_ulangi{
        ret.push_str(token)
    }
    ret
}

