
//use serde::{Deserialize, Serialize};
//use serde_json::json;
//use serde_json::{/*Result,*/ Value};
//use std::fs;
use crate::parsing::perintah;
use crate::parsing::tipe;
//versi baru
pub fn parse(
    baris:std::sync::mpsc::Receiver<perintah>,
    kirim:std::sync::mpsc::Sender<String>
){
    let mut fn_ = [false,false];
    //let mut blok = 0;
    loop {
        match baris.recv().unwrap(){
            perintah::putar=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"putar\"}}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    )
                ).unwrap()
            }
            perintah::putus=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"putus\"}}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    )
                ).unwrap()
            }
            perintah::lanjut=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"lanjut\"}}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    )
                ).unwrap()
            }
            perintah::batas=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"batas\"}}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    )
                ).unwrap()
            }
            perintah::variabel_null(a,b)=>{
                kirim.send(
                    format!("{}{{ \"tipe\":\"var\",\"data\":\"{}\",\"nama\":\"{}\" }}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    a,b
                    )
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
                        }else if d.parse::<u64>().is_ok(){
                            "langsung_int"
                        } else if d.parse::<f64>().is_ok() {
                            "langsung_f"
                        } else {
                            "var"
                        },
                        d
                    )
                ).unwrap();
            }
            //perintah::arit_tambah(a,b,c)=>{
            //}
            perintah::boolean(o)=>{
                if o {
                    kirim.send(
                        format!("{}{{\"tipe\":\"benar\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                    ).unwrap()
                } else {
                    kirim.send(
                        format!("{}{{\"tipe\":\"salah\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                    ).unwrap()
                }
            }
            perintah::jika=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"jika\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap();
            }
            perintah::lalu=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"lalu\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::lalu_jika=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"lalu_jika\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::jika_=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"jika_\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::jika_tutup=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"jika_tutup\"}}",if fn_[1] {","}else{fn_[1] = true;""})    
                ).unwrap()
            }
            perintah::i32_eqz=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"_i32_eqz\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::_i32_konst(o)=>{
                kirim.send(
                    format!("{1}{{\"tipe\":\"_i32_konst\",\"nilai\":\"{}\"}}",o,if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::kurang=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"kurang\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::halaman(o)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"halaman\",\"nilai\":\"{}\"}}",if fn_[1] {","} else { fn_[1] = true ; ""},o)
                ).unwrap()
            }
            perintah::tulis(a,b)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"tulis\",\"nama\":\"{}\",\"nilai\":{:?}}}",
                    if fn_[0] {","} else { fn_[0] = true ; "" },
                    a,
                    b
                    )
                ).unwrap()
            }
            perintah::cpu(nama,publik)=>{
                kirim.send(
                    format!("{}{{\"fn\":\"{}\",\"publik\":{},\"nilai\":[",
                    if fn_[0] { "]},"} else { fn_[0] = true ; "" }
                    ,nama,publik)
                ).unwrap()
            }
            perintah::modul_masuk(nama)=>{
                kirim.send(
                    format!("{{\"mod\":\"{}\",\"nilai\":[",nama)
                ).unwrap()
            }
            perintah::modul_keluar=>{
                kirim.send(
                    format!("{}}}\n",if fn_[0] {fn_ = [false,false] ; "]}]"} else {"]"})
                ).unwrap()
            }
            perintah::blok_buka=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"blok_t_buka\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::blok_tutup=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"blok_t_tutup\"}}",if fn_[1] {","}else{fn_[1] = true;""})

                ).unwrap()
            }
            perintah::blok(a)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"blok\",\"nilai\":\"{}\"}}",if fn_[1] {","} else { fn_[1] = true ; ""},a)
                ).unwrap()
            }
            perintah::br(o)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"br\",\"nilai\":\"{}\"}}",if fn_[1] {","} else { fn_[1] = true ; ""},o)
                ).unwrap()
            }
            perintah::if_br(a,b)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"if_br\",\"nilai\":[\"{}\",\"{}\"]}}",if fn_[1] {","} else { fn_[1] = true ; ""},a,b)
                ).unwrap()
            }
            perintah::blok_=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"blok_\"}}",if fn_[0] {","} else { fn_[0] = true ; ""})
                ).unwrap()
            }
            
            perintah::selesai=>{
                kirim.send("".to_string()).unwrap();
                break
            }
        }
    }
}
/*
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
            perintah::variabel_null(a,b,c)=>{
                let v = format!(
                    "{0}{{\"data\":\"{1}\",\"tipe\":\"{2}\",\"nama\":\"{3}\" }}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    match a {
                        tipe::_string(o)=>{o},
                        tipe::_u8=>{String::from("u8")} 
                    },
                    b,
                    c,
                );
                println!("{}",v);
                kirim.send(
                    v
                ).unwrap();
            }
            perintah::variabel(a,b,c,d)=>{
                //println!("{{{:#?},{},{},{}}}",a,b,c,d);
                let v = format!(
                    "{0}{{\"data\":\"{1}\",\"tipe\":\"{2}\",\"nama\":\"{3}\",\"nilai\":{nilai}}}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    match a {
                     tipe::_string(o)=>{o},
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
            perintah::arit_tambah(a,b,c)=>{
                kirim.send(
                    format!("{0}{{\"tipe\":\"arit_tambah\",\"var\":\"{1}\",\"nilai\":[{2},{3}]}}",
                    if fn_[0] {","} else { fn_[0] = true ; "" },
                    a,
                    b,
                    c,
                    )
                ).unwrap();
                println!("testing tambah");
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
*/