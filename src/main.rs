use std::env;
use std::sync::mpsc::channel;
extern crate wat;
mod lexer;
mod parser;
mod token_read;
mod con_read;
mod con_write;
fn main(){
    const PROYEK:usize = 2;
    let (kirim, terima) = channel();
    let (kirim2, terima2) = channel();
    let (kirim3, terima3) = channel();
    let (kirim4, terima4) = channel();
    let (jeda1, jeda2 ) = channel();
    let (lanjut1 , lanjut2) = channel();
    let _kode = std::thread::spawn(move || { con_read::baca(&PROYEK,kirim,terima2) });
    let _lex = std::thread::spawn(move || {lexer::lexer(kirim2,terima,kirim3)});
    let _lex_f = std::thread::spawn(move || {con_write::tuliskan(terima3,&PROYEK,jeda1)});
    let _token_read = std::thread::spawn(move || {token_read::baca(&PROYEK,kirim4,lanjut2,jeda2)});
    let _pohon_ = std::thread::spawn(move || {parser::parser(terima4,lanjut1)}).join();
    
}


