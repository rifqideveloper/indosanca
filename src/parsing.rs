use serde::{Deserialize, Serialize};


#[derive(Debug,Clone)]
#[derive(Serialize, Deserialize )]
pub enum Str{
    Some(String),
    arr([usize;2]),
    None,
}

//#[derive(Debug,Clone,Copy)]
#[derive(Serialize, Deserialize ,Debug)]
pub enum Tipe{
    _string(Str),
    nomer(String),
    _i8(Vec<Option<i8>>),
    _u8(Vec<Option<u8>>),
    _i16(Vec<Option<i16>>),
    _u16(Vec<Option<u16>>),
    _i32(Vec<Option<i32>>),
    _u32(Vec<Option<u32>>),
    _i64(Vec<Option<i64>>),
    _u64(Vec<Option<u64>>),
    /*
    _u8_ar(usize,Vec<String>),
    _i8_ar(usize,Vec<String>),
    _u16_ar(usize,Vec<String>),
    _i16_ar(usize,Vec<String>),
    _u32_ar(usize,Vec<String>),
    _i32_ar(usize,Vec<String>),
    _u64_ar(usize,Vec<String>),
    _i64_ar(usize,Vec<String>),
    */
    penujuk_(String,Option<u64>),
    minta(String,Option<u64>),
    None,
}
impl Clone for Tipe {
    fn clone(&self) -> Tipe {
         self.clone()
    }
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
    cetak(Vec<String>),
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
    selesai(String),
}
//pub mod lexer;
//pub mod parse;
pub mod parse_2;
pub mod parse_3;
pub mod lexer_2;
pub mod parse_1;
pub mod parse_3_1;

