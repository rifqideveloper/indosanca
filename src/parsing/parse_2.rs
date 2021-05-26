
//use serde::{Deserialize, Serialize};
//use serde_json::json;
use serde_json::{/*Result,*/ Value};
//use std::fs;
pub fn baca(
    baris:std::sync::mpsc::Receiver<String>,
    kirim:std::sync::mpsc::Sender<std::string::String>
){
    let mut fn_ = (false,false);
    loop{
        let buf = baris.recv().expect("msg: &str");
        if buf == "" {break}
        match serde_json::from_str(&buf) {
            Ok(t)=>{
                let json: Value = t;
                if json["tipe"] == "modul_masuk" {
                    kirim.send(
                        format!("{{\"mod\":{},\"nilai\":[",json["nama"])
                    ).expect("msg: &str")
                } else if json["tipe"] == "modul_keluar" {
                    kirim.send(
                        format!("{}}}\n",if fn_.0 {fn_ = (false,false) ; "]}]"} else {"]"})
                    ).expect("msg: &str")
                } else if json["tipe"] == "fn_cpu" {
                    kirim.send(
                        format!("{}{{\"fn\":{},\"nilai\":[",
                        if fn_.0 {","} else { fn_.0 = true ; "" }
                        ,json["nama"])
                    ).expect("msg: &str")
                } else {
                    kirim.send(format!("{}{}",
                    if fn_.1 {","} else { fn_.1 = true ; "" }
                    ,json)).expect("msg: &str")
                }
            }
            Err(_)=>{}
        }
    }
    kirim.send("".to_string()).expect("msg: &str")
}