#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum tipe{
    _string(String),
    _u8,
}
#[allow(non_camel_case_types)]
pub enum perintah {
    variabel(tipe,String,String,String),
    modul_masuk(String),
    modul_keluar,
    cetak(String),
    cpu(String,bool),
    tulis(String,String),
    blok_buka,
    blok_tutup,
    selesai
}

pub mod lexer;
pub mod parse;
pub mod parse_2;
pub mod parse_3;
