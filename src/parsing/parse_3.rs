#[derive(Debug)]
struct Data<T> {
    tipe:String,
    nilai:T
}
#[derive(Debug)]
struct Var {
    tipe:String,
    id:u64,
    nama:String,
    data:(bool,u64),
    dibaca:(bool,bool),
    ditulis:(bool,bool),
    rumah:String,
}
use serde_json::json;
use std::collections::HashMap;
pub fn parse(
    kirim:std::sync::mpsc::Sender<std::vec::Vec<&str>>,
    terima:std::sync::mpsc::Receiver<std::option::Option<serde_json::Value>>
){
    kirim.send(["main","main"].to_vec()).expect("msg: &str");
    let mut id :u64 = 0;
    let mut _var:Vec<Var>= Vec::new();
    let mut _data_= HashMap::new();
    let mut bahaya = false;
    match terima.recv().expect("msg: &str") {
        Some(_data)=>{
            let mut i = 0;
            'main:loop{
                if _data["nilai"][i]["tipe"].to_string().starts_with("\"var_") {
                    //print!("test");
                    _data_.insert(id,Data{
                        tipe:_data["nilai"][i]["data"].to_string(),
                        nilai:_data["nilai"][i]["nilai"].to_string(),
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
                        dibaca:(false,dibaca),
                        ditulis:(false,ditulis),
                        rumah:"main:main".to_string(),
                    });
                    id += 1 ;
                    //println!("{:?}\n{:?}",_data_,_var);
                    //std::process::exit(0);
                } else if _data["nilai"][i]["tipe"] == "tulis" {
                    let t = _data["nilai"][i]["var"].to_string();
                    //println!("test");
                    for mut _i in &mut _var {
                        if _i.nama == t {
                            if _i.ditulis.1 {
                                //print!("{}",_i.ditulis.1);
                                if _i.data.0 {
                                    if !bahaya {
                                        println!("bahaya terdeteksi");
                                        println!("atom harus digunakan dengan hati-hati");
                                        println!("bantuan : tambahkan peringatan");
                                        println!("|→ #!bahaya ");
                                        println!("|  let atom <_> _ = _");
                                        std::process::exit(18);
                                    }
                                    bahaya = false;
                                }
                                //println!("test");
                                if let Some(x) =_data_.get_mut(&_i.data.1)   {
                                    *x = Data{
                                        tipe:x.tipe.to_string(),
                                        nilai:_data["nilai"][i]["nilai"].to_string()
                                    };
                                    if _i.dibaca.0 {
                                        //
                                    }
                                    #[cfg(debug_assertions)]
                                    println!("[_data_,HashMap]\n{:#?}",_data_);
                                }
                                _i.ditulis.0 = true;
                                i += 1;
                                continue 'main
                            }else{
                                println!("varabel {} tidak boleh berubah",t);
                                println!("bantuan : tambahkan mut dalam variabel {}",t);
                                println!("contoh  :    ↓\n\tlet mut _ <_> = _");
                                println!("             ↑");
                                std::process::exit(18);
                            }
                        }
                    }
                    println!("varabel {} tidak ditemuakan",t);
                    std::process::exit(18);
                } else if _data[i] == json!(null) {break}
                i += 1
            }
        }
        None=>{}
    }
    #[allow(unreachable_code)]
    kirim.send([].to_vec()).expect("msg: &str");
}