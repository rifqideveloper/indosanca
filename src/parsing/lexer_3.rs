use std::io::Read;
pub fn lexer(
    nama_file: std::sync::mpsc::Receiver<String>,
    kirim_token : std::sync::mpsc::Sender<(usize,String,String,)>
){
    let mut file_baris = (1usize,nama_file.recv().unwrap());
    let mut buf = String::with_capacity(10);
    let mut file = std::fs::File::open(&file_baris.1).unwrap();
    file_baris.1 = file_baris.1.replace("\\","/");
    let mut buf_1 = [0];
    //let mut buf_ : Vec<String> = Vec::with_capacity(2);
    let mut token = false;
    kirim_token.send((file_baris.0.clone(),file_baris.1.clone(),"modul".to_string())).unwrap();
    
    
    kirim_token.send((file_baris.0.clone(),file_baris.1.clone(),file_baris.1.split("/").collect::<Vec<&str>>().last().unwrap().to_string())).unwrap();
    kirim_token.send((file_baris.0.clone(),file_baris.1.clone(),";".to_string())).unwrap();
    buf.clear();
    loop {
        let nomer = file.read(&mut buf_1).unwrap();
        if "\n".as_bytes() == buf_1 {
            file_baris.0 += 1
        }
        if nomer == 0 {
            if !buf.is_empty() {
                kirim_token.send( (file_baris.0.clone(),file_baris.1.clone(),buf.trim_end().to_string()) ).unwrap();
            }
            kirim_token.send((file_baris.0.clone(),file_baris.1.clone(),";".to_string())).unwrap();
            kirim_token.send((file_baris.0.clone(),file_baris.1.clone(),"modul_".to_string())).unwrap();
            if let Ok(o) = nama_file.try_recv() {
                file = std::fs::File::open(&o).unwrap();
                file_baris.0 = 1;
                file_baris.1 = o.replace("\\","/",);
                kirim_token.send((file_baris.0.clone(),file_baris.1.clone(),"modul".to_string())).unwrap();
                kirim_token.send((file_baris.0.clone(),file_baris.1.clone(),file_baris.1.split("/").collect::<Vec<&str>>().last().unwrap().to_string())).unwrap();
                kirim_token.send((file_baris.0.clone(),file_baris.1.clone(),";".to_string())).unwrap();
                buf.clear();
            } else {
                break
            }
        }
        if buf_1 != " ".as_bytes() {
            buf.push_str(std::str::from_utf8(&buf_1).unwrap());
            token = true
        } else if token {
            token = false;
            if buf.ends_with("\n") || buf.ends_with(";"){
                kirim_token.send( (file_baris.0.clone(),file_baris.1.clone(),buf.trim_end().to_string() )).unwrap();
                kirim_token.send( (file_baris.0.clone(),file_baris.1.clone(),";".to_string())).unwrap();
            } else {
                kirim_token.send( (file_baris.0.clone(),file_baris.1.clone(),buf.trim_end().to_string() )).unwrap();
            }
            buf.clear();
        }
    }
    kirim_token.send((0, "".to_string(), "".to_string())).unwrap();
}