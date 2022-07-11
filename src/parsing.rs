use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize,PartialEq)]
pub enum Str {
    Some(String),
    arr([usize; 2]),
    None,
}

//#[derive(Debug,Clone,Copy)]
#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub enum Tipe {
    _string(Str),
    nomer(String),
    /*
    _i8(Vec<Option<i8>>),
    _u8(Vec<Option<u8>>),
    _i16(Vec<Option<i16>>),
    _u16(Vec<Option<u16>>),
    _i32(Vec<Option<i32>>),
    _u32(Vec<Option<u32>>),
    _i64(Vec<Option<i64>>),
    _u64(Vec<Option<u64>>),
    */
    int(mem_tipe, u64),
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
    penujuk_(String, Option<u64>),
    minta(String, Option<u64>),
    None,
}
impl Clone for Tipe {
    fn clone(&self) -> Tipe {
        self.clone()
    }
}

#[derive(Serialize, Deserialize,Clone,Debug,PartialEq,Copy)]
pub enum mem_mode {
    statik,
    imutabel,
    mutabel,
}
#[derive(Clone,Debug,PartialEq,Serialize, Deserialize)]
pub enum mem_tipe {
    unint(Option<Vec<Option<u64>>>),
    int(Option<Vec<Option<i64>>>),
    float(Option<Vec<Option<f64>>>),

}
#[derive(Clone)]
pub enum args {
    Int(
        u32,
        bool,
        mem_mode,
        String,
        Option<usize>,
        Option<Vec<usize>>,
    ),
    Str_Lansung(String),
    Str_int(i128),
    penunjuk(u64),//penunjuk
    penunjuk_nama(String),
    internar_memory(String),//memory yang hanya dapat diakses kompiler
    null

}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub enum eterator {
    Putar(String),
    Blok(String),
    Iter(String,args,args)
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub enum perintah {
    /*
    konst(String,Tipe),
    glob_var(String,bool,Tipe),
    */
    _let(String, mem_mode, Tipe),
    jika(Vec<String>),
    variabel_null(String, String),
    modul_masuk(String),
    modul_keluar,
    tidur (args),
    cetak(Vec<String>),
    halaman(String),
    warnalatarbelakang(String),
    gambarlatarbelakang(String),
    judul(String),
    navbar(String),
    navbar_tombol(String, String, Vec<String>),
    tombol(String),
    isi(String, String),
    warna(String, String),
    warnalatarbelakangid(String, String),
    ukurankata(String, String),
    cpu(String, bool, Vec<args>, Vec<args>),
    panggil_fn(Vec<String>, Vec<Tipe>),
    klik(String, Vec<String>),
    tulis(String, Vec<String>),
    //arit_tambah(String,usize,usize),
    swict(u8, Vec<String>),
    kasus(Option<Vec<String>>),
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
    if_br(String, u8),
    br(String),
    putar(Vec<args>),
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
#[derive(Clone,Debug)]
pub struct Arrmap<Key, Value> {
    map: Vec<(Key, Value)>,
}
impl<Key: std::cmp::PartialEq + Clone, Value: Clone> Arrmap<Key, Value> {
    pub const fn new() -> Arrmap<Key, Value> {
        Arrmap { map: Vec::new() }
    }
    pub fn index(&self, index: &usize) -> Option<&(Key, Value)> {
        if self.map.len() > *index {
            return None;
        }
        Some(& self.map[*index] )
    }
    /*
    pub fn index_mut(&self, index: &usize) -> Option<&mut (Key, Value)> {
        if self.map.len() > *index {
            return None;
        }
        return Some(&mut self.map[*index] )
    }
    */
    pub fn get(&self, key: &Key) -> Option<&Value> {
        let mut i = self.map.len();
        while i != 0 {
            i-= 1;
            if self.map[i].0 == *key {
                return Some(&self.map[i].1);
            }
        }
        None
    }
    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        let mut i = self.map.len();
        while i != 0 {
            i-= 1;
            if self.map[i].0 == *key {
                return Some(&mut self.map[i].1);
            }
        }
        None
    }
    pub fn get_all_mut(&mut self, key: &Key) -> Option<&mut(Key,Value)> {
        let mut i = self.map.len();
        while i != 0 {
            i-= 1;
            if self.map[i].0 == *key {
                return Some(&mut self.map[i] );
            }
        }
        None
    }
    pub fn insert(&mut self, key: Key, value: Value) -> Option<(Key, Value)> {
        self.map.push((key, value));
        None
    }
    pub fn iter(&mut self) ->& std::vec::Vec<(Key, Value)>{
        &self.map
    }
    pub fn iter_mut(&mut self) ->&mut std::vec::Vec<(Key, Value)>{
        &mut self.map
    }
    pub fn clear(&mut self) {
        self.map.clear();
    }

}
#[derive(Clone, Debug,PartialEq)]
pub struct Let_ {
    pub id: u64,
    pub tipe: Tipe,
    pub mut_: mem_mode, //jika none maka static
    pub sudah_dibaca: bool,
    pub sudah_ditulis: bool,
    pub bisa_diprediksi: bool,
    pub nama_file: String,
    pub nomer_baris: u64,
    pub _vec: bool,
}
impl Let_ {
    pub fn baru(
        id: &mut u64,
        mut_: mem_mode,
        _vec: bool,
        tipe: Tipe,
        nama_file: String,
        nomer_baris: u64,
    ) -> Let_ {
        Let_ {
            id: id.clone(),
            tipe,
            mut_,
            sudah_dibaca: false,
            sudah_ditulis: false,
            bisa_diprediksi: true,
            nama_file,
            nomer_baris,
            _vec: false,
        }
    }
}
#[derive(Debug,Clone)]
pub enum nilai {
    tambah,
    string(String),
    angka(isize),
    penunjuk(String),
    aritmatik(Box<aritmatik>)
}
#[derive(Debug,Clone)]
pub enum aritmatik {
    tambah(nilai,nilai)
}
#[derive(Clone)]
pub enum Pohon {
    Let(String, usize),
    Cetak(args),
    tidur(args),
    tulis(String,usize,nilai),
    blok(eterator),
    blok_,
    br(String),
    Error,
    panic(Option<String>),
    Selesai,
}

//pub mod lexer;
//pub mod parse;
pub mod parse_2;
//pub mod parse_3;
pub mod lexer_2;
pub mod parse_1;
//pub mod parse_3_1;
pub mod parse_3_2;
