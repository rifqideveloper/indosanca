use std::sync::mpsc::channel;
extern crate wat;
mod codeart;
mod dok;
mod file_management;
mod konversi;
mod parsing;
mod error;
mod jalankan;
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
lazy_static! {
    static ref BUFF: Mutex<String> = std::sync::Mutex::new(String::with_capacity(100));
    //static ref ARGS: Vec<String> = std::env::args().collect::<Vec<String>>();//
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

fn kopilasi(ARGS:&'static Vec<String>) -> Result<(),u64> {
    /*
        setiap untaiyan(thread) memiliki tugas masing2 dan akan saling
    berkomunikasi melalui kanal dibawah.
    guna ataupun memodifikasi dengan bebas aplikasi in semau anda tanpa batasan
    untuk lebih jelas lihat di license aplikasi di https://github.com/rifqideveloper/indosanca/blob/master/LICENSE
    
    */
    const PROYEK: usize = 2;
    //tahapan kopilasi hanya berlanjut jika _ERROR = false
    let waktu: std::time::Instant = std::time::Instant::now();
    //use std::sync::Arc;
    //let foo = Arc::new(String::with_capacity(100));
    //kode error 1
    #[allow(unused_variables)]
    let (perpus, kompilasi, versi, nama_app, turbo, pola,kom) =
        file_management::seting::seting(&mut BUFF.lock().unwrap(), &PROYEK, &ARGS.to_vec());
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

        let (test1, test2) = channel();
        let (a, b) = channel();
        //let (c,d) = channel();
        let (file_ind, index) = channel();
        let (x, y) = channel();
        let (m, p) = channel();
        let (k, r) = channel();
        static mut POHON: std::vec::Vec<crate::parsing::parse_3::Pohon> = Vec::new();
        unsafe { POHON.reserve(10); }
        
        //let (tunggu_pohon,t) = channel();
        //let (tunggu_pohon_2,t_2) = channel();
        let (kirim_pwa,terima_pwa) = channel();
        //let (Pohon,akar) = std::sync::mpsc::sync_channel(bound: usize);
        //
        //let (mulai,tunggu) = channel();
        //let (mulai2,tunggu2) = channel();
        tread! {join ->
            //kode error 10
            {file_management::kode_2::baca(&mut BUFF.lock().unwrap(),&ARGS[PROYEK], test1)},"test".to_string(),test1,
            //kode error 11
            {
                parsing::lexer_2::baca(test2, a)
                //parsing::lexer::baca_2(test2, a)
            },"test2".to_string(),test2,
            //{ parsing::lexer::baca(&mut BUFF.lock().unwrap(),&ARGS[PROYEK], a) },"lexer".to_string(),lex,
            //kode error 12
            { 
                parsing::parse_1::parse(b , file_ind)
            //    parsing::parse::parse( b , file_ind )
            },"parse".to_string(),parse,
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
                //parsing::parse_3::parse_2(m,r,&ARGS[PROYEK],unsafe{ &mut POHON });
                parsing::parse_3_1::parse(r,kirim_pwa,m,kom);
                println!(
                    "parsing selesai : {}/detik\n]\n",
                    waktu.elapsed().as_secs_f32()
                );
                //#[cfg(debug_assertions)]
                //println!("[pohon]\n{:#?}", unsafe { &POHON });
                /*
                if pola.1 {
                    //kode error 20
                    println!(
                        "[ optimalisasi selesai : {}/detik ]",
                        waktu.elapsed().as_secs_f32()
                    );
                }
                */
                //if kom.0 { tunggu_pohon.send(()).unwrap();}
                //if kom.1 { tunggu_pohon_2.send(()).unwrap();}

                //parsing::parse_3::parse(m,r,&ARGS[PROYEK],unsafe{ &mut POHON })
            },"tulis_parse_3".to_string(),tulis_parse_2,
            //{},"pohon".to_string(),pohon
            {
                if kom.0  {
                    //t.recv().unwrap();
                    println!(
                        "[ was selesai : {}/detik ]",
                        waktu.elapsed().as_secs_f32()
                    );
                }
            },"was_".to_string(),was_,
            {
                if kom.1   {
                    //t_2.recv().unwrap();
                    //let nama_app = nama_app.clone();
                    //konversi::pwa::app(unsafe { &POHON }, &ARGS[PROYEK], nama_app.clone());
                    konversi::pwa_op::konversi(terima_pwa);
                    println!(
                        "[ pwa selesai : {}/detik ]",
                        waktu.elapsed().as_secs_f32()
                    );
                }
            },"pwa_".to_string(),pwa_,
        };
        
    }
    /*
    if pola.2 {
        // versi lama 0.8.1
        #[cfg(debug_assertions)]
        println!("[pohon]\n{:#?}", unsafe { &POHON });
        let mut tread : Vec<std::thread::JoinHandle<(crate::error::ErrorKode,&str)>>  = Vec::with_capacity(1);
        let mut kom = (false, false, false, false);
        kompilasi.iter().for_each(|i| match i.as_str() {
            "wasm" => {
                if !kom.0 {
                    //crate::error::error_konversi("wasm", 1);
                    tread.push(
                        std::thread::Builder::new()
                            .name("wasm".to_string())
                            .spawn(|| {
                                (crate::error::ErrorKode::Error("\tfitur sedang bermasalah\n".to_string()),"wasm")
                                //(konversi::web_2::app_3(unsafe { &POHON }, &ARGS[PROYEK]),"wasm")
                            })
                            .unwrap(),
                    );
                    kom.0 = true
                }
            }
            "win64" => {
                if !kom.1 {
                    kom.1 = true
                }
            }
            "interperetasi" => unsafe {
                if !kom.2 {
                    let nama_app = nama_app.clone();
                    tread.push(
                        std::thread::Builder::new()
                            .name("interperetasi".to_string())
                            .spawn(move || {
                                bincode::serialize_into(
                                    std::io::BufWriter::with_capacity(
                                        1000,
                                        if let Ok(o) = File::create(format!(
                                            "{}/target/inter/{}.bin",
                                            &ARGS[PROYEK], nama_app
                                        )) {
                                            o
                                        } else {
                                            let mut v = format!("{}/target/inter", &ARGS[PROYEK]);
                                            std::fs::create_dir_all(&v).unwrap();
                                            v.push_str("/");
                                            v.push_str(&nama_app);
                                            v.push_str(".bin");
                                            File::create(v).unwrap()
                                        },
                                    ),
                                    &POHON,
                                )
                                .unwrap();
                                (crate::error::ErrorKode::Oke,"interperetasi")
                            })
                            .unwrap(),
                    );
                    kom.2 = true;
                }
            },
            "pwa" => {
                if !kom.3 {
                    let nama_app = nama_app.clone();
                    tread.push(
                        std::thread::Builder::new()
                            .name("PWA kompilasi".to_string())
                            .spawn(move || {
                                (konversi::pwa::app(unsafe { &POHON }, &ARGS[PROYEK], nama_app),"pwa")
                            })
                            .unwrap(),
                    );
                    kom.3 = true;
                }
            }
            "pwa-node" =>{
                let nama_app = nama_app.clone();
                tread.push(
                    std::thread::Builder::new()
                    .name("PWA - node kompilasi".to_string())
                    .spawn(move || {
                        (crate::konversi::pwanode::node(unsafe { &POHON }, &ARGS[PROYEK], nama_app),"PWA - node")
                    }).unwrap()
                )
            }
            _ => {
                panic!()
            }
        });
        tread.into_iter().for_each(|i|match i.join() {
            Ok(ok)=>{
                crate::error::error_konversi(ok.1,ok.0)
            }
            Err(_)=>{

            }
        });
    }
    */
    println!("[ selesai : {}/detik ]", waktu.elapsed().as_secs_f32());
    Ok(())
}
fn main() -> Result<(),u64> {
    /*
    //global seting
    let seting : (bool)= if let Ok(o) = std::fs::read_to_string(".\\seting.bin") {
        bincode::deserialize(&o.as_bytes()).unwrap()
    } else {
        const SET_BAWAAN : (bool) = (true);
        std::fs::write(".\\seting.bin", bincode::serialize(&SET_BAWAAN).unwrap() ).unwrap();
        SET_BAWAAN
    };
    */
    static mut ARGS: Vec<String> = Vec::new();
    unsafe {
        ARGS = std::env::args().collect::<Vec<String>>();
    }
    if unsafe{ ARGS.len() != 1 } {
        kopilasi(unsafe{&ARGS})
    } else {
        crate::codeart::gui()
    }
}
#[allow(unused_imports)]
use std::process::Command;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn manual_test() {
        static mut ARGS: Vec<String> = Vec::new();
        unsafe {
            ARGS = ["".to_string(),"konversi".to_string(),"testing/testing".to_string()].to_vec();
        }
        kopilasi(unsafe{&ARGS}).unwrap();
    }
}
