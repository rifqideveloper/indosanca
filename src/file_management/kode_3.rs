pub fn baca(
    nama_forder:&String,
    kirim_alamat : std::sync::mpsc::Sender<String>
){
    for path in std::fs::read_dir(format!("{}\\kode", nama_forder)).unwrap() {
        kirim_alamat.send(path.unwrap().path().display().to_string()).unwrap()
    }
}