#![feature(with_options)]
#![feature(mutex_unlock)]
#![feature(const_fn_trait_bound)]
//#![feature(let_chains)]
use std::sync::mpsc::channel;
extern crate wat;
mod codeart;
mod dok;
mod error;
mod file_management;
mod jalankan;
mod konversi;
mod parsing;
#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
lazy_static! {
    static ref BUFF: Mutex<String> = std::sync::Mutex::new(String::with_capacity(100));
    //static ref variabel:Mutex<HashMap<String,Vec<Box<crate::parsing::Let_>>>> =  Mutex::new( HashMap::new() );
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

fn kopilasi(ARGS: &'static Vec<String>) -> Result<(), u64> {
    std::env::set_var("RUST_BACKTRACE", "1");
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
    let (perpus, kompilasi, versi, nama_app, turbo, pola, kom) =
        file_management::seting::seting(&mut BUFF.lock().unwrap(), &PROYEK, &ARGS.to_vec());
    if pola.0 {
        let (nama_file, kirim_alamat) = channel();
        let (a, b) = channel();
        //let (c,d) = channel();
        let (file_ind, index) = channel();
        let (x, y) = channel();
        let (m, p) = channel();
        let (k, r) = channel();
        //static mut POHON: std::vec::Vec<crate::parsing::parse_3::Pohon> = Vec::new();
        //unsafe { POHON.reserve(10); }

        let (kirim_pwa, terima_pwa) = channel();
        let (kirim_x86, terima_x86) = channel();
        static mut variabel: crate::parsing::Arrmap<String, Vec<crate::parsing::Let_>> =
            crate::parsing::Arrmap::new();
        //let mut variabel:Mutex<HashMap<String,Vec<Box<crate::parsing::Let_>>>> =  Mutex::new( HashMap::new() );
        tread! {join ->
            //kode error 10
            {
                file_management::kode_3::baca(&ARGS[PROYEK], nama_file)
                //file_management::kode_2::baca(&mut String::with_capacity(100),&ARGS[PROYEK], test1)
            },"kode".to_string(),kode,
            //kode error 11
            {
                parsing::lexer_3::lexer(kirim_alamat, a);
                println!(
                    "[ lexer   : {}/detik ]",
                    waktu.elapsed().as_secs_f32()
                );
                //parsing::lexer_2::baca(test2, a)
                //parsing::lexer::baca_2(test2, a)
            },"lexer".to_string(),lexer,
            //{ parsing::lexer::baca(&mut BUFF.lock().unwrap(),&ARGS[PROYEK], a) },"lexer".to_string(),lex,
            //kode error 12
            {
                parsing::parse_1::parse(b , file_ind);
                println!(
                    "[ parse 1 : {}/detik ]",
                    waktu.elapsed().as_secs_f32()
                );
            //    parsing::parse::parse( b , file_ind )
            },"parse".to_string(),parse,
            //kode error 13
            //{file_management::parse::tulis(mulai,&ARGS[PROYEK],"parse",d)},"parse_f".to_string(),parse_f,
            //kode error 14
            //{file_management::baca::file(tunggu,&mut BUFF.lock().unwrap(),file_ind,format!("{}\\parsing\\parse",&ARGS[PROYEK]))},"parse_f_1".to_string(),parse_f_1,
            //kode error 15
            {
                parsing::parse_2::parse(index,x);
                println!(
                    "[ parse 2 : {}/detik ]",
                    waktu.elapsed().as_secs_f32()
                );
            },"parse_2_".to_string(),parse_2_,
            //kode error 16
            //kode error 17
            {
                file_management::index_2::baca(&mut BUFF.lock().unwrap(), y, &ARGS[PROYEK], p, k,turbo);
                println!(
                    "[ index : {}/detik ]",
                    waktu.elapsed().as_secs_f32()
                );
            },"inx2".to_string(),inx2,
             //kode error 18
            {
                //parsing::parse_3::parse_2(m,r,&ARGS[PROYEK],unsafe{ &mut POHON });
                if 
                    parsing::parse_3_2::parse(
                    unsafe{&mut variabel},
                    r,kirim_pwa,m,kom)
                {
                    println!(
                        "[ parsing selesai : {}/detik ]",
                        waktu.elapsed().as_secs_f32()
                    );
                } else {
                    println!(
                        "[ parsing gagal : {}/detik ]",
                        waktu.elapsed().as_secs_f32()
                    );
                }

                
            },"tulis_parse_3".to_string(),tulis_parse_2,
            //{},"pohon".to_string(),pohon
            {
                if kom.0  {
                    //t.recv().unwrap();
                    crate::konversi::x86_64::x86_64(terima_x86,&ARGS[PROYEK]);
                    println!(
                        "[ x86_64 selesai : {}/detik ]",
                        waktu.elapsed().as_secs_f32()
                    );
                }
            },"was_".to_string(),was_,
            {
                    //t_2.recv().unwrap();
                    //let nama_app = nama_app.clone();
                   // konversi::pwa_2::app(unsafe { &POHON }, &ARGS[PROYEK], nama_app.clone());
                    if !kom.1 {} else
                    if konversi::pwa_op_2::konversi(unsafe{&variabel},terima_pwa,&ARGS[PROYEK]) {
                        println!(
                            "[ pwa selesai : {}/detik ]",
                            waktu.elapsed().as_secs_f32()
                        );
                    } else {
                        println!(
                            "[ pwa gagal : {}/detik ]",
                            waktu.elapsed().as_secs_f32()
                        );
                    }
            },"pwa_".to_string(),pwa_,
        };
    }
    println!("[ selesai : {}/detik ]", waktu.elapsed().as_secs_f32());
    Ok(())
}
fn main() -> Result<(), u64> {
    static mut ARGS: Vec<String> = Vec::new();
    unsafe {
        ARGS = std::env::args().collect::<Vec<String>>();
    }
    if unsafe { ARGS.len() != 1 } {
        kopilasi(unsafe { &ARGS })
    } else {
        //crate::codeart::gui()
        Ok(())
    }
}
#[allow(unused_imports)]
use std::process::Command;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn halo_dunia() {
        static mut ARGS: Vec<String> = Vec::new();
        unsafe {
            ARGS = [
                "".to_string(),
                "konversi".to_string(),
                "testing/halo_dunia".to_string(),
            ]
            .to_vec();
        }
        kopilasi(unsafe { &ARGS }).unwrap();
    }
    #[test]
    fn buat_variabel() {
        static mut ARGS: Vec<String> = Vec::new();
        unsafe {
            ARGS = [
                "".to_string(),
                "konversi".to_string(),
                "testing/buat_variabel".to_string(),
            ]
            .to_vec();
        }
        kopilasi(unsafe { &ARGS }).unwrap();
    }
    #[test]
    fn manual_test() {
        static mut ARGS: Vec<String> = Vec::new();
        unsafe {
            ARGS = [
                "".to_string(),
                "konversi".to_string(),
                "testing/testing".to_string(),
            ]
            .to_vec();
        }
        kopilasi(unsafe { &ARGS }).unwrap();
    }
}
