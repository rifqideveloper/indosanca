use std::sync::mpsc::channel;
//use std::fs::File;
//use std::io::prelude::*;
extern crate wat;
mod file_management;
mod parsing;
mod konversi;
mod codeart;
mod dok;
use std::fs::File;
#[macro_use]
extern crate lazy_static;

use std::sync::{Mutex/*,Arc, Barrier*/};
lazy_static! {
    static ref BUFF: Mutex<String> = std::sync::Mutex::new(String::with_capacity(100));
    static ref ARGS: Vec<String> = std::env::args().collect::<Vec<String>>();//
    //static ref BUF: String = String::with_capacity(100);
}
macro_rules! tread {
    ($a:block, $b:expr,$( $x:block , $y:expr),+) => {
        $(
            std::thread::Builder::new().name($y).spawn(move || $x ).expect("");
        )*
        std::thread::Builder::new().name($b).spawn(move || $a ).expect("")
            .join().expect("");
    };
    (join ->  $( $x:block , $y:expr,$i:ident),+ $(,)*) => {
        // prototype
        $(
            let $i = std::thread::Builder::new().name($y).spawn(move || $x ).expect("");
        )*
        $(
            $i.join().expect("");
        )*
    };
    ($x:block, $y:expr)=>{
        std::thread::Builder::new().name($y).spawn(move || $x).expect("")
            .join().expect("");
    }
}

fn kopilasi() {
    /*
        setiap untaiyan(thread) memiliki tugas masing2 dan akan saling
    berkomunikasi melalui kanal dibawah.
    guna ataupun memodifikasi dengan bebas aplikasi in semau anda tanpa batasan 
    untuk lebih jelas lihat dilicense aplikasi di https://github.com/rifqideveloper/indosanca/blob/master/LICENSE
    */
    const PROYEK:usize = 2;
    
    //tahapan kopilasi hanya berlanjut jika _ERROR = false
    let waktu : std::time::Instant = std::time::Instant::now();
    //use std::sync::Arc;
    //let foo = Arc::new(String::with_capacity(100));
    //kode error 1
    #[allow(unused_variables)]
    let (perpus,kompilasi,versi,nama_app,turbo,pola) = file_management::seting::seting(&mut BUFF.lock().unwrap(),&PROYEK,&ARGS.to_vec()) ;
    static mut POHON :std::vec::Vec<crate::parsing::parse_3::Pohon> = Vec::new();
    if pola.0 {
        /*
        **versi lama 0.1.0**
        let (kirim_lex,lex) = channel();
        let (ke_lex_f,lex_f) = channel();
        let (lanjut_token,tunggu_lexer) = channel();
        let (kirim_ke_parser,token) = channel();
        let (ke_pohon,terima_pohon) = channel();
        //let proyek = args[PROYEK].clone();
        std::thread::Builder::new().name("con_read".to_string()).spawn(move || 
            con_read::baca(ARGS[PROYEK].to_string(), kirim_lex)
        ).expect("");
        std::thread::Builder::new().name("lexer".to_string()).spawn(move || {
            lexer::lexer( lex, ke_lex_f);
            //lexerjson::lexer(lex, ke_lex_f);
        }).expect("");
        std::thread::Builder::new().name("con_write".to_string()).spawn(move || {
            con_write::tuliskan(lex_f, ARGS[PROYEK].to_string(), lanjut_token)
        }).expect("");
        std::thread::Builder::new().name("token_read".to_string()).spawn(move || {
            token_read::baca(ARGS[PROYEK].to_string(), kirim_ke_parser , tunggu_lexer)
        }).expect("");
        std::thread::Builder::new().name("parse".to_string()).spawn(move || {
            parser::parser(token, ke_pohon)
        }).expect("");
        std::thread::Builder::new().name("pohon".to_string()).spawn(move || {
            pohon::tulis(ARGS[PROYEK].to_string(), terima_pohon)
        }).expect("").join().expect("");
        */
        //gabung_kode::kode(&ARGS[PROYEK]);
        
        //file_management::kode::baca(&ARGS[PROYEK]);
        let (test1,test2) = channel();
        let (a,b) = channel();
        //let (c,d) = channel();
        let (file_ind,index)= channel();
        let (x,y) = channel();
        let (m,p) = channel();
        let (k,r) = channel();
        //
        //let (mulai,tunggu) = channel();
        //let (mulai2,tunggu2) = channel();
        tread!{join ->
            //kode error 10
            {file_management::kode_2::baca(&mut BUFF.lock().unwrap(),&ARGS[PROYEK], test1)},"test".to_string(),test1,
            //kode error 11
            {parsing::lexer::baca_2(test2, a)},"test2".to_string(),test2,
            //{ parsing::lexer::baca(&mut BUFF.lock().unwrap(),&ARGS[PROYEK], a) },"lexer".to_string(),lex,
            //kode error 12
            { parsing::parse::parse( b , file_ind )},"parse".to_string(),parse,
            //kode error 13
            //{file_management::parse::tulis(mulai,&ARGS[PROYEK],"parse",d)},"parse_f".to_string(),parse_f,
            //kode error 14
            //{file_management::baca::file(tunggu,&mut BUFF.lock().unwrap(),file_ind,format!("{}\\parsing\\parse",&ARGS[PROYEK]))},"parse_f_1".to_string(),parse_f_1,
            //kode error 15
            {parsing::parse_2::parse(index,x)},"parse_2_".to_string(),parse_2_,
            //kode error 16
            //kode error 17
            {file_management::index_2::baca(&mut BUFF.lock().unwrap(), y, &ARGS[PROYEK], p, k,turbo)},"inx2".to_string(),inx2,
             //kode error 18
            {
                parsing::parse_3::parse_2(m,r,&ARGS[PROYEK],unsafe{ &mut POHON })
                //parsing::parse_3::parse(m,r,&ARGS[PROYEK],unsafe{ &mut POHON })
            },"tulis_parse_3".to_string(),tulis_parse_2,
            //{},"pohon".to_string(),pohon
        };
        //file_management::hapus_baris::baris(&ARGS[PROYEK], "parse", 0);
        println!("parsing selesai : {}/detik\n]", waktu.elapsed().as_secs_f32());
    }
    if pola.1 {
        //kode error 20
        println!("[optimalisasi selesai : {}/detik]", waktu.elapsed().as_secs_f32());
    }
    if pola.2 {
        #[cfg(debug_assertions)]
        println!("[pohon]\n{:#?}",unsafe{ &POHON });
        let mut tread = Vec::new();
        let mut kom  = (false,false,false);
        kompilasi.iter().for_each(|i| 
            match i.as_str() {
                "wasm" => {
                    if !kom.0 {
                        tread.push(
                            std::thread::Builder::new().name("wasm".to_string()).spawn( || {
                                konversi::web_2::app_3(unsafe{ &POHON } ,&ARGS[PROYEK]);
                            }).unwrap()
                        ); 
                        kom.0 = true
                    }
                }
                "win64" => {
                    if !kom.1 {
                        kom.1 = true
                    }
                }
                "interperetasi" =>{
                    unsafe {
                        if !kom.2 {
                            let nama_app = nama_app.clone();
                            tread.push(
                                std::thread::Builder::new().name("interperetasi".to_string()).spawn(move || {
                                    bincode::serialize_into(
                                        std::io::BufWriter::with_capacity( 1000 , 
                                            if let Ok(o) = File::create(format!("{}/target/inter/{}.bin",&ARGS[PROYEK],nama_app) ) {
                                                o
                                            } else {
                                                let mut v = format!("{}/target/inter",&ARGS[PROYEK]) ;
                                                std::fs::create_dir_all(&v).unwrap();
                                                v.push_str("/");
                                                v.push_str(&nama_app);
                                                v.push_str(".bin");
                                                File::create(v).unwrap()
                                            }
                                        ), 
                                        &POHON
                                    ).unwrap();
                                }).unwrap()
                            );
                            kom.2 = true;
                        }
                    }
                }
                _=>{
                    panic!()
                }
            }
        );
        tread.into_iter().for_each(|i| i.join().unwrap() );
        /*
        unsafe {
            if !pola.0 || !pola.1 {
                POHON = bincode::deserialize_from(
                    std::io::BufReader::new(File::open(format!("{}/parsing/pohon.bin",&ARGS[PROYEK])).unwrap())
                ).unwrap();
            }
        }
        *//*
        tread!(join -> 
            {
                if kom.0 {
                    //konversi::wasm::wat(&ARGS[PROYEK]);
                    //konversi::web::app( unsafe{ &POHON } ,&ARGS[PROYEK] );
                    //konversi::web_2::app(unsafe{ &POHON } ,&ARGS[PROYEK]);
                    konversi::web_2::app_3(unsafe{ &POHON } ,&ARGS[PROYEK]);
                    println!("[konversi/wasm selesai : {}/detik]", waktu.elapsed().as_secs_f32());
                }
            },"wasm".to_string(),was,
            {
                if kom.1 {
                    std::fs::create_dir_all(format!("{}\\target\\win64",&ARGS[PROYEK])).expect("tidak dapat membuat target direktori (target)");
                    //kode error 40
                    println!("[konversi/win64 selesai : {}/detik]", waktu.elapsed().as_secs_f32());
                }
            },"win64".to_string(),win64
        );
        */
        //println!("[konversi selesai : {}/detik]", waktu.elapsed().as_secs_f32());
    }
    println!("[ selesai : {}/detik ]", waktu.elapsed().as_secs_f32());
   
}
fn main(){
    if ARGS.len() != 1 {
        kopilasi()
    } else {
        codeart::ide()
    }
    /*
    match ARGS.len(){
        3 => kopilasi(),
        2..4=>{
            match ARGS[1].as_str(){
                "bantuan"=> dok::bantuan(),
                
                _=>{
                    println!("command line argumen tidak sesuai");
                    std::process::exit(1);
                }
            }
        }
        1 => codeart::ide(),
        _ =>{
            println!("command line argumen tidak sesuai");
            std::process::exit(1);
        }
    }
    */
}
#[allow(unused_imports)]
use std::process::Command;
#[cfg(test)]
mod tests {
    use super::*;
    use Command;
    #[test]
    #[ignore]
    fn parsing_() {
        if !Command::new("./target/debug/indosanca.exe")
        .args(&["parsing", "testing/testing"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success() {
            panic!("");
        }
    }
    #[test]
    #[ignore]
    fn proyek() {// belum siap // cetak belum selesai
        if !Command::new("./target/debug/indosanca.exe")
        .args(&["proyek", "testing/proyek"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success() {
            panic!("");
        }
    }
    #[test]
    //#[ignore]
    fn token_slice(){
        let _t = "hallo ".to_string();
        let _x = 6;


        println!("{}",&_t[5..])
    }
    #[test]
    fn parsing_dekralasi() {
        if !Command::new("./target/debug/indosanca.exe")
        .args(&["parsing", "testing/var_deklarasi"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success() {
            panic!("");
        }
    }
    #[test]
    #[should_panic]
    fn parsing_tulis_ulang_gagal() {
        if Command::new("./target/debug/indosanca.exe")
        .args(&["parsing", "testing/var_tulis_ulang_gagal"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .code()
        .unwrap() == 18 {
            panic!("");
        }
    }
    #[test]
    fn parsing_tulis_ulang_mut() {
        if !Command::new("./target/debug/indosanca.exe")
        .args(&["parsing", "testing/var_tulis_ulang_mut"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success() {
            panic!("");
        }
    }
    #[test]
    fn parsing_kepemilikan_1() {
        if !Command::new("./target/debug/indosanca.exe")
        .args(&["parsing", "testing/parsing_kepemilikan_1"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success() {
            panic!("");
        }
    }
    #[test]
    #[should_panic]
    #[ignore]
    fn parsing_salah_tipe() {//belum siap
        if Command::new("./target/debug/indosanca.exe")
        .args(&["parsing", "testing/parsing_salah_tipe"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success() {
            panic!("");
        }
    }
    
}