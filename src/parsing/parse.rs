pub fn parse(
    nama_app:String,
    terima:std::sync::mpsc::Receiver<Vec<String>>
    ,kirim:std::sync::mpsc::Sender<String>,
    lanjut:std::sync::mpsc::Sender<bool>
){
    kirim.send(format!("{{\"tipe\":\"program\",\"nama\":\"{nama}\"}}\n",nama = nama_app)).expect("parse_");
    let mut buf :Vec<String> =  Vec::from( terima.recv().expect("") );
    buf.reserve(20);
    while buf.len() != 0 {
        match (buf[0].as_str(),buf[1].as_str()){
            ("let","<")=>{
                match buf[2].as_str(){
                    "u8"=>{
                        kirim.send(format!("{{\"tipe\":\"var_\",\"data\":\"{}\",\"nama\":\"{}\",\"nilai\":\"{}\"}}\n",buf[2],buf[4],buf[6])).expect("parse_");
                    }
                    ">"=>{
                        kirim.send(format!("{{\"tipe\":\"var_\",\"data\":\"auto\",\"nama\":\"{}\",\"nilai\":\"{}\"}}\n",buf[3],buf[5])).expect("parse_");
                    }
                    _=>{}
                }
            }
            ("cetak",_)=>{
                kirim.send(format!("{{\"tipe\":\"cetak\",\"nilai\":\"{nama}\"}}\n",nama = buf[1])).expect("parse_");
            }
            ("<","mod")=>{
                kirim.send(format!("{{\"tipe\":\"modul_masuk\",\"nama\":\"{nama}\"}}\n",nama = buf[2])).expect("parse_");
            }
            ("mod",">")=>{
                kirim.send(format!("{{\"tipe\":\"modul_keluar\"}}\n")).expect("parse_");
            }
            ("cpu",_)=>{
                kirim.send(format!("{{\"tipe\":\"fn_cpu\",\"nama\":\"{}\"}}\n",buf[1])).expect("parse_");
            }
            _=>{}
        }
        buf =  terima.recv().expect("") ;
    }
    lanjut.send(true).expect("lanjut ke parse_2");
    //println!("testting");
    kirim.send("".to_string()).expect("parse selesai");
    
}