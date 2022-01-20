use crate::parsing::perintah;
use crate::parsing::Tipe;


macro_rules! operasi_logika_aritmatik {
    ($v:expr) => {
        let mut i = 0;
        while $v.len() > i {
            match $v[i].as_str() {
                "=="=>{
                    match ($v[i-1].as_str(), $v[i + 1].as_str()) {
                        ("benar","benar")|("salah","salah")=>{
                            $v.remove(i);
                            $v.remove(i);
                            $v[i-1] = "benar".to_string();
                            continue
                        }
                        ("benar","salah")|("salah","benar")=>{
                            $v.remove(i);
                            $v.remove(i);
                            $v[i-1] = "salah".to_string();
                            continue
                        }
                        _=>{}
                    }
                }
                "!=" =>{
                    match ($v[i-1].as_str(), $v[i + 1].as_str()) {
                        ("benar","benar")|("salah","salah")=>{
                            $v.remove(i);
                            $v.remove(i);
                            $v[i-1] = "salah".to_string();
                            continue
                        }
                        ("benar","salah")|("salah","benar")=>{
                            $v.remove(i);
                            $v.remove(i);
                            $v[i-1] = "benar".to_string();
                            continue
                        }
                        _=>{}
                    }
                }
                "!" =>{
                    match $v[i + 1].as_str(){
                        "benar"=>{
                            $v.remove(i);
                            $v[i] = "salah".to_string();
                            i += 2;
                            continue
                        }
                        "salah"=>{
                            $v.remove(i);
                            $v[i] = "benar".to_string();
                            i += 2;
                            continue
                        }
                        _=>{}
                    }
                }
                _=>{}
            }
            i += 1
        }
    };
}
pub fn parse(
    baris:std::sync::mpsc::Receiver<perintah>,
    kirim:std::sync::mpsc::Sender<String>
){
    let mut fn_ = [false,false];
    //let mut blok = 0;
    let mut lewati_jika = (false,0);
    'l0:loop {
        match baris.recv().unwrap(){
            //perintah::konst(_,_)|perintah::glob_var(_,_,_)=>{}
            //perintah::_i32_konst(_)=>{}
            perintah::_let(nama,_mut,tipe)=>{
                //panic!();
                kirim.send(
                    format!("{}{{\"tipe\":\"let\",\"tipe_\":\"{},\"nama\":\"{}\"{}}}",
                        if fn_[1] {","} else { fn_[1] = true ; "" },
                        match tipe {
                            Tipe::_string(o)=>{
                                if let crate::parsing::Str::Some(o) = o {
                                    format!("str_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "str\"".to_string()
                                }
                            }
                            Tipe::nomer(_)=>{
                                "nomer\"".to_string()
                            }
                            Tipe::_i8(o)=>{
                                if let Some(o) = o {
                                    format!("i8_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "i8\"".to_string()
                                }
                            }
                            Tipe::_u8(o)=>{
                                if let Some(o) = o {
                                    format!("u8_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "u8\"".to_string()
                                }
                            }
                            Tipe::_i16(o)=>{
                                if let Some(o) = o {
                                    format!("i16_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "i16\"".to_string()
                                }
                            }
                            Tipe::_u16(o)=>{
                                if let Some(o) = o {
                                    format!("u16_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "u16\"".to_string()
                                }
                            }
                            Tipe::_i32(o)=>{
                                if let Some(o) = o {
                                    format!("i32_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "i32\"".to_string()
                                }
                            }
                            Tipe::_u32(o)=>{
                                if let Some(o) = o {
                                    format!("u32_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "u32\"".to_string()
                                }
                            }
                            Tipe::_i64(o)=>{
                                if let Some(o) = o {
                                    format!("i64_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "i64\"".to_string()
                                }
                            }
                            Tipe::_u64(o)=>{
                                if let Some(o) = o {
                                    format!("u64_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "u64\"".to_string()
                                }
                            }
                            Tipe::penujuk_(o)=>{
                                format!("penujuk_\",\"nilai\":\"{}\"",o)
                            }
                            Tipe::minta(o)=>{
                                format!("minta\",\"nilai\":\"{}\"",o)
                            }
                            Tipe::None => {
                                format!("None\"")
                            }
                        },
                        nama,
                        if _mut {",\"mut\":\"\""} else {""}
                    )
                ).unwrap()
            }
            /*
            perintah::konst(l,o)=>{
                //println!("konstan !!!");
                kirim.send(
                    format!("{}{{\"tipe\":\"konst\",\"nilai\":\"{}\",\"nama\":\"{}\"}}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    match o {
                        Tipe::_string(o)=>{

                            let mut v = o.clone();
                            v.remove(0);
                            v.pop();
                            v.push_str("str");
                            v
                        },
                        Tipe::nomer(o)=>{
                            let mut v = o;
                            v.push_str("nomer");
                            v
                        }
                        //sementara
                        _=>{panic!()}
                    },
                    l
                    )
                ).unwrap()
            }
            perintah::glob_var(nama,_mut,tipe)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"glob\",\"tipe\":\"{}\",\"nama\":\"{}\",\"mut\":{}}}",
                        if fn_[1] {","} else { fn_[1] = true ; "" },
                        match tipe {
                            Tipe::_string(_)=>{"string"}
                            Tipe::nomer(_)=>{"nomer"}
                            Tipe::_i8(o)=>{"i8"}
                            Tipe::_u8(o)=>{"u8"}
                            Tipe::_i16(o)=>{"i16"}
                            Tipe::_u16(o)=>{"u16"}
                            Tipe::_i32(o)=>{"i32"}
                            Tipe::_u32(o)=>{"u32"}
                            Tipe::_i64(o)=>{"i64"}
                            Tipe::_u64(o)=>{"u64"}
                        },
                        nama,
                        _mut
                    )
                ).unwrap()
            }
            */
            perintah::putar=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"putar\"}}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    )
                ).unwrap()
            }
            perintah::putus=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"putus\"}}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    )
                ).unwrap()
            }
            perintah::lanjut=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"lanjut\"}}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    )
                ).unwrap()
            }
            perintah::batas=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"batas\"}}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    )
                ).unwrap()
            }
            perintah::variabel_null(a,b)=>{
                kirim.send(
                    format!("{}{{ \"tipe\":\"var\",\"data\":\"{}\",\"nama\":\"{}\" }}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    a,b
                    )
                ).unwrap()
            }
            perintah::cetak(_nilai)=>{
                let mut d = _nilai;
                kirim.send(
                    format!("{}{{ \"tipe\" : \"cetak\", \"nilai\": [\"{}\",\"{}\"] }}",
                        if fn_[1] {","} else { fn_[1] = true ; "" },
                        if d.starts_with("\""){
                            //println!("string");
                            d.remove(0);
                            d.pop();
                            "lansung"
                        }else if d.parse::<u64>().is_ok(){
                            "langsung_int"
                        } else if d.parse::<f64>().is_ok() {
                            "langsung_f"
                        } else {
                            "var"
                        },
                        d
                    )
                ).unwrap();
            }
            //perintah::arit_tambah(a,b,c)=>{
            //}
            perintah::boolean(o)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"{}\"}}",if fn_[1] {","}else{fn_[1] = true;""},
                        if o {
                            "benar"
                        } else {
                            "salah"
                        }
                    )
                ).unwrap()
            }
            perintah::swict(x,mut y)=>{
                operasi_logika_aritmatik!(y);
                kirim.send(
                    format!("{}{{\"tipe\":\"swict\",\"jenis\":\"{}\",\"nilai\":{:?}}}",if fn_[1] {","}else{fn_[1] = true;""},x,y)
                ).unwrap();
            }
            perintah::kasus(x)=>{
                if let Some(x) = x {
                    kirim.send(
                        format!("{}{{\"tipe\":\"kasus\",\"lalu\":\"1\",\"nilai\":{:?}}}",if fn_[1] {","}else{fn_[1] = true;""},x)
                    ).unwrap();
                } else {
                    kirim.send(
                        format!("{}{{\"tipe\":\"kasus\",\"lalu\":\"0\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                    ).unwrap();
                }
                
            }
            perintah::swict_(x)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"swict_tutup\",\"jenis\":\"{}\"}}",if fn_[1] {","}else{fn_[1] = true;""},x)
                ).unwrap();
            }
            perintah::i32_eqz=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"_i32_eqz\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::i32_eq=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"_i32_eq\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::sama_lebih_besar=>{}
            perintah::sama_lebih_kecil=>{}
            perintah::lebih_besar=>{}
            perintah::lebih_kecil=>{}
            perintah::sama=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"sama\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::tidak_sama=>{}
            perintah::sama_dengan=>{}
            perintah::_i32_konst(o)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"_i32_konst\",\"nilai\":\"{}\"}}",if fn_[1] {","}else{fn_[1] = true;""},o)
                ).unwrap()
            }
            perintah::kurang=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"kurang\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::tambah=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"tambah\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::bagi=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"bagi\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::kali=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"kali\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::modus=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"modus\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::halaman(o)=>{
                /*
                let mut o = o.clone();
                let x = if o.starts_with("\"") {
                    o.remove(0);
                    o.pop();
                    "0"
                } else {
                    "1"
                };
                */
                let mut y = o.clone();
                let x = if y.starts_with("\"") && y.ends_with("\"") {
                    y.remove(0);
                    y.pop();
                    "0"
                } else {
                    "1"
                };
                kirim.send(
                    format!("{}{{\"tipe\":\"halaman\",\"nilai\":\"{}\",\"var\":\"{}\"}}",if fn_[1] {","} else { fn_[1] = true ; ""},y,
                    x
                )
                ).unwrap()
            }
            perintah::navbar(id)=>{

            }
            perintah::navbar_tombol(id,nav_id,fn__)=>{
                
            }
            perintah::warnalatarbelakang(o)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"warnalatarbelakang\",\"nilai\":\"{}\"}}",if fn_[1] {","} else { fn_[1] = true ; ""},o)
                ).unwrap()
            }
            perintah::gambarlatarbelakang(o)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"gambarlatarbelakang\",\"nilai\":\"{}\"}}",if fn_[1] {","} else { fn_[1] = true ; ""},o)
                ).unwrap()
            }
            perintah::judul(o)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"judul\",\"nilai\":\"{}\"}}",if fn_[1] {","} else { fn_[1] = true ; ""},o)
                ).unwrap()
            }
            perintah::tombol(o)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"tombol\",\"nilai\":\"{}\"}}",if fn_[1] {","} else { fn_[1] = true ; ""},o)
                ).unwrap()
            }
            perintah::klik(id,fungsi)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"klik\",\"nilai\":\"{}\",\"fn\":{:?}}}",if fn_[1] {","} else { fn_[1] = true ; ""},id,fungsi)
                ).unwrap()
            }
            perintah::isi(o,l)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"isi\",\"nilai\":[\"{}\",\"{}\"]}}",if fn_[1] {","} else { fn_[1] = true ; ""},o,l)
                ).unwrap()
            }
            perintah::warna(o,l)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"warna\",\"nilai\":[\"{}\",\"{}\"]}}",if fn_[1] {","} else { fn_[1] = true ; ""},o,l)
                ).unwrap()
            }
            perintah::warnalatarbelakangid(o,l)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"warnalatarbelakangid\",\"nilai\":[\"{}\",\"{}\"]}}",if fn_[1] {","} else { fn_[1] = true ; ""},o,l)
                ).unwrap()
            }
            perintah::ukurankata(o,l)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"ukurankata\",\"nilai\":[\"{}\",\"{}\"]}}",if fn_[1] {","} else { fn_[1] = true ; ""},o,l)
                ).unwrap()
            }
            perintah::tulis(a,b)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"tulis\",\"nama\":\"{}\",\"nilai\":{:?}}}",
                    if fn_[0] {","} else { fn_[0] = true ; "" },
                    a,
                    b
                    )
                ).unwrap()
            }
            perintah::cpu(nama,publik)=>{
                kirim.send(
                    format!("{}{{\"fn\":\"{}\",\"publik\":{},\"nilai\":[",
                    if fn_[0] { fn_[1] = false;"]},"} else { fn_[0] = true ; "" }
                    ,nama,publik)
                ).unwrap()
            }
            perintah::panggil_fn(nama,tipe)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"panggil_fn\",\"nama\":\"{}\"}}",if fn_[1] {","}else{fn_[1] = true;""}
                        ,{
                            let mut v = nama[0].clone();
                            for i in 1..nama.len() {
                                v.push_str("_" );
                                v.push_str(&nama[i])
                            }
                            v
                        }                    
                    )
                ).unwrap()
            }
            perintah::modul_masuk(nama)=>{
                kirim.send(
                    format!("{{\"mod\":\"{}\",\"nilai\":[",nama)
                ).unwrap()
            }
            perintah::modul_keluar=>{
                kirim.send(
                    format!("{}}}\n",if fn_[0] {fn_ = [false,false] ; "]}]"} else {"]"})
                ).unwrap()
            }
            perintah::blok(a)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"blok\",\"nilai\":\"{}\"}}",if fn_[1] {","} else { fn_[1] = true ; ""},a)
                ).unwrap()
            }
            perintah::br(o)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"br\",\"nilai\":\"{}\"}}",if fn_[1] {","} else { fn_[1] = true ; ""},o)
                ).unwrap()
            }
            perintah::if_br(a,b)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"if_br\",\"nilai\":[\"{}\",\"{}\"]}}",if fn_[1] {","} else { fn_[1] = true ; ""},a,b)
                ).unwrap()
            }
            perintah::blok_=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"blok_\"}}",if fn_[0] {","} else { fn_[0] = true ; ""})
                ).unwrap()
            }
            perintah::selesai=>{
                kirim.send("".to_string()).unwrap();
                break
            }
        }
    }
}