use std::fs::File;
//use std::io::{BufRead, BufReader};
use std::io::prelude::*;
//use std::sync::mpsc::channel;
pub fn bangun_lexer(_lex:File , main_:std::sync::mpsc::Receiver<std::string::String> ,selesai:std::sync::mpsc::Sender<std::string::String>) {
    //let mut _baris:String = String::with_capacity(10);
    let mut _baris = String::with_capacity(10);
    let mut buf = String::with_capacity(10);
    let mut _lex = _lex;
    /*let mut log = [false,false];*/
    main_.recv().expect("");
    selesai.send("".to_string()).expect("");
    loop {
        _baris = main_.recv().expect("");
        if _baris == " " { break }
        token(&mut buf,/*&mut log,*/&mut _lex,&mut _baris);
        selesai.send("".to_string()).expect("erro di lex_x");
    }
}
fn token(buf:&mut String,/*log:&mut[bool; 2],*/_lex:& mut File,_baris:&mut String){
    for i in _baris.split(""){
        match i {
            "#" =>{ return }
            "\t" => { continue }
            " " => { continue }
            _ =>{ buf.push_str(i) }
        }
        match buf.as_str() {
            "cpu" => {
                _lex.write(format!("('f0')<=>('{}')\n",_baris.replace("cpu", " ").trim()).as_bytes()).expect("");
                buf.clear();
                return
            }
            "gpu" => {
                _lex.write(format!("('f1')<=>('{}')\n",_baris.replace("cpu", " ").trim()).as_bytes()).expect("");
                buf.clear();
                return
            }
            "cetak" => {
                _str(_baris.replace("cetak", ""),_lex);
                buf.clear();
                return
            }
            "cons" => {}
            "din" => {}
            "heap" => {}
            "ini" => {}
            _ => {}
        }
    }
}
fn _str(kalimat:String,lex:& mut File){
    if kalimat.contains("\"") {
        lex.write(format!("('c0')<=>('str')<=>({})\n",kalimat).as_bytes()).expect("");
    }
}
