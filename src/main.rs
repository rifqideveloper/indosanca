use std::env;
use std::sync::mpsc::channel;
extern crate wat;
mod lexer;
mod parser;
mod con_read;
mod con_write;
fn main(){
    const PROYEK:usize = 2;
    let (kirim, terima) = channel();
    let (kirim2, terima2) = channel();
    let (kirim3, terima3) = channel();
    let _kode = std::thread::spawn(move || { 
        let args: Vec<String> = env::args().collect();
        con_read::baca(args[PROYEK].clone(),kirim,terima2) 
    });
    let _lex = std::thread::spawn(move || {lexer::lexer(kirim2,terima,kirim3)});
    let _lex_f = std::thread::spawn(move || {con_write::tuliskan(terima3,&PROYEK)}).join();
    
    
}


