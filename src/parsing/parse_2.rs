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
    baris: std::sync::mpsc::Receiver<(u64, String, perintah)>,
    kirim: std::sync::mpsc::Sender<String>,
) {
    let mut fn_ = [false, false];
    //let mut blok = 0;
    let mut lewati_jika = (false, 0);
    'l0: loop {
        let perintah_masuk = baris.recv().unwrap();
        let nomer_baris = perintah_masuk.0;
        let nama_file = perintah_masuk.1;
        match perintah_masuk.2 {
            //perintah::konst(_,_)|perintah::glob_var(_,_,_)=>{}
            //perintah::_i32_konst(_)=>{}
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
                                /*
                                if o.len() > 1 {
                                    v.push_str("\"nilai\":");
                                    if o.len() == 1 {
                                        v.push_str(&format!("{}", o[0].unwrap()))
                                    } else {
                                        v.push_str("[");
                                        let mut i = 0;
                                        loop {
                                            if let Some(o) = o[i] {
                                                v.push_str(&format!("{:?}", o))
                                            } else {
                                                v.push_str("None")
                                            }
                                            i += 1;
                                            if i >= o.len() {
                                                break;
                                            }
                                            v.push_str(",")
                                        }
                                        v.push_str("]");
                                    }
                                } else if o.len() == 0 {
                                    v.push_str(&format!("\"cap\":{},\"nilai\":[]", o.capacity()));
                                } else {
                                    if let Some(s) = o[0] {
                                        v.push_str(&format!("\"cap\":{},\"nilai\":[", s));
                                        let mut i = 0;
                                        loop {
                                            if let Some(o) = o[i] {
                                                v.push_str(&format!("{}", o))
                                            } else {
                                                break;
                                                //v.push_str("None")
                                            }
                                            i += 1;
                                            if i >= o.len() {
                                                break;
                                            }
                                            v.push_str(",")
                                        }
                                        v.push(']');
                                        println!("{}",v)
                                    } else {
                                        panic!()
                                    }
                                }
                                */
                                /*
                                v.push_str("u8_\",\"len\":\"");
                                // v.push_str(&format!("{}\",", o.len()));
                                if !vec {
                                    v.push_str("\"nilai\":");
                                    if o.len() == 1 {
                                        v.push_str(&format!("{}", o[0].unwrap()))
                                    } else {
                                        v.push_str("[");
                                        let mut i = 0;
                                        loop {
                                            if let Some(o) = o[i] {
                                                v.push_str(&format!("{:?}", o))
                                            } else {
                                                v.push_str("None")
                                            }
                                            i += 1;
                                            if i >= o.len() {
                                                break;
                                            }
                                            v.push_str(",")
                                        }
                                        v.push_str("]");
                                    }
                                } else {
                                    v.push_str(format!("\"cap\":{},\"nilai\":[", o.capacity()));
                                    /*
                                    let mut i = 0;
                                    loop {
                                        if let Some(o) = o[i] {
                                            v.push_str(&format!("{}", o))
                                        } else {
                                            break
                                            //v.push_str("None")
                                        }
                                        i += 1;
                                        if i >= o.len() {
                                            break;
                                        }
                                        v.push_str(",")
                                    }
                                    v.push(']');
                                    */
                                }
                                */
                                /*
                                if !vec {
                                    v.push_str("u8_\",\"len\":\"");
                                    v.push_str(&format!("{}\",\"nilai\":", o.len()));
                                    if o.len() == 1 {
                                        if let Some(o) = o[0] {
                                            v.push_str(&format!("{:?}", o))
                                        }
                                    } else {
                                        v.push_str("[");
                                        let mut i = 0;
                                        loop {
                                            if let Some(o) = o[i] {
                                                v.push_str(&format!("{:?}", o))
                                            } else {
                                                v.push_str("None")
                                            }
                                            i += 1;
                                            if i >= o.len() {
                                                break;
                                            }
                                            v.push_str(",")
                                        }

                                        v.push_str("]");
                                        //v.push_str(&format!("{:?}",o))
                                    }
                                } else {
                                    v.push_str(&format!("u8_\",\"len\":\"{}\",\"cap\":{},\"nilai\":",o.len(),o.capacity()));
                                    //panic!();
                                    //v.push_str("u8_\",\"len\":\"");
                                }
                                */
                                //v.push_str(&format!("{:?}",if o.len() == 1 {o[0]} else {o}));
                                v
                            }
                            /*
                            Tipe::_i8(o)=>{
                                /*
                                if o.len() == 1 {
                                    if let Some() = o[0] {
                                        format!("i8_\",\"nilai\":\"{}\"",o)
                                    } else {
                                        "i8\"".to_string()
                                    }
                                } else {
                                    if let Some(o) = o[0] {
                                        format!("i8_\",\"nilai\":\"{}\"",o)
                                    } else {
                                        "i8\"".to_string()
                                    }
                                }
                                */

                                format!("i8_\",\"len\":\"{}\",\"nilai\":{}",o.len(),
                                    if o.len() == 1 {
                                        if let Some(o) = o[0] {
                                            format!("\"{}\"",o)
                                            //"\"" + o.to_string()
                                        } else {
                                            "\"\"".to_string()
                                        }
                                    } else if let Some(k) = o[0] {
                                        panic!();
                                        let mut v = "[".to_string();
                                        v.push_str(k.to_string().as_str());
                                        for i in 1..o.len(){
                                            v.push(',');
                                            v.push_str(o[i].unwrap().to_string().as_str())
                                        }
                                        v.push(']');
                                        v
                                    } else {
                                        "".to_string()
                                    }
                                )

                            }
                            Tipe::_u8(o)=>{
                                format!("u8_\",\"len\":\"{}\",\"nilai\":{}",o.len(),
                                    if o.len() == 1 {
                                        if let Some(o) = o[0] {
                                            format!("\"{}\"",o)
                                            //"\"" + o.to_string()
                                        } else {
                                            "\"\"".to_string()
                                        }
                                    } else if let Some(k) = o[0] {
                                        panic!();
                                        let mut v = "[".to_string();
                                        v.push_str(k.to_string().as_str());
                                        for i in 1..o.len(){
                                            v.push(',');
                                            v.push_str(o[i].unwrap().to_string().as_str())
                                        }
                                        v.push(']');
                                        v
                                    } else {
                                        "".to_string()
                                    }
                                )
                                /*
                                if let Some(o) = o {
                                    format!("u8_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "u8\"".to_string()
                                }
                                */
                            }
                            Tipe::_i16(o)=>{
                                format!("i16_\",\"len\":\"{}\",\"nilai\":\"{}\"",o.len(),
                                    if o.len() == 1 {
                                        if let Some(o) = o[0] {
                                            format!("\"{}\"",o)
                                            //"\"" + o.to_string()
                                        } else {
                                            "\"\"".to_string()
                                        }
                                    } else if let Some(k) = o[0] {
                                        let mut v = "[".to_string();
                                        v.push_str(k.to_string().as_str());
                                        for i in 1..o.len(){
                                            v.push(',');
                                            v.push_str(o[i].unwrap().to_string().as_str())
                                        }
                                        v.push(']');
                                        v
                                    } else {
                                        "".to_string()
                                    }
                                )
                                /*
                                if let Some(o) = o {
                                    format!("i16_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "i16\"".to_string()
                                }*/
                            }
                            Tipe::_u16(o)=>{
                                format!("u16_\",\"len\":\"{}\",\"nilai\":\"{}\"",o.len(),
                                    if o.len() == 1 {
                                        if let Some(o) = o[0] {
                                            format!("\"{}\"",o)
                                            //"\"" + o.to_string()
                                        } else {
                                            "\"\"".to_string()
                                        }
                                    } else if let Some(k) = o[0] {
                                        let mut v = "[".to_string();
                                        v.push_str(k.to_string().as_str());
                                        for i in 1..o.len(){
                                            v.push(',');
                                            v.push_str(o[i].unwrap().to_string().as_str())
                                        }
                                        v.push(']');
                                        v
                                    } else {
                                        "".to_string()
                                    }
                                )
                                /*
                                if let Some(o) = o {
                                    format!("u16_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "u16\"".to_string()
                                }
                                */
                            }
                            Tipe::_i32(o)=>{
                                format!("i32_\",\"len\":\"{}\",\"nilai\":\"{}\"",o.len(),
                                    if o.len() == 1 {
                                        if let Some(o) = o[0] {
                                            format!("\"{}\"",o)
                                            //"\"" + o.to_string()
                                        } else {
                                            "\"\"".to_string()
                                        }
                                    } else if let Some(k) = o[0] {
                                        let mut v = "[".to_string();
                                        v.push_str(k.to_string().as_str());
                                        for i in 1..o.len(){
                                            v.push(',');
                                            v.push_str(o[i].unwrap().to_string().as_str())
                                        }
                                        v.push(']');
                                        v
                                    } else {
                                        "".to_string()
                                    }
                                )
                                /*
                                if let Some(o) = o {
                                    format!("i32_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "i32\"".to_string()
                                }
                                */
                            }
                            Tipe::_u32(o)=>{
                                format!("u32_\",\"len\":\"{}\",\"nilai\":\"{}\"",o.len(),
                                    if o.len() == 1 {
                                        if let Some(o) = o[0] {
                                            format!("\"{}\"",o)
                                            //"\"" + o.to_string()
                                        } else {
                                            "\"\"".to_string()
                                        }
                                    } else if let Some(k) = o[0] {
                                        let mut v = "[".to_string();
                                        v.push_str(k.to_string().as_str());
                                        for i in 1..o.len(){
                                            v.push(',');
                                            v.push_str(o[i].unwrap().to_string().as_str())
                                        }
                                        v.push(']');
                                        v
                                    } else {
                                        "".to_string()
                                    }
                                )
                                /*
                                if let Some(o) = o {
                                    format!("u32_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "u32\"".to_string()
                                }
                                */
                            }
                            Tipe::_i64(o)=>{
                                format!("i64_\",\"len\":\"{}\",\"nilai\":\"{}\"",o.len(),
                                    if o.len() == 1 {
                                        if let Some(o) = o[0] {
                                            format!("\"{}\"",o)
                                            //"\"" + o.to_string()
                                        } else {
                                            "\"\"".to_string()
                                        }
                                    } else if let Some(k) = o[0] {
                                        let mut v = "[".to_string();
                                        v.push_str(k.to_string().as_str());
                                        for i in 1..o.len(){
                                            v.push(',');
                                            v.push_str(o[i].unwrap().to_string().as_str())
                                        }
                                        v.push(']');
                                        v
                                    } else {
                                        "".to_string()
                                    }
                                )
                                /*
                                if let Some(o) = o {
                                    format!("i64_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "i64\"".to_string()
                                }
                                */
                            }
                            Tipe::_u64(o)=>{
                                format!("u64_\",\"len\":\"{}\",\"nilai\":\"{}\"",o.len(),
                                    if o.len() == 1 {
                                        if let Some(o) = o[0] {
                                            format!("\"{}\"",o)
                                            //"\"" + o.to_string()
                                        } else {
                                            "\"\"".to_string()
                                        }
                                    } else if let Some(k) = o[0] {
                                        let mut v = "[".to_string();
                                        v.push_str(k.to_string().as_str());
                                        for i in 1..o.len(){
                                            v.push(',');
                                            v.push_str(o[i].unwrap().to_string().as_str())
                                        }
                                        v.push(']');
                                        v
                                    } else {
                                        "".to_string()
                                    }
                                )

                                /*
                                if let Some(o) = o {
                                    format!("u64_\",\"nilai\":\"{}\"",o)
                                } else {
                                    "u64\"".to_string()
                                }
                                */
                            }
                            */
                            /*
                            Tipe::_u8_ar(len, v) => {
                                format!("\"u8_ar\",\"nilai\":[{},{:?}]", len, v)
                            }
                            Tipe::_i8_ar(len, v) => {
                                format!("\"i8_ar\",\"nilai\":[{},{:?}]", len, v)
                            }
                            Tipe::_u16_ar(len, v) => {
                                format!("\"u16_ar\",\"nilai\":[{},{:?}]", len, v)
                            }
                            Tipe::_i16_ar(len, v) => {
                                format!("\"i16_ar\",\"nilai\":[{},{:?}]", len, v)
                            }
                            Tipe::_u32_ar(len, v) => {
                                format!("\"u32_ar\",\"nilai\":[{},{:?}]", len, v)
                            }
                            Tipe::_i32_ar(len, v) => {
                                format!("\"i32_ar\",\"nilai\":[{},{:?}]", len, v)
                            }
                            Tipe::_u64_ar(len, v) => {
                                format!("\"u64_ar\",\"nilai\":[{},{:?}]", len, v)
                            }
                            Tipe::_i64_ar(len, v) => {
                                format!("\"i64_ar\",\"nilai\":[{},{:?}]", len, v)
                            }
                            */
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
                    if fn_[1] { "," } else { "" },
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
            perintah::cpu(nama, publik) => kirim
                .send(format!(
                    "{}{{\"nomer_baris\":{},\"nama_file\":\"{}\",\"fn\":\"{}\",\"publik\":{},\"nilai\":[",
                    if fn_[0] {
                        fn_[1] = false;
                        "]},"
                    } else {
                        fn_[0] = true;
                        ""
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
            perintah::selesai(error) => {
                if !error.is_empty() {
                    kirim.send("".to_string()).unwrap();
                    kirim.send(error).unwrap();
                } else {
                    kirim.send("".to_string()).unwrap();
                }
                break;
            } /*
              _ =>{
                  panic!()
              }
              */
        }
    }
}
