use crate::parsing::args::*;
use crate::parsing::perintah;
use crate::parsing::Tipe;

macro_rules! operasi_logika_aritmatik {
    ($v:expr) => {
        let mut i = 0;
        while $v.len() > i {
            match $v[i].as_str() {
                "==" => match ($v[i - 1].as_str(), $v[i + 1].as_str()) {
                    ("benar", "benar") | ("salah", "salah") => {
                        $v.remove(i);
                        $v.remove(i);
                        $v[i - 1] = "benar".to_string();
                        continue;
                    }
                    ("benar", "salah") | ("salah", "benar") => {
                        $v.remove(i);
                        $v.remove(i);
                        $v[i - 1] = "salah".to_string();
                        continue;
                    }
                    _ => {}
                },
                "!=" => match ($v[i - 1].as_str(), $v[i + 1].as_str()) {
                    ("benar", "benar") | ("salah", "salah") => {
                        $v.remove(i);
                        $v.remove(i);
                        $v[i - 1] = "salah".to_string();
                        continue;
                    }
                    ("benar", "salah") | ("salah", "benar") => {
                        $v.remove(i);
                        $v.remove(i);
                        $v[i - 1] = "benar".to_string();
                        continue;
                    }
                    _ => {}
                },
                "!" => match $v[i + 1].as_str() {
                    "benar" => {
                        $v.remove(i);
                        $v[i] = "salah".to_string();
                        i += 2;
                        continue;
                    }
                    "salah" => {
                        $v.remove(i);
                        $v[i] = "benar".to_string();
                        i += 2;
                        continue;
                    }
                    _ => {}
                },
                _ => {}
            }
            i += 1
        }
    };
}
pub fn parse(
    baris: std::sync::mpsc::Receiver<(usize, String, perintah)>,
    kirim: std::sync::mpsc::Sender<String>,
) {
    let mut fn_ = [false, false];
    //let mut blok = 0;
    let mut _blok_: Vec<bool> = Vec::with_capacity(5);
    let mut lewati_jika = (false, 0);
    let mut buf = String::with_capacity(200);
    //let mut lapisan :Vec<bool>= Vec::with_capacity(4);
    let mut putar = false;
    'l0: loop {
        let perintah_masuk = baris.recv().unwrap();
        let nomer_baris = perintah_masuk.0;
        let nama_file = perintah_masuk.1;
        let mut comma = false;
        /*if let Some(b) = _blok_.last_mut() {
            if *b {
                buf.push(',')
            } else {
                *b = true
            }
            fn_[1] = true;
        } else if fn_[1] {
            buf.push(',')
        } else {
            fn_[1] = true;
        }
        */
        match perintah_masuk.2 {
            //perintah::konst(_,_)|perintah::glob_var(_,_,_)=>{}
            //perintah::_i32_konst(_)=>{}
            perintah::jika(logika) => {
                if let Some(coma) = _blok_.last_mut() {
                    if *coma {
                        buf.push(',')
                    } else {
                        *coma = true
                    }
                } else if fn_[1] && !putar {
                    buf.push(',')
                } else {
                    fn_[1] = true
                }
                buf.push_str("{\"nomer_baris\":");
                buf.push_str(&format!("{}", nomer_baris));
                buf.push_str(",\"nama_file\":\"");
                buf.push_str(&nama_file);
                buf.push_str("\",\"tipe\":\"jika\",\"nilai\":[");
                for i in 0..logika.len() {
                    if i != 0 {
                        buf.push(',')
                    }
                    buf.push_str(&logika[i]);
                }
                buf.push_str("]}");
                kirim.send(buf.clone()).unwrap();
            }
            perintah::_let(nama, mut_, tipe) => {
                if let Some(coma) = _blok_.last_mut() {
                    if *coma {
                        buf.push(',')
                    } else {
                        *coma = true
                    }
                } else if fn_[1] && !putar {
                    buf.push(',')
                } else {
                    fn_[1] = true
                }
                buf.push_str("{\"nomer_baris\":");
                buf.push_str(&format!("{}", nomer_baris));
                buf.push_str(",\"nama_file\":\"");
                buf.push_str(&nama_file);
                buf.push_str("\",\"tipe\":\"var\",\"nama\":\"");
                buf.push_str(&nama);
                buf.push_str("\",\"mut\":\"");
                match mut_ {
                    crate::parsing::mem_mode::imutabel => buf.push_str("i"),
                    crate::parsing::mem_mode::mutabel => buf.push_str("m"),
                    crate::parsing::mem_mode::statik => buf.push_str("s"),
                }
                buf.push_str("\",\"mem\":{\"");
                match tipe {
                    crate::parsing::Tipe::int(unsigh, uint) => {
                        //let nilai;
                        match unsigh {
                            crate::parsing::mem_tipe::unint(v) => {
                                //nilai = v.unwrap().len(). ;
                                buf.push_str("uint\":");
                                buf.push_str(&format!("{}", uint));
                                buf.push_str("},\"nilai\":[");
                                if let Some(v) = v {
                                    buf.push_str(&format!("{}", v.len()));
                                    v.into_iter().for_each(|i| {
                                        buf.push(',');
                                        if let Some(t) = i {
                                            buf.push_str(&format!("{}", t));
                                        } else {
                                            buf.push_str("none");
                                        }
                                    });
                                } else {
                                    panic!()
                                }
                            }
                            crate::parsing::mem_tipe::int(v) => {
                                //nilai = v.unwrap().len() ;buf.push_str("int\":");
                                buf.push_str(&format!("{}", uint));
                                buf.push_str("},\"nilai\":[");
                                if let Some(v) = v {
                                    buf.push_str(&format!("{}", v.len()));
                                    v.into_iter().for_each(|i| {
                                        buf.push(',');
                                        if let Some(t) = i {
                                            buf.push_str(&format!("{}", t));
                                        } else {
                                            buf.push_str("none");
                                        }
                                    });
                                } else {
                                    panic!()
                                }
                            }
                            crate::parsing::mem_tipe::float(v) => {
                                //nilai = v.unwrap().len() ;
                                buf.push_str("float\":");
                                buf.push_str(&format!("{}", uint));
                                buf.push_str("},\"nilai\":[");
                                if let Some(v) = v {
                                    buf.push_str(&format!("{}", v.len()));
                                    v.into_iter().for_each(|i| {
                                        buf.push(',');
                                        if let Some(t) = i {
                                            buf.push_str(&format!("{}", t));
                                        } else {
                                            buf.push_str("none");
                                        }
                                    });
                                } else {
                                    panic!()
                                }
                            }
                        }
                    }
                    _ => panic!(),
                }

                //buf.push_str("},\"nilai\":[");
                buf.push_str("]}");
                kirim.send(buf.clone()).unwrap();
            }
            perintah::blok(nama) => {
                if putar {
                    putar = false;
                    continue;
                }
                if let Some(coma) = _blok_.last_mut() {
                    if *coma {
                        buf.push(',')
                    } else {
                        *coma = true
                    }
                } else if fn_[1] {
                    buf.push(',')
                } else {
                    fn_[1] = true
                }
                _blok_.push(false);
                buf.push_str("{\"nomer_baris\":");
                buf.push_str(&format!("{}", nomer_baris));
                buf.push_str(",\"nama_file\":\"");
                buf.push_str(&nama_file);
                buf.push_str("\",\"tipe\":\"blok\",\"nama\":\"");
                buf.push_str(&nama);
                buf.push_str("\",\"nilai\":[");
                kirim.send(buf.clone()).unwrap();
            }
            perintah::blok_ => {
                _blok_.pop();
                buf.push_str("]}");
                kirim.send(buf.clone()).unwrap();
            }
            perintah::cetak(data) => {
                if let Some(coma) = _blok_.last_mut() {
                    if *coma {
                        buf.push(',')
                    } else {
                        *coma = true
                    }
                } else if fn_[1] {
                    buf.push(',')
                } else {
                    fn_[1] = true
                }
                buf.push_str("{\"nomer_baris\":");
                buf.push_str(&format!("{}", nomer_baris));
                buf.push_str(",\"nama_file\":\"");
                buf.push_str(&nama_file);
                buf.push_str("\",\"tipe\":\"cetak\",\"nilai\":[");
                for i in 0..data.len() {
                    if i != 0 {
                        buf.push(',')
                    }
                    if data[i].starts_with("\"") {
                        buf.push_str("\"langsung\",");
                        buf.push_str(&data[i]);
                    } else {
                        buf.push_str("\"var\",\"");
                        buf.push_str(&data[i]);
                        buf.push('\"');
                    }
                }
                buf.push_str("]}");
                kirim.send(buf.clone()).unwrap();
            }
            perintah::cpu(nama, publik, args, kembali) => {
                if fn_[0] {
                    buf.push_str("]},");
                } else {
                    fn_[0] = true
                }
                buf.push_str("{\"nomer_baris\":");
                buf.push_str(&format!("{}", nomer_baris));
                buf.push_str(",\"nama_file\":\"");
                buf.push_str(&nama_file);
                buf.push_str("{\",\"fn\":\"");
                buf.push_str(&nama);
                buf.push_str("\",\"publik\":");
                buf.push_str(if publik { "true" } else { "false" });
                buf.push_str(",\"nilai\":[");
                kirim.send(buf.clone()).unwrap();
            }
            perintah::modul_masuk(nama) => {
                kirim
                    .send(format!(
                        "{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"mod\":\"{}\",\"nilai\":[",
                        nomer_baris, nama_file, nama
                    ))
                    .unwrap();
            }
            perintah::modul_keluar => {
                /*
                if fn_[0]{
                    buf.push_str("]}")
                }
                */
                kirim.send("]}]}\n".to_string()).unwrap();
                fn_ = [false, false]
            }
            perintah::tidur(arg) => {
                kirim
                    .send(format!(
                        "{}{{\"tipe\":\"tidur\",\"tidur\":{} }}",
                        if let Some(coma) = _blok_.last_mut() {
                            if *coma {
                                ','
                            } else {
                                *coma = true;
                                ' '
                            }
                        } else if fn_[1] && !putar {
                            ','
                        } else {
                            fn_[1] = true;
                            ' '
                        },
                        match arg {
                            Int(_, _, _, _, _, _) => {
                                panic!()
                            }
                            Str_Lansung(_) => {
                                panic!()
                            }
                            Str_int(I) => format!("{}", I),
                            penunjuk(_) => {
                                panic!()
                            }
                            penunjuk_nama(_) =>{
                                panic!()
                            }
                            null =>{
                                panic!()
                            }
                            internar_memory(_)=>{
                                panic!()
                            }
                        }
                    ))
                    .unwrap();
            }
            perintah::putar(v) => {
                putar = true;
                kirim
                    .send(format!(
                        "{}{{\"tipe\":\"putar\",\"arg\":[",
                        if let Some(coma) = _blok_.last_mut() {
                            if *coma {
                                ','
                            } else {
                                *coma = true;
                                ' '
                            }
                        } else if fn_[1]
                        /*&& !putar*/
                        {
                            ','
                        } else {
                            fn_[1] = true;
                            ' '
                        }
                    ))
                    .unwrap();
                //let mut comma = false;
                if !v.is_empty() {
                    match &v[0] {
                        Int(_, _, _, _, _, _) => {
                            panic!()
                        }
                        Str_Lansung(_) => {
                            panic!()
                        }
                        Str_int(int) => {
                            kirim
                                .send(format!(
                                    "{}{}", int,""
                                ))
                                .unwrap();
                        }
                        penunjuk(_) => {
                            panic!()
                        }
                        penunjuk_nama(nama_variabel_loop) => {
                            match &v[1] {
                                Str_int(int) => {
                                    kirim.send(format!("\"iter\",\"{}\",{}",nama_variabel_loop,int)).unwrap();
                                }
                                _=>{panic!()}
                            }
                        }
                        null=>{
                            panic!()
                        }
                        internar_memory(_)=>{
                            panic!()
                        }
                    }
                } 
                kirim.send("],\"nilai\":[".to_string()).unwrap();
                _blok_.push(false);
                continue;
            }
            perintah::tulis(nama_var, arg) => {
                kirim
                    .send(format!(
                        "{}{{\"tipe\":\"tulis\",\"nama\":\"{}\",\"nilai\":[",
                        if let Some(coma) = _blok_.last_mut() {
                            if *coma {
                                ','
                            } else {
                                *coma = true;
                                ' '
                            }
                        } else if fn_[1] {
                            ','
                        } else {
                            fn_[1] = true;
                            ' '
                        },
                        nama_var,
                    ))
                    .unwrap();
                for i in 0..arg.len() {
                    if i != 0 {kirim.send(",".to_string()).unwrap();}
                    if let Ok(angka) = arg[i].parse::<isize>() {
                        kirim.send(format!("{}", angka)).unwrap()
                    } else {
                        kirim.send(format!("\"{}\"", arg[i])).unwrap()
                    }
                }
                kirim.send("]}".to_string()).unwrap();
            }
            /* versi lama jangan di hapus
            perintah::_let(nama, _mut, tipe) => {
                //panic!();
                kirim
                    .send(format!(
                        "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"let\",\"tipe_\":\"{},\"nama\":\"{}\"{}}}",
                        if fn_[1] {
                            ","
                        } else {
                            fn_[1] = true;
                            ""
                        },
                        nomer_baris,
                        nama_file,
                        match tipe {
                            Tipe::_string(o) => {
                                if let crate::parsing::Str::Some(o) = o {
                                    format!("str_\",\"nilai\":{}", o)
                                } else {
                                    "str\"".to_string()
                                }
                            }
                            Tipe::nomer(_) => {
                                "nomer\"".to_string()
                            }
                            Tipe::_u8(o) => {
                                let mut v = String::with_capacity(100);
                                v.push_str("u8_\",");
                                //v.push_str(&format!("u8_\",\"len\":\"{}\",", o.len()));
                                if o.len() == 1 {
                                    v.push_str("\"nilai\":");
                                    if let Some(d) = o[0] {
                                        v.push_str(&format!("{}",d))
                                    }
                                    //panic!()
                                }
                                v
                            }
                            Tipe::penujuk_(o, index) => {
                                format!(
                                    "penujuk_\",\"nilai\":\"{}\"{}",
                                    o,
                                    if let Some(ind) = index {
                                        format!(",\"index\":{}", ind)
                                    } else {
                                        "".to_string()
                                    }
                                )
                            }
                            Tipe::minta(o, index) => {
                                format!(
                                    "minta\",\"nilai\":\"{}\"{}",
                                    o,
                                    if let Some(ind) = index {
                                        format!(",\"index\":{}", ind)
                                    } else {
                                        "".to_string()
                                    }
                                )
                            }
                            Tipe::None => {
                                format!("None\"")
                            }
                            _ => {
                                panic!()
                            }
                        },
                        nama,
                        if _mut { ",\"mut\":\"\"" } else { "" }
                    ))
                    .unwrap()
            }
            perintah::putar => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",tipe\":\"putar\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file
                ))
                .unwrap(),
            perintah::putus => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"putus\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file
                ))
                .unwrap(),
            perintah::lanjut => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"lanjut\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file
                ))
                .unwrap(),
            perintah::batas => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"batas\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file
                ))
                .unwrap(),
            perintah::variabel_null(a, b) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"var\",\"data\":\"{}\",\"nama\":\"{}\" }}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    a,
                    b
                ))
                .unwrap(),
            perintah::cetak(_nilai) => {
                let mut v = format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"cetak\",\"nilai\":[",
                    if fn_[1] { "," } else { fn_[1] = true ; "" },
                    nomer_baris,
                    nama_file
                );
                let mut _tipe_ = String::new();
                let mut _nilai_ = String::new();
                _nilai.iter().for_each(|f| {
                    if f.starts_with("\"") {
                        if !f.ends_with("\"") {
                            panic!()
                        } else if _tipe_.is_empty() {
                            _tipe_.push_str("langsung")
                        } else if _tipe_ != "langsung" {
                            panic!()
                        }
                        _nilai_.push_str(&f[1..f.len() - 1]);
                    } else if f == "[" {

                    } else if let Ok(_) = f.parse::<u64>() {
                        if !_tipe_.is_empty() {
                            panic!()
                        }
                        _tipe_.push_str("langsung_int");
                        _nilai_.push_str(&f);
                    } else if let Ok(_) = f.parse::<f64>() {
                        if !_tipe_.is_empty() {
                            panic!()
                        }
                        _tipe_.push_str("langsung_f");
                        _nilai_.push_str(&f);
                    } else if _tipe_.is_empty() {
                        _tipe_.push_str("var");
                        _nilai_.push_str(&f);
                    } else {
                        println!("{:?}",_nilai);
                        panic!()
                    }
                });
                v.push('"');
                v.push_str(_tipe_.as_str());
                v.push('"');
                v.push(',');
                v.push('"');
                v.push_str(_nilai_.as_str());
                v.push('"');
                v.push_str("]}");
                kirim.send(v).unwrap();
                /*
                let mut d = _nilai[0].clone();
                kirim
                    .send(format!(
                        "{}{{ \"tipe\" : \"cetak\", \"nilai\": [\"{}\",\"{}\"] }}",
                        if fn_[1] {
                            ","
                        } else {
                            fn_[1] = true;
                            ""
                        },
                        if d.starts_with("\"") {
                            //println!("string");
                            d.remove(0);
                            d.pop();
                            "lansung"
                        } else if d.parse::<u64>().is_ok() {
                            "langsung_int"
                        } else if d.parse::<f64>().is_ok() {
                            "langsung_f"
                        } else {
                            "var"
                        },
                        d
                    ))
                    .unwrap();
                    */
            }
            //perintah::arit_tambah(a,b,c)=>{
            //}
            perintah::boolean(o) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"{}\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    if o { "benar" } else { "salah" }
                ))
                .unwrap(),
            perintah::swict(x, mut y) => {
                operasi_logika_aritmatik!(y);
                kirim
                    .send(format!(
                        "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"swict\",\"jenis\":\"{}\",\"nilai\":{:?}}}",
                        if fn_[1] {
                            ","
                        } else {
                            fn_[1] = true;
                            ""
                        },
                        nomer_baris,
                        nama_file,
                        x,
                        y
                    ))
                    .unwrap();
            }
            perintah::kasus(x) => {
                if let Some(x) = x {
                    kirim
                        .send(format!(
                            "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"kasus\",\"lalu\":\"1\",\"nilai\":{:?}}}",
                            if fn_[1] {
                                ","
                            } else {
                                fn_[1] = true;
                                ""
                            },
                            nomer_baris,
                            nama_file,
                            x
                        ))
                        .unwrap();
                } else {
                    kirim
                        .send(format!(
                            "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"kasus\",\"lalu\":\"0\"}}",
                            if fn_[1] {
                                ","
                            } else {
                                fn_[1] = true;
                                ""
                            },
                            nomer_baris,
                            nama_file
                        ))
                        .unwrap();
                }
            }
            perintah::swict_(x) => {
                kirim
                    .send(format!(
                        "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"swict_tutup\",\"jenis\":\"{}\"}}",
                        if fn_[1] {
                            ","
                        } else {
                            fn_[1] = true;
                            ""
                        },
                        nomer_baris,
                        nama_file,
                        x
                    ))
                    .unwrap();
            }
            perintah::i32_eqz => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"_i32_eqz\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file
                ))
                .unwrap(),
            perintah::i32_eq => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"_i32_eq\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file
                ))
                .unwrap(),
            perintah::sama_lebih_besar => {}
            perintah::sama_lebih_kecil => {}
            perintah::lebih_besar => {}
            perintah::lebih_kecil => {}
            perintah::sama => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"sama\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file
                ))
                .unwrap(),
            perintah::tidak_sama => {}
            perintah::sama_dengan => {}
            perintah::_i32_konst(o) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"_i32_konst\",\"nilai\":\"{}\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    o
                ))
                .unwrap(),
            perintah::kurang => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"kurang\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file
                ))
                .unwrap(),
            perintah::tambah => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"tambah\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file
                ))
                .unwrap(),
            perintah::bagi => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"bagi\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file
                ))
                .unwrap(),
            perintah::kali => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"kali\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file
                ))
                .unwrap(),
            perintah::modus => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"modus\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file
                ))
                .unwrap(),
            perintah::halaman(o) => {
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
                kirim
                    .send(format!(
                        "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"halaman\",\"nilai\":\"{}\",\"var\":\"{}\"}}",
                        if fn_[1] {
                            ","
                        } else {
                            fn_[1] = true;
                            ""
                        },
                        nomer_baris,
                    nama_file,
                        y,
                        x
                    ))
                    .unwrap()
            }
            perintah::navbar(id) => {}
            perintah::navbar_tombol(id, nav_id, fn__) => {}
            perintah::warnalatarbelakang(o) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"warnalatarbelakang\",\"nilai\":\"{}\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    o
                ))
                .unwrap(),
            perintah::gambarlatarbelakang(o) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"gambarlatarbelakang\",\"nilai\":\"{}\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    o
                ))
                .unwrap(),
            perintah::judul(o) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"judul\",\"nilai\":\"{}\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    o
                ))
                .unwrap(),
            perintah::tombol(o) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"tombol\",\"nilai\":\"{}\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    o
                ))
                .unwrap(),
            perintah::klik(id, fungsi) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"klik\",\"nilai\":\"{}\",\"fn\":{:?}}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    id,
                    fungsi
                ))
                .unwrap(),
            perintah::isi(o, l) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"isi\",\"nilai\":[\"{}\",\"{}\"]}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    o,
                    l
                ))
                .unwrap(),
            perintah::warna(o, l) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"warna\",\"nilai\":[\"{}\",\"{}\"]}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    o,
                    l
                ))
                .unwrap(),
            perintah::warnalatarbelakangid(o, l) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"warnalatarbelakangid\",\"nilai\":[\"{}\",\"{}\"]}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    o,
                    l
                ))
                .unwrap(),
            perintah::ukurankata(o, l) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"ukurankata\",\"nilai\":[\"{}\",\"{}\"]}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    o,
                    l
                ))
                .unwrap(),
            perintah::tulis(a, b) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"tulis\",\"nama\":\"{}\",\"nilai\":{:?}}}",
                    if fn_[0] {
                        ","
                    } else {
                        fn_[0] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    a,
                    b
                ))
                .unwrap(),
            perintah::cpu(nama, publik,args,kembali) => kirim
                .send(format!(
                    "{}{{\"args\":{:?},\"kembali\":{:?},\"nomer_baris\":{},\"nama_file\":\"{}\",\"fn\":\"{}\",\"publik\":{},\"nilai\":[",
                    if fn_[0] {
                        fn_[1] = false;
                        "]},"
                    } else {
                        fn_[0] = true;
                        ""
                    },
                    {
                        let mut v :Vec<String>= Vec::new();
                        args.into_iter().for_each(|i|{
                            match i {
                                crate::parsing::args::u8_(pointer, _mut_, nama)=>{
                                    v.push(nama);
                                    v.push("u8_".to_string());
                                    v.push(if pointer {"true"} else {"false"} .to_string());
                                    v.push(if _mut_ {"true"} else {"false"} .to_string());

                                }
                                crate::parsing::args::u8_ar(_, _, _, _) =>{

                                }
                                crate::parsing::args::u8_vec(_, _, _) =>{

                                }
                            }
                        });
                        v
                    },
                    {
                        let mut v :Vec<String>= Vec::new();
                        kembali.into_iter().for_each(|i|{
                            match i {
                                crate::parsing::args::u8_(pointer, _mut_, _)=>{
                                    v.push("u8_".to_string());
                                    v.push(if pointer {"true"} else {"false"} .to_string());
                                    v.push(if _mut_ {"true"} else {"false"} .to_string());

                                }
                                crate::parsing::args::u8_ar(_, _, _, _) =>{

                                }
                                crate::parsing::args::u8_vec(_, _, _) =>{

                                }
                            }
                        });
                        v
                    },
                    nomer_baris,
                    nama_file,
                    nama,
                    publik
                ))
                .unwrap(),
            perintah::panggil_fn(nama, tipe) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"panggil_fn\",\"nama\":\"{}\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    {
                        let mut v = nama[0].clone();
                        for i in 1..nama.len() {
                            v.push_str("_");
                            v.push_str(&nama[i])
                        }
                        v
                    }
                ))
                .unwrap(),
            perintah::modul_masuk(nama) => kirim
                .send(format!("{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"mod\":\"{}\",\"nilai\":[",
                nomer_baris,
                    nama_file,
                nama))
                .unwrap(),
            perintah::modul_keluar => kirim
                .send(format!(
                    "{}}}\n",
                    if fn_[0] {
                        fn_ = [false, false];
                        "]}]"
                    } else {
                        "]"
                    }
                ))
                .unwrap(),
            perintah::blok(a) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"blok\",\"nilai\":\"{}\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    a
                ))
                .unwrap(),
            perintah::br(o) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"br\",\"nilai\":\"{}\"}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    o
                ))
                .unwrap(),
            perintah::if_br(a, b) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"if_br\",\"nilai\":[\"{}\",\"{}\"]}}",
                    if fn_[1] {
                        ","
                    } else {
                        fn_[1] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file,
                    a,
                    b
                ))
                .unwrap(),
            perintah::blok_ => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"tipe\":\"blok_\"}}",
                    if fn_[0] {
                        ","
                    } else {
                        fn_[0] = true;
                        ""
                    },
                    nomer_baris,
                    nama_file
                ))
                .unwrap(),
            */
            perintah::selesai(error) => {
                if !error.is_empty() {
                    kirim.send("".to_string()).unwrap();
                    kirim.send(error).unwrap();
                } else {
                    kirim.send("".to_string()).unwrap();
                }
                break;
            }

            _ => {
                //panic!()
            }
        }
        if putar {
            kirim.send("]}".to_string()).unwrap();
            putar = false;
        }

        buf.clear()
    }
}
