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
    data:(bool,u64),
    dibaca:[bool;2],
    ditulis:[bool;2],
    rumah:std::vec::Vec<String>,
    nesting:u64
}
#[allow(dead_code)]
impl Var {
    fn boleh_ditulis(&self)-> bool{
        self.ditulis[1]
    }
    fn boleh_dibaca(&self)-> bool{
        self.dibaca[1]
    }
    fn sudah_ditulis(&self)-> bool{
        self.ditulis[0]
    }
    fn sudah_dibaca(&self)-> bool{
        self.dibaca[0]
    }
    fn peringatan(&self)-> bool{
        self.data.0
    }
}



use serde_json::json;
use std::collections::HashMap;
use std::io::BufWriter;
use std::fs::File;
use serde::{Serialize, Deserialize};
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Tipe{
    _u8(u8),
    _String(String)
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Variabel{
    nilai:Tipe,
    id:u64,
    
}
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Nilai {
    lansung(String),
    penujuk(Variabel),
    minta(Variabel)
}
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Pohon{
    cetak(Nilai),
    var(Variabel),
}
pub fn parse(
    kirim:std::sync::mpsc::Sender<([String; 3],bool)>,
    terima:std::sync::mpsc::Receiver<std::option::Option<serde_json::Value>>,
    path:&String
){
    let lokasi :Vec<[String; 3]>= Vec::from([["main".to_string(),"main".to_string(),"".to_string()]]);
    let mut id :u64 = 0;
    let mut _var:Vec<Var>= Vec::with_capacity(2);
    let mut _data_ : HashMap<u64, Data> = HashMap::with_capacity(2);
    let mut bahaya = false;
    let mut alokasi = ([0u64,0u64],false);
    let mut pohon :Vec<Pohon> = Vec::with_capacity(5);
    let mut nesting = 0u64;
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
                                                    t.nilai.parse::<u8>().unwrap()
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
                                        tipe data tidak sesuai",lokasi[0][0],lokasi[0][1]);
                                        std::process::exit(18);
                                    }
                                   
                                }
                                _=>{
                                    println!(">> kode/{}.is/{}\n",lokasi[0][0],lokasi[0][1]);
                                    println!("tipe data tidak tidak diketahui ?");
                                    std::process::exit(18);
                                }
                            },
                            nilai: {
                                let mut t = _data["nilai"][i]["nilai"]["nilai"].to_string();
                                t.remove(0);
                                t.pop();
                                t
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
                                |       ↓↓↓\n\
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