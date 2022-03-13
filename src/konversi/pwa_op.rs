pub fn konversi(terima: std::sync::mpsc::Receiver<crate::parsing::parse_3_1::Pohon>) {
    let mut v = terima.recv().unwrap();
    //pertama main seterusnya lib
    //jika kosong maka main
    let mut perpus = "";
    'luar:loop {
        match v {
            crate::parsing::parse_3_1::Pohon::fungsi(nama) =>{

            }
            crate::parsing::parse_3_1::Pohon::tambah_fungsi(nama) =>{
                perpus = nama.as_str() ;
            }
            crate::parsing::parse_3_1::Pohon::selesai => { 
                loop{
                    match terima.recv().unwrap() {
                        crate::parsing::parse_3_1::Pohon::_let(var) =>{

                        }
                        crate::parsing::parse_3_1::Pohon::selesai => {
                            v = terima.recv().unwrap();
                            //jika selesai dua kali maka app sudah jadi
                            if let crate::parsing::parse_3_1::Pohon::selesai = v {
                                break 'luar
                            }
                            continue 'luar
                        }
                        _=>{
                            panic!()
                        }
                    }
                }
            }
            crate::parsing::parse_3_1::Pohon::_let(var)=>{
                //panic!()
            }
            crate::parsing::parse_3_1::Pohon::kopilasi_error =>{
                return
            }
            _=>{}
        }
        v = terima.recv().unwrap();
    }
}