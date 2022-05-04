use crate::parsing::Tipe;
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum Nilai {
    langsung_str(String),
    langsung_int(u64),
    langsung_float(f64),
    penujuk_(u64),
    minta(u64),
    None,
}
#[derive(Clone)]
pub struct let_ {
    pub id: u64,
    pub tipe: Tipe,
    mut_: Option<bool>, //jika none maka static
    sudah_dibaca: bool,
    sudah_ditulis: bool,
    bisa_diprediksi: bool,
    nama_file: String,
    nomer_baris: u64,
    _vec: bool,
}
impl let_ {
    fn baru(
        id: &mut u64,
        mut_: Option<bool>,
        _vec: bool,
        tipe: Tipe,
        nama_file: String,
        nomer_baris: u64,
    ) -> let_ {
        let v = let_ {
            id: id.clone(),
            tipe,
            mut_,
            sudah_dibaca: false,
            sudah_ditulis: false,
            bisa_diprediksi: true,
            nama_file,
            nomer_baris,
            jika_vec,
        };
        *id += 1;
        v
    }
}
#[derive(Clone)]
pub enum Pohon {
    fungsi(String),
    tambah_fungsi(String),
    panggil_fn(String),
    cetak(Nilai),
    _let(Box<let_>),
    var(u64, Tipe),
    panggil_var(u64),
    tulis(u64, Tipe),
    konst_str(u64, String),
    const_32(i32),
    local_set(i32),
    halaman(Nilai),
    warnalatarbelakang(String),
    gambarlatarbelakang(String),
    judul(String),
    tombol(String),
    isi(String, String),
    warna(String, String),
    warnalatarbelakangid(String, String),
    ukurankata(String, String),
    klik(String, Vec<String>),
    i32_eqz,
    i32_eq,
    blok(String),
    blok_,
    putar,
    putus,
    lanjut,
    batas,
    add,
    sub,
    div_s,
    div_u,
    mul,
    bandingkan_str_str(u64, u64),
    r#if,
    if_,
    jika_tutup,
    lalu_jika,
    if_br(String, String),
    br(String),
    main(String),
    selesai,
    kopilasi_error,
}
/*
impl Clone for Pohon {
    fn clone(&self) -> Pohon {
         *self
    }
}
*/
macro_rules! kirim {
    (terakhir $a:expr,$pwa:expr,$z:expr,$has:expr) => {
        {
                use ansi_term::Colour::Yellow;
            //pegiriman ke
                if $a.0 {

                }
                if $a.1 {
                    $pwa.send($z).unwrap();
                    $has.into_iter().for_each(|i|i.1.into_iter().for_each(|x|
                        if !x.bisa_diprediksi || x.mut_ == None {
                            //$pwa.send( Box::new( Pohon::_let( x.clone() ) )).unwrap();
                        } else if x.sudah_dibaca {
                            if x.mut_ == Some(true){
                                println!("{} {}:{}:0\nvariabel{} tanpi tidak pernah ditulis ? varibel tidak akan dialokasikan\n",
                                Yellow.paint("peringatan!"),
                                x.nama_file,x.nomer_baris,
                                " mutabel"
                                );
                            }
                        } else if x.sudah_ditulis{
                            //varibel harus mutabel jika ingin ditulis
                            if x.mut_ == Some(false) {
                                panic!()
                            }
                            println!("{} {}:{}:0\nvariabel mutabel tanpi tidak pernah dibaca ? varibel tidak akan dialokasikan\n",
                                Yellow.paint("peringatan!"),
                                x.nama_file,
                                x.nomer_baris,
                            );
                        } else if x.mut_ == Some(true) {
                            println!("{} {}:{}:0\nvariabel mutabel tanpi tidak pernah ditulis dan dibaca ? varibel tidak akan dialokasikan\n",
                                Yellow.paint("peringatan!"),
                                x.nama_file,
                                x.nomer_baris
                            );
                        } else {
                            println!("{} {}:{}:0\nvariabel tanpi tidak pernah dibaca ? varibel tidak akan dialokasikan\n",
                                Yellow.paint("peringatan!"),
                                x.nama_file,
                                x.nomer_baris
                            );
                        }
                    ));
                    /*
                    $has.into_iter().for_each(|i|{
                        if !i.1.bisa_diprediksi {
                            $pwa.send( Pohon::_let( i.1.clone() )).unwrap();
                        } else if i.1.sudah_dibaca {
                            println!("{} {}:{}:0\nvariabel{} tanpi tidak pernah ditulis dan dibaca ? varibel tidak akan dialokasikan\n",Yellow.paint("peringatan!"),i.1.nama_file,i.1.nomer_baris,if i.1.mut_ {" mutabel"}else{""});
                        } else if i.1.sudah_ditulis{
                            //varibel harus mutabel jika ingin ditulis
                            if !i.1.mut_ {
                                panic!()
                            }
                            println!("{} {}:{}:0\nvariabel mutabel tanpi tidak pernah ditulis dan dibaca ? varibel tidak akan dialokasikan\n",Yellow.paint("peringatan!"),i.1.nama_file,i.1.nomer_baris,);
                        } else if i.1.mut_ {
                            println!("{} {}:{}:0\nvariabel mutabel tanpi tidak pernah ditulis dan dibaca ? varibel tidak akan dialokasikan\n",Yellow.paint("peringatan!"),i.1.nama_file,i.1.nomer_baris);
                        } else {
                            println!("{} {}:{}:0\nvariabel tanpi tidak pernah dibaca ? varibel tidak akan dialokasikan\n",Yellow.paint("peringatan!"),i.1.nama_file,i.1.nomer_baris);
                        }
                    });
                    */
                    $pwa.send($z).unwrap();
                }
            //nativ
            /*
            if $a.0 {

            }
            if $a.1 {
                $pwa.send($z).unwrap();
                $has.into_iter().for_each(|i|
                    if i.1.mut_ {
                        if i.1.sudah_ditulis {
                            if i.1.sudah_dibaca {
                                $pwa.send( Pohon::_let( i.1.clone() )).unwrap();
                            } else {
                                println!("{} {}:{}:0\nvariabel mutabel tanpi tidak pernah dibaca tapi pernah ditulis ? varibel tidak akan dialokasikan\n",Yellow.paint("peringatan!"),i.1.nama_file,i.1.nomer_baris);
                            }
                        } else if i.1.sudah_dibaca{
                            println!("{} {}:{}:0\nvariabel mutabel tanpi tidak pernah dibaca ? varibel tidak akan dialokasikan\n",Yellow.paint("peringatan!"),i.1.nama_file,i.1.nomer_baris);
                        } else {
                            println!("{} {}:{}:0\nvariabel mutabel tanpi tidak pernah ditulis dan dibaca ? varibel tidak akan dialokasikan\n",Yellow.paint("peringatan!"),i.1.nama_file,i.1.nomer_baris);
                        }
                    } else if !i.1.sudah_dibaca {
                        println!("{} {}:{}:0\nvariabel tidak pernah dibaca ? varibel tidak akan dialokasikan\n",Yellow.paint("peringatan!"),i.1.nama_file,i.1.nomer_baris);
                    }

                );
                $pwa.send($z).unwrap();
            }
            */
            //drop(($a,$b,$z,$has))
        }
    };
    ($a:expr,$b:expr,$($z:expr),+) => {
        {
            if $a.0 {
            }
            if $a.1 {
                $(
                    $b.send($z).unwrap();
                )*
            }
        }
    };
}
/*
fn kirim(pohon: Pohon, kom: (bool, bool), kirim_pwa: &std::sync::mpsc::Sender<Pohon>) {
    if kom.0 {}
    if kom.1 {
        kirim_pwa.send(pohon).unwrap();
    }
}
*/

use std::collections::HashMap;
pub fn parse(
    terima: std::sync::mpsc::Receiver<std::option::Option<serde_json::Value>>,
    kirim_pwa: std::sync::mpsc::Sender<Box<Pohon>>,
    kirim_json: std::sync::mpsc::Sender<([std::string::String; 3], bool)>,
    kom: (bool, bool),
) {
    //panic!();
    //jika allokasi tidak pernah digunakan maka memori tidak diallokasi
    let mut allokasi: HashMap<String, Vec<Box<let_>>> = HashMap::with_capacity(1);
    let mut data = Vec::with_capacity(1);
    let mut id = 0;
    data.push((
        ["main".to_string(), "main".to_string(), "".to_string()],
        false,
    ));
    'awal: for data_iter in 0.. {
        kirim_json.send(data[data_iter].clone()).unwrap();
        if data.len() != 1 {
            //nama fungsi tidak boleh sama
            for i in 0..data.len() {
                if data[i].0 == data[data_iter].0 {
                    println!(
                        "Error!! [ ada fungsi dengan nama sama difile yang sama \"{}\" ]",
                        data[data_iter].0[1]
                    );
                    kirim!(kom, kirim_pwa,Box::new(Pohon::kopilasi_error));
                    return;
                }
            }
            //perpustaka
            kirim!(
                kom,
                kirim_pwa,
                Box::new(
                Pohon::tambah_fungsi(format!(
                    "{}_{}_{}",
                    data[data_iter].0[0], data[data_iter].0[1], data[data_iter].0[2]
                )))
            );
        }
        if let Some(mut json) = terima.recv().unwrap() {
            let mut i = 0;
            loop {
                match json["nilai"][i]["tipe"].as_str() {
                    Some(o) => {
                        match o {
                            "jika" => {
                                let y = i.clone();
                                i += 1;
                                if let Some(blok) = json["nilai"][i]["tipe"].as_str() {
                                    let mut _jika_ = (true, false);
                                    for x in 0.. {
                                        match &json["nilai"][y]["nilai"][x] {
                                            serde_json::Value::Array(arr) => {}
                                            serde_json::Value::Bool(boo) => _jika_.1 = *boo,
                                            serde_json::Value::Null => break,
                                            serde_json::Value::Number(num) => {}
                                            serde_json::Value::String(str_) => {}
                                            serde_json::Value::Object(obj) => {}
                                        }
                                    }
                                    if _jika_.0 {
                                        if _jika_.1 {
                                            //conde diterima
                                            continue;
                                        }
                                    } else {
                                    }
                                }
                            }
                            "tidur" =>{
                                
                            }
                            "var" => {
                                //println!("{:#?}", json["nilai"][i]);
                                let nama_file = json["nilai"][i]["nama_file"].as_str().unwrap();
                                let nama = json["nilai"][i]["nama"].as_str().unwrap();
                                let nomer_baris = json["nilai"][i]["nomer_baris"].as_u64().unwrap();
                                let mut_ = json["nilai"][i]["mut"].as_str().unwrap();
                                let mut_ = if mut_ == "i" {
                                    Some(false)
                                } else if mut_ == "m" {
                                    Some(true)
                                } else if mut_ == "s" {
                                    None
                                } else {
                                    panic!()
                                };
                                let mut data = Vec::with_capacity(
                                    json["nilai"][i]["nilai"][0].as_u64().unwrap() as usize
                                );
                                if let Some(bytes) = json["nilai"][i]["mem"]["uint"].as_u64() {
                                    if data.capacity() == 1 {
                                        data.push( json["nilai"][i]["nilai"][1].as_u64() )

                                    }
                                    let nilai = Tipe::uint(bytes,data);
                                    let nilai = let_::baru(
                                        &mut id,
                                        mut_ ,
                                        false ,
                                        nilai,
                                        nama_file.to_string(),
                                        nomer_baris
                                    );
                                    if let Some(var)= allokasi.get_mut(nama) {
                                        if false {
                                            panic!()
                                        }
                                        var.push(
                                            Box::new(nilai)
                                        );
                                        panic!()
                                    } else {
                                        //let mut_ = nilai.mut_;
                                        if nilai.mut_ == None {
                                            //let v = Box::new(Pohon::_let(nilai.clone()));
                                            //kirim!(kom, kirim_pwa,Pohon::_let(Box::new( nilai.clone() )));
                                            //kirim!(kom, kirim_pwa, crate::parsing::parse_3_1::Pohon(nilai.clone()));
                                        }
                                        allokasi.insert(nama.to_string(), vec![Box::new(nilai)] );
                                        /*
                                        if mut_ == None {
                                            if let Some(nilai) = allokasi.get(&nama.to_string()) {
                                                let nilai = nilai.last().unwrap().clone();
                                                kirim!(kom, kirim_pwa, Pohon::_let(nilai));
                                            }
                                        }
                                        */
                                    }
                                    
                                } else {
                                    panic!()
                                }

                                /*
                                let nama = json["nilai"][i]["nama"].as_str().unwrap();
                                let obj;
                                //let data = Vec::with_capacity(json["nilai"][i]["nilai"][0].as_u64().unwrap() as usize);
                                let mut_ = match json["nilai"][i]["mut"].as_str().unwrap() {
                                    "i"=>Some(false),
                                    "u"=>Some(true),
                                    "s"=>None,
                                    _=>panic!()
                                };
                                if let Some(bytes) = json["nilai"][i]["mem"]["uint"].as_u64() {
                                    /*obj = let_::baru(&mut id,mut_ , false ,
                                        Tipe::uint(bytes,data),
                                        json["nilai"][i]["nama_file"].as_str().unwrap().to_string(),
                                        json["nilai"][i]["nomer_baris"].as_u64().unwrap()
                                    );*/

                                } else {
                                    panic!()
                                }
                                if let Some(var)= allokasi.get_mut(nama) {
                                    var.push(obj)
                                } else {
                                    let x = vec![obj];
                                    allokasi.insert(nama.to_string(), x);
                                }
                                */
                                /* overflow
                                let nama = json["nilai"][i]["nama"].as_str().unwrap();
                                if let Some(bytes) = json["nilai"][i]["mem"]["uint"].as_u64() {
                                    let mut_ = match json["nilai"][i]["mut"].as_str().unwrap() {
                                        "i"=>Some(false),
                                        "u"=>Some(true),
                                        "s"=>None,
                                        _=>panic!()
                                    };
                                    let mut data = Vec::with_capacity(1);
                                    println!("{:?}",data);
                                    let obj = let_::baru(&mut id,mut_ , false ,
                                        Tipe::uint(bytes,data),
                                        json["nilai"][i]["nama_file"].as_str().unwrap().to_string(),
                                        json["nilai"][i]["nomer_baris"].as_u64().unwrap()
                                    );
                                    if let Some(var)= allokasi.get_mut(nama) {
                                        var.push(obj)
                                    } else {
                                        allokasi.insert(nama.to_string(), vec![obj]);
                                    }
                                }
                                */
                                /*
                                let jika_vec;
                                let tipe_ = match json["nilai"][i]["tipe_"].as_str().unwrap() {
                                    "str" => {
                                        jika_vec = false;
                                        crate::parsing::Tipe::_string(crate::parsing::Str::None)
                                    }
                                    "str_" => {
                                        jika_vec = false;
                                        crate::parsing::Tipe::_string(crate::parsing::Str::Some(
                                            json["nilai"][i]["nilai"].as_str().unwrap().to_string(),
                                        ))
                                    }
                                    "u8_" => {
                                        let mut v = Vec::new();
                                        if let serde_json::Value::Number(n) =
                                            &json["nilai"][i]["cap"]
                                        {
                                            v.reserve(n.as_u64().unwrap() as usize);
                                            for x in 0..v.capacity() {
                                                if let serde_json::Value::Number(n) =
                                                    &json["nilai"][i]["nilai"][x]
                                                {
                                                    v.push(Some(n.as_u64().unwrap() as u8));
                                                } else {
                                                    break;
                                                }
                                            }
                                            jika_vec = true;
                                        } else if let serde_json::Value::Array(arr) =
                                            &json["nilai"][i]["nilai"]
                                        {
                                            // array harus lebih dari satu dan tidah boleh nol
                                            if arr.len() == 1 {
                                                panic!()
                                            }
                                            if arr.len() == 0 {
                                                panic!()
                                            }
                                            v.reserve_exact(arr.len());
                                            (0..arr.capacity()).into_iter().for_each(|f| {
                                                v.push(Some(arr[f].as_u64().unwrap() as u8));
                                            });
                                            jika_vec = false;
                                        } else if let serde_json::Value::Number(n) =
                                            &json["nilai"][i]["nilai"]
                                        {
                                            v.reserve(1);
                                            v.push(Some(n.as_u64().unwrap() as u8));
                                            jika_vec = false;
                                        } else {
                                            panic!()
                                        }
                                        crate::parsing::Tipe::_u8(v)
                                    }
                                    _ => {
                                        panic!()
                                    }
                                };
                                let mut_ = if let Some(_) = json["nilai"][i]["mut"].as_str() {
                                    true
                                } else {
                                    false
                                };
                                let nama_file =
                                    json["nilai"][i]["nama_file"].as_str().unwrap().to_string();
                                let nomer_baris = json["nilai"][i]["nomer_baris"].as_u64().unwrap();
                                let nilai_var = let_::baru(
                                    id,
                                    Some(mut_),
                                    jika_vec,
                                    tipe_,
                                    nama_file,
                                    nomer_baris,
                                );
                                /*let_ {
                                    id,
                                    tipe: tipe_,
                                    mut_: mut_,
                                    sudah_dibaca: false,
                                    sudah_ditulis: false,
                                    nama_file: nama_file,
                                    nomer_baris: nomer_baris,
                                    _vec: jika_vec,
                                };*/
                                let nama = json["nilai"][i]["nama"].as_str().unwrap().to_string();
                                if let Some(data) = allokasi.get_mut(&nama) {
                                    data.push(nilai_var);
                                } else {
                                    allokasi.insert(nama, vec![nilai_var]);
                                }

                                id += 1;
                                */
                            }
                            "panggil_fn" => {}
                            "cetak" => {
                                let mut hasil: Vec<Nilai> = Vec::with_capacity(2);
                                let mut x = 0;
                                while let Some(tipe) = json["nilai"][i]["nilai"][x].as_str() {
                                    if tipe == "var" {
                                        x += 1;
                                        let nama = json["nilai"][i]["nilai"][x].as_str().unwrap().to_string();
                                        let nama = json["nilai"][i]["nilai"][x].as_str().unwrap();
                                        if let Some(variabel) = allokasi.get_mut(nama) {
                                            if let Some(variabel) = variabel.last_mut() {
                                                variabel.sudah_dibaca = true;
                                                match &variabel.tipe {
                                                    Tipe::_string(s) => {
                                                        if variabel.bisa_diprediksi {
                                                        } else {
                                                        }
                                                    }
                                                    Tipe::uint(bit, uint) => {
                                                        if variabel.bisa_diprediksi {
                                                            let bytes = bit/8;
                                                            if bytes == 1 {
                                                                //println!("{:?}",uint);
                                                                /*
                                                                kirim!(
                                                                    kom,
                                                                    kirim_pwa,
                                                                    Pohon::cetak(
                                                                        Nilai::langsung_int(
                                                                            uint[0].unwrap()
                                                                        )
                                                                    )
                                                                )
                                                                */
                                                            } else {
                                                                panic!();
                                                                /* prototy
                                                                kirim!(
                                                                    kom,
                                                                    kirim_pwa,
                                                                    Pohon::cetak(
                                                                        Nilai::arr(
                                                                            uint
                                                                        )
                                                                    )
                                                                )
                                                                */
                                                            }
                                                        } else {
                                                        }
                                                    }
                                                    Tipe::int(bytes, int) => {
                                                        if variabel.bisa_diprediksi {
                                                        } else {
                                                        }
                                                    }
                                                    Tipe::penujuk_(p, q) => {
                                                        if variabel.bisa_diprediksi {
                                                        } else {
                                                        }
                                                    }
                                                    _ => {
                                                        panic!()
                                                    }
                                                }
                                                /*
                                                if variabel.bisa_diprediksi {
                                                    match variabel.tipe {
                                                        Tipe::_string(s)=>{

                                                        }
                                                    }
                                                    /*
                                                    if let Tipe::_u8(int) = &variabel.tipe {
                                                        if int.len() == 1 {
                                                            //bukan arr
                                                            if let Some(int) = int[0] {
                                                                kirim!(
                                                                    kom,
                                                                    kirim_pwa,
                                                                    Pohon::cetak(
                                                                        Nilai::langsung_int(
                                                                            int as u64
                                                                        )
                                                                    ) /**/
                                                                );
                                                                hasil.push(Nilai::None);
                                                            } else {
                                                                use ansi_term::Colour::Red;
                                                                println!(
                                                                    "{} {}:{}:0\nvarabel '{}' belum diiniliasi\n",
                                                                    Red.paint("Error!"),
                                                                    json["nilai"][i]["nama_file"].as_str().unwrap(),
                                                                    json["nilai"][i]["nomer_baris"].as_u64().unwrap(),
                                                                    nama
                                                                );
                                                                kirim_json
                                                                    .send((
                                                                        [
                                                                            "".to_string(),
                                                                            "".to_string(),
                                                                            "".to_string(),
                                                                        ],
                                                                        true,
                                                                    ))
                                                                    .unwrap();
                                                                kirim!(
                                                                    kom,
                                                                    kirim_pwa,
                                                                    Pohon::kopilasi_error /**/
                                                                );
                                                                return;
                                                            }
                                                        } else if int.len() > 1 {
                                                            //array
                                                        } else {
                                                            panic!()
                                                        }
                                                        //kirim!(kom, kirim_pwa, Pohon /**/);
                                                    } else {
                                                        panic!()
                                                    }
                                                    */
                                                } else {
                                                }
                                                */
                                            } else {
                                                panic!()
                                            }
                                            /* versi lama
                                            variabel.sudah_dibaca = true;
                                            if variabel.bisa_diprediksi {
                                                if let Tipe::_u8(int) = &variabel.tipe {
                                                    if int.len() == 1 {
                                                        //bukan arr
                                                        if let Some(int) = int[0] {
                                                            kirim!(kom, kirim_pwa, Pohon::cetak(Nilai::langsung_int(int as u64)) /**/);
                                                            hasil.push(
                                                                Nilai::None
                                                            );
                                                        } else {
                                                            use ansi_term::Colour::Red;
                                                            println!(
                                                                "{} {}:{}:0\nvarabel '{}' belum diiniliasi\n",
                                                                Red.paint("Error!"),
                                                                json["nilai"][i]["nama_file"].as_str().unwrap(),
                                                                json["nilai"][i]["nomer_baris"].as_u64().unwrap(),
                                                                nama
                                                            );
                                                            kirim_json
                                                                .send((
                                                                    [
                                                                        "".to_string(),
                                                                        "".to_string(),
                                                                        "".to_string(),
                                                                    ],
                                                                    true,
                                                                ))
                                                                .unwrap();
                                                            kirim!(kom, kirim_pwa, Pohon::kopilasi_error /**/);
                                                            return
                                                        }
                                                    } else if int.len() > 1 {
                                                        //array
                                                    } else {
                                                        panic!()
                                                    }
                                                    //kirim!(kom, kirim_pwa, Pohon /**/);
                                                } else {
                                                    panic!()
                                                }
                                            } else {
                                            }
                                            */
                                        } else {
                                            use ansi_term::Colour::Red;
                                            println!(
                                                "{} {}:{}:0\nvarabel '{}' tidak ditemukan ?\n",
                                                Red.paint("Error!"),
                                                json["nilai"][i]["nama_file"].as_str().unwrap(),
                                                json["nilai"][i]["nomer_baris"].as_u64().unwrap(),
                                                nama
                                            );
                                            kirim_json
                                                .send((
                                                    [
                                                        "".to_string(),
                                                        "".to_string(),
                                                        "".to_string(),
                                                    ],
                                                    true,
                                                ))
                                                .unwrap();

                                            kirim!(kom, kirim_pwa, Box::new( Pohon::kopilasi_error ) /**/);
                                            return;
                                        }
                                        
                                    } else if tipe == "langsung" {
                                        x += 1;
                                        if let Some(_str_) = json["nilai"][i]["nilai"][x].as_str() {
                                            //format
                                            if hasil.is_empty() {
                                                if _str_.to_string().contains("{}") {
                                                    x += 1;
                                                    break;
                                                }
                                            }
                                            kirim!(
                                                kom,
                                                kirim_pwa,
                                                Box::new(
                                                Pohon::cetak(Nilai::langsung_str(
                                                    _str_.to_string()
                                                ))) /**/
                                            );
                                            hasil.push(Nilai::None);
                                        } else {
                                            panic!()
                                        }
                                    } else {
                                        panic!()
                                    }
                                    x += 1;
                                }
                                /*
                                loop {
                                    /*
                                    if let Some(tipe) = json["nilai"][i]["nilai"][x].as_str() {
                                        match tipe {
                                            "var" => {
                                                x += 1;
                                                if let Some(nama) =
                                                    json["nilai"][i]["nilai"][x].as_str()
                                                {
                                                    if let Some(var) = allokasi.get_mut(nama) {
                                                        if true {
                                                            match &var.tipe {
                                                                crate::parsing::Tipe::_u8(u) => {
                                                                    if u.len() == 1 {
                                                                        hasil.push(
                                                                            Nilai::langsung_int(
                                                                                u[0].unwrap()
                                                                                    as u64,
                                                                            ),
                                                                        );
                                                                    }
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                        var.sudah_dibaca = true;
                                                    } else {
                                                        use ansi_term::Colour::Red;
                                                        kirim_json
                                                            .send((
                                                                [
                                                                    "".to_string(),
                                                                    "".to_string(),
                                                                    "".to_string(),
                                                                ],
                                                                true,
                                                            ))
                                                            .unwrap();
                                                        kirim!(
                                                            kom,
                                                            kirim_pwa,
                                                            Pohon::kopilasi_error /**/
                                                        );
                                                        println!(
                                                            "{} {}:{}:0\nvarabel '{}' tidak ditemukan ?\n",
                                                            Red.paint("Error!"),
                                                            json["nilai"][i]["nama_file"]
                                                                .as_str()
                                                                .unwrap(),
                                                            json["nilai"][i]["nomer_baris"]
                                                                .as_u64()
                                                                .unwrap(),
                                                            nama
                                                        );
                                                        return;
                                                    }
                                                } else {
                                                    panic!()
                                                }
                                            }
                                            "langsung" =>{

                                            }
                                            _ => {
                                                panic!()
                                            }
                                        }
                                    } else {
                                        break;
                                    }
                                    */
                                    x += 1;
                                }
                                */
                                /*
                                if hasil.len() == 0 {
                                    kirim!(kom, kirim_pwa, Pohon::cetak(Nilai::None) /**/);
                                }
                                if let Nilai::langsung_str(str_) = &hasil[0] {
                                    if hasil.len() == 1 {
                                        kirim!(
                                            kom,
                                            kirim_pwa,
                                            Pohon::cetak(Nilai::langsung_str(str_.clone())) /**/
                                        );
                                    } else {
                                        for i in 1..hasil.len() {}
                                    }
                                } else {
                                    for i in hasil {
                                        match i {
                                            Nilai::None => {
                                                panic!()
                                            }
                                            _ => {
                                                kirim!(kom, kirim_pwa, Pohon::cetak(i) /**/);
                                            }
                                        }
                                    }
                                }
                                */
                                /*
                                let mut hasil: Vec<Nilai> = Vec::with_capacity(1);
                                let mut x = 0;
                                let mut cek_format: Option<(
                                    /*jumlah total*/ usize,
                                    /*jumlah yang masuk*/ usize,
                                )> = None;
                                loop {
                                    if let Some(tipe) = json["nilai"][i]["nilai"][x].as_str() {
                                        match tipe {
                                            "var" => {
                                                //cara variabel
                                                if let Some(var) = allokasi.get_mut(
                                                    &json["nilai"][i]["nilai"][x + 1]
                                                        .as_str()
                                                        .unwrap()
                                                        .to_string(),
                                                ) {
                                                    var.sudah_dibaca = true
                                                } else {
                                                    use ansi_term::Colour::Red;
                                                    kirim_json
                                                        .send((["".to_string(), "".to_string(), "".to_string()], true))
                                                        .unwrap();
                                                    kirim!(kom, kirim_pwa, Pohon::kopilasi_error/**/ );
                                                    println!("{} {}:{}:0\n\n",Red.paint("Error!"),
                                                        json["nilai"][i]["nama_file"].as_str().unwrap(),
                                                        json["nilai"][i]["nomer_baris"].as_u64().unwrap()
                                                    );
                                                    return
                                                }
                                            }
                                            _ => {
                                                panic!()
                                            }
                                        }
                                    } else {
                                        //jika catak kosong print baris baru "/n"
                                        if hasil.len() == 0 {
                                            kirim!(
                                                kom,
                                                kirim_pwa,
                                                //print baris baru
                                                Pohon::cetak(Nilai::None)
                                            )
                                        } else {
                                            hasil.into_iter().for_each(|f| {});
                                        }
                                        break;
                                    }
                                    x += 2
                                }
                                */
                                //panic!();
                            }
                            "blok" => {
                                kirim!(
                                    kom,
                                    kirim_pwa,
                                    Box::new(
                                    Pohon::blok(
                                        json["nilai"][i]["nama"].as_str().unwrap().to_string()
                                    ))
                                );
                            }
                            "blok_" => {
                                kirim!(kom, kirim_pwa, Box::new(Pohon::blok_));
                            }
                            "putus" => {
                                kirim!(kom, kirim_pwa, Box::new( Pohon::putus));
                                /*
                                if let Some(o) = swict.last() {
                                    if o.2 && !o.4 && o.0 == 0 {
                                        pohon.push(Pohon::br("1".to_string()));
                                        i += 1;
                                        continue;
                                    }
                                }
                                pohon.push(Pohon::putus)
                                */
                            }
                            /*
                            "putar" => kirim!(kom, kirim_pwa, Pohon::putar),
                            "batas" => kirim!(kom, kirim_pwa, Pohon::putar),
                            "lanjut" => kirim!(kom, kirim_pwa, Pohon::putar),
                            */
                            /*
                            "putar" => kirim(Pohon::putar,kom,&kirim_pwa),
                            "batas" => kirim(Pohon::batas,kom,&kirim_pwa),
                            "lanjut" => kirim(Pohon::lanjut,kom,&kirim_pwa),
                            */
                            _ => {
                                panic!()
                            }
                        }
                        /*
                        match o {
                            "putar" => kirim_pwa.send(Pohon::putar),
                            "batas" => kirim_pwa.send(Pohon::batas),
                            "batas" => kirim_pwa.send(Pohon::batas),
                            "lanjut" => kirim_pwa.send(Pohon::lanjut),
                            _=>{}
                        }
                        */
                    }
                    None => {
                        //allokasi memori untuk funsi ini
                        kirim!(terakhir kom, kirim_pwa,Box::new( Pohon::selesai ), &allokasi/**/ );
                        //untuk fungsi lain
                        if data.len() != 1 {
                            allokasi.clear();
                            continue 'awal;
                        }
                        break 'awal;
                    }
                }
                i += 1
            }
        } else {
            break;
        }
    }
    kirim!(kom, kirim_pwa, Box::new( Pohon::selesai));
    kirim_json
        .send((["".to_string(), "".to_string(), "".to_string()], true))
        .unwrap();
    //kirim!(terakhir kom, kirim_pwa, Pohon::selesai,allokasi/**/ );
    //
    /*
    allokasi.iter().for_each(|F|if let Pohon::_let(_,_,_,d,_) = F.1{
        if *d {
            kirim!(kom, kirim_pwa, F.1.clone()/**/ )
        }
    }else {panic!()});
    kirim!(kom, kirim_pwa, Pohon::selesai/**/ );
    */
}
