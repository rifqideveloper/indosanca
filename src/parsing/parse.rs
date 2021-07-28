struct apa;
impl apa {
    fn _str(&self,t:&String) -> bool {
        t.ends_with("\"") && t.starts_with("\"")
    }
    fn nomer(&self,t:&String)-> bool {
        t.parse::<f32>().is_ok()
    }
}

macro_rules! _int {
    ($a:ident,$b:expr) => {
        if $b.parse::<$a>().is_ok(){
            format!("{{ \"tipe\":\"nomer\" ,\"nilai\": \"{}\" }}",$b)
        } else {
            format!("{{ \"tipe\":\"minta\" ,\"nilai\": \"{}\" }}",$b)
        }
    };
}
use crate::parsing::perintah;
//use crate::parsing::tipe;
fn token(buf:&Vec<String>,kirim:&std::sync::mpsc::Sender<perintah>,dalam_fn:&mut bool){
    match buf[0].as_str(){
        "let"=>{
            let (indx,nama) = if !*dalam_fn {
                std::process::exit(1);
            } else if buf[1] == "<"{
                (2,buf[4].clone())
            } else {
                (3, buf[5].clone()  )
            };
            kirim.send(
                perintah::variabel_null(buf[indx].clone(),nama.clone())
            ).unwrap();
            if indx + 2 < buf.len()-1{
                let mut v = Vec::new();
                for i in indx+4..buf.len() {
                    v.push(buf[i].clone())
                }
                kirim.send(
                    perintah::tulis(nama,v)
                ).unwrap()
            }
            //pengaman aktiv
        }
        "putar"=>{
            kirim.send(
                perintah::putar
            ).unwrap();
        }
        "putar_batas"=>{
            kirim.send(
                perintah::batas
            ).unwrap();
        }
        "lanjut"=>{
            kirim.send(
                if !*dalam_fn{
                    panic!()
                } else {
                    perintah::lanjut
                }
                
            ).unwrap()
        }
        "putus"=>{
            kirim.send(
                if !*dalam_fn{
                    panic!()
                } else {
                    perintah::putus
                }
                
            ).unwrap()
        }
        "$putus"|"$lanjut"=>{
            kirim.send(
                perintah::br(buf[1].clone())
            ).unwrap()
        }
        "cetak"=>{
            kirim.send(
                if !*dalam_fn{
                    panic!()
                } else {
                    perintah::cetak(buf[1].clone())
                }
                
            ).unwrap()
        }
        "cpu"=>{
            let (nama,publik) = if buf[0].as_str() == "pub"{
                (buf[2].clone(),true)
            } else {
                (buf[1].clone(),false)
            };
            *dalam_fn = true;
            kirim.send(
                perintah::cpu(nama,publik)
            ).unwrap()
        }
        "{"=>{
            kirim.send(
                perintah::blok_buka
            ).unwrap();
        }
        "}"=>{
            kirim.send(
                perintah::blok_tutup
            ).unwrap();

        }
        "blok"=>{
            kirim.send(
                perintah::blok(buf[1].clone())   
            ).unwrap();
        }
        "blok_"=>{
            kirim.send(
                perintah::blok_   
            ).unwrap();
        }
        _ if buf[0] == "!" && buf[1] == "("=> {
            //belum siap
            //fungsi arg
            if !*dalam_fn {
                println!("token tidak sesuai");
                std::process::exit(1);
            }
        }
        _ if buf[0] == "<" && buf[1] == "mod"=> {
            kirim.send(
                perintah::modul_masuk(buf[2].clone())
            ).expect("parse_");
        }
        _ if buf[0] == "<" && buf[1] == "mod"=> {
            kirim.send(
                perintah::modul_masuk(buf[2].clone())
            ).expect("parse_");
        }
        _ if buf[0] == "mod" && buf[1] == ">"=> {
            *dalam_fn = false;
            kirim.send(
                perintah::modul_keluar
            ).unwrap()
        }
        _=>{
            println!("salah token {:?}",buf);
            std::process::exit(1);
        }
    }
    /*versi lama
    match (buf[0].as_str(),buf[1].as_str()){
        ("<","mod")=>{
            
        }
        ("mod",">")=>{
            *dalam_fn = false;
            kirim.send(
                perintah::modul_keluar
            ).unwrap()
        }
        ("cetak",_)=>{
            
        }
        ("cpu",_)=>{
            let (nama,publik) = if buf[0].as_str() == "pub"{
                (buf[2].clone(),true)
            } else {
                (buf[1].clone(),false)
            };
            *dalam_fn = true;
            kirim.send(
                perintah::cpu(nama,publik)
            ).unwrap()
        }
        ("!","(")=>{
            //belum siap
            //fungsi arg
            if !*dalam_fn {
                println!("token tidak sesuai");
                std::process::exit(1);
            }
        }
        ("let",_)=>{
            let  (indx,nama) = if !*dalam_fn {
                std::process::exit(1);
            } else if buf[1] == "<"{
                (2,buf[4].clone())
            } else {
                (3, buf[5].clone()  )
            };
            kirim.send(
                perintah::variabel_null(buf[indx].clone(),nama.clone())
            ).unwrap();
            if indx + 2 < buf.len()-1{
                let mut v = Vec::new();
                for i in indx+4..buf.len() {
                    v.push(buf[i].clone())
                }
                kirim.send(
                    perintah::tulis(nama,v)
                ).unwrap()
            }
            //pengaman aktiv
        }
        (_,"=")=>{
            /*
            kirim.send(
                perintah::tulis(buf[0].clone(),
                    if apa._str(&buf[2]) {
                        //belum siap
                        format!("")
                    } else if apa.nomer(&buf[2]) {
                        format!("{{\"tipe\":\"nomer\",\"nilai\":\"{}\" }}",buf[2])
                    } else {
                        std::process::exit(1);
                    }
                )
            ).unwrap()
            */
        }
        ("jika",_)=>{
            //uji coba
            let mut log :Vec<String> = Vec::with_capacity(1); 
            for i in 1..buf.len()- 1 {
                match buf[i].as_str(){
                    "benar"=>{
                        log.push("benar".to_string());
                    }
                    "salah"=>{
                        log.push("salah".to_string());
                    }
                    _=>{}
                }
            }
        }
        _=>{
            println!("salah token");
            std::process::exit(1);
        }
    }
    */
}
pub fn parse(
    terima:std::sync::mpsc::Receiver<Vec<String>>
    ,kirim:std::sync::mpsc::Sender<perintah>,

){
    //kirim.send(format!("{{\"tipe\":\"program\",\"nama\":\"{nama}\"}}\n",nama = nama_app)).expect("parse_");
    #[allow(unused_assignments)]
    let mut buf :Vec<String> =  Vec::with_capacity(20);
    let mut dalam_fn = false;
    let mut duplikat:(bool,u64,Vec<Vec<String>>,bool,bool) = (false,0,Vec::with_capacity(2),false,false);
    while { buf = terima.recv().expect(""); buf.len() != 0 }  {
        if buf[0] == "duplikat" {
            if let Ok(o) = buf[1].parse::<u64>() {
                duplikat.1 = o;
                duplikat.0 = true;
            } else {
                panic!()
            }
        } else if buf[0] == "duplikat_batas" {
            if duplikat.3 && duplikat.4 {
                token(&Vec::from(["blok".to_string(),"d0".to_string()]), &kirim,&mut dalam_fn);
                for i in 0..duplikat.1{
                    token(&Vec::from(["blok".to_string(),"d1".to_string()]), &kirim,&mut dalam_fn);
                    for x in &duplikat.2{
                        let mut v :Vec<String>= Vec::with_capacity(x.len());
                        for y in x{
                            if y == "$!" {
                                v.push(format!("{}",i))
                            } else if y == "$putus"{
                                v.push("$putus".to_string());
                                v.push("$d0".to_string())
                            } else if y == "$lanjut"{
                                v.push("$lanjut".to_string());
                                v.push("$d1".to_string())
                            } else {
                                v.push(y.clone())
                            }
                        }
                        token(&v, &kirim, &mut dalam_fn)
                    }
                    token(&Vec::from(["blok_".to_string()]), &kirim,&mut dalam_fn);
                }
                token(&Vec::from(["blok_".to_string()]), &kirim,&mut dalam_fn);
            }else if duplikat.3 {
                token(&Vec::from(["blok".to_string(),"d0".to_string()]), &kirim,&mut dalam_fn);
                for i in 0..duplikat.1{
                    for x in &duplikat.2{
                        let mut v :Vec<String>= Vec::with_capacity(x.len());
                        for y in x{
                            if y == "$!" {
                                v.push(format!("{}",i))
                            } else if y == "$putus" {
                                v.push("$putus".to_string());
                                v.push("$d0".to_string())
                            } else {
                                v.push(y.clone())
                            }
                        }
                        token(&v, &kirim, &mut dalam_fn)
                    }
                }
                token(&Vec::from(["blok_".to_string()]), &kirim,&mut dalam_fn);
            }else if duplikat.4 {
                for i in 0..duplikat.1{
                    token(&Vec::from(["blok".to_string(),"d1".to_string()]), &kirim,&mut dalam_fn);
                    for x in &duplikat.2{
                        let mut v :Vec<String>= Vec::with_capacity(x.len());
                        for y in x{
                            if y == "$!" {
                                v.push(format!("{}",i))
                            } else if y == "$lanjut" {
                                v.push("$lanjut".to_string());
                                v.push("$d1".to_string())
                            } else {
                                v.push(y.clone())
                            }
                        }
                        token(&v, &kirim, &mut dalam_fn)
                    }
                    token(&Vec::from(["blok_".to_string()]), &kirim,&mut dalam_fn);
                }

            } else {
                for i in 0..duplikat.1{
                    for x in &duplikat.2{
                        let mut v :Vec<String>= Vec::with_capacity(x.len());
                        for y in x{
                            if y == "$!" {
                                v.push(format!("{}",i))
                            } else {
                                v.push(y.clone())
                            }
                        }
                        token(&v, &kirim, &mut dalam_fn)
                    }
                }
            }
            duplikat.0 = false;
            duplikat.2.clear();
        } else if duplikat.0 {
            if buf[0] == "$"{
                match buf[1].as_str() {
                    "putus"=>{
                        duplikat.2.push(Vec::from(["$putus".to_string()]));
                        duplikat.3 = true
                    }
                    "lanjut"=>{
                        duplikat.2.push(Vec::from(["$lanjut".to_string()]));
                        duplikat.4 = true
                    }
                    _=>{}
                }
            } else {
                duplikat.2.push(buf);
            }
            //println!("{:?}",duplikat.2)
        } else {
            token(&buf, &kirim, &mut dalam_fn)
        }
    }
    /*
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
                        kirim.send(format!("{{\"tipe\":\"{}\",\"data\":\"{}\",\"nama\":\"{}\",\"nilai\":{}}}",tipe,buf[indx],buf[indx+2],
                            _int!(u8,buf[indx+4])
                        )).expect("parse_u8");
                    }
                    "u16"=>{
                        kirim.send(format!("{{\"tipe\":\"{}\",\"data\":\"{}\",\"nama\":\"{}\",\"nilai\":{}}}",tipe,buf[indx],buf[indx+2],
                            _int!(u16,buf[indx+4])
                        )).expect("parse_u16");
                    }
                    "u32"=>{
                        kirim.send(format!("{{\"tipe\":\"{}\",\"data\":\"{}\",\"nama\":\"{}\",\"nilai\":{}}}",tipe,buf[indx],buf[indx+2],
                            _int!(u32,buf[indx+4])
                        )).expect("parse_u32");
                    }
                    "u64"=>{
                        kirim.send(format!("{{\"tipe\":\"{}\",\"data\":\"{}\",\"nama\":\"{}\",\"nilai\":{}}}",tipe,buf[indx],buf[indx+2],
                            _int!(u64,buf[indx+4])
                        )).expect("parse_u64");
                    }
                    "i8"=>{
                        kirim.send(format!("{{\"tipe\":\"{}\",\"data\":\"{}\",\"nama\":\"{}\",\"nilai\":{}}}",tipe,buf[indx],buf[indx+2],
                            _int!(i8,buf[indx+4])
                        )).expect("parse_i8");
                    }
                    "i16"=>{
                        kirim.send(format!("{{\"tipe\":\"{}\",\"data\":\"{}\",\"nama\":\"{}\",\"nilai\":{}}}",tipe,buf[indx],buf[indx+2],
                            _int!(i16,buf[indx+4])
                        )).expect("parse_i16");
                    }
                    "i32"=>{
                        kirim.send(format!("{{\"tipe\":\"{}\",\"data\":\"{}\",\"nama\":\"{}\",\"nilai\":{}}}",tipe,buf[indx],buf[indx+2],
                            _int!(i32,buf[indx+4])
                        )).expect("parse_i32");
                    }
                    "i64"=>{
                        kirim.send(format!("{{\"tipe\":\"{}\",\"data\":\"{}\",\"nama\":\"{}\",\"nilai\":{}}}",tipe,buf[indx],buf[indx+2],
                            _int!(i64,buf[indx+4])
                        )).expect("parse_i64");
                    }
                    ">"=>{
                        kirim.send(format!("{{\"tipe\":\"{}\",\"data\":\"auto\",\"nama\":\"{}\",\"nilai\":\"{}\"}}",tipe,buf[indx+1],buf[indx+3])).expect("parse_");
                    }
                    _=>{}
                }
            }
            (_,"=")=>{
                //pemulaan
                if buf[2].parse::<f64>().is_ok(){
                    kirim.send(format!("{{\"tipe\":\"tulis\",\"var\":\"{0}\",\"nilai\": {{ \"tipe\":\"nomer\",\"nilai\":\"{1}\" }} }}",buf[0],buf[2])).expect("parse_");
                } else if apa_str(&buf[2]) {
                    kirim.send(format!("{{\"tipe\":\"tulis\",\"var\":\"{0}\",\"nilai\": {{ \"tipe\":\"str\",\"nilai\":\"{1}\" }} }}",buf[0],buf[2])).expect("parse_");
                } else {
                    kirim.send(format!("{{\"tipe\":\"tulis\",\"var\":\"{0}\",\"nilai\": {{ \"tipe\":\"minta\",\"nilai\":\"{1}\" }} }}",buf[0],buf[2])).expect("parse_");
                }
                
            }
            ("cetak",_)=>{
                kirim.send(format!("{{\"tipe\":\"cetak\",\"nilai\":\"{}\"}}",buf[1])).expect("parse_");
            }
            ("<","mod")=>{
                kirim.send(format!("{{\"tipe\":\"modul_masuk\",\"nama\":\"{nama}\"}}",nama = buf[2])).expect("parse_");
            }
            ("mod",">")=>{
                kirim.send(format!("{{\"tipe\":\"modul_keluar\"}}")).expect("parse_");
            }
            ("cpu",_)=>{
                kirim.send(format!("{{\"tipe\":\"fn_cpu\",\"nama\":\"{}\"}}",buf[1])).expect("parse_");
            }
            _=>{}
        }
        buf =  terima.recv().expect("") ;
    }
    */
    //println!("testting");
    kirim.send(perintah::selesai).expect("parse gagal selesai");
    
}