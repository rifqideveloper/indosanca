use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::io::{ BufReader};
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct seting{
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
   seting : seting,
   dep : dep
}
pub fn seting(proyek:&usize) -> (Vec<String>,Vec<String>, Vec<String>, String, String){
    let args = std::env::args().collect::<Vec<String>>();
    let file = &format!("{}\\seting.toml",args[*proyek]);
    let perpus:Vec<String>= vec!["std".to_string()];
    let mut kompilasi= vec!["wasm".to_string()];
    let mut versi = "0.1.0".to_string();
    let mut nama_app = "app".to_string();
    //let mut pembuat:Vec<String>= vec!["?".to_string()];
    if Path::new(file).exists() {
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
    (args,perpus,kompilasi,versi,nama_app)
}