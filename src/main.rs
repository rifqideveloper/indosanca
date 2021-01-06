use std::env;
use std::fs::File;
use std::sync::mpsc::channel;
extern crate wat;
mod lexer;
mod parser;
mod con_read;
fn main(){
    const PROYEK:usize = 2;
    let (kirim, terima) = channel();
    let (kirim2, terima2) = channel();
    let _kode = std::thread::spawn(move || { 
        let args: Vec<String> = env::args().collect();
        con_read::baca(args[PROYEK].clone(),kirim,terima2) 
    });
    let _lex = std::thread::spawn(move || {
        let args: Vec<String> = env::args().collect();
        lexer::bangun_lexer(file_lex_(format!("{}\\lexer",args[PROYEK].clone())),terima,kirim2);
    });
    let args: Vec<String> = env::args().collect();
    if args[PROYEK-1] == "wasm" { 
        let _parser = std::thread::spawn(move || {
            print!("[wasm kompilasi]");
            let mut lex_list: Vec<String>= Vec::with_capacity(2);
            let _selesai = _lex.join().expect("");
            lex_list.push(format!("{}\\lexer",args[PROYEK]));
            parser::parser(lex_list,format!("{}\\parser",args[PROYEK]));
        }).join();
        //let _test = wat::parse_file("./foo.wat").expect("")  
    }
}

fn file_lex_(path:String) -> File{ 
    File::create(path).expect("gagal membuat lexer_ {}") 
}

