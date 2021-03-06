
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
                    ke_lex_f.send(format!("\n\t('str'){}",sintak)).expect("");
                    if duplikat.0 {
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
                    ke_lex_f.send(fungsi(&mut sintak,&_funsi[0].to_string(),&_funsi[1].to_string(),&_buf)).expect("");
                    continue;
                }
                "cetak" =>{
                    sintak.clear();
                    ke_lex_f.send("\n('c0')".to_string()).expect("");
                    if duplikat.0 {duplikat.2.push_str(&"\n('c0')".to_string())}
                    continue;
                }
                "<duplikat"=>{
                    print!("duplikat pembuka\n");
                    duplikat.0 = true;
                    duplikat.1 = _buf.split("=").collect::<Vec<&str>>()[1].trim().parse::<u32>().unwrap();
                    continue
                }
                "duplikat>"=>{
                    ulangi(&duplikat.2, &duplikat.1,&ke_lex_f);
                    duplikat.0 = false;
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
    format!("('{}')('{}')",&f,n.replace(fn_, " ").trim())
}
fn ulangi(token:&String,jumlah_ulangi:&u32,kirim:&std::sync::mpsc::Sender<std::string::String>){
    for _i in 1..*jumlah_ulangi{
        kirim.send(token.to_string()).expect("")
    }
}

