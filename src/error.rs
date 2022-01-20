use ansi_term::Colour::Red;
use ansi_term::Colour::Green;
use ansi_term::Colour::Blue;
pub enum ErrorKode {
    Oke,
    Error(String),
    ErrorSalahToken,
    ErrVariabelTidakDitemukan
}
pub fn error_konversi(nama:&str,kode:ErrorKode){
    let _enabled = ansi_term::enable_ansi_support();
    let mut x = String::new();
    print!("{} {}...{}{}",Blue.paint("kompilasi"),nama,
        match kode {
            ErrorKode::Oke => Green.paint("ok\n"),
            ErrorKode::Error(o) => {
                x = o ;
                Red.paint("error\n")
            }
            ErrorKode::ErrorSalahToken => Red.paint("error\nToken tidak sesuai"),
            ErrorKode::ErrVariabelTidakDitemukan => Red.paint("error\nvariabel tidak ditemukan\n")
        },
        x
    );
    
}
