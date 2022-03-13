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
    id: u64,
    tipe: Tipe,
    mut_: bool,
    sudah_dibaca: bool,
    sudah_ditulis: bool,
    nama_file: String,
    nomer_baris: u64,
    jika_vec: bool,
}
#[derive(Clone)]
pub enum Pohon {
    fungsi(String),
    tambah_fungsi(String),
    panggil_fn(String),
    cetak(Nilai),
    _let(let_),
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
    (terakhir $a:expr,$b:expr,$z:expr,$has:expr) => {
        {
            use ansi_term::Colour::Yellow;
            //pegiriman ke
            if $a.0 {

            }
            if $a.1 {
                $b.send($z).unwrap();
                $has.into_iter().for_each(|i|
                    if i.1.mut_ {
                        if i.1.sudah_ditulis {
                            if i.1.sudah_dibaca {
                                $b.send( Pohon::_let( i.1.clone() )).unwrap();
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
                $b.send($z).unwrap();
            }
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
    kirim_pwa: std::sync::mpsc::Sender<Pohon>,
    kirim_json: std::sync::mpsc::Sender<([std::string::String; 3], bool)>,
    kom: (bool, bool),
) {
    //panic!();
    //jika allokasi tidak pernah digunakan maka memori tidak diallokasi
    let mut allokasi: HashMap<String, let_> = HashMap::new();
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
                    kirim!(kom, kirim_pwa, Pohon::kopilasi_error);
                    return;
                }
            }
            //perpustaka
            kirim!(
                kom,
                kirim_pwa,
                Pohon::tambah_fungsi(format!(
                    "{}_{}_{}",
                    data[data_iter].0[0], data[data_iter].0[1], data[data_iter].0[2]
                ))
            );
        }
        if let Some(json) = terima.recv().unwrap() {
            for i in 0.. {
                match json["nilai"][i]["tipe"].as_str() {
                    Some(o) => {
                        match o {
                            "let" => {
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
                                let nilai_var = let_ {
                                    id,
                                    tipe: tipe_,
                                    mut_: mut_,
                                    sudah_dibaca: false,
                                    sudah_ditulis: false,
                                    nama_file: nama_file,
                                    nomer_baris: nomer_baris,
                                    jika_vec: jika_vec,
                                };
                                allokasi.insert(
                                    json["nilai"][i]["nama"].as_str().unwrap().to_string(),
                                    nilai_var,
                                );
                                id += 1;
                            }
                            "cetak" => {
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

                                //panic!();
                            }
                            "blok" => {
                                kirim!(
                                    kom,
                                    kirim_pwa,
                                    Pohon::blok(
                                        json["nilai"][i]["nilai"].as_str().unwrap().to_string()
                                    )
                                );
                            }
                            "blok_" => {
                                kirim!(kom, kirim_pwa, Pohon::blok_);
                            }
                            "putus" => {
                                kirim!(kom, kirim_pwa, Pohon::putus);
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
                            "putar" => kirim!(kom, kirim_pwa, Pohon::putar),
                            "batas" => kirim!(kom, kirim_pwa, Pohon::putar),
                            "lanjut" => kirim!(kom, kirim_pwa, Pohon::putar),
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
                        kirim!(terakhir kom, kirim_pwa, Pohon::selesai, &allokasi/**/ );
                        //untuk fungsi lain
                        if data.len() != 1 {
                            allokasi.clear();
                            continue 'awal;
                        }
                        break 'awal;
                    }
                }
            }
        } else {
            break;
        }
    }
    kirim!(kom, kirim_pwa, Pohon::selesai);
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
