fn tipe_(terima: &std::sync::mpsc::Receiver<(u64, String, String)>) -> crate::parsing::Tipe {
    fn nilai<T: std::str::FromStr>(
        v: &mut std::vec::Vec<std::option::Option<T>>,
        terima: &std::sync::mpsc::Receiver<(u64, String, String)>,
    ) {
        let mut x = terima.recv().unwrap();
        //println!("{:?}",x);
        match x.2.as_str() {
            "=" => {
                v.reserve_exact(1);
                if let Ok(ok) = terima.recv().unwrap().2.parse() {
                    v.push(Some(ok))
                } else {
                    panic!()
                }
            }
            ":" => {
                if let Ok(ok) = terima.recv().unwrap().2.parse() {
                    v.reserve_exact(ok);
                    if terima.recv().unwrap().2 == "=" {
                        if terima.recv().unwrap().2 == "[" {
                            x = terima.recv().unwrap();
                            loop {
                                if x.2 == "]" {
                                    break;
                                }
                                if x.2 == "," {
                                    continue;
                                }
                                if let Ok(ok) = x.2.parse() {
                                    v.push(Some(ok))
                                }
                            }
                        } else {
                            panic!()
                        }
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
            }
            _ => {
                panic!()
            }
        }
    }
    match terima.recv().unwrap().2.as_str() {
        "u8" => {
            let mut v = Vec::new();
            nilai(&mut v, terima);
            //println!("{:?}",v);
            crate::parsing::Tipe::int(crate::parsing::mem_tipe::unint(Some(v)),8,)
        }
        "u16" =>{
            let mut v = Vec::new();
            nilai(&mut v, terima);
            //println!("{:?}",v);
            crate::parsing::Tipe::int(crate::parsing::mem_tipe::unint(Some(v)),16)
        }
        "u32" =>{
            let mut v = Vec::new();
            nilai(&mut v, terima);
            //println!("{:?}",v);
            crate::parsing::Tipe::int(crate::parsing::mem_tipe::unint(Some(v)),32)
        }
        "u64" =>{
            let mut v = Vec::new();
            nilai(&mut v, terima);
            //println!("{:?}",v);
            crate::parsing::Tipe::int(crate::parsing::mem_tipe::unint(Some(v)),64)
        }
        "i8" => {
            let mut v = Vec::new();
            nilai(&mut v, terima);
            //println!("{:?}",v);
            crate::parsing::Tipe::int(crate::parsing::mem_tipe::int(Some(v)),8)
        }
        "i16" =>{
            let mut v = Vec::new();
            nilai(&mut v, terima);
            //println!("{:?}",v);
            crate::parsing::Tipe::int(crate::parsing::mem_tipe::int(Some(v)),16)
        }
        "i32" =>{
            let mut v = Vec::new();
            nilai(&mut v, terima);
            //println!("{:?}",v);
            crate::parsing::Tipe::int(crate::parsing::mem_tipe::int(Some(v)),32)
        }
        "i64" =>{
            let mut v = Vec::new();
            nilai(&mut v, terima);
            //println!("{:?}",v);
            crate::parsing::Tipe::int(crate::parsing::mem_tipe::int(Some(v)),64)
        }
        "uint" => {
            let mut bites = 32; 
            if terima.recv().unwrap().2 == "<" {
                if let Ok(int) = terima.recv().unwrap().2.parse() {
                    if terima.recv().unwrap().2 == ">" {
                        bites = int
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
            let mut v = Vec::new();
            nilai(&mut v, terima);
            crate::parsing::Tipe::int(crate::parsing::mem_tipe::unint(Some(v)),bites)
        }
        "int" => {
            let mut bit = 32; 
            if terima.recv().unwrap().2 == "<" {
                if let Ok(int) = terima.recv().unwrap().2.parse() {
                    if terima.recv().unwrap().2 == ">" {
                        bit = int
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
            let mut v = Vec::new();
            nilai(&mut v, terima);
            crate::parsing::Tipe::int(crate::parsing::mem_tipe::int(Some(v)),bit)
        }
        "Vec" => {
            panic!()
        }
        _ => {
            panic!()
        }
    }
}
pub fn parse(
    terima: std::sync::mpsc::Receiver<(u64, String, String)>,
    kirim: std::sync::mpsc::Sender<(u64, String, crate::parsing::perintah)>,
) {
    //lokalisasi
    const JIKA: &str = if cfg!(international) { "if" } else { "jika" };
    const LALU: &str = if cfg!(international) { "else" } else { "lalu" };
    const CETAK: &str = if cfg!(international) { "print" } else { "cetak" };
    const BENAR: &str = if cfg!(international) { "true" } else { "benar" };
    const SALAH: &str = if cfg!(international) { "false" } else { "salah" };
    const STATIK: &str = if cfg!(international) { "static" } else { "statik" };
    const TIDUR: &str = if cfg!(international) { "sleep" } else { "tidur" };
    const PUTAR: &str = if cfg!(international) { "loop" } else { "putar" };

    loop {
        let mut v = terima.recv().unwrap();
        //println!("{}",v);
        match v.2.as_str() {
            "let" => {
                v = terima.recv().unwrap();
                match v.2.as_str() {
                    "mut" =>{
                        let nama = terima.recv().unwrap().2;
                        let tipe = tipe_(&terima);
                        kirim.send(
                            (
                                v.0,
                                v.1,
                                crate::parsing::perintah::_let(
                                    nama,
                                    crate::parsing::mem_mode::mutabel,
                                    tipe
                                )
                            )
                        ).unwrap()
                    }
                    STATIK =>{
                        let nama = terima.recv().unwrap().2;
                        let tipe = tipe_(&terima);
                        kirim.send(
                            (
                                v.0,
                                v.1,
                                crate::parsing::perintah::_let(
                                    nama,
                                    crate::parsing::mem_mode::statik,
                                    tipe
                                )
                            )
                        ).unwrap()
                    }
                    _=>{
                        let tipe = tipe_(&terima);
                        kirim.send(
                            (
                                v.0,
                                v.1,
                                crate::parsing::perintah::_let(
                                    v.2,
                                    crate::parsing::mem_mode::imutabel,
                                    tipe
                                )
                            )
                        ).unwrap()
                    }
                }
                /*
                if v.2 == "mut" {
                    let nama = terima.recv().unwrap();
                    let tipe = tipe_(&terima);
                    kirim
                        .send((v.0, v.1, crate::parsing::perintah::_let(nama.2, true, tipe)))
                        .unwrap();
                } else {
                    kirim
                        .send((
                            v.0,
                            v.1,
                            crate::parsing::perintah::_let(v.2, false, tipe_(&terima)),
                        ))
                        .unwrap();
                }
                */
            }
            JIKA => {
                let mut nilai = Vec::with_capacity(1);
                loop{
                    v = terima.recv().unwrap();
                    if v.2 == ";"{
                        kirim.send(
                            (v.0,v.1.clone(),crate::parsing::perintah::jika(nilai))
                        ).unwrap();
                        break
                    } else if v.2 == "}"{
                        panic!();
                        kirim.send(
                            (v.0,v.1.clone(),crate::parsing::perintah::jika(nilai))
                        ).unwrap();
                        kirim.send(
                            (v.0,v.1,crate::parsing::perintah::blok(String::new()))
                        ).unwrap();
                        break
                    } else if v.2 == BENAR {
                        nilai.push("true".to_string())
                    } else if v.2 == SALAH {
                        nilai.push("false".to_string())
                    }
                }
            }
            LALU => {
                /*
                let nilai = Vec::with_capacity(1);
                let sebaris = false
                let i = terima.recv().unwrap();
                if i.2 == JIKA {
                    kirim.send(
                        (v.0,v.1,crate::parsing::perintah::lalu_jika(sebaris,nilai))
                    ).unwrap();
                } else {
                    kirim.send(
                        (v.0,v.1,crate::parsing::perintah::lalu(sebaris,nilai))
                    ).unwrap();
                }
                */
            }
            PUTAR =>{
                let mut nilai :Vec<crate::parsing::args>= Vec::with_capacity(5);
                loop{
                    v = terima.recv().unwrap();
                    if v.2 == ";" || v.2 == "}"{
                        break
                    }
                    if let Ok(int) = v.2.parse::<i128>() {
                        nilai.push(
                            crate::parsing::args::Str_int(
                                int
                            )
                        )
                    }
                }
                kirim
                    .send((v.0, v.1, crate::parsing::perintah::putar(nilai)))
                    .unwrap();
            }
            CETAK => {
                let mut nilai = Vec::with_capacity(1);
                loop {
                    v = terima.recv().unwrap();
                    if v.2 == ";" || v.2 == "\n" || v.2 == "\r" {
                        break;
                    }
                    nilai.push(v.2);
                }
                kirim
                    .send((v.0, v.1, crate::parsing::perintah::cetak(nilai)))
                    .unwrap();
            }
            "{" => {
                kirim
                    .send((v.0, v.1, crate::parsing::perintah::blok(String::new())))
                    .unwrap();
            }
            "}" =>{
                kirim
                    .send((v.0, v.1, crate::parsing::perintah::blok_))
                    .unwrap();
            }
            TIDUR =>{
                v = terima.recv().unwrap();
                let nilai = if let Ok(o) = v.2.parse::<i128>() {
                    crate::parsing::args::Str_int(o)
                } else {
                    panic!()
                };
                kirim
                    .send((v.0, v.1, crate::parsing::perintah::tidur(
                        nilai
                    )))
                    .unwrap();
            }
            "fn" => {
                //panic!();
                v = terima.recv().unwrap();
                let no_baris = v.0.clone();
                let halaman = v.1.clone();
                let (x, y) = if v.2 != "pub" {
                    (v.2, false)
                } else {
                    (terima.recv().unwrap().2, true)
                };
                let mut arg = (false, Vec::with_capacity(1));
                let mut kembali = Vec::with_capacity(1);
                loop {
                    v = terima.recv().unwrap();
                    match v.2.as_str() {
                        "->" => arg.0 = true,
                        ";" => break,
                        _ => {
                            if !arg.0 {
                                let nama = v.2.clone();
                                v = terima.recv().unwrap();
                                if v.2 != ":" {
                                    println!("{}",v.2);
                                    println!("{:?}",nama);
                                    println!("{}:{}:0",halaman,no_baris);
                                    panic!()
                                }
                                v = terima.recv().unwrap();

                                let point = if v.2 == "&" {
                                    v = terima.recv().unwrap();
                                    true
                                } else {
                                    false
                                };
                                let _mut_ = if v.2 == "mut" {
                                    v = terima.recv().unwrap();
                                    crate::parsing::mem_mode::mutabel
                                } else if v.2 == "statik" {
                                    v = terima.recv().unwrap();
                                    crate::parsing::mem_mode::statik
                                } else {
                                    crate::parsing::mem_mode::imutabel
                                };
                                let _tipe_ = match v.2.as_str() {
                                    "u8" => crate::parsing::args::Int(8,point, _mut_, "".to_string(),None,None),
                                    _ => {
                                        panic!()
                                    }
                                };
                                arg.1.push(_tipe_)
                            } else {
                                let point = if v.2 == "&" {
                                    v = terima.recv().unwrap();
                                    true
                                } else {
                                    false
                                };
                                let _mut_ = if v.2 == "mut" {
                                    v = terima.recv().unwrap();
                                    crate::parsing::mem_mode::mutabel
                                } else if v.2 == "statik" {
                                    v = terima.recv().unwrap();
                                    crate::parsing::mem_mode::statik
                                } else {
                                    crate::parsing::mem_mode::imutabel
                                };
                                let _tipe_ = match v.2.as_str() {
                                    "u8" => crate::parsing::args::Int(8,point, _mut_, "".to_string(),None,None),
                                    _ => {
                                        panic!()
                                    }
                                };
                                kembali.push(_tipe_)
                            }
                        }
                    }
                }
                kirim
                    .send((
                        no_baris,
                        halaman,
                        crate::parsing::perintah::cpu(x, y, arg.1, kembali),
                    ))
                    .unwrap();
            }
            "modul" => {
                kirim
                    .send((
                        v.0,
                        v.1,
                        crate::parsing::perintah::modul_masuk(terima.recv().unwrap().2),
                    ))
                    .unwrap();
            }
            "modul_" => {
                kirim
                    .send((v.0, v.1, crate::parsing::perintah::modul_keluar))
                    .unwrap();
            }
            "konfig" => {
                panic!()
            }
            "" => {
                kirim
                    .send((v.0, v.1, crate::parsing::perintah::selesai("".to_string())))
                    .unwrap();
                break;
            }
            ";" => continue,
            _ => {
                let x = terima.recv().unwrap();
                match x.2.as_str() {
                    "+=" =>{
                        let mut vv = Vec::with_capacity(2);
                        vv.push(v.2); 
                        vv.push("+".to_string());
                        for _ in 0..1 {
                            let y = terima.recv().unwrap();
                            if y.2 == ";" {break}
                            vv.push(y.2);
                        }
                        //println!("{:?}",vv);
                        let vv = crate::parsing::perintah::tulis(vv[0].clone(),vv);
                        kirim.send((v.0,v.1,vv)).unwrap();
                    }
                    "=" => {}
                    "(" => {
                        //panggil fungsi
                        panic!()
                    }
                    ";"=>{

                    }
                    _ => {
                        //panic!()
                    }
                }
            }
        }
    }
}
