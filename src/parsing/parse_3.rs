
use serde_json::json;
use std::collections::HashMap;
use std::io::BufWriter;
use std::fs::File;
use serde::{Serialize, Deserialize};
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Tipe{
    _u8(Option<u8>),
    _String(String),
}
#[derive(Debug)]
struct Data {
    tipe:String,
    nilai:String
}
#[derive(Debug)]
struct Var {
    tipe:String,
    id:u64,
    nama:String,
}
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Variabel{
    pub nilai:Tipe,
    pub id:u64,
    
}
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Nilai {
    lansung(String),
    lansung_int(u64),
    lansung_float(f64),
    penujuk(Variabel),
    minta(Variabel),
    None
}
/*
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Arit {
    tambah(u64 ,Nilai,Nilai),
    kurang(u64,Nilai,Nilai),
    kali(u64,Nilai,Nilai),
    bagi(u64,Nilai,Nilai),
    modus(u64,Nilai,Nilai),
    sin(u64,Nilai),
    cos(u64,Nilai),
    Tan(u64,Nilai)
}
*/
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Pohon{
    cetak(Nilai),
    var(u64,Tipe),
    tulis(u64,Tipe),
    const_32(i32),
    local_set(i32),
    halaman(String),
    i32_eqz,
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
    r#if,
    if_,
    jika_tutup,
    lalu_jika,
    if_br(String,String),
    br(String),
    main(String)    

}
//
#[derive(rust_embed::RustEmbed)]
#[folder = "std/parsing"]
#[prefix = "prefix/"]
struct STD;
//
pub fn parse_2(
    kirim:std::sync::mpsc::Sender<([String; 3],bool)>,
    terima:std::sync::mpsc::Receiver<std::option::Option<serde_json::Value>>,
    path:&String,
    pohon :&mut Vec<Pohon>,
){
    let mut id :u64 = 0;
    let mut _var:Vec<Var>= Vec::with_capacity(2);
    let mut _data_ : HashMap<u64, Data> = HashMap::with_capacity(2);
    let lokasi :Vec<[String; 3]>= Vec::from([["main".to_string(),"main".to_string(),"".to_string()]]);
    let mut jika :Vec<u8>= Vec::new();
    let mut lalu = false;
    loop {
        kirim.send((lokasi.last().unwrap().clone(),false)).unwrap();
        match terima.recv().unwrap(){
            Some(o)=>{
                for i in 0..{
                    match o["nilai"][i]["tipe"].as_str() {
                        Some(v)=>{
                            match v {
                                "putar"=>{
                                    pohon.push(
                                        Pohon::putar
                                    )
                                }
                                "batas"=>{
                                    pohon.push(
                                        Pohon::batas
                                    )
                                }
                                "lanjut"=>{
                                    pohon.push(
                                        Pohon::lanjut
                                    )
                                }
                                "putus"=>{
                                    pohon.push(
                                        Pohon::putus
                                    )
                                }
                                "blok"=>{
                                    pohon.push(Pohon::blok(o["nilai"][i]["nilai"].as_str().unwrap().to_string()))
                                }
                                "blok_"=>{
                                    pohon.push(
                                        Pohon::blok_
                                    )
                                }
                                "var"=>{
                                    //sementara
                                    _var.push(
                                        Var{
                                            id:id,
                                            tipe:o["nilai"][i]["data"].as_str().unwrap().to_string(),
                                            nama:o["nilai"][i]["nama"].as_str().unwrap().to_string(),
                                        }
                                    );
                                    pohon.push(
                                        Pohon::var(
                                            id,Tipe::_u8(None),
                                        )
                                    );
                                    id += 1;
                                }
                                "tulis"=>{
                                    //belum selesai
                                    let nama = o["nilai"][i]["nama"].as_str().unwrap();
                                    let nilai = &o["nilai"][i]["nilai"];
                                    for i in _var.iter(){
                                        if i.nama == nama {
                                            if nilai[1] == json!(null) {
                                                pohon.push(
                                                    Pohon::tulis(i.id,
                                                        //SEMENTARA
                                                        Tipe::_u8(Some(nilai[0].as_str().unwrap().parse::<u8>().unwrap()))
                                                    )
                                                );
                                                break
                                            }
                                            let mut v : Vec<Pohon> = Vec::with_capacity(3);
                                            let mut ari = (false,false,false,false);
                                            for x in 0..{
                                                if nilai[x] == json!(null) {break}
                                                match nilai[x].as_str().unwrap(){
                                                    "+"=>{
                                                        ari.0 = true;
                                                        /*
                                                        v.push(
                                                            Pohon::add
                                                        );*/
                                                    }
                                                    "-"=>{
                                                        ari.1 = true;
                                                    }
                                                    "/"=>{
                                                        ari.2 = true;
                                                    }
                                                    "*"=>{
                                                        ari.3 = true;
                                                    }
                                                    _=>{
                                                        match nilai[x].as_str().unwrap().parse::<i32>(){
                                                            Ok(a)=>{
                                                                v.push(
                                                                    Pohon::const_32(a)  
                                                                );
                                                                if ari.0 {
                                                                    ari.0 = false;
                                                                    v.push(
                                                                        Pohon::add
                                                                    );
                                                                } else if ari.1 {
                                                                    ari.1 = false;
                                                                    v.push(
                                                                        Pohon::sub
                                                                    );
                                                                } else if ari.2 {
                                                                    match i.tipe.as_str(){
                                                                        "u8"=>{
                                                                            v.push(
                                                                                Pohon::div_u
                                                                            );
                                                                        }
                                                                        "i8"=>{
                                                                            v.push(
                                                                                Pohon::div_s
                                                                            );
                                                                        }
                                                                        _=>{}
                                                                    }
                                                                    ari.2 = false;
                                                                    
                                                                } else if ari.3 {
                                                                    ari.3 = false;
                                                                    v.push(
                                                                        Pohon::mul
                                                                    );
                                                                }
                                                            }
                                                            Err(_)=>{

                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            if !v.is_empty(){
                                                use std::convert::TryInto;
                                                for x in &v{
                                                    match x {
                                                        Pohon::const_32(a)=>{
                                                            pohon.push(
                                                                Pohon::const_32(*a)
                                                            );
                                                        }
                                                        Pohon::add=>{
                                                            pohon.push(
                                                                Pohon::add
                                                            );
                                                        }
                                                        Pohon::sub=>{
                                                            pohon.push(
                                                                Pohon::sub
                                                            );
                                                        }
                                                        Pohon::div_s=>{
                                                            pohon.push(
                                                              Pohon::div_s  
                                                            );
                                                        }
                                                        Pohon::div_u=>{
                                                            pohon.push(
                                                                Pohon::div_u  
                                                            );
                                                        }
                                                        Pohon::mul=>{
                                                            pohon.push(
                                                                Pohon::mul  
                                                            );
                                                        }
                                                        _=>{}
                                                    }
                                                }
                                                pohon.push(
                                                    Pohon::local_set(i.id.try_into().unwrap())
                                                )
                                            }
                                            print!("testing {:?}",v);
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
                                }
                                "br"=>{
                                    pohon.push(
                                        Pohon::br(o["nilai"][i]["nilai"].as_str().unwrap().to_string())
                                    )
                                }
                                "jika"=>{
                                    jika.push(1);
                                    pohon.push(
                                        Pohon::blok("if".to_string())
                                    )

                                }
                                "jika_"=>{
                                    //???
                                    //pohon.push(Pohon::if_br("".to_string(),"$if".to_string())   )
                                    pohon.push(
                                        Pohon::if_br("$then".to_string(),"".to_string())
                                    )
                                }
                                "jika_tutup"=>{
                                    if lalu{
                                        lalu = false;
                                    } else {
                                        pohon.push(
                                            Pohon::jika_tutup
                                        );
                                        
                                    }

                                }
                                "lalu"=>{
                                    //prototy
                                    if let Pohon::jika_tutup = pohon.last().unwrap() {
                                        pohon.pop().unwrap();
                                        
                                        lalu = true;
                                    } else {
                                        panic!()
                                    }
                                }
                                "lalu_jika"=>{
                                    //prototyp
                                        pohon.push(
                                            Pohon::lalu_jika
                                        );
                                }
                                "blok_t_tutup"=>{
                                    pohon.push(
                                        Pohon::blok_
                                    );
                                }
                                "blok_t_buka"=>{
                                    if lalu{
                                        
                                    }else if jika.is_empty() {
                                        pohon.push(
                                            Pohon::blok("test".to_string())
                                        )
                                    } else {
                                        jika.pop();
                                    }
                                }
                                "_i32_eqz"=>{
                                    pohon.push(
                                        Pohon::i32_eqz
                                    )
                                }
                                "_i32_konst"=>{
                                    let v = o["nilai"][i]["nilai"].as_str().unwrap();
                                    let v = format!("\ni32.const {}\n",v);
                                    pohon.push(
                                        Pohon::main(v)
                                    )
                                }
                                "kurang"=>{
                                    pohon.push(
                                        Pohon::sub  
                                    );
                                }
                                "benar"=>{
                                    pohon.push(
                                        Pohon::main("i32.const 0".to_string())   
                                    )
                                }
                                "salah"=>{
                                    pohon.push(
                                        Pohon::main("i32.const 1".to_string())   
                                    )
                                }
                                "if_br"=>{
                                    let (a,b) = (o["nilai"][i]["nilai"][0].as_str().unwrap(),o["nilai"][i]["nilai"][1].as_str().unwrap());
                                    pohon.push(
                                        Pohon::if_br(a.to_string(),b.to_string())
                                    )
                                }
                                "halaman"=>{
                                    pohon.push(
                                        Pohon::halaman(
                                            o["nilai"][i]["nilai"].as_str().unwrap().to_string()
                                        )
                                    )
                                }
                                "cetak"=>{
                                    /* prototipe
                                    let mut x = 0;
                                    loop{
                                        match o["nilai"][i]["nilai"][x].as_str(){
                                            Some(_)=>{
                                                match o["nilai"][i]["nilai"][0].as_str().unwrap() {
                                                    "lansung"=>{
                                                        pohon.push(
                                                            Pohon::cetak(Nilai::lansung(
                                                                o["nilai"][i]["nilai"][x + 1].as_str().unwrap().to_string()
                                                            ))
                                                        );
                                                    }
                                                    _=>{}
                                                }
                                            }
                                            None=>{break}
                                        }
                                        x += 2
                                    }*/
                                    match o["nilai"][i]["nilai"][0].as_str().unwrap(){
                                        "lansung"=>{
                                            pohon.push(
                                                Pohon::cetak(Nilai::lansung(
                                                    o["nilai"][i]["nilai"][1].as_str().unwrap().to_string()
                                                ))
                                            );
                                        }
                                        "langsung_int"=>{
                                            pohon.push(
                                                Pohon::cetak(
                                                    Nilai::lansung_int(
                                                        o["nilai"][i]["nilai"][1].as_str().unwrap().parse::<u64>().unwrap()
                                                    )
                                                )  
                                            );
                                        }
                                        "langsung_f"=>{
                                            pohon.push(
                                                Pohon::cetak(
                                                    Nilai::lansung_float(
                                                        o["nilai"][i]["nilai"][1].as_str().unwrap().parse::<f64>().unwrap()
                                                    )
                                                )  
                                            );
                                        }
                                        "var"=>{
                                            for x in &_var{

                                                if o["nilai"][i]["nilai"][1].as_str().unwrap() == x.nama {
                                                    pohon.push(
                                                        Pohon::cetak(Nilai::penujuk(
                                                            Variabel{
                                                                id:x.id,
                                                                /*sementara*/nilai:Tipe::_u8(None)
                                                            }
                                                        ))
                                                    );
                                                    break
                                                }
                                            }
                                        }
                                        _=>{}
                                    }
                                    
                                }
                                _=>{}
                            }
                        }
                        None=>{break}
                    }                    
                }
            }
            None=>{panic!()}
        }
        break
    }
    kirim.send((["".to_string(),"".to_string(),"".to_string()],true)).unwrap();
    println!("{:#?}",pohon);
    bincode::serialize_into(
        BufWriter::with_capacity(
            1000, 
            File::create(
                format!("{}/parsing/pohon.bin",path)
            ).unwrap()
    ), &pohon).unwrap();
}
/*
pub fn parse(
    kirim:std::sync::mpsc::Sender<([String; 3],bool)>,
    terima:std::sync::mpsc::Receiver<std::option::Option<serde_json::Value>>,
    path:&String,
    pohon :&mut Vec<Pohon>,
){
    let lokasi :Vec<[String; 3]>= Vec::from([["main".to_string(),"main".to_string(),"".to_string()]]);
    let mut id :u64 = 1;
    let mut _var:Vec<Var>= Vec::with_capacity(2);
    let mut _data_ : HashMap<u64, Data> = HashMap::with_capacity(2);
    let mut bahaya = false;
    let mut alokasi = ([0u64,0u64],false);
    let mut nesting = 0u64;
    let mut _null = false;
    // standar library
    //let _std : std::borrow::Cow<[u8]> = STD::get("prefix/parse_2").unwrap() ;
    let _std = serde_json::from_str::<serde_json::Value>( std::str::from_utf8( STD::get("prefix/parse_2").unwrap().as_ref() ).unwrap());
    //println!("{:#?}",_std);
    //
    kirim.send((lokasi[0].clone(),false)).unwrap();
    match terima.recv().expect("msg: &str") {
        Some(_data)=>{
            'main:for i in 0.. {
                if _data["nilai"][i]["tipe"].to_string() == "\"blok\""{
                    nesting += 1 
                } else if _data["nilai"][i]["tipe"].to_string() == "\"cetak\"" {
                    pohon.push(
                        Pohon::cetak(
                            Nilai::lansung(
                                {
                                    if _data["nilai"][i]["nilai"][0] != "lansung"{
                                        panic!();
                                    }
                                    let mut t = _data["nilai"][i]["nilai"][1].to_string();
                                    t.remove(0);
                                    t.pop();
                                    t
                                }
                            )
                        )
                    );
                    //println!("{:#?}",pohon);
                }else if _data["nilai"][i]["tipe"].to_string().starts_with("\"var_") {
                    //print!("test");
                    if _data["nilai"][i]["nilai"] == json!(null){
                        println!("null !!!");
                        let v = Var{
                            id:id,
                            tipe:_data["nilai"][i]["data"].to_string(),
                            nama:_data["nilai"][i]["nama"].to_string(),
                            data:(false,0),
                            dibaca:[true,false],
                            ditulis:[true,false],
                            rumah:lokasi[lokasi.len()-1].to_vec(),
                            nesting:nesting
                        };
                        id+=1;
                        println!("{:?}",v);
                        _var.push(v);
                        _null = true;
                        continue
                    }
                    if _data["nilai"][i]["nilai"]["tipe"] == "minta"{
                        //dalam proses
                        //println!("{} {}",_data["nilai"][i]["nilai"],_var[0].nama);
                        for n in 0.._var.len()  {
                            if _data["nilai"][i]["nilai"]["nilai"].to_string() == _var[n].nama{
                                //println!("ketemu");
                                let (data,dibaca,ditulis) = if _data["nilai"][i]["tipe"] == "var_mut"{
                                    (false,false,true)
                                } else if _data["nilai"][i]["tipe"] == "var_mutex" {
                                    (false,true,true)
                                } else if _data["nilai"][i]["tipe"] == "var_atom" {
                                    (true,true,true)
                                } else {
                                    (false,true,false)
                                };
                                if !_var[n].sudah_dibaca(){
                                    alokasi.0[0] += 1 ;
                                    match _var[n].tipe.as_str(){
                                        "\"u8\""|"\"i8\""=>{
                                            alokasi.0[1] += 8
                                        }
                                        _=>{}
                                    }
                                }
                                _var[n] = Var{
                                    id:_var[n].id,
                                    tipe:_data["nilai"][i]["data"].to_string(),
                                    nama:_data["nilai"][i]["nama"].to_string(),
                                    data:(data,_var[n].data.1),
                                    dibaca:[true,dibaca],
                                    ditulis:[true,ditulis],
                                    rumah:lokasi[lokasi.len()-1].to_vec(),
                                    nesting:nesting
                                };
                                let t = _data_.get(&_var[n].data.1).unwrap();
                                if _var[n].tipe != t.tipe {
                                    panic!()
                                }
                                //belum selesai
                                pohon.push(
                                    Pohon::var(
                                        Variabel{
                                            id:_var[n].id,
                                            nilai: if _var[n].tipe != t.tipe {
                                                panic!()
                                            } else if t.tipe == "\"u8\"" {
                                                Tipe::_u8(
                                                    Some(t.nilai.parse::<u8>().unwrap())
                                                )
                                            } else {
                                                panic!()
                                            },
                                            
                                        }
                                    )
                                );
                                //println!("{:#?}",_var);
                                
                            }
                        }
                        
                    } else {//_data["nilai"][i]["nilai"].to_string() //_data["nilai"][i]["data"].to_string()
                        _data_.insert(id,Data{
                            tipe: match _data["nilai"][i]["data"].as_str() {
                                Some("u8")=>{
                                    if _data["nilai"][i]["nilai"]["tipe"].as_str() == Some("nomer") || _data["nilai"][i]["nilai"]["tipe"].as_str() == _data["nilai"][i]["data"].as_str(){
                                        _data["nilai"][i]["data"].to_string()
                                    } else {
                                        println!("kode/{}.is/{}\n\n\
                                        tipe data tidak sesuai\n\
                                        butuh u8 dapat {:?}",lokasi[0][0],lokasi[0][1],_data["nilai"][i]["data"].as_str());
                                        std::process::exit(18);
                                    }
                                   
                                }
                                _=>{
                                    println!("erro{}",_data);
                                    println!(">> kode/{}.is/{}\ntipe data tidak tidak diketahui ?",lokasi[0][0],lokasi[0][1]);
                                    std::process::exit(18);
                                }
                            },
                            nilai: {
                                let mut t = _data["nilai"][i]["nilai"]["nilai"].to_string();
                                t.remove(0);
                                t.pop();
                                if t == "null"{
                                    println!("null terdeteksi");
                                    std::process::abort()
                                } else {
                                    t
                                }
                            },
                        });
                        id += 1 ;
                        let (data,dibaca,ditulis) = if _data["nilai"][i]["tipe"] == "var_mut"{
                            (false,false,true)
                        } else if _data["nilai"][i]["tipe"] == "var_mutex" {
                            (false,true,true)
                        } else if _data["nilai"][i]["tipe"] == "var_atom" {
                            (true,true,true)
                        } else {
                            (false,true,false)
                        };
                        _var.push(Var{
                            id:id,
                            tipe:_data["nilai"][i]["data"].to_string(),
                            nama:_data["nilai"][i]["nama"].to_string(),
                            data:(data,id-1),
                            dibaca:[false,dibaca],
                            ditulis:[false,ditulis],
                            rumah:lokasi[lokasi.len()-1].to_vec(),
                            nesting:nesting
                        });
                        //sementara
                        //hanya untuk u8
                        pohon.push(
                            Pohon::var(
                                Variabel{
                                    nilai:Tipe::_u8(Some( _data_.get(&(&id-1)).unwrap().nilai.parse::<u8>().unwrap())),
                                    id:id-1 
                                }
                            )
                        );
                        id += 1 ;

                    }
                    
                    //println!("{:?}\n{:?}",_data_,_var);
                    //std::process::exit(0);
                } else if _data["nilai"][i]["tipe"] == "tulis" {
                    
                    let t = _data["nilai"][i]["var"].to_string();
                    //println!("test");
                    for mut _i in &mut _var {
                        if _i.nama == t {
                            if _i.boleh_ditulis() {
                                //print!("{}",_i.ditulis.1);
                                if _i.data.0 {
                                    if !bahaya {
                                        println!("\
                                        kode/{}.is/{}\n\
                                        bahaya terdeteksi\n\
                                        atom harus digunakan dengan hati-hati\n\
                                        bantuan : tambahkan peringatan\n\
                                        contoh :\n\
                                        |→ #[bahaya] \n\
                                        |  let atom <_> _ = _\n\
                                        |",lokasi[0][0],lokasi[0][1]);
                                        std::process::exit(18);
                                    }
                                    bahaya = false;
                                }
                                if let Some(x) =_data_.get_mut(&_i.data.1)   {
                                    if _data["nilai"][i]["nilai"]["tipe"] == "nomer"{
                                        *x = Data{
                                        tipe:x.tipe.to_string(),
                                        nilai:_data["nilai"][i]["nilai"]["nilai"].to_string()
                                        };
                                        if _i.dibaca[0] {
                                            //
                                        }
                                        #[cfg(debug_assertions)]
                                        println!("[_data_,HashMap]\n{:#?}",_data_);
                                    }else {
                                        std::process::exit(18);
                                       
                                    }
                                }
                                _i.ditulis[0] = true;
                                continue 'main
                            }else{
                                
                                println!("kode/{}.is/{}\n\n\
                                varabel {2} tidak boleh berubah\n\
                                bantuan : tambahkan mut dalam variabel {2}\n\
                                contoh  :\n\
                                |\n\
                                |   let mut <_> _ = _\n\
                                |       ↑↑↑\n"
                                ,lokasi[0][0],lokasi[0][1],t);
                                std::process::exit(18);
                                //quit::with_code(18)
                            }
                        }
                    }
                    println!("kode/{}.is/{}\n\n\
                    varabel {} tidak ditemuakan\nmungkin sudah jatuh sebelum ditulis ?"
                    ,lokasi[0][0],lokasi[0][1],t);
                    std::process::exit(18);
                } else if _data[i] == json!(null) {break}
            }
        }
        None=>{}
    }
    #[allow(unreachable_code)]
    kirim.send((["".to_string(),"".to_string(),"".to_string()],true)).unwrap();
    //println!("{:#?}",pohon);
    bincode::serialize_into(
        BufWriter::with_capacity(
            1000, 
            File::create(
                format!("{}/parsing/pohon.bin",path)
            ).unwrap()
        ), &pohon).unwrap();
    println!("[\n(sebelum optimalisasi)\nalokasi = {}\nmemory = {}{}/bit\n",alokasi.0[0],alokasi.0[1],if alokasi.1 { "+" } else { "" });
}
*/