use std::fs;    
use std::fs::File;
use std::io::{BufRead, BufReader};
struct Data<'a>{
    kirim:&'a std::sync::mpsc::Sender<std::string::String>
}
impl Data<'_>{
    fn file(&self,x:&str,buf:&mut String){
        self.kirim.send(
            format!("<mod {}\n",
                x.clone().split(&['\\','/'][..]).last().expect("").replace(".is","")
            )
        ).expect("msg: &str");
        let mut read = BufReader::new(File::open(&x).expect(""));
        while read.read_line( buf).expect("") != 0 {
            *buf = buf.trim_start().to_string(); //buf.trim_start();
            if !buf.is_empty(){
                self.kirim.send(
                    if buf.ends_with("\n") {buf.to_string()} else { buf.to_string() + "\n"}
                ).expect("msg: &str");
            }
            buf.clear()
        }
        self.kirim.send("mod>\n".to_string()).expect("msg: &str");
    }
    fn selesai(self){
        self.kirim.send("".to_string()).expect("")
    }
}
pub fn baca(
    buf:&mut String,
    path:&String,
    kirim:std::sync::mpsc::Sender<std::string::String>
){
    let data = Data{
        kirim:&kirim
    };
    for path in fs::read_dir(format!("{}\\kode",path)).unwrap(){
        data.file(& path.unwrap().path().display().to_string(), buf);
    }
    data.selesai()
}