use std::sync::mpsc::channel;
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
    //angka dibawah nyadiguankan untuk mempermudah pengembangan aplikasi
    const PROYEK:usize = 2;//ubah proyek angka ke 1 pada versi akhir
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
    //membaca semua file nyang ada di directori yang dipilih dan dikirim ke lexer
    let _kode = std::thread::spawn(move || {con_read::baca(&PROYEK,kirim,terima2)});
    //data yang diterima akan diubah menjadi token dan dikirim ke lex_f
    let _lex = std::thread::spawn(move || {lexer::lexer(kirim2,terima,kirim3)});
    //data yang diterima akan ditulis ke lexer file dan dibaca oleh token_read
    let _lex_f = std::thread::spawn(move || {con_write::tuliskan(terima3,&PROYEK,jeda1)});
    /*
    token read akan membaca lexer file secara bergantian dan jika
    _lex_f selesai token_read akan membaca sisa nya
    data akan dikirim ke _parser_
    pastikan _lex_f dan _token_read tersingkronisasi dengan baik dangan mengunakan channel
    */
    let _token_read = std::thread::spawn(move || {token_read::baca(&PROYEK,kirim4,lanjut2,jeda2)});
    //data yang diterima akan diproses menjadi pohon sintak abstrak dan data dikirim ke _pohom_
    let _parser_ = std::thread::spawn(move || {parser::parser(terima4,lanjut1,kirim5)});
    //data yang diterima akan dikirim ke file parse
    let _pohon_ = std::thread::spawn(move || {pohon::tulis(&PROYEK,terima5)}).join();
    //pokom sintak akan dioptimalkan
    //fitur in belum dibuat

    let args: Vec<String> = std::env::args().collect();
    //membersihkan terget kompilasi sebelumnya
    std::fs::remove_dir_all(format!("{}\\target",args[PROYEK])).ok();
    //membuat target direktori
    std::fs::create_dir_all(format!("{}\\target",args[PROYEK])).expect("tidak dapat membuat target direktori");
    if args[PROYEK-1] == "wasm"{
        drop(args);
        //konversi pohon sintak ke js,wasm,html
        let (kirim7,terima7) = channel();
        let (kirim8,terima8) = channel();
        let (kirim9,terima9) = channel();
        let _baca_ke_js = std::thread::spawn(move || {baca_pohon::baca(&PROYEK,kirim7,terima8)});
        let _konversi_js = std::thread::spawn(move || {js::konvesi(terima7,kirim8,kirim9)});

        let _tulis_js = std::thread::spawn(move || {tulis_ke::js(terima9,&PROYEK)}).join();
        let _tulis_html = std::thread::spawn(move || {
            tulis_ke::html(&PROYEK)
        }).join();
        //let _selesai = _tulis_js.join();
    }else {
        panic!("target kompilasi tidak dikenali")
    }
    

}


