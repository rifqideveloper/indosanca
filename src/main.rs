use std::sync::mpsc::channel;
//use std::fs::File;
//use std::io::prelude::*;
extern crate wat;
mod seting;
mod lexer ;
mod parser;
mod token_read;
mod con_read;
mod con_write;
mod pohon;
mod baca_pohon;
mod js;
mod tulis_ke;

fn main(){
    let sekarang = std::time::Instant::now();
    //angka dibawah nyadiguankan untuk mempermudah pengembangan aplikasi
    const PROYEK:usize = 1;//<-- ubah proyek angka ke 0 pada versi akhir
    /*
    setiap untaiyan(thread) memiliki tugas masing2 dan akan saling
    berkomunikasi melalui kanal dibawah
    !!peringatan!!
    aplikasi kompilasi ini tidak mengecek kesalaham hanya menkopilasi kode ketarget yang diinginkan
    pemeriksaan kesalahan diserahkan sepenuhnya pada ide atau pengguna 
    saya diadak bertangung jawab pada pun kesalahan penguna
    guna ataupun memodifikasi dengan bebas aplikasi in semau anda tanpa batasan 
    untuk lebih jelas lihat dilicense aplikasi di https://github.com/rifqideveloper/indosanca/blob/master/LICENSE
    */
    let (kirim,terima) = channel();
    let (kirim2,terima2) = channel();
    let (kirim3,terima3) = channel();
    let (kirim4,terima4) = channel();
    let (kirim5,terima5) = channel();
    let (jeda1,jeda2) = channel();
    let (lanjut1,lanjut2) = channel();
    let (args,perpus,kompilasi,versi,nama_app) = seting::seting(&PROYEK);
    //
    std::fs::create_dir_all(format!("{}\\parsing",&args[PROYEK])).expect("tidak dapat membuat target direktori (parsing)");
    //membaca semua file nyang ada di directori yang dipilih dan dikirim ke lexer
    let arg_clone_con_read = args[PROYEK].clone();
    std::thread::Builder::new().name("_kode".to_string()).spawn(move || {con_read::baca(arg_clone_con_read,kirim,terima2)}).expect("");
    //data yang diterima akan diubah menjadi token dan dikirim ke lex_f
    std::thread::Builder::new().name("_lex".to_string()).spawn(move || {lexer::lexer(kirim2,terima,kirim3)}).expect("");
    //data yang diterima akan ditulis ke lexer file dan dibaca oleh token_read
    let arg_clone_con_write = args[PROYEK].clone();
    std::thread::Builder::new().name("_lex_f".to_string()).spawn(move || {con_write::tuliskan(terima3,arg_clone_con_write,jeda1)}).expect("");
    /*
    token read akan membaca lexer file secara bergantian dan jika
    _lex_f selesai token_read akan membaca sisa nya
    data akan dikirim ke _parser_
    pastikan _lex_f dan _token_read tersingkronisasi dengan baik dangan mengunakan channel
    */
    let arg_clone_token_read = args[PROYEK].clone();
    std::thread::Builder::new().name("_token_read".to_string()).spawn(move || {token_read::baca(arg_clone_token_read,kirim4,lanjut2,jeda2)}).expect("");
    //data yang diterima akan diproses menjadi pohon sintak abstrak dan data dikirim ke _pohom_
    std::thread::Builder::new().name("_parser_".to_string()).spawn(move || {parser::parser(terima4,lanjut1,kirim5)}).expect("");
    //data yang diterima akan dikirim ke file parse
    let arg_clone_token_pohon = args[PROYEK].clone();
    std::thread::Builder::new().name("_pohon_".to_string()).spawn(move || {pohon::tulis(arg_clone_token_pohon,terima5)}).expect("").join().expect("");
    
    /*pokom sintak akan dioptimalkan
    fitur ini belum dibuat
    */
    println!("[parsing selesai : {}/detik]", sekarang.elapsed().as_secs_f32());
     //membersihkan terget kompilasi sebelumnya,jika forder tidak ada akan error tapi akan dihiraukan
    std::fs::remove_dir_all(format!("{}\\target",&args[PROYEK])).ok();
    //membuat target direktori
    std::fs::create_dir_all(format!("{}\\target",&args[PROYEK])).expect("tidak dapat membuat target direktori (target)");
    
    //std::thread.pust();
    
    let mut jalan_wasm = false;
    let mut jalan_asm64 = false;
    let mut jalan_win64 = false;
    for i in kompilasi {
        match i.as_str() {
            "wasm" => jalan_wasm = true,
            "asm64" => jalan_asm64 = true,
            "win64" => {
                jalan_asm64 = true;
                jalan_win64 = true;
            }
            _ =>{}
        }
    }
    let arg_clone_wasm = args[PROYEK].clone();
    let wasm = std::thread::Builder::new().name("wasm".to_string()).spawn( move || {
        if jalan_wasm {
            //konversi pohon sintak ke js,wasm,html
            println!("[wasm dimuali]");
            let clo_1 = arg_clone_wasm;
            let clo_2 = clo_1.clone();
            let clo_3 = clo_1.clone();
            let (kirim7,terima7) = channel();
            let (kirim8,terima8) = channel();
            let (kirim9,terima9) = channel();
            std::fs::create_dir_all(format!("{}\\target\\www",clo_1)).expect("tidak dapat membuat target direktori (www)");
            let _baca_ke_js = std::thread::spawn(move || baca_pohon::baca(clo_1,kirim7,terima8));
            let _konversi_js = std::thread::spawn(move || js::konvesi(terima7,kirim8,kirim9));
            let _tulis_js = std::thread::spawn(move || tulis_ke::js(terima9,clo_2));
            let _tulis_html = std::thread::spawn(move || tulis_ke::html(clo_3));
            _tulis_js.join().expect("");
            _tulis_html.join().expect("");
            println!("[wasm selesai : {}/detik]", sekarang.elapsed().as_secs_f32())
        }
    }).expect("");

    let nativ = std::thread::Builder::new().name("nativ".to_string()).spawn( move || {
        let asm64 = std::thread::Builder::new().name("asm64".to_string()).spawn( move || {
            if jalan_asm64 {
                println!("[asm64 mulai]");
                //
                println!("[asm64 selesai]");
            }
        }).expect("");
        asm64.join().expect("");
        let win64 = std::thread::Builder::new().name("win64".to_string()).spawn( move || {
            if jalan_win64 {
                println!("[win64 mulai]");
                //
                println!("[win64 selesai]");
            }
        }).expect("");

        win64.join().expect("");
    }).expect("");
    wasm.join().expect("");
    nativ.join().expect("");
    
    
    //gitignore
    /*
    let mut file = File::create(format!("{}\\.gitignore",std::env::args().collect::<Vec<String>>()[PROYEK])).expect("");
        file.write_all(b"/target\n/parsing\n/aset").expect("")
    */
    println!("semua selesai : {}/detik", sekarang.elapsed().as_secs_f32());
}