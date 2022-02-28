//prototipe
pub fn kode( bin: std::vec::Vec<crate::parsing::parse_3::Pohon> ){
    //prototipe
    println!("\n<<<<< aplikasi dimulai >>>>>");
    bin.iter().for_each(|i|{
        match i {
            crate::parsing::parse_3::Pohon::cetak(o) => {
                    match o {
                        crate::parsing::parse_3::Nilai::langsung_str(o) =>{
                            print!("{}",o)
                        }
                        _=>{}
                    }
            }
            _=>{}
        }
    });
    println!("\n<<<<< aplikasi selesai >>>>>")
}