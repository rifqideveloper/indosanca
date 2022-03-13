fn tipe_(terima: &std::sync::mpsc::Receiver<(u64,String,String)>) -> crate::parsing::Tipe {
    fn nilai<T: std::str::FromStr>(
        v: &mut std::vec::Vec<std::option::Option<T>>,
        terima: &std::sync::mpsc::Receiver<(u64,String,String)>,
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
            _ => { panic!() }
        }
    }
    match terima.recv().unwrap().2.as_str() {
        "u8" => {
            let mut v = Vec::new();
            nilai(&mut v, terima);
            //println!("{:?}",v);
            crate::parsing::Tipe::_u8(v)
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
    terima: std::sync::mpsc::Receiver<(u64,String,String)>,
    kirim: std::sync::mpsc::Sender<(u64,String,crate::parsing::perintah)>,
) {
    loop {
        let mut v = terima.recv().unwrap();
        //println!("{}",v);
        match v.2.as_str() {
            "let" => {
                v = terima.recv().unwrap();
                if v.2 == "mut" {
                    let nama = terima.recv().unwrap();
                    let tipe = tipe_(&terima);
                    kirim
                        .send((v.0,v.1,crate::parsing::perintah::_let(nama.2, true, tipe)))
                        .unwrap();
                } else {
                    kirim
                        .send((v.0,v.1,crate::parsing::perintah::_let(v.2, false, tipe_(&terima))))
                        .unwrap();
                }
            }
            "cetak" =>{
                let mut nilai = Vec::with_capacity(1);
                loop {
                    v = terima.recv().unwrap();
                    if v.2 == ";" {
                        break
                    }
                    nilai.push(v.2);
                }
                kirim.send(
                    (v.0,v.1,crate::parsing::perintah::cetak(nilai))
                ).unwrap();
            }
            "fn" => {
                //panic!();
                v = terima.recv().unwrap();
                let (x, y) = if v.2 != "pub" {
                    (v.2, false)
                } else {
                    (terima.recv().unwrap().2, true)
                };
                kirim.send((v.0,v.1,crate::parsing::perintah::cpu(x, y))).unwrap();
            }
            "modul" => {
                kirim
                    .send((v.0,v.1,crate::parsing::perintah::modul_masuk(
                        terima.recv().unwrap().2,
                    )))
                    .unwrap();
            }
            "modul_" => {
                kirim.send((v.0,v.1,crate::parsing::perintah::modul_keluar)).unwrap();
            }
            "" => {
                kirim.send(
                    (v.0,v.1,crate::parsing::perintah::selesai("".to_string()))
                ).unwrap();
                break
            },
            ";" => continue,
            _ => {
                let x = terima.recv().unwrap();
                match x.2.as_str() {
                    "=" => {}
                    "(" => {
                        //panggil fungsi
                        panic!()
                    }
                    _ => {}
                }
            }
        }
    }
}
