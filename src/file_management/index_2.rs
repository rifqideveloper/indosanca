use std::fs::File;
use std::io::{BufRead, BufReader};
use serde_json::{Value};
use serde_json::json;
pub fn baca(
    buf:&mut String,
    terima:std::sync::mpsc::Receiver<std::string::String>,
    path:&String,
    terima_parse_3:std::sync::mpsc::Receiver<std::vec::Vec<&str>>,
    kirim_parse_3:std::sync::mpsc::Sender<std::option::Option<serde_json::Value>>,
    turbo:bool,
){
    use std::io::Write;
    if !turbo{
        let mut file = std::fs::File::create(format!("{}\\parsing\\parse_2",path)).expect("");
        buf.push_str(&terima.recv().expect("")) ;
        while buf != ""{
            file.write(buf.as_bytes()).expect("");
            buf.clear();
            buf.push_str(&terima.recv().expect(""));
        }
        //drop(file);
        //
        file = File::open(format!("{}\\parsing\\parse_2",path)).expect("");
        //File::with_options().create(true).read(true).write(true).open(format!("{}\\parsing\\parse_2",path)).unwrap()
        'main:loop{
            let mut reader = BufReader::new(&file);
            let data = terima_parse_3.recv().unwrap();
            if data.is_empty() {break}
            loop{
                buf.clear();
                if reader.read_line(buf).expect("") == 0 {break}
                let u :Value = serde_json::from_str(&buf).expect("");
                //println!("{}",u);
                if u["mod"] == data[0] {
                    let mut i  = 0 ;
                    loop{
                        if u["nilai"][i]["fn"] ==  data[1]{
                            //println!("{}",u["nilai"][i]);
                            kirim_parse_3.send(Some(u["nilai"][i].clone())).expect("msg: &str");
                            continue 'main
                        }
                        if u["nilai"][i] == json!(null){
                            println!("funsi '{0}' tidak ditemukan di modul '{1}'\nbantuan : tambahkan 'fn {0}' di modul '{1}'",data[1],data[0]);
                            std::process::exit(17);
                        }
                        i += 1
                    }
                } 
                
            } 
            println!("mudul '{0}' tidak ditemukan\nbantuan : buat file '{0}.is' di forder 'kode'",data[0]);
            std::process::exit(17);
        }
    } else {
        //turbo
        //gagal
        loop{
            let t = terima.recv().expect("");
            if t.is_empty() {break}
            buf.push_str(t.as_str());
        }
        let mut data : Vec<Value> = Vec::new();
        for i in buf.split("\n") {
            data.push(serde_json::from_str(i).expect(""));
        }
        'main_:loop{
            let _fn =terima_parse_3.recv().unwrap();
            if _fn.is_empty(){break}
            for u in &data {
                if u["mod"] == _fn[0] {
                    let mut i  = 0 ;
                    loop{
                        if u["nilai"][i]["fn"] ==  _fn[1]{
                            //println!("{}",u["nilai"][i]);
                            kirim_parse_3.send(Some(u["nilai"][i].clone())).expect("msg: &str");
                            continue 'main_
                        }
                        if u["nilai"][i] == json!(null){
                            println!("funsi '{0}' tidak ditemukan di modul '{1}'\nbantuan : tambahkan 'fn {0}' di modul '{1}'",_fn[1],_fn[0]);
                            std::process::exit(17);
                        }
                        i += 1
                    }
                }
            }
            println!("mudul '{0}' tidak ditemukan\nbantuan : buat file '{0}.is' di forder 'kode'",_fn[0]);
            std::process::exit(17);
        }
        buf.clear();
        //println!("{:?}",data);
        
    }
}