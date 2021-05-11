use std::sync::mpsc::channel;
//use std::fs::File;
//use std::io::prelude::*;
extern crate wat;
mod file_management;
mod parsing;
mod konversi;
#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref ARGS: Vec<String> = std::env::args().collect::<Vec<String>>();//<-- ubah proyek angka ke 1 pada versi akhir
}
macro_rules! tread {
    ($a:block, $b:expr,$( $x:block , $y:expr),+) => {
        $(
            std::thread::Builder::new().name($y).spawn(move || $x ).expect("");
        )*
        std::thread::Builder::new().name($b).spawn(move || $a ).expect("")
            .join().expect("");
    };
    (join ->  $( $x:block , $y:expr,$i:ident),+ ) => {
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
fn main(){
    /*
    setiap untaiyan(thread) memiliki tugas masing2 dan akan saling
    berkomunikasi melalui kanal dibawah
                            !!peringatan!!
    aplikasi kompilasi ini tidak mengecek kesalaham hanya menkopilasi kode ketarget yang diinginkan
    pemeriksaan kesalahan diserahkan sepenuhnya pada ide atau pengguna 
    saya tiadak bertangung jawab pada pun kesalahan penguna
    guna ataupun memodifikasi dengan bebas aplikasi in semau anda tanpa batasan 
    untuk lebih jelas lihat dilicense aplikasi di https://github.com/rifqideveloper/indosanca/blob/master/LICENSE
    */
    const PROYEK:usize = 2;
    let waktu : std::time::Instant = std::time::Instant::now();
    let (perpus,kompilasi,versi,nama_app,pola) = file_management::seting::seting(&PROYEK,&ARGS.to_vec());
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
        file_management::kode::baca(&ARGS[PROYEK]);
        let (a,b) = channel();
        let (c,d) = channel();
        let (lanjut_parse_2,mulai_parse_2) = channel();
        tread!{join ->
            {parsing::lexer::baca(&ARGS[PROYEK], a)},"lexer".to_string(),lex,
            {parsing::parse::parse(nama_app, b , c ,lanjut_parse_2)},"parse".to_string(),parse,
            {
                
                //let (indexing,indx)= channel();
                let (file_ind,index)= channel();
                let (x,y) = channel();
                mulai_parse_2.recv().expect("menunggu parse pertama");
                tread!{join -> 
                    //{file_management::index::baca(indx,file_ind)},"index".to_string(),indext,
                    {file_management::baca::file(file_ind,format!("{}\\parsing\\parse",&ARGS[PROYEK]))},"parse_f_1".to_string(),parse_f_1,
                    {parsing::parse_2::baca(index,x)},"parse_2_".to_string(),parse_2_,
                    {
                        file_management::tulis_ke::parsing::tulis("parse_2",&ARGS[PROYEK],y)
                    },"tulis_parse_2".to_string(),tulis_parse_2
                };
                
                
                
            },"parse_2".to_string(),parse_2,
            {file_management::parse::tulis(&ARGS[PROYEK],"parse",d)},"parse_f".to_string(),parse_f
           
        };
        //file_management::hapus_baris::baris(&ARGS[PROYEK], "parse", 0);
        println!("[parsing selesai : {}/detik]", waktu.elapsed().as_secs_f32());
    }
    if pola.1 {
        println!("[optimalisasi selesai : {}/detik]", waktu.elapsed().as_secs_f32());
    }
    if pola.2 {
        let mut kom  = (false,false);
        for i in kompilasi {
            match i.as_str() {
                "wasm" => {kom.0 = true}
                "win64" => {kom.1 = true}
                _ =>{}
            }
        }
        tread!(join -> 
            {
                if kom.0 {
                    std::fs::create_dir_all(format!("{}\\target\\www",&ARGS[PROYEK])).expect("tidak dapat membuat target direktori (target)");
                    std::fs::create_dir_all(format!("{}\\target\\www\\aset",&ARGS[PROYEK])).expect("tidak dapat membuat target direktori (target)");
                    std::fs::create_dir_all(format!("{}\\target\\www\\wasm",&ARGS[PROYEK])).expect("tidak dapat membuat target direktori (target)");
                    let ((a,b),(c,d),(e,f)) = (channel(),channel(),channel());
                    tread!(
                        {file_management::tulis_ke::wasm::wasm(&ARGS[PROYEK],f)},"wasm/wasm".to_string(),
                        {konversi::wasm::tulis(&ARGS[PROYEK],a,c,e)},"wasm".to_string(),
                        {file_management::tulis_ke::wasm::js(&ARGS[PROYEK],b)},"wasm/js".to_string(),
                        {file_management::tulis_ke::wasm::html(&ARGS[PROYEK],d)},"wasm/html".to_string()
                    );
                    println!("[konversi/wasm selesai : {}/detik]", waktu.elapsed().as_secs_f32());
                }
            },"wasm".to_string(),was,
            {
                if kom.1 {
                    std::fs::create_dir_all(format!("{}\\target\\win64",&ARGS[PROYEK])).expect("tidak dapat membuat target direktori (target)");
                    println!("[konversi/win64 selesai : {}/detik]", waktu.elapsed().as_secs_f32());
                }
            },"win64".to_string(),win64
        );
        println!("[konversi selesai : {}/detik]", waktu.elapsed().as_secs_f32());
    }
    println!("[semua selesai : {}/detik]", waktu.elapsed().as_secs_f32());
}