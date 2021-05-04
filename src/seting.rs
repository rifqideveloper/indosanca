use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::io::{ BufReader};
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Seting{
    nama_app :String,
    versi :String,
    pembuat :Vec<String>,
    kompilasi:Vec<String>
}
#[derive(Deserialize)]
struct dep{
    
}
#[derive(Deserialize)]
struct Config {
   seting : Seting,
   dep : dep
}

fn bangun_proyek(f:& String){
    std::fs::create_dir_all(format!("{}\\kode",f)).expect("tidak dapat membuat target direktori (kode)");
    std::fs::create_dir_all(format!("{}\\parsing",f)).expect("tidak dapat membuat target direktori (parsing)");
    std::fs::create_dir_all(format!("{}\\aset",f)).expect("tidak dapat membuat target direktori (aset)");
    std::fs::create_dir_all(format!("{}\\target",f)).expect("tidak dapat membuat target direktori (target)");
    File::create(format!("{}\\kode\\main.is",f)).expect("")
        .write_all(b"cpu main\n\tcetak \"hello dunia\"").expect("");
    File::create(format!("{}\\.gitignore",f)).expect("")
        .write_all(b"/target\n/parsing").expect("");
    File::create(format!("{}\\seting.toml",f)).expect("")
        .write_all(b"\
    [seting]\
    \nnama_app = \"app\"\
    \nversi = \"0.1.0\"\
    \npembuat = [\"?\"]\
    \nkompilasi = [\"wasm\"]\
    \n[dep]\
    ").expect("");
    println!("[bangun proyek]");
} 

pub fn seting(proyek:&usize ,args:&Vec<String>) -> (Vec<String>, Vec<String>, String, String ,(bool,bool,bool)){
    let pola = match args[*proyek - 1].as_str() {
        "parsing" => (true,false,false),
        "proyek" => {
            bangun_proyek(&args[*proyek]);
            (true,true,true)
        },
        "bangun" => (false,true,true),
        "instan" => (false,false,true),
        "optimal" => (false,true,false),
        _ => (true,true,true),
    };
    //let mut pembuat:Vec<String>= vec!["?".to_string()];
    let file = format!("{}\\seting.toml",args[*proyek]);
    let perpus:Vec<String>= vec!["std".to_string()];
    let mut kompilasi= vec!["wasm".to_string()];
    let mut versi = "0.1.0".to_string();
    let mut nama_app = "app".to_string();
    if Path::new(&file).exists() {
        let mut f = BufReader::with_capacity(10, File::open(file).expect(""));
        let mut buf = String::with_capacity(10);
        f.read_to_string(&mut buf).expect("");
        
        let config: Config = toml::from_str(buf.as_str()).expect("erro seting");
        
        nama_app = config.seting.nama_app;
        kompilasi = config.seting.kompilasi;
        versi = config.seting.versi;
        //pembuat = config.seting.pembuat;
        
        //(seting["version"].as_str(), Some("0.1.0"));
        //sementara
    } 
    (perpus,kompilasi,versi,nama_app,pola)
}