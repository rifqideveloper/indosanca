use std::fs::OpenOptions;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde_json::{Value};
use serde_json::json;
use std::io::Write;
use std::io::SeekFrom;
use std::io::Seek;
use std::io::Read;
struct file_iter {
    nama:File,
    buf_:[u8;1],
}
impl file_iter {
    fn next(&mut self,buf:&mut String) -> usize {
        let mut _len = self.nama.read(&mut self.buf_).unwrap();
        while self.buf_ != "\n".as_bytes() {
            buf.push_str(
                std::str::from_utf8(&self.buf_).unwrap()
            );
            _len += self.nama.read(&mut self.buf_).unwrap();
        }
        _len
    }
    fn reset(&mut self){
        self.nama.rewind().unwrap();
    }
}
pub fn baca(
    buf:&mut String,
    terima:std::sync::mpsc::Receiver<std::string::String>,
    path:&String,
    terima_parse_3:std::sync::mpsc::Receiver<([String; 3],bool)>,
    kirim_parse_3:std::sync::mpsc::Sender<std::option::Option<serde_json::Value>>,
    turbo:bool,
){  

    let mut file = OpenOptions::new().read(true).write(true).open(format!("{}\\parsing\\parse_2",path)).unwrap();
    for x in terima.into_iter() {
        file.write_all(x.as_bytes()).unwrap();
    }
    file.rewind().unwrap();
    let mut file = file_iter {nama:file,buf_:[0u8]};
    'main:for data in terima_parse_3.iter() {
        if data.0 == ["".to_string(),"".to_string(),"".to_string()] {break}
        while file.next(buf) != 0 {
            let v :Value = serde_json::from_str(&buf).unwrap();
            if v["mod"] == data.0[0] {
                for i in 0.. { 
                    if v["nilai"][i]["fn"] == data.0[1] {
                        kirim_parse_3.send(Some(v["nilai"][i].clone())).expect("msg: &str");
                        continue 'main
                    } else if v["nilai"][i]["fn"] == json!(null) {
                        panic!()
                    }
                }
            }
        }
        panic!();
    }
    /*
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
        loop {
            match terima.recv().unwrap() {
                o if !o.is_empty() => {
                    file.write(o.as_bytes()).unwrap();
                }
                _=>{
                    if let Ok(error) = terima.recv_timeout(std::time::Duration::from_millis(10)) {

                    }
                    break
                }
            }
        }
        /* versi lama
        buf.push_str(&terima.recv().expect("")) ;
        loop {
            if buf.is_empty() {}
        }
        while buf != ""{
            file.write(buf.as_bytes()).expect("");
            buf.clear();
            buf.push_str(&terima.recv().expect(""));
        }
        */
        drop(file);
        //
        'main:for data in terima_parse_3.iter() {
            if data.0 == ["".to_string(),"".to_string(),"".to_string()] {break}
            let mut reader = BufReader::with_capacity(1000,File::open(format!("{}\\parsing\\parse_2",path)).expect(""));
            while {buf.clear();reader.read_line(buf).expect("") != 0} {
                let u :Value = match serde_json::from_str(&buf){
                    Ok(o)=>{o}
                    Err(_)=>{
                        println!("json format error");
                        std::process::exit(1);           
                    }
                };
                if u["mod"] == data.0[0] {
                    //println!("testing {:?}",data.0);std::process::exit(17);
                    for i in 0.. { 
                        if u["nilai"][i]["fn"] == data.0[1]{
                            kirim_parse_3.send(Some(u["nilai"][i].clone())).expect("msg: &str");
                            continue 'main
                        } 
                        if u["nilai"][i] == json!(null){
                            println!("{}",i);
                            println!("fungsi '{0}' tidak ditemukan di modul '{1}'\nbantuan : tambahkan 'fn {0}' di modul '{1}'",data.0[1],data.0[0]);
                            std::process::exit(17);
                        }
                    }
                    
                }
            }
        }
    } else {
        //turbo gagal
        //buf.clear();
        /*
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
        */
        let mut buff = String::with_capacity(1000);
        loop {
            if let Ok(t) = terima.recv() {
                if !t.is_empty() {
                    buff.push_str(t.as_str());
                } else {

                }
            } else {
                panic!()
            }

        }
        //panic!();
    } 
    */
}