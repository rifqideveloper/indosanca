use serde::{Deserialize, Serialize };
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use crate::parsing::Tipe;
#[derive(Debug)]
struct Data {
    tipe: String,
    nilai: String,
}
#[derive(Debug)]
struct Var {
    tipe: String,
    id: u64,
    nama: String,
}
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Variabel {
    pub nilai: Tipe,
    pub id: u64,
}
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Nilai {
    lansung_str(String),
    lansung_int(u64),
    lansung_float(f64),
    penujuk_(u64),
    minta(u64),
    None,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Pohon {
    fungsi(String),
    panggil_fn(String),
    cetak(Nilai),
    _let(u64, Tipe , bool , bool ),
    var(u64, Tipe),
    panggil_var(u64),
    tulis(u64, Tipe),
    konst_str(u64, String),
    const_32(i32),
    local_set(i32),
    halaman(Nilai),
    warnalatarbelakang(String),
    gambarlatarbelakang(String),
    judul(String),
    tombol(String),
    isi(String, String),
    warna(String, String),
    warnalatarbelakangid(String, String),
    ukurankata(String, String),
    klik(String, Vec<String>),
    i32_eqz,
    i32_eq,
    blok(String),
    blok_,
    putar,
    putus,
    lanjut,
    batas,
    add,
    sub,
    div_s,
    div_u,
    mul,
    bandingkan_str_str(u64, u64),
    r#if,
    if_,
    jika_tutup,
    lalu_jika,
    if_br(String, String),
    br(String),
    main(String),
}
#[derive(Debug)]
#[allow(non_camel_case_types)]
enum operasi_logika_aritmatika {
    Bool(bool),
    Nomer(i128),
    _str(String),
    sama_dengan,
    tidak_sama,
    lebih_besar,
    lebih_kecil,
    nama_var(u64),
    tambah,

}
//
#[derive(rust_embed::RustEmbed)]
#[folder = "std/parsing"]
#[prefix = "prefix/"]
struct STD;
//
pub fn parse_2(
    kirim: std::sync::mpsc::Sender<([String; 3], bool)>,
    terima: std::sync::mpsc::Receiver<std::option::Option<serde_json::Value>>,
    path: &String,
    pohon: &mut Vec<Pohon>,
) {
    let mut id: u64 = 0;
    let mut _var: Vec<Var> = Vec::with_capacity(2);
    let mut _data_: HashMap<u64, Data> = HashMap::with_capacity(2);
    let mut lokasi: Vec<[String; 3]> =
        Vec::from([["main".to_string(), "main".to_string(), "".to_string()]]);
    let mut jika: Vec<u8> = Vec::new();
    let mut lalu = false;
    let mut _data_var: HashMap<String, (u64, Tipe, usize , bool)> = HashMap::new();
    let mut swict: Vec<(u8, bool, bool, String, bool)> = Vec::with_capacity(1);
    for y in 0.. {
        kirim.send((lokasi[y].clone(), false)).unwrap();
        match terima.recv().unwrap() {
            Some(o) => {
                let mut i = 0;
                loop {
                    match o["nilai"][i]["tipe"].as_str() {
                        Some(v) => {
                            match v {
                                "putar" => pohon.push(Pohon::putar),
                                "batas" => pohon.push(Pohon::batas),
                                "lanjut" => pohon.push(Pohon::lanjut),
                                "putus" => {
                                    if let Some(o) = swict.last() {
                                        if o.2 && !o.4 && o.0 == 0 {
                                            pohon.push(Pohon::br("1".to_string()));
                                            i += 1;
                                            continue;
                                        }
                                    }
                                    pohon.push(Pohon::putus)
                                }
                                "blok" => pohon.push(Pohon::blok(
                                    o["nilai"][i]["nilai"].as_str().unwrap().to_string(),
                                )),
                                "blok_" => pohon.push(Pohon::blok_),
                                /*
                                "konst" => {
                                    let mut v =
                                        o["nilai"][i]["nilai"].as_str().unwrap().to_string();
                                    let nama = o["nilai"][i]["nama"].as_str().unwrap().to_string();
                                    println!("dibuat varible ");
                                    if v.ends_with("str") {
                                        v.pop();
                                        v.pop();
                                        v.pop();
                                        //println!("dibuat varible ");
                                        id += 1;
                                        _data_var
                                            .insert(nama, (id , Tipe::_String(Some(v.clone()))));
                                        pohon.push(Pohon::konst_str(id,v.clone()))
                                    }else if v.ends_with("nomer") {
                                        v.pop();
                                        v.pop();
                                        v.pop();
                                        v.pop();
                                        v.pop();
                                        id += 1;
                                        _data_var.insert(nama, (id,Tipe::nomer(v.parse().unwrap())));
                                    } else if v.ends_with("u8") {

                                    } else {
                                        panic!()
                                    }
                                }
                                "glob" => {
                                    //???
                                    panic!();
                                }
                                */
                                "let" => {
                                    //panic!();
                                    id += 1;
                                    let data = match o["nilai"][i]["tipe_"].as_str().unwrap() {
                                        "str" => crate::parsing::Tipe::_string(crate::parsing::Str::None),
                                            "str_" => crate::parsing::Tipe::_string(crate::parsing::Str::Some(o["nilai"][i]["nilai"].as_str().unwrap().to_string())),
                                            "u8" => crate::parsing::Tipe::_u8(None),
                                            "u8_" => crate::parsing::Tipe::_u8(Some( o["nilai"][i]["nilai"].as_str().unwrap().parse().unwrap() )),
                                            "i8" => crate::parsing::Tipe::_i8(None),
                                            "i8_" => crate::parsing::Tipe::_i8(Some( o["nilai"][i]["nilai"].as_str().unwrap().parse().unwrap() )),
                                            "i16" => crate::parsing::Tipe::_i16(None),
                                            "i16_" => crate::parsing::Tipe::_i16(Some( o["nilai"][i]["nilai"].as_str().unwrap().parse().unwrap() )),
                                            "u16" => crate::parsing::Tipe::_u16(None),
                                            "u16_" => crate::parsing::Tipe::_u16(Some( o["nilai"][i]["nilai"].as_str().unwrap().parse().unwrap() )),
                                            "i32" => crate::parsing::Tipe::_i32(None),
                                            "i32_" => crate::parsing::Tipe::_i32(Some( o["nilai"][i]["nilai"].as_str().unwrap().parse().unwrap() )),
                                            "u32" => crate::parsing::Tipe::_u32(None),
                                            "u32_" => crate::parsing::Tipe::_u32(Some( o["nilai"][i]["nilai"].as_str().unwrap().parse().unwrap() )),
                                            "u64" => crate::parsing::Tipe::_u64(None),
                                            "u64_" => crate::parsing::Tipe::_u64(Some( o["nilai"][i]["nilai"].as_str().unwrap().parse().unwrap() )),
                                            "i64" => crate::parsing::Tipe::_i64(None),
                                            "i64_" => crate::parsing::Tipe::_i64(Some( o["nilai"][i]["nilai"].as_str().unwrap().parse().unwrap() )),
                                            
                                        _=>{
                                            panic!()
                                        }
                                    };
                                    let _mut = if let Some(_) = o["nilai"][i]["mut"].as_str() {true} else {false};
                                    _data_var.insert(o["nilai"][i]["nama"].as_str().unwrap().to_string(), (id , data.clone(), pohon.len(),_mut.clone()));
                                    pohon.push( Pohon::_let( id , data ,_mut,false) );
                                    //???
                                }
                                /*
                                "var" => {
                                    //sementara
                                    _var.push(Var {
                                        id: id,
                                        tipe: o["nilai"][i]["data"].as_str().unwrap().to_string(),
                                        nama: o["nilai"][i]["nama"].as_str().unwrap().to_string(),
                                    });
                                    pohon.push(Pohon::var(id, Tipe::_u8(None)));
                                    id += 1;
                                }
                                */
                                "tulis" => {
                                    //belum selesai
                                        //mengecek jika var ada
                                    if let Some(nama) = _data_var.get_key_value(&o["nilai"][i]["nama"].as_str().unwrap().to_string()) {
                                        //mengecek jika var mut
                                        if nama.1.3 {
                                            /*
                                            let i = 0 ;
                                            loop {
                                                let x = o["nilai"][i]["nilai"][i].as_str().unwrap().to_string();
                                                break
                                            }
                                            */
                                            let mut v : Vec<&str> = Vec::new(); 
                                            for x in 0..{
                                                if let Some(x) = o["nilai"][i]["nilai"][x].as_str() {
                                                    v.push(x)
                                                } else {
                                                    break
                                                }
                                            }
                                            match nama.1.1 {
                                                Tipe::_u8(_)=>{
                                                    //if v.len() == 1 {
                                                        pohon.push(
                                                            Pohon::tulis(
                                                                nama.1.0,
                                                                Tipe::_u8(
                                                                    if let Ok(o) = v[0].parse() {
                                                                        Some(o)
                                                                    } else {
                                                                        panic!()
                                                                    }
                                                                )
                                                            )
                                                        );
                                                    //} 
                                                }
                                                _=>{}
                                            }
                                            /*
                                            pohon.push(
                                                Pohon::tulis(
                                                    nama.1.0,
                                                    nama.1.1.clone()
                                                )
                                            );
                                            */
                                        } else {
                                            panic!()
                                        }
                                    } else {
                                        panic!()
                                    }
                                    //let nilai = &o["nilai"][i]["nilai"];
                                    /*
                                    for i in _var.iter() {
                                        if i.nama == nama {
                                            if nilai[1] == json!(null) {
                                                pohon.push(Pohon::tulis(
                                                    i.id,
                                                    //SEMENTARA
                                                    Tipe::_u8(Some(
                                                        nilai[0]
                                                            .as_str()
                                                            .unwrap()
                                                            .parse::<u8>()
                                                            .unwrap(),
                                                    )),
                                                ));
                                                break;
                                            }
                                            let mut v: Vec<Pohon> = Vec::with_capacity(3);
                                            let mut ari = (false, false, false, false);
                                            for x in 0.. {
                                                if nilai[x] == json!(null) {
                                                    break;
                                                }
                                                match nilai[x].as_str().unwrap() {
                                                    "+" => {
                                                        ari.0 = true;
                                                        /*
                                                        v.push(
                                                            Pohon::add
                                                        );*/
                                                    }
                                                    "-" => {
                                                        ari.1 = true;
                                                    }
                                                    "/" => {
                                                        ari.2 = true;
                                                    }
                                                    "*" => {
                                                        ari.3 = true;
                                                    }
                                                    _ => {
                                                        match nilai[x]
                                                            .as_str()
                                                            .unwrap()
                                                            .parse::<i32>()
                                                        {
                                                            Ok(a) => {
                                                                v.push(Pohon::const_32(a));
                                                                if ari.0 {
                                                                    ari.0 = false;
                                                                    v.push(Pohon::add);
                                                                } else if ari.1 {
                                                                    ari.1 = false;
                                                                    v.push(Pohon::sub);
                                                                } else if ari.2 {
                                                                    match i.tipe.as_str() {
                                                                        "u8" => {
                                                                            v.push(Pohon::div_u);
                                                                        }
                                                                        "i8" => {
                                                                            v.push(Pohon::div_s);
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                    ari.2 = false;
                                                                } else if ari.3 {
                                                                    ari.3 = false;
                                                                    v.push(Pohon::mul);
                                                                }
                                                            }
                                                            Err(_) => {}
                                                        }
                                                    }
                                                }
                                            }
                                            if !v.is_empty() {
                                                use std::convert::TryInto;
                                                for x in &v {
                                                    match x {
                                                        Pohon::const_32(a) => {
                                                            pohon.push(Pohon::const_32(*a));
                                                        }
                                                        Pohon::add => {
                                                            pohon.push(Pohon::add);
                                                        }
                                                        Pohon::sub => {
                                                            pohon.push(Pohon::sub);
                                                        }
                                                        Pohon::div_s => {
                                                            pohon.push(Pohon::div_s);
                                                        }
                                                        Pohon::div_u => {
                                                            pohon.push(Pohon::div_u);
                                                        }
                                                        Pohon::mul => {
                                                            pohon.push(Pohon::mul);
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                pohon.push(Pohon::local_set(
                                                    i.id.try_into().unwrap(),
                                                ))
                                            }
                                            //#[cfg(debug_assertions)]print!("testing {:?}", v);
                                            /*
                                            let v = serde_json::from_str::<Vec<String>>(
                                                nilai.as_str().unwrap()
                                            ).unwrap();
                                            //let x = Vec::new();
                                            if v[1] == "+"{
                                                //testing contoh
                                                pohon.push(
                                                    Pohon::arit(
                                                        Arit::tambah(0,
                                                        Nilai::lansung_int(
                                                            v[0].parse().unwrap()
                                                        ),Nilai::lansung_int(
                                                            v[2].parse().unwrap()
                                                        ))
                                                    )
                                                );
                                                break
                                            }
                                            */
                                        }
                                    }
                                    */
                                }
                                "br" => pohon.push(Pohon::br(
                                    o["nilai"][i]["nilai"].as_str().unwrap().to_string(),
                                )),
                                "panggil_fn" => {
                                    //mengecek jika funsi ada

                                    //memanggil fungsi
                                    let o = o["nilai"][i]["nama"].as_str().unwrap();
                                    //let v = ;
                                    //let x = o.to_string().split('_').collect::<Vec<&str>>()[1];

                                    lokasi.push([
                                        o.to_string().split('_').collect::<Vec<&str>>()[0]
                                            .to_string(),
                                        o.to_string().split('_').collect::<Vec<&str>>()[1]
                                            .to_string(),
                                        "".to_string(),
                                    ]);
                                    pohon.push(Pohon::panggil_fn(
                                        /*o["nilai"][i]["nama"].as_str().unwrap()*/
                                        o.to_string(),
                                    ));
                                }
                                "swict" => {
                                    let jenis = o["nilai"][i]["jenis"]
                                        .as_str()
                                        .unwrap()
                                        .parse::<u8>()
                                        .unwrap();
                                    match jenis {
                                        0 | 1 => {
                                            pohon.push(Pohon::blok("then".to_string()));
                                            let nama = o["nilai"][i]["nilai"][0]
                                                .as_str()
                                                .unwrap()
                                                .to_string();
                                            if let Some(x) = _data_var.get(&nama) {
                                                swict.push((jenis, false, false, nama, false));
                                            } else {
                                                println!("variable '{}' tidak ada", nama);
                                                panic!()
                                            }
                                        }
                                        
                                        2 => {
                                            
                                            let mut v =
                                                Vec::<operasi_logika_aritmatika>::with_capacity(1);
                                            let mut x = 0;
                                            let mut y = 0;
                                            loop {
                                                if let Some(b) = o["nilai"][i]["nilai"][x].as_str()
                                                {
                                                    match b {
                                                        "+"=>{
                                                            v.push(
                                                                operasi_logika_aritmatika::tambah
                                                            )
                                                        }
                                                        "benar" => v.push(
                                                            operasi_logika_aritmatika::Bool(true),
                                                        ),
                                                        "salah" => v.push(
                                                            operasi_logika_aritmatika::Bool(false),
                                                        ),
                                                        "==" => match &v[y - 1] {
                                                            operasi_logika_aritmatika::_str(s) => {
                                                                if let Some(l) = o["nilai"][i]["nilai"][x + 1].as_str()
                                                                {
                                                                    if l.starts_with('"') == l.ends_with('"')
                                                                    {
                                                                        let mut l = l.to_string();
                                                                        l.remove(0);
                                                                        l.pop();
                                                                        y -= 1;
                                                                        v[y] = operasi_logika_aritmatika::Bool(&l == s);
                                                                        x += 2;
                                                                        continue;
                                                                    } else if let Some(d) =
                                                                        _data_var.get(l)
                                                                    {
                                                                        if let Tipe::_string(d) =
                                                                            &d.1
                                                                        {
                                                                            if let crate::parsing::Str::Some(d) = d {
                                                                                println!("testing {:?}",(d,s));
                                                                                y -= 1;
                                                                                v[y] = operasi_logika_aritmatika::Bool(d == s);
                                                                                x += 2;
                                                                                
                                                                                continue
                                                                            }
                                                                        }
                                                                    }
                                                                } else {

                                                                }
                                                            }
                                                            _ => {}
                                                        },
                                                        o if o.starts_with('"') == o.ends_with('"') =>
                                                        {
                                                            let mut o = o.to_string();
                                                            o.remove(0);
                                                            o.pop();
                                                            v.push(operasi_logika_aritmatika::_str(
                                                                o,
                                                            ))
                                                        }
                                                        o if matches!(_data_var.get(o) ,Some(s) if {v.push(operasi_logika_aritmatika::nama_var(s.0)); true} ) /* let Some(s) =  _data_var.get(o) */ => {} 
                                                        _ => {}
                                                    }
                                                } else {
                                                    pohon.push(Pohon::blok("then".to_string()));
                                                    pohon.push(Pohon::blok("if".to_string()));
                                                    v.iter().for_each(|i| match i {
                                                        operasi_logika_aritmatika::Bool(b) => {
                                                            let mut v = String::with_capacity(12);
                                                            v.push_str("i32.const ");
                                                            v.push_str(if *b { "0" } else { "1" });
                                                            v.push_str("\n");
                                                            pohon.push(Pohon::main(v));
                                                        }
                                                        operasi_logika_aritmatika::nama_var(nama)=>{
                                                            pohon.push(
                                                                Pohon::panggil_var(
                                                                    *nama
                                                                )
                                                            )
                                                        }

                                                        _ => {}
                                                    });
                                                    pohon.push(Pohon::if_br(
                                                        "1".to_string(),
                                                        "".to_string(),
                                                    ));
                                                    swict.push((
                                                        jenis,
                                                        false,
                                                        false,
                                                        "".to_string(),
                                                        false,
                                                    ));
                                                    break;
                                                }
                                                x += 1;
                                                y += 1;
                                            }
                                        }
                                        _ => {
                                            panic!()
                                        }
                                    }
                                }
                                "swict_tutup" => {
                                    let jenis = o["nilai"][i]["jenis"]
                                        .as_str()
                                        .unwrap()
                                        .parse::<u8>()
                                        .unwrap();
                                    if let Some(x) = swict.last() {
                                        match jenis {
                                            y if (y == 0 || y == 1) && y == x.0 && x.1 => {
                                                pohon.push(Pohon::blok_);
                                                pohon.push(Pohon::blok_);
                                            }
                                            2 => {
                                                if x.0 != 2 {
                                                    panic!()
                                                }
                                                //pohon.push(Pohon::br("1".to_string()));
                                                pohon.push(Pohon::blok_);
                                                pohon.push(Pohon::blok_);
                                            }
                                            _ => {
                                                panic!()
                                            }
                                        }
                                        swict.pop();
                                    } else {
                                        panic!()
                                    }
                                }
                                "kasus" => {
                                    if let Some(x) = swict.last_mut() {
                                        if x.4 {
                                            loop {
                                                i += 1;
                                                if let Some(k) = o["nilai"][i]["tipe"].as_str() {
                                                    if k == "swict_tutup" {
                                                        swict.pop();
                                                        i += 1;
                                                        break;
                                                    }
                                                } else {
                                                    println!("erro swict_ tidak ditemukan");
                                                    panic!()
                                                }
                                            }
                                            continue;
                                        }
                                        if x.1 {
                                            pohon.push(Pohon::blok_);
                                        } else {
                                            x.1 = true;
                                            x.2 = true
                                        }
                                        if o["nilai"][i]["lalu"].as_str().unwrap() == "1" {
                                            //println!("{:?}",o["nilai"][i].as_str());
                                            if let Some(y) = _data_var.get(&x.3) {
                                                pohon.push(Pohon::blok("if".to_string()));
                                                let mut nilai = o["nilai"][i]["nilai"][0]
                                                    .as_str()
                                                    .unwrap()
                                                    .to_string();
                                                if nilai.ends_with('"') == nilai.starts_with('"') {
                                                    if let Tipe::_string(y) = &y.1 {
                                                        nilai.remove(0);
                                                        nilai.pop();
                                                        if let crate::parsing::Str::Some(y) = y {
                                                            if y == &nilai {
                                                                x.4 = true;
                                                                let mut stop = false;
                                                                while !stop {
                                                                    if let Pohon::blok(b) =
                                                                        pohon.last().unwrap()
                                                                    {
                                                                        if b == "then" {
                                                                            stop = true
                                                                        }
                                                                    }
                                                                    pohon.pop();
                                                                }
                                                            } else {
                                                                panic!()
                                                            }
                                                        } else {
                                                        }
                                                    } else {
                                                        panic!()
                                                    }
                                                }
                                            } else {
                                                panic!()
                                            }
                                        } else {
                                            x.1 = false;
                                            x.2 = false
                                        }
                                    } else {
                                        panic!()
                                    }
                                }
                                /*
                                "jika" => {
                                    jika.push(1);
                                    pohon.push(Pohon::blok("then".to_string()));
                                    pohon.push(Pohon::blok("if".to_string()))
                                }
                                "jika_b" => {}
                                "jika_" => {
                                    //???
                                    //pohon.push(Pohon::if_br("".to_string(),"$if".to_string())   )
                                    pohon.push(Pohon::if_br("0".to_string(), "".to_string()))
                                }
                                "jika_tutup" => {
                                    if lalu {
                                        lalu = false;
                                    } else {
                                        pohon.push(Pohon::jika_tutup);
                                    }
                                }
                                "lalu" => {
                                    pohon.pop();
                                }
                                "lalu_jika" => {
                                    //prototyp
                                    pohon.push(Pohon::blok("if".to_string()));
                                }
                                */
                                /*
                                "blok_t_tutup" => {
                                    pohon.push(Pohon::blok_);
                                }
                                "blok_t_buka" => {
                                    if lalu {
                                        lalu = false;
                                    } else if jika.is_empty() {
                                        pohon.push(Pohon::blok("test".to_string()))
                                    } else {
                                        jika.pop();
                                    }
                                }
                                */
                                "_i32_eqz" => pohon.push(Pohon::i32_eqz),
                                "_i32_eq" => {
                                    /*
                                    pohon.push(
                                        Pohon::i32_eq
                                    )
                                    */
                                }
                                "_i32_konst" => {
                                    let v = o["nilai"][i]["nilai"].as_str().unwrap();
                                    let v = format!("\ni32.const {}\n", v);
                                    pohon.push(Pohon::main(v))
                                }
                                "tambah" => pohon.push(Pohon::add),
                                "kurang" => {
                                    pohon.push(Pohon::sub);
                                }
                                "bagi" => {}
                                "kali" => pohon.push(Pohon::mul),
                                "modus" => {}
                                "benar" => pohon.push(Pohon::main("i32.const 0".to_string())),
                                "salah" => pohon.push(Pohon::main("i32.const 1".to_string())),
                                "if_br" => {
                                    let (a, b) = (
                                        o["nilai"][i]["nilai"][0].as_str().unwrap(),
                                        o["nilai"][i]["nilai"][1].as_str().unwrap(),
                                    );
                                    pohon.push(Pohon::if_br(a.to_string(), b.to_string()))
                                }
                                "halaman" => {
                                    //println!("<<testing {}>>",o["nilai"][i]["nilai"].as_str().unwrap().to_string().as_str());
                                    pohon.push(
                                        Pohon::halaman(
                                        match o["nilai"][i]["var"]
                                            .as_str()
                                            .unwrap()
                                            .to_string()
                                            .as_str()
                                        {
                                            "0" => Nilai::lansung_str(
                                                o["nilai"][i]["nilai"]
                                                    .as_str()
                                                    .unwrap()
                                                    .to_string(),
                                            ),
                                            "1" => {
                                                let nama = o["nilai"][i]["nilai"]
                                                    .as_str()
                                                    .unwrap()
                                                    .to_string();
                                                if let Some(o) = _data_var.get(&nama) {
                                                    /*
                                                    if let Tipe::_string(_) = o.1 {
                                                        Nilai::penujuk_(o.0)
                                                    } else {
                                                        println!("variable '{}' harus string", nama);
                                                        panic!()
                                                    }
                                                    */
                                                    Nilai::penujuk_(o.0)
                                                } else {
                                                    println!("variable '{}' tidak ada", nama);
                                                    panic!()
                                                }
                                            }
                                            _ => {
                                                panic!()
                                            }
                                        },
                                    ));
                                }
                                "klik" => {
                                    let id = o["nilai"][i]["nilai"].as_str().unwrap().to_string();
                                    let mut _mod_nama: Vec<String> = Vec::with_capacity(2);
                                    for x in 0.. {
                                        match o["nilai"][i]["fn"][x].as_str() {
                                            Some(n) => _mod_nama.push(n.to_string()),
                                            None => break,
                                        }
                                    }
                                    //mengecek jika fungsi sudah pernah dipanggi
                                    //jika tidak daftarkan fungsi
                                    let mut fn_ada = false;
                                    for i in &lokasi {
                                        if i[0] == _mod_nama[0] && i[1] == _mod_nama[1] {
                                            fn_ada = true;
                                            break;
                                        }
                                    }
                                    if !fn_ada {
                                        //???
                                        lokasi.push([
                                            _mod_nama[0].to_string(),
                                            _mod_nama[1].to_string(),
                                            "".to_string(),
                                        ]);
                                    }
                                    //
                                    pohon.push(Pohon::klik(id, _mod_nama));
                                }
                                "warnalatarbelakang" => pohon.push(Pohon::warnalatarbelakang({
                                    let v = o["nilai"][i]["nilai"].as_str().unwrap().to_string();
                                    match v.as_str() {
                                        "merah" => "red".to_string(),
                                        "biru" => "blue".to_string(),
                                        "birutua" => "darkblue".to_string(),
                                        "hitam" => "black".to_string(),
                                        "kuning" => "yellow".to_string(),
                                        "emas" => "gold".to_string(),
                                        "hijau" => "green".to_string(),
                                        "putih" => "white".to_string(),
                                        "abuabu" => "grey".to_string(),
                                        "cokelat" => "chocolate".to_string(),
                                        "tomat" => "tomato".to_string(),
                                        _ => v,
                                    }
                                })),
                                "gambarlatarbelakang" => pohon.push(Pohon::gambarlatarbelakang(
                                    o["nilai"][i]["nilai"].as_str().unwrap().to_string(),
                                )),
                                "judul" => pohon.push(Pohon::judul(
                                    o["nilai"][i]["nilai"].as_str().unwrap().to_string(),
                                )),
                                "tombol" => {
                                    pohon.push(Pohon::tombol(
                                        o["nilai"][i]["nilai"].as_str().unwrap().to_string(),
                                    ))
                                    //println!("{:?}",o["nilai"][i]["nilai"].as_str().unwrap().to_string())
                                }
                                "isi" => pohon.push(Pohon::isi(
                                    o["nilai"][i]["nilai"][0].as_str().unwrap().to_string(),
                                    o["nilai"][i]["nilai"][1].as_str().unwrap().to_string(),
                                )),
                                "warna" => pohon.push(Pohon::warna(
                                    o["nilai"][i]["nilai"][0].as_str().unwrap().to_string(),
                                    o["nilai"][i]["nilai"][1].as_str().unwrap().to_string(),
                                )),
                                "warnalatarbelakangid" => pohon.push(Pohon::warnalatarbelakangid(
                                    o["nilai"][i]["nilai"][0].as_str().unwrap().to_string(),
                                    o["nilai"][i]["nilai"][1].as_str().unwrap().to_string(),
                                )),
                                "ukurankata" => pohon.push(Pohon::ukurankata(
                                    o["nilai"][i]["nilai"][0].as_str().unwrap().to_string(),
                                    o["nilai"][i]["nilai"][1].as_str().unwrap().to_string(),
                                )),
                                "cetak" => {
                                    match o["nilai"][i]["nilai"][0].as_str().unwrap() {
                                        "lansung" => {
                                            pohon.push(Pohon::cetak(Nilai::lansung_str(
                                                o["nilai"][i]["nilai"][1]
                                                    .as_str()
                                                    .unwrap()
                                                    .to_string(),
                                            )));
                                        }
                                        "langsung_int" => {
                                            pohon.push(Pohon::cetak(Nilai::lansung_int(
                                                o["nilai"][i]["nilai"][1]
                                                    .as_str()
                                                    .unwrap()
                                                    .parse::<u64>()
                                                    .unwrap(),
                                            )));
                                        }
                                        "langsung_f" => {
                                            pohon.push(Pohon::cetak(Nilai::lansung_float(
                                                o["nilai"][i]["nilai"][1]
                                                    .as_str()
                                                    .unwrap()
                                                    .parse::<f64>()
                                                    .unwrap(),
                                            )));
                                        }
                                        /*
                                        "var" => {
                                            for x in &_var {
                                                if o["nilai"][i]["nilai"][1].as_str().unwrap()
                                                    == x.nama
                                                {
                                                    pohon.push(Pohon::cetak(Nilai::penujuk(
                                                        Variabel {
                                                            id: x.id,
                                                            /*sementara*/
                                                            nilai: Tipe::_u8(None),
                                                        },
                                                    )));
                                                    break;
                                                }
                                            }
                                        }
                                        */
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        None => break,
                    }
                    i += 1
                }
            }
            None => {}
        }
        if let Some(o) = pohon.last() {
            match o {
                Pohon::jika_tutup => pohon.push(Pohon::blok_),
                _ => {}
            }
        }
        if y == lokasi.len() - 1 {
            break;
        }
        pohon.push(Pohon::fungsi({
            let mut v = String::with_capacity(40);
            v.push_str(lokasi[y + 1][0].as_str());
            v.push('_');
            v.push_str(lokasi[y + 1][1].as_str());
            v
        }))
    }
    kirim
        .send((["".to_string(), "".to_string(), "".to_string()], true))
        .unwrap();
    //#[cfg(debug_assertions)]println!("{:#?}", pohon);
    bincode::serialize_into(
        BufWriter::with_capacity(
            1000,
            File::create(format!("{}/parsing/pohon.bin", path)).unwrap(),
        ),
        &pohon,
    )
    .unwrap();
}
