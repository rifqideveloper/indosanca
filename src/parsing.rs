#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum tipe{
    _string(String),
    _u8,
}
#[allow(non_camel_case_types)]
pub enum perintah {
    variabel_null(String,String),
    modul_masuk(String),
    modul_keluar,
    cetak(String),
    halaman(String),
    cpu(String,bool),
    tulis(String,Vec<String>),
    //arit_tambah(String,usize,usize),
    blok_buka,
    blok_tutup,
    blok(String),
    blok_,
    jika,
    jika_,
    jika_tutup,
    lalu,
    lalu_jika,
    boolean(bool),
    if_br(String,u8),
    br(String),
    putar,
    batas,
    putus,
    i32_eqz,
    _i32_konst(i32),
    kurang,
    lanjut,
    selesai
}

pub mod lexer;
pub mod parse;
pub mod parse_2;
pub mod parse_3;
