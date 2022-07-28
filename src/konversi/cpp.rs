use std::io::SeekFrom;
use std::io::prelude::*;

const iostream: &str = "#include<iostream>\n";
const cout: &str = "std::cout<<";
static mut _iostream: bool = false;

fn cetak(arg: crate::parsing::args, 
    main_file: &mut std::fs::File,
    main_func: &mut std::fs::File
) {
    match arg {
        crate::parsing::args::Str_Lansung(str_) => {
            if unsafe { !_iostream } {
                unsafe { _iostream = true }
                main_file.write_all(iostream.as_bytes()).unwrap()
            }
            main_func.write_all(format!("{}\"{}\";",cout,str_).as_bytes()).unwrap()
        }
        _ => {/*panic!()*/}
    }
}
pub fn konversi(
    _var: &crate::parsing::Arrmap<String, Vec<crate::parsing::Let_>>,
    terima: std::sync::mpsc::Receiver<crate::parsing::Pohon>,
    path: &String,
) -> bool {

    std::fs::create_dir_all(format! {"{}\\parsing\\cpp\\instruksi",path});
    std::fs::create_dir_all(format! {"{}\\target\\debug\\cpp",path});
    
    
    let mut main_ = std::fs::File::options().read(true).write(true).create(true).truncate(true).open(format! {"{}\\parsing\\cpp\\instruksi\\main",path}).unwrap();
    let mut main_func = std::fs::File::options().read(true).write(true).create(true).truncate(true).open(format! {"{}\\parsing\\cpp\\instruksi\\main_f",path}).unwrap();
    loop {
        match terima.recv().unwrap() {
            crate::parsing::Pohon::Let(nama, index) => {}
            crate::parsing::Pohon::Cetak(arg) => {
                cetak(arg, &mut main_,&mut main_func)
            }
            crate::parsing::Pohon::tidur(arg) => {}
            crate::parsing::Pohon::tulis(nama, index, nilai) => {}
            crate::parsing::Pohon::blok(nama) => {}
            crate::parsing::Pohon::blok_ => {}
            crate::parsing::Pohon::br(nama) => {}
            crate::parsing::Pohon::Error => {}
            crate::parsing::Pohon::panic(pesan) => {}
            crate::parsing::Pohon::Selesai => break,
           
        }
    }
    let mut cpp = std::fs::File::create(format! {"{}\\target\\debug\\cpp\\main.cpp",path}).unwrap();
    let mut buf = [0];

    main_.seek(SeekFrom::Start(0)).unwrap();
    main_func.seek(SeekFrom::Start(0)).unwrap();

    while main_.read(&mut buf).unwrap() != 0 {
        cpp.write(&mut buf).unwrap();
    }
    cpp.write_all(b"int main() {").unwrap();
    while main_func.read(&mut buf).unwrap() != 0 {
        cpp.write(&mut buf).unwrap();
    }
    cpp.write_all(b"return 0;}").unwrap();

    true
}
