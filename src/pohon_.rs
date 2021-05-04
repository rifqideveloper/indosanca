pub fn baca(_proyek:&String,kirim:std::sync::mpsc::Sender<std::string::String>){
    
    kirim.send("".to_string()).expect("msg: &str");
}