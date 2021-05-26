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
    kompilasi:Vec<String>,
    turbo:bool,
}
#[derive(Deserialize)]
struct dep{
    
}
#[derive(Deserialize)]
struct Config {
   seting : Seting,
   dep : dep
}
fn buat_seting_file(f:& String){
    match File::create(format!("{}\\seting.toml",f)) {
        Ok(mut o)=>{
            if let Err(_) = o.write_all(b"\
            [seting]\
            \nnama_app = \"app\"\
            \nversi = \"0.1.0\"\
            \npembuat = [\"?\"]\
            \nkompilasi = [\"wasm\"]\
            \nturbo = false\
            \n[dep]\
            ") {
                println!("'{0}\\seting.toml' dapat dibuat tapi tidak dapat ditulis ?\ninfo : jika file '{0}' tidak ada maka program akan membuatkan file",f);
                std::process::exit(1);
            }
        }
        Err(_)=>{
            println!("'{0}\\seting.toml' tidak dapat dibuat ?\ninfo : jika file '{0}' tidak ada maka program akan membuatkan file",f);
            std::process::exit(1);
        }
    }
    

    
    
}
use futures::future;
async fn buat_dir(path:String){
    if let Err(_) = std::fs::create_dir_all(&path) {
        println!("tidak dapat membuat target direktori '{0}\\kode' ?",path);
        std::process::exit(1);
    }
}

async fn bangun_proyek(f:& String){
    future::join4(
        buat_dir(format!("{}\\kode",f)), 
        buat_dir(format!("{}\\parsing",f)), 
        buat_dir(format!("{}\\aset",f)), 
        buat_dir(format!("{}\\target",f))
    ).await;
    match File::create(format!("{}\\kode\\main.is",f)) {
        Ok(mut o)=>{
            if let Err(_) = o.write_all(b"cpu main\n\tcetak \"hello dunia\"") {
                println!("'{0}\\kode\\main.is' dapat dibuat tapi tidak dapat ditulis ?",f);
                std::process::exit(1);
            }
        }
        Err(_)=>{
            println!("'{0}\\kode\\main.is' tidak dapat dibuat ?",f);
            std::process::exit(1);
        }
    }
    match File::create(format!("{}\\.gitignore",f)) {
        Ok(mut o)=>{
            if let Err(_) = o.write_all(b"/target\n/parsing") {
                println!("'{0}\\.gitignore' dapat dibuat tapi tidak dapat ditulis ?",f);
                std::process::exit(1);
            }
        }
        Err(_)=>{
            println!("'{0}\\.gitignore' tidak dapat dibuat ?",f);
            std::process::exit(1);
        }
    }
    buat_seting_file(f);
    println!("[bangun proyek]");
} 
fn bantuan(){
    let (input,mut line) = (std::io::stdin(),String::with_capacity(10));
    loop{
        println!("{}[2J", 27 as char);
        println!("  .*.  😀apa yang saya bisa bantu ?");
        println!("  :::   ketik 'keluar' untuk keluar");
        println!("   :::   📂ketik 'dokument' untuk membuka dokumentasi");
        println!("    :::   🏃🏼‍♀️🏋🏼‍♀️🚴🏼‍♀️ketik 'latihan' untuk memulai latihan");
        println!("   :::   ketik ");
        println!("  :::   ketik ");
        println!(" :::   ketik ");
        println!("  ::   ketik ");
        println!("  /   😊ketik 'siapa anda' kamu ingin tahu siapa aku");
        input.read_line(&mut line).unwrap();
        match line.trim_end() {
            "keluar"=>{break}
            "siapa anda"=>{
                println!("sebelum saya menjawab sebutkan nama mu dulu");
                let mut nama = String::new();
                loop{
                    input.read_line(&mut nama).unwrap();
                    nama = nama.trim_end().to_string();
                    if !nama.is_empty() {break} else {
                        println!("jawab dulu pertanyaan saya");
                    }
                }
                if nama.len() > 3 {println!("{} nama pendek sekali tapi bagus",nama)}
                else if nama.len() < 20 {println!("{} nama panjang sekali tapi bagus",nama)}
                else {println!("{} nama yang bagus",nama)}
                println!("ok nama saya indosanca dan saya adalah kompiler👩‍💻👨‍💻");
                println!("saya berasal dari indonesia 🇮🇩");
                println!("apa anda tahu kompiler itu apa ? (ketik 'y' untuk iya dan 't' untuk tidak)");
                loop{
                    line.clear();
                    input.read_line(&mut line).unwrap();
                    match line.trim_end(){
                        "y"=>{println!("bagus 💯👍 pintar sekali");break}
                        "t"=>{
                            println!("kompiler/Kompilator (Inggris: compiler) adalah sebuah program komputer yang berguna untuk menerjemahkan program komputer yang ditulis dalam bahasa pemrograman tertentu menjadi program yang ditulis dalam bahasa pemrograman lain.Terlepas dari pengertiannya yang demikian relatif luas, istilah kompilator biasa digunakan untuk program komputer yang menerjemahkan program yang ditulis dalam bahasa pemrograman tingkat tinggi (semacam bahasa Pascal, C++, BASIC, FORTRAN, Visual Basic, Visual C#, Java, xBase, atau COBOL) menjadi bahasa mesin, biasanya dengan bahasa Assembly sebagai perantara. ");
                            println!("sumber Dari Wikipedia bahasa Indonesia, ensiklopedia bebas https://id.wikipedia.org/wiki/Kompilator");
                            println!("\nuntuk saya menerjemahkan ke web(js/html/wasm) dan assembli");
                            break
                        }
                        _=>{println!("jawab yang benar");}
                    }
                }
                input.read_line(&mut line).unwrap();
            }
            _=>{}
        }
        line.clear()
    }
    
}
pub fn seting(buf:&mut String,proyek:&usize ,args:&Vec<String>) -> (Vec<String>, Vec<String>, String, String,bool ,(bool,bool,bool)){
    let pola = match args[*proyek - 1].as_str() {
        "parsing" => (true,false,false),
        "proyek" => {
            futures::executor::block_on(bangun_proyek(&args[*proyek]));
            (true,true,true)
        },
        "bangun" => (false,true,true),
        "instan" => (false,false,true),
        "optimal" => (false,true,false),
        "bantuan"=>{
            bantuan();
            std::process::exit(0);
        }
        _ => (true,true,true),
    };
    //let mut pembuat:Vec<String>= vec!["?".to_string()];
    let file = format!("{}\\seting.toml",args[*proyek]);
    let perpus:Vec<String>= vec!["std".to_string()];
    if !Path::new(&file).exists() {buat_seting_file(&args[*proyek])}
        match BufReader::with_capacity(10, 
            match File::open(&file) {
            Ok(o)=>o,
            _=>{
                println!("'{0}' ada tapi tidak dapat dibuka ?\ninfo : jika file '{0}' tidak ada maka program akan membuatkan file",file);
                std::process::exit(1);
            }
        }).read_to_string(buf){
            Ok(_)=>{}
            Err(_)=>{
                println!("'{}' ada tapi tidak dapat dibaca ?",file);
                std::process::exit(1);
            }
        }
        let config: Config = match toml::from_str(buf.as_str()){
            Ok(o)=>{o}
            Err(_)=>{
                println!("'{0}' tidak terformat dengan baik\nbantuan : perbaiki '{0}' atau hapus '{0}'\ninfo : jika file '{0}' tidak ada maka program akan membuatkan file",file);
                std::process::exit(1);
            }
        };
        let nama_app = config.seting.nama_app;
        let kompilasi = config.seting.kompilasi;
        let versi = config.seting.versi;
        let turbo = config.seting.turbo;
        buf.clear();
        //unsafe
        
        //pembuat = config.seting.pembuat;
        
        //(seting["version"].as_str(), Some("0.1.0"));
        //sementara
    
    (perpus,kompilasi,versi,nama_app,turbo,pola)
}