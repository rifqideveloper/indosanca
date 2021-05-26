pub fn parse(
    terima:std::sync::mpsc::Receiver<Vec<String>>
    ,kirim:std::sync::mpsc::Sender<String>,

){
    //kirim.send(format!("{{\"tipe\":\"program\",\"nama\":\"{nama}\"}}\n",nama = nama_app)).expect("parse_");
    let mut buf :Vec<String> =  Vec::from( terima.recv().expect("") );
    buf.reserve(20);
    while buf.len() != 0 {
        match (buf[0].as_str(),buf[1].as_str()){
            ("let",_)=>{
                let  (indx,tipe) = if buf[1] == "<"{
                    (2,"var_".to_string())
                } else {
                    (3, format!{ "var_{}" , buf[1] }  )
                };
                match buf[indx].as_str(){
                    "u8"=>{
                        kirim.send(format!("{{\"tipe\":\"{}\",\"data\":\"{}\",\"nama\":\"{}\",\"nilai\":\"{}\"}}\n",tipe,buf[indx],buf[indx+2],buf[indx+4])).expect("parse_");
                    }
                    ">"=>{
                        kirim.send(format!("{{\"tipe\":\"{}\",\"data\":\"auto\",\"nama\":\"{}\",\"nilai\":\"{}\"}}\n",tipe,buf[indx+1],buf[indx+3])).expect("parse_");
                    }
                    _=>{}
                }
            }
            (_,"=")=>{
                kirim.send(format!("{{\"tipe\":\"tulis\",\"var\":\"{0}\",\"nilai\":\"{1}\"}}\n",buf[0],buf[2])).expect("parse_");
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
    
    //println!("testting");
    kirim.send("".to_string()).expect("parse selesai");
    
}