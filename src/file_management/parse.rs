pub fn tulis(
    mulai:std::sync::mpsc::Sender<()>,
    _proyek:&String,
    f:&str,terima:std::sync::mpsc::Receiver<std::string::String>,
    
){
    use std::io::Write;
    let mut file;
    match std::fs::File::create(format!("{}\\parsing\\{}",_proyek,f)){
        Ok(o)=>{file = o}
        Err(_)=>{
            std::fs::create_dir_all(format!("{}\\parsing",_proyek)).expect("msg: &str");
            file = std::fs::File::create(format!("{}\\parsing\\{}",_proyek,f)).expect("msg: &str")
        }
    };
    let mut _buf = String::with_capacity(40);
    loop {
        _buf = terima.recv().expect("");
        if _buf == "" {
            file.write(_buf.as_bytes()).expect("");
            break
        }
        file.write(_buf.as_bytes()).expect("");
    }
    drop(file);
    mulai.send(()).unwrap();
}