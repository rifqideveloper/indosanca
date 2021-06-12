
//use serde::{Deserialize, Serialize};
//use serde_json::json;
//use serde_json::{/*Result,*/ Value};
//use std::fs;
use crate::parsing::perintah;
use crate::parsing::tipe;


pub fn baca(
    baris:std::sync::mpsc::Receiver<perintah>,
    kirim:std::sync::mpsc::Sender<String>
){
    let mut fn_ = [false,false];
    let _arg = ([false,false],"");
    //let blok = [false,false];
    //panic!();
    //#[allow(unreachable_code)]
   
    loop{
        match baris.recv().unwrap(){
            perintah::variabel(a,b,c,d)=>{
                //println!("{{{:#?},{},{},{}}}",a,b,c,d);
                let v = format!(
                    "{}{{\"data\":\"{}\",\"tipe\":\"{}\",\"nama\":\"{}\",\"nilai\":{nilai}}}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    match a {
                     tipe::_string(o)=>{o },
                     tipe::_u8=>{String::from("u8")} 

                    },
                    b,
                    c,
                    nilai = d
                );
                //println!("{}",v);
                kirim.send(
                    v
                ).unwrap()
                
            }
            perintah::cetak(_nilai)=>{
                let mut d = _nilai;
                kirim.send(
                    format!("{}{{ \"tipe\" : \"cetak\", \"nilai\": [\"{}\",\"{}\"] }}",
                        if fn_[1] {","} else { fn_[1] = true ; "" },
                        if d.starts_with("\""){
                            //println!("string");
                            d.remove(0);
                            d.pop();
                            "lansung"
                        } else {
                            "var"
                        },
                        d
                    )
                ).unwrap();

                /*
                kirim.send(
                    
                    format!("{0} {{ \"tipe\" : \"cetak\"{1},\"nilai\": {2} }}",
                        if fn_[0] {","} else { fn_[0] = true ; "" },
                        
                    )
                    
                    
                ).unwrap()
                */
            }
            perintah::modul_masuk(nama)=>{
                kirim.send(
                    format!("{{\"mod\":\"{}\",\"nilai\":[",nama)
                ).unwrap()
            }
            perintah::cpu(nama,publik)=>{
                kirim.send(
                    format!("{}{{\"fn\":\"{}\",\"publik\":{},\"nilai\":[",
                    if fn_[0] { "]},"} else { fn_[0] = true ; "" }
                    ,nama,publik)
                ).unwrap()
            }
            perintah::modul_keluar=>{
                kirim.send(
                    format!("{}}}\n",if fn_[0] {fn_ = [false,false] ; "]}]"} else {"]"})
                ).unwrap()
            }
            perintah::tulis(a,b)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"tulis\",\"var\":\"{}\",\"nilai\":{}}}",
                    if fn_[0] {","} else { fn_[0] = true ; "" }
                    ,a,b)
                ).unwrap()
            }
            perintah::blok_buka =>{
                kirim.send(
                    format!("{}{{\"tipe\":\"blok\"}}",if fn_[0] {","} else { fn_[0] = true ; "" })
                ).unwrap()
            }
            perintah::blok_tutup =>{
                kirim.send(
                    format!("{}{{\"tipe\":\"blok_tutup\"}}",if fn_[0] {","} else { fn_[0] = true ; "" })
                ).unwrap()
            }
            perintah::selesai=>{
                kirim.send("".to_string()).unwrap();
                break
            }

        }
        /*
        let buf = match baris.recv(){
            Ok(o)=>{
                if o != ""{o} else {break}
            }
            Err(_)=>{panic!()}
            
        };
        
        match serde_json::from_str(&buf) {
            Ok(t)=>{
                let json: Value = t;
                match kirim.send(
                    if json["tipe"] == "modul_masuk" {
                        format!("{{\"mod\":{},\"nilai\":[",json["nama"])
                    } else if json["tipe"] == "modul_keluar" {
                        format!("{}}}\n",if fn_.0 {fn_ = (false,false) ; "]}]"} else {"]"})
                    } else if json["tipe"] == "fn_cpu" {
                        format!("{}{{\"fn\":{},\"nilai\":[",
                        if fn_.0 {","} else { fn_.0 = true ; "" }
                        ,json["nama"])
                    } else if json["tipe"] == "cetak" {
                        println!("{}",json["tipe"]);
                        panic!();
                        /*
                        format!("{0} {{ \"tipe\" : \"cetak\",\"nilai\": {} }}",
                            if fn_.0 {","} else { fn_.0 = true ; "" },

                        )*/
                    } else {
                        format!("{}{}",
                        if fn_.1 {","} else { fn_.1 = true ; "" }
                        ,json)
                    }
                ){
                    Ok(_)=>{}
                    Err(_)=>{panic!()}
                }
            }
            Err(_)=>{
                println!("{}",buf);
                panic!()
            }
        }
        */
    }
    
}