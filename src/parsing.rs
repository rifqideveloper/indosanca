use serde::{Deserialize, Serialize};


#[derive(Debug,Clone)]
#[derive(Serialize, Deserialize )]
pub enum Str{
    Some(String),
    arr([usize;2]),
    None,
}

#[derive(Debug,Clone)]
#[derive(Serialize, Deserialize )]
pub enum Tipe{
    _string(Str),
    nomer(String),
    _i8(Option<i8>),
    _u8(Option<u8>),
    _i16(Option<i16>),
    _u16(Option<u16>),
    _i32(Option<i32>),
    _u32(Option<u32>),
    _i64(Option<i64>),
    _u64(Option<u64>),
    penujuk_(String),
    minta(String),
    None,
}
#[allow(non_camel_case_types)]
pub enum perintah {
    /*
    konst(String,Tipe),
    glob_var(String,bool,Tipe),
    */
    _let(String,bool,Tipe),
    variabel_null(String,String),
    modul_masuk(String),
    modul_keluar,
    cetak(String),
    halaman(String),
    warnalatarbelakang(String),
    gambarlatarbelakang(String),
    judul(String),
    navbar(String),
    navbar_tombol(String,String,Vec<String>),
    tombol(String),
    isi(String,String),
    warna(String,String),
    warnalatarbelakangid(String,String),
    ukurankata(String,String),
    cpu(String,bool),
    panggil_fn(Vec<String>,Vec<Tipe>),
    klik(String,Vec<String>),
    tulis(String,Vec<String>),
    //arit_tambah(String,usize,usize),
    swict(u8,Vec<String>),
    kasus(Option< Vec<String>>),
    swict_(u8),
    /*
    blok_buka,
    blok_tutup,
    */
    blok(String),
    blok_,
    /*
    jika,
    jika_,
    jika_tutup,
    lalu,
    lalu_,
    lalu_jika,
    lalu_jika_,
    jika_b(u64),
    */
    
    boolean(bool),
    if_br(String,u8),
    br(String),
    putar,
    batas,
    putus,
    i32_eqz,
    i32_eq,
    lebih_kecil,
    lebih_besar,
    sama_lebih_kecil,
    sama_lebih_besar,
    sama,
    tidak_sama,
    sama_dengan,
    _i32_konst(i32),
    tambah,
    kurang,
    bagi,
    kali,
    modus,
    lanjut,
    selesai
}
pub mod lexer;
pub mod parse;
pub mod parse_2;
pub mod parse_3;

