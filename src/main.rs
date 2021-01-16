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
    const PROYEK:usize = 2;
    let (kirim,terima) = channel();
    let (kirim2,terima2) = channel();
    let (kirim3,terima3) = channel();
    let (kirim4,terima4) = channel();
    let (kirim5,terima5) = channel();
    let (jeda1,jeda2) = channel();
    let (lanjut1,lanjut2) = channel();
    let _kode = std::thread::spawn(move || {con_read::baca(&PROYEK,kirim,terima2)});
    let _lex = std::thread::spawn(move || {lexer::lexer(kirim2,terima,kirim3)});
    let _lex_f = std::thread::spawn(move || {con_write::tuliskan(terima3,&PROYEK,jeda1)});
    let _token_read = std::thread::spawn(move || {token_read::baca(&PROYEK,kirim4,lanjut2,jeda2)});
    let _parser_ = std::thread::spawn(move || {parser::parser(terima4,lanjut1,kirim5)});
    let _pohon_ = std::thread::spawn(move || {pohon::tulis(&PROYEK,terima5)}).join();

    let args: Vec<String> = std::env::args().collect();
    if args[PROYEK-1] == "wasm"{
        drop(args);
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
    }
    

}


