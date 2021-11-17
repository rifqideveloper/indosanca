
//use serde::{Deserialize, Serialize};
//use serde_json::json;
//use serde_json::{/*Result,*/ Value};
//use std::fs;
use crate::parsing::perintah;
use crate::parsing::tipe;
//versi baru
macro_rules! aritmatik {
    ($v:expr,$o:expr, $i:expr ) => {
        $v[$i] = perintah::_i32_konst($o);
            $v.remove( $i + 1 );
            $v.remove( $i + 1 );
        continue
    };
}
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
            perintah::_let(nama,_mut,tipe)=>{
                //panic!();
                kirim.send(
                    format!("{}{{\"tipe\":\"let\",\"tipe_\":\"{}\",\"nama\":\"{}\",\"mut\":{}}}",
                        if fn_[1] {","} else { fn_[1] = true ; "" },
                        match tipe {
                            tipe::_string(_)=>{"string"}
                            tipe::_i8=>{"i8"}
                            tipe::_u8=>{"u8"}
                            tipe::_i16=>{"i16"}
                            tipe::_u16=>{"u16"}
                            tipe::_i32=>{"i32"}
                            tipe::_u32=>{"u32"}
                            tipe::_i64=>{"i64"}
                            tipe::_u64=>{"u64"}
                        },
                        nama,
                        _mut
                    )
                ).unwrap()
            }
            perintah::konst(l,o)=>{
                //println!("konstan !!!");
                kirim.send(
                    format!("{}{{\"tipe\":\"konst\",\"nilai\":\"{}\",\"nama\":\"{}\"}}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    match o {
                        tipe::_string(o)=>{
                            let mut v = o.clone();
                            v.remove(0);
                            v.pop();
                            v.push_str("str");
                            v
                        },
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
                            tipe::_string(_)=>{"string"}
                            tipe::_i8=>{"i8"}
                            tipe::_u8=>{"u8"}
                            tipe::_i16=>{"i16"}
                            tipe::_u16=>{"u16"}
                            tipe::_i32=>{"i32"}
                            tipe::_u32=>{"u32"}
                            tipe::_i64=>{"i64"}
                            tipe::_u64=>{"u64"}
                        },
                        nama,
                        _mut
                    )
                ).unwrap()
            }
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
            // jika sudah tidak digunakan
            perintah::jika_b(_lapisan)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"jika\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap();
                for i in baris.recv().into_iter() {
                    match i {
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
                        perintah::jika_=>{
                            kirim.send(
                                "{}{{\"tipe\":\"jika_\"}}".to_string()
                            ).unwrap();
                            break
                        }
                        _=>{}
                    }
                }
                
                /*
                kirim.send(
                    format!("{}{{\"tipe\":\"jika_b\"}}",if fn_[1] {","}else{""})
                ).unwrap();
                */
            }
            perintah::lalu=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"br\",\"nilai\":\"1\"}},{{\"tipe\":\"jika_tutup\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap();
            }
            perintah::lalu_=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"blok_t_tutup\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::lalu_jika=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"br\",\"nilai\":\"1\"}},{{\"tipe\":\"blok_t_tutup\"}},{{\"tipe\":\"lalu_jika\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap();
                for i in baris.recv().into_iter() {
                    match i {
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
                        perintah::jika_=>{
                            kirim.send(
                                "{}{{\"tipe\":\"jika_\"}}".to_string()
                            ).unwrap();
                            break
                        }
                        _=>{}
                    }
                }
            }
            perintah::lalu_jika_=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"blok_t_tutup\"}},{{\"tipe\":\"blok_t_tutup\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap();
            }
            perintah::jika=>{
                let mut j = false;
                let mut v = Vec::with_capacity(5);
                //optimalisasi 1
                'l1:loop{
                    let t = baris.recv().unwrap();                    
                    match t {
                        perintah::boolean(x)=>{
                            //optimalisasi hasil
                            // jika hasil hanya banar maka operasi logika tidak tiperlukan
                            if !j {
                                j = true;
                            }
                            v.push(t)                            
                        }
                        perintah::i32_eq=>{
                            //harus di ubah ke macro
                            match baris.recv().unwrap(){
                                perintah::boolean(o)=>{
                                    if let Some(q) = v.last_mut(){
                                        match q {
                                            perintah::boolean(k)=>{
                                                *q = perintah::boolean(o == *k)
                                            }
                                            _=>{}
                                        } 
                                    }
                                }
                                _=>{}
                            }
                        }
                        perintah::i32_eqz=>{
                            //hampir sama seperti i32.eq
                        }

                        perintah::_i32_konst(_)=>{
                            let x = v.len();
                            //jika benar maka aritmatik harus diatur ulang
                            let mut z = false;
                            v.push(t);
                            loop {
                                let y = baris.recv().unwrap();
                                match y {
                                    perintah::modus|perintah::bagi|perintah::kurang|perintah::tambah|perintah::_i32_konst(_)=>{v.push(y)}
                                    perintah::kali=>{v.push(y); z = true ;}
                                    perintah::jika_=>{v.push(y); break}
                                    _=>{}
                                }
                            }
                            //
                            if z {
                                
                            }    
                            let mut i = x ;
                            loop{
                                match v[i]{
                                    perintah::_i32_konst(n)=>{
                                        match v[i+1] {
                                            perintah::tambah=>{
                                                match v[i+2]{
                                                    perintah::_i32_konst(m)=>{
                                                        aritmatik!(v,n + m, i);
                                                    }
                                                    _=>{}
                                                }
                                            }
                                            perintah::kurang=>{
                                                match v[i+2]{
                                                    perintah::_i32_konst(m)=>{
                                                        aritmatik!(v,n - m, i);

                                                    }
                                                    _=>{}
                                                }
                                            }
                                            perintah::bagi=>{
                                                match v[i+2]{
                                                    perintah::_i32_konst(m)=>{
                                                        aritmatik!(v,n / m, i);

                                                    }
                                                    _=>{}
                                                }
                                            }
                                            perintah::kali=>{
                                                match v[i+2]{
                                                    perintah::_i32_konst(m)=>{
                                                        aritmatik!(v,n * m, i);                                                        
                                                    }
                                                    _=>{}
                                                }
                                            }
                                            perintah::modus=>{
                                                match v[i+2]{
                                                    perintah::_i32_konst(m)=>{
                                                        aritmatik!(v,n % m, i);
                                                    }
                                                    _=>{}
                                                }
                                            }
                                            perintah::jika_=>{
                                                //diatur di seting
                                                if i == x {
                                                    kirim.send(
                                                        format!("{}{{\"tipe\":\"_i32_konst\",\"nilai\":\"{}\"}}",if fn_[1] {","}else{fn_[1] = true;""},n)
                                                    ).unwrap();
                                                    kirim.send(
                                                        ",{\"tipe\":\"jika_\"}".to_string()
                                                    ).unwrap();
                                                    continue 'l0
                                                }
                                                break 'l1
                                            }
                                            _=>{}
                                        }
                                    }
                                    _=>{}
                                }
                                i = i + 1;
                            }
                            
                        }
                        perintah::jika_=>{
                            if v.len() == 1 {
                                if let perintah::boolean(O) = v[0] {
                                    if O {
                                        lewati_jika.0 = true;
                                        continue 'l0
                                    }
                                } 
                            }
                            
                            break
                        }
                        _=>{}
                    }
                }
                kirim.send(
                    format!("{}{{\"tipe\":\"jika\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap();
                //belum selesai
                for i in v {
                    match i {
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
                        perintah::sama=>{
                            kirim.send(
                                format!("{}{{\"tipe\":\"_i32_eq\"}}",if fn_[1] {","}else{fn_[1] = true;""})
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
                        perintah::_i32_konst(o)=>{
                            kirim.send(
                                format!("{}{{\"tipe\":\"_i32_konst\",\"nilai\":\"{}\"}}",if fn_[1] {","}else{fn_[1] = true;""},o)
                            ).unwrap()
                        }
                        _=>{}
                    }
                }
                kirim.send(
                    format!("{}{{\"tipe\":\"jika_\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap();
                /*
                loop{
                    match baris.recv().unwrap(){
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
                        perintah::_i32_konst(o)=>{
                            match baris.recv().unwrap(){
                                perintah::i32_eq=>{
                                    match baris.recv().unwrap(){
                                        perintah::_i32_konst(q)=>{
                                            kirim.send(
                                                format!("{0}{{\"tipe\":\"_i32_konst\",\"nilai\":\"{1}\"}},{{\"tipe\":\"_i32_konst\",\"nilai\":\"{2}\"}},{{\"tipe\":\"_i32_eq\"}}",if fn_[1] {","}else{fn_[1] = true;""},o,q)
                                            ).unwrap();
                                        }
                                        _=>{
                                            panic!()
                                        }
                                    }
                                }
                                perintah::i32_eqz=>{
                                    //hampir sama dengan i32_eq
                                }
                                perintah::tambah=>{
                                    /*kirim.send(
                                        format!("{}{{\"tipe\":\"tambah\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                                    ).unwrap();*/
                                }
                                _=>{}
                            }
                        }
                        perintah::jika_=>{
                            kirim.send(
                                format!("{}{{\"tipe\":\"jika_\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                            ).unwrap();            
                            break
                        }
                        _=>{}
                    }
                }*/
            }
            /*
            perintah::lalu=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"lalu\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::lalu_jika=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"lalu_jika\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            */
            perintah::jika_=>{
                if lewati_jika.0 {
                    for i in baris.recv().into_iter(){
                        if let perintah::jika_tutup = i {
                            continue 'l0
                        }
                    }
                }
                kirim.send(
                    format!("{}{{\"tipe\":\"jika_\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::jika_tutup=>{
                if lewati_jika.0 {
                    lewati_jika.0 = false;
                    /*
                    kirim.send(
                        format!("{}{{\"tipe\":\"blok_t_tutup\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                    ).unwrap();
                    */
                    continue 'l0
                }
                kirim.send(
                    format!("{}{{\"tipe\":\"jika_tutup\"}}",if fn_[1] {","}else{fn_[1] = true;""})    
                ).unwrap()
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
                /*
                let mut arit = Vec::new();
                arit.push(perintah::_i32_konst(o));
                loop {
                    //rumus aritmatik & logika
                    //perkalian terlebih dahulu
                    match baris.recv().expect("") {
                        perintah::_i32_konst(o)=>{
                            arit.push(
                                perintah::_i32_konst(o)
                            )
                        }
                        perintah::sama=>{
                            match baris.recv().unwrap() {
                                perintah::_i32_konst(o)=>{
                                    arit.push(
                                      perintah::_i32_konst(o)  
                                    );
                                }
                                _=>{
                                    panic!()
                                }
                            }
                            arit.push(
                                perintah::sama
                            )
                        }
                        perintah::jika_=>{
                            arit.push(
                                perintah::jika_
                            );
                            break
                        }
                        _=>{break}
                    }
                }
                for i in arit {
                    match i {
                        perintah::_i32_konst(o)=>{
                            kirim.send(
                                format!("{1}{{\"tipe\":\"_i32_konst\",\"nilai\":\"{}\"}}",o,if fn_[1] {","}else{fn_[1] = true;""})
                            ).unwrap();
                        }
                        perintah::sama=>{
                            kirim.send(
                                format!("{}{{\"tipe\":\"_i32_eq\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                            ).unwrap()
                        }
                        perintah::jika_=>{
                            kirim.send(
                                format!("{}{{\"tipe\":\"jika_\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                            ).unwrap()
                        }
                        _=>{}
                    }
                }
                /*
                kirim.send(
                    format!("{1}{{\"tipe\":\"_i32_konst\",\"nilai\":\"{}\"}}",o,if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
                */
                */
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
                let mut o = o.clone();
                let x = if o.starts_with("\"") {
                    o.remove(0);
                    o.pop();
                    "0"
                } else {
                    "1"
                };
                kirim.send(
                    format!("{}{{\"tipe\":\"halaman\",\"nilai\":\"{}\",\"var\":\"{}\"}}",if fn_[1] {","} else { fn_[1] = true ; ""},o,x)
                ).unwrap()
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
            perintah::blok_buka=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"blok_t_buka\"}}",if fn_[1] {","}else{fn_[1] = true;""})
                ).unwrap()
            }
            perintah::blok_tutup=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"blok_t_tutup\"}}",if fn_[1] {","}else{fn_[1] = true;""})

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
/*
pub fn baca(
    baris:std::sync::mpsc::Receiver<perintah>,
    kirim:std::sync::mpsc::Sender<String>
){
    let mut fn_ = [false,false];
    let _arg = ([false,false],"");
    //let blok = [false,false];
    //panic!();
    //#[allow(unreachable_code)]
   
    loop{
        match baris.recv().unwrap(){
            perintah::variabel_null(a,b,c)=>{
                let v = format!(
                    "{0}{{\"data\":\"{1}\",\"tipe\":\"{2}\",\"nama\":\"{3}\" }}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    match a {
                        tipe::_string(o)=>{o},
                        tipe::_u8=>{String::from("u8")} 
                    },
                    b,
                    c,
                );
                println!("{}",v);
                kirim.send(
                    v
                ).unwrap();
            }
            perintah::variabel(a,b,c,d)=>{
                //println!("{{{:#?},{},{},{}}}",a,b,c,d);
                let v = format!(
                    "{0}{{\"data\":\"{1}\",\"tipe\":\"{2}\",\"nama\":\"{3}\",\"nilai\":{nilai}}}",
                    if fn_[1] {","} else { fn_[1] = true ; "" },
                    match a {
                     tipe::_string(o)=>{o},
                     tipe::_u8=>{String::from("u8")} 
                    },
                    b,
                    c,
                    nilai = d
                );
                //println!("{}",v);
                kirim.send(
                    v
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
                        } else {
                            "var"
                        },
                        d
                    )
                ).unwrap();
                /*
                kirim.send(
                    
                    format!("{0} {{ \"tipe\" : \"cetak\"{1},\"nilai\": {2} }}",
                        if fn_[0] {","} else { fn_[0] = true ; "" },
                        
                    )
                    
                    
                ).unwrap()
                */
            }
            perintah::modul_masuk(nama)=>{
                kirim.send(
                    format!("{{\"mod\":\"{}\",\"nilai\":[",nama)
                ).unwrap()
            }
            perintah::cpu(nama,publik)=>{
                kirim.send(
                    format!("{}{{\"fn\":\"{}\",\"publik\":{},\"nilai\":[",
                    if fn_[0] { "]},"} else { fn_[0] = true ; "" }
                    ,nama,publik)
                ).unwrap()
            }
            perintah::modul_keluar=>{
                kirim.send(
                    format!("{}}}\n",if fn_[0] {fn_ = [false,false] ; "]}]"} else {"]"})
                ).unwrap()
            }
            perintah::tulis(a,b)=>{
                kirim.send(
                    format!("{}{{\"tipe\":\"tulis\",\"var\":\"{}\",\"nilai\":{}}}",
                    if fn_[0] {","} else { fn_[0] = true ; "" }
                    ,a,b)
                ).unwrap()
            }
            perintah::blok_buka =>{
                kirim.send(
                    format!("{}{{\"tipe\":\"blok\"}}",if fn_[0] {","} else { fn_[0] = true ; "" })
                ).unwrap()
            }
            perintah::blok_tutup =>{
                kirim.send(
                    format!("{}{{\"tipe\":\"blok_tutup\"}}",if fn_[0] {","} else { fn_[0] = true ; "" })
                ).unwrap()
            }
            perintah::arit_tambah(a,b,c)=>{
                kirim.send(
                    format!("{0}{{\"tipe\":\"arit_tambah\",\"var\":\"{1}\",\"nilai\":[{2},{3}]}}",
                    if fn_[0] {","} else { fn_[0] = true ; "" },
                    a,
                    b,
                    c,
                    )
                ).unwrap();
                println!("testing tambah");
            }
            perintah::selesai=>{
                kirim.send("".to_string()).unwrap();
                break
            }

        }
        /*
        let buf = match baris.recv(){
            Ok(o)=>{
                if o != ""{o} else {break}
            }
            Err(_)=>{panic!()}
            
        };
        
        match serde_json::from_str(&buf) {
            Ok(t)=>{
                let json: Value = t;
                match kirim.send(
                    if json["tipe"] == "modul_masuk" {
                        format!("{{\"mod\":{},\"nilai\":[",json["nama"])
                    } else if json["tipe"] == "modul_keluar" {
                        format!("{}}}\n",if fn_.0 {fn_ = (false,false) ; "]}]"} else {"]"})
                    } else if json["tipe"] == "fn_cpu" {
                        format!("{}{{\"fn\":{},\"nilai\":[",
                        if fn_.0 {","} else { fn_.0 = true ; "" }
                        ,json["nama"])
                    } else if json["tipe"] == "cetak" {
                        println!("{}",json["tipe"]);
                        panic!();
                        /*
                        format!("{0} {{ \"tipe\" : \"cetak\",\"nilai\": {} }}",
                            if fn_.0 {","} else { fn_.0 = true ; "" },

                        )*/
                    } else {
                        format!("{}{}",
                        if fn_.1 {","} else { fn_.1 = true ; "" }
                        ,json)
                    }
                ){
                    Ok(_)=>{}
                    Err(_)=>{panic!()}
                }
            }
            Err(_)=>{
                println!("{}",buf);
                panic!()
            }
        }
        */
    }
    
}
*/