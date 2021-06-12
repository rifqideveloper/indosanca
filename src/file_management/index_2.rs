use std::fs::File;
use std::io::{BufRead, BufReader};
use serde_json::{Value};
use serde_json::json;
pub fn baca(
    buf:&mut String,
    terima:std::sync::mpsc::Receiver<std::string::String>,
    path:&String,
    terima_parse_3:std::sync::mpsc::Receiver<([String; 3],bool)>,
    kirim_parse_3:std::sync::mpsc::Sender<std::option::Option<serde_json::Value>>,
    turbo:bool,
){
    use std::io::Write;
    if !turbo{
        let mut file = match std::fs::File::create(format!("{}\\parsing\\parse_2",path)){
            Ok(t)=>{t}
            Err(_)=>{
                if let Err(_) = std::fs::create_dir_all(format!("{}\\parsing",path)) {
                    println!("tidak dapat membuat target direktori '{0}' ?",path);
                    std::process::exit(16);
                }
                match std::fs::File::create(format!("{}\\parsing\\parse_2",path)){
                    Ok(o)=>{o}
                    Err(_)=>{
                        println!("tidak dapat membuat target file '{0}\\parsing\\parse_2' ?",path);
                        std::process::exit(16);
                    }
                }
            }
        };
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
            let mut reader = BufReader::with_capacity(1000,&file);
            let data = terima_parse_3.recv().unwrap();
            if data.0 == ["".to_string(),"".to_string(),"".to_string()] {break}
            loop{
                buf.clear();
                if reader.read_line(buf).expect("") == 0 {break}
                let u :Value = match serde_json::from_str(&buf){
                    Ok(o)=>{o}
                    Err(_)=>{
                        println!("json format error");
                        std::process::exit(1);           
                    }
                };
                //println!("{}",u);
                if u["mod"] == data.0[0] {
                    let mut i  = 0 ;
                    loop{
                        if u["nilai"][i]["fn"] ==  data.0[1]{
                            //println!("{}",u["nilai"][i]);
                            if u["nilai"][i]["publik"] != json!(data.1) {
                                println!("\
                                {1}/{0}\n\n\
                                fungsi '{0}' pribadi \n\
                                info    : fungsi pribadi hanya dapat diakses oleh fungsi lokal\n\
                                bantuan : ubah fungsi ke pulik\n\
                                contoh  :\n\
                                |   \n\
                                |   cpu pub {0}\n\
                                |   \n",data.0[1],data.0[0]);
                                std::process::exit(17);
                            }
                            kirim_parse_3.send(Some(u["nilai"][i].clone())).expect("msg: &str");
                            continue 'main
                        }
                        if u["nilai"][i] == json!(null){
                            println!("funsi '{0}' tidak ditemukan di modul '{1}'\nbantuan : tambahkan 'fn {0}' di modul '{1}'",data.0[1],data.0[0]);
                            
                            std::process::exit(17);
                        }
                        i += 1
                    }
                } 
                
            } 
            println!("mudul '{0}' tidak ditemukan\nbantuan : buat file '{0}.is' di forder 'kode'",data.0[0]);
            std::process::exit(17);
        }
    } else {
        //turbo
        //gagal
        //buf.clear();
        let mut r :Vec<Value> = Vec::with_capacity(2);
        loop{
            match terima.recv(){
                Ok(t)=>{
                    if !t.is_empty(){
                        buf.push_str(&t);
                        if buf.ends_with("\n"){
                            match serde_json::from_str(buf){
                                Ok(o)=>{r.push(o) }
                                Err(_)=>{}
                            };
                            buf.clear()
                        }
                    } else {
                        buf.clear();
                        break
                    }
                }
                Err(_)=>{
                    std::process::exit(17);
                }
            }
            
        }
        let mut data :([String; 3],bool) = terima_parse_3.recv().unwrap();
        'main_:loop{
            if data.0 == ["".to_string(),"".to_string(),"".to_string()] {break}
            for u in &r {
                if u["mod"] == data.0[0] {
                    let mut i  = 0 ;
                    loop{
                        if u["nilai"][i]["fn"] ==  data.0[1]{
                            //println!("{}",u["nilai"][i]);
                            if u["nilai"][i]["publik"] != json!(data.1) {
                                println!("\
                                {1}/{0}\n\n\
                                fungsi '{0}' pribadi \n\
                                info    : fungsi pribadi hanya dapat diakses oleh fungsi lokal\n\
                                bantuan : ubah fungsi ke pulik\n\
                                contoh  :\n\
                                |   \n\
                                |   cpu pub {0}\n\
                                |   \n",data.0[1],data.0[0]);
                                std::process::exit(17);
                            }
                            kirim_parse_3.send(Some(u["nilai"][i].clone())).unwrap();
                            continue 'main_
                        }
                        if u["nilai"][i] == json!(null){
                            println!("funsi '{0}' tidak ditemukan di modul '{1}'\nbantuan : tambahkan 'fn {0}' di modul '{1}'",data.0[1],data.0[0]);
                            std::process::exit(17);
                        }
                        i += 1
                    }
                }
            }
            println!("mudul '{0}' tidak ditemukan\nbantuan : buat file '{0}.is' di forder 'kode'",data.0[0]);
            std::process::exit(17);
        }
        buf.clear();
        data = terima_parse_3.recv().unwrap();
        //println!("{:?}",data);
    }
}