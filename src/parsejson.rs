


pub fn parse(nama_app:String,terima:std::sync::mpsc::Receiver<std::vec::Vec<std::string::String>>,kirim:std::sync::mpsc::Sender<std::string::String>){
    kirim.send(format!("{{\"tipe\":\"program\",\"nama\":\"{nama}\"}}\n",nama = nama_app)).expect("msg: &str");

    let mut buf = terima.recv().expect("");
    while buf.len() != 0 {
        match buf[0].as_str() {
            "cpu"=>{
               kirim.send(format!(
                   "{{\"tipe\":\"fn/cpu\",\"nama\":\"{}\"}}\n",
                   buf[1]
               )).expect("msg: &str");
            }
            "let"=>{
                let mut x = 2;
                kirim.send(format!(
                    "{{\"tipe\":\"var\",\"data\":\"{}\",\"nama\":\"{}\"}}\n",
                    {
                        if buf[x] == "str"{x = 4;"str"}
                        else if buf[x] == ">"{x = 3;"auto"}
                        else {x = 4;"auto"}
                    },
                    buf[x]
                )).expect("msg: &str");
                if buf[x+2].starts_with("(str)") {
                    kirim.send(format!(
                        "{{\"tipe\":\"str\",\"nilai\":\"{}\"}}\n",
                        {
                            let mut t = buf[x+2].clone();
                            t.remove(0);
                            t.remove(0);
                            t.remove(0);
                            t.remove(0);
                            t.remove(0);
                            t
                        }
                    )).expect("msg: &str")
                }
                kirim.send("{\"tipe\":\"var/akhir\"}\n".to_string()).expect("msg: &str");
            }
            _=>{}
        }
        buf = terima.recv().expect("");
    }
    kirim.send("".to_string()).expect("msg: &str");
}

