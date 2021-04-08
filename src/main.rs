use std::sync::mpsc::channel;
//use std::fs::File;
//use std::io::prelude::*;
extern crate wat;
mod lexer;
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
    const PROYEK:usize = 2;//<-- ubah proyek angka ke 1 pada versi akhir
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
    //
    std::fs::create_dir_all(format!("{}\\parsing",std::env::args().collect::<Vec<String>>()[PROYEK])).expect("tidak dapat membuat target direktori (parsing)");
    //membaca semua file nyang ada di directori yang dipilih dan dikirim ke lexer
    std::thread::Builder::new().name("_kode".to_string()).spawn(move || {con_read::baca(&PROYEK,kirim,terima2)}).expect("");
    //data yang diterima akan diubah menjadi token dan dikirim ke lex_f
    std::thread::Builder::new().name("_lex".to_string()).spawn(move || {lexer::lexer(kirim2,terima,kirim3)}).expect("");
    //data yang diterima akan ditulis ke lexer file dan dibaca oleh token_read
    std::thread::Builder::new().name("_lex_f".to_string()).spawn(move || {con_write::tuliskan(terima3,&PROYEK,jeda1)}).expect("");
    /*
    token read akan membaca lexer file secara bergantian dan jika
    _lex_f selesai token_read akan membaca sisa nya
    data akan dikirim ke _parser_
    pastikan _lex_f dan _token_read tersingkronisasi dengan baik dangan mengunakan channel
    */
    std::thread::Builder::new().name("_token_read".to_string()).spawn(move || {token_read::baca(&PROYEK,kirim4,lanjut2,jeda2)}).expect("");
    //data yang diterima akan diproses menjadi pohon sintak abstrak dan data dikirim ke _pohom_
    std::thread::Builder::new().name("_parser_".to_string()).spawn(move || {parser::parser(terima4,lanjut1,kirim5)}).expect("");
    //data yang diterima akan dikirim ke file parse
    let _pohon_ = std::thread::Builder::new().name("_pohon_".to_string()).spawn(move || {pohon::tulis(&PROYEK,terima5)}).expect("").join();
    /*pokom sintak akan dioptimalkan
    fitur ini belum dibuat
    */
    let args = std::env::args().collect::<Vec<String>>();
     //membersihkan terget kompilasi sebelumnya,jika forder tidak ada akan error tapi akan dihiraukan
    std::fs::remove_dir_all(format!("{}\\target",args[PROYEK])).ok();
    //membuat target direktori
    std::fs::create_dir_all(format!("{}\\target",args[PROYEK])).expect("tidak dapat membuat target direktori (target)");
    //std::thread.pust();
    let wasm = std::thread::Builder::new().name("wasm".to_string()).spawn( move || {
        if args[PROYEK-1].clone().contains("wasm") {
            //konversi pohon sintak ke js,wasm,html
            println!("[wasm dimuali]");
            std::fs::create_dir_all(format!("{}\\target\\www",args[PROYEK])).expect("tidak dapat membuat target direktori (www)");
            let (kirim7,terima7) = channel();
            let (kirim8,terima8) = channel();
            let (kirim9,terima9) = channel();
            let _baca_ke_js = std::thread::spawn(move || baca_pohon::baca(&PROYEK,kirim7,terima8));
            let _konversi_js = std::thread::spawn(move || js::konvesi(terima7,kirim8,kirim9));
            let _tulis_js = std::thread::spawn(move || tulis_ke::js(terima9,&PROYEK));
            let _tulis_html = std::thread::spawn(move || tulis_ke::html(&PROYEK));
            let mut _selesai = _tulis_js.join();
                _selesai = _tulis_html.join();
            println!("[wasm selesai]")
        }
    }).expect("");
    wasm.join().expect("");
    
    
    //gitignore
    /*
    let mut file = File::create(format!("{}\\.gitignore",std::env::args().collect::<Vec<String>>()[PROYEK])).expect("");
        file.write_all(b"/target\n/parsing\n/aset").expect("")
    */
    println!("selesai dalam : {}/detik", sekarang.elapsed().as_secs_f32());
}