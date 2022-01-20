//
//
pub fn node(pohon: &std::vec::Vec<crate::parsing::parse_3::Pohon>, path: &String, nama: String) -> crate::error::ErrorKode {
    
    
    let proyek = format!("{}\\target\\debug\\pwanode\\publik",path);
    std::fs::create_dir_all(&proyek).unwrap();
    if let Ok(mut o) = std::process::Command::new("node").args(&["--version"]).spawn() /*  crate::jalankan::execute("npm",&[""]) */{
        pohon.iter().for_each(|i|match i {
            _=>{}
        });
        o.wait().unwrap();
        crate::error::ErrorKode::Oke
    } else {
        crate::error::ErrorKode::Error("\tnode js tidak ditemukan disarankan untuk mendownload di https://nodejs.org/en/\n".to_string())
    }

}