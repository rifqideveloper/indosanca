struct apa;
impl apa {
    fn _str(&self, t: &String) -> bool {
        t.ends_with("\"") && t.starts_with("\"")
    }
    fn nomer(&self, t: &String) -> bool {
        t.parse::<f32>().is_ok()
    }
}

macro_rules! _int {
    ($a:ident,$b:expr) => {
        if $b.parse::<$a>().is_ok() {
            format!("{{ \"tipe\":\"nomer\" ,\"nilai\": \"{}\" }}", $b)
        } else {
            format!("{{ \"tipe\":\"minta\" ,\"nilai\": \"{}\" }}", $b)
        }
    };
}
fn rv(buf: &Vec<String>, kirim: &std::sync::mpsc::Sender<perintah>, x: usize) {
    let mut bukan = false;
    /*
    let mut i = x.clone();
    while if i < buf.len() {false} else { i += 1; true} {
        match buf[i].as_str(){
            /*prototyp
            x if matches!(x.parse::<f32>(),
                Ok(y)
                if {true}
            ) =>{}
            */
            "benar"=>{
                kirim.send(
                    perintah::boolean(true)
                ).unwrap()
            }
            "salah"=>{
                kirim.send(
                    perintah::boolean(false)
                ).unwrap()
            }
            "!"=>{
                bukan = true;
                continue
            }
            //i32.add
            "+"=>{
                kirim.send(
                    perintah::tambah
                ).unwrap();
            }
            //i32.sub
            "-"=>{
                kirim.send(
                    perintah::kurang
                ).unwrap();
            }
            //???
            "/"=>{}
            //i32.mul
            "*"=>{}
            //???
            "%"=>{}
            //???
            "."=>{}
            //i32.Ge_s
            ">"=>{

            }
            //i32.it_s
            "<"=>{

            }
            "="=>{
                panic!()
            }
            ","=>{
            }
            x if matches!(x.parse::<i32>(),
                Ok(y)
                if {kirim.send(perintah::_i32_konst(y)).unwrap(); true}
            ) =>{}
            _=>{
                panic!()
            }
        }
        if bukan {
            kirim.send(
                perintah::i32_eqz
            ).unwrap();
            bukan = false;
        }
    }*/
    fn pol(pola: &mut u8, kirim: &std::sync::mpsc::Sender<perintah>) {
        match pola {
            1 => {
                kirim.send(perintah::sama).unwrap();
            }
            2 => {
                kirim.send(perintah::tidak_sama).unwrap();
            }
            3 => {
                kirim.send(perintah::sama_lebih_besar).unwrap();
            }
            4 => {
                kirim.send(perintah::sama_lebih_kecil).unwrap();
            }
            5 => {
                kirim.send(perintah::lebih_kecil).unwrap();
            }
            6 => {
                kirim.send(perintah::lebih_besar).unwrap();
            }
            _ => return,
        }
        *pola = 0
    }
    fn logi(l: bool, bukan: &mut bool, kirim: &std::sync::mpsc::Sender<perintah>) {
        if *bukan {
            kirim.send(perintah::boolean(!l)).unwrap();
            *bukan = false
        } else {
            kirim.send(perintah::boolean(l)).unwrap();
        }
    }

    //jika peringatan true setelah loop selesai maka error
    for i in x..buf.len() {
        match buf[i].as_str() {
            "benar" => {
                logi(true, &mut bukan, kirim);
            }
            "salah" => {
                logi(false, &mut bukan, kirim);
            }
            "!" => {
                bukan = true;
            }
            //i32.add
            "+" => kirim.send(perintah::tambah).unwrap(),
            //i32.sub
            "-" => kirim.send(perintah::kurang).unwrap(),
            //???
            "/" => kirim.send(perintah::bagi).unwrap(),
            //i32.mul
            "*" => kirim.send(perintah::kali).unwrap(),
            "%" => kirim.send(perintah::modus).unwrap(),
            //???
            "." => {}
            "=" => kirim.send(perintah::sama_dengan).unwrap(),
            //pola 1
            "==" => kirim.send(perintah::i32_eq).unwrap(),
            //pola 2
            "!=" => {
                kirim.send(perintah::tidak_sama).unwrap()
                /*
                kirim.send(
                    perintah::i32_eqz
                ).unwrap()
                */
            }
            //pola 3
            //sama lebih besar
            "<=" => kirim.send(perintah::sama_lebih_besar).unwrap(),
            //pola 4
            //sama lebih kecil
            ">=" => kirim.send(perintah::sama_lebih_kecil).unwrap(),
            //pola 5
            //lebih kecil
            //i32.Ge_s
            ">" => kirim.send(perintah::lebih_kecil).unwrap(),
            //pola 6
            //lebih besar
            //i32.it_s
            "<" => kirim.send(perintah::lebih_besar).unwrap(),
            //prototipe
            x if matches!(x.parse::<i32>(),
                Ok(y)
                if {
                    kirim.send(perintah::_i32_konst(y)).unwrap();
                    true
                }
            ) => {}
            /*
            variabel =>{}
            */
            _ => {}
        }
        if bukan {
            panic!()
        }
        //
    }
    /*
    match buf[x].as_str(){
        "benar"=>{
            kirim.send(
                perintah::boolean(true)
            ).unwrap()
        }
        "salah"=>{
            kirim.send(
                perintah::boolean(false)
            ).unwrap()
        }
        "!"=>{
            match buf[x+1].as_str(){
                "benar"=>{
                    kirim.send(
                        perintah::boolean(true)
                    ).unwrap()
                }
                "salah"=>{
                    kirim.send(
                        perintah::boolean(false)
                    ).unwrap()
                }
                _=>{   }
            }
            kirim.send(
                perintah::i32_eqz
            ).unwrap()
        }
        _=>{}
    }
    */
}
/*
fn jika(i:usize,buf:&mut std::vec::Vec<std::string::String>,kirim:&std::sync::mpsc::Sender<perintah>){
    let mut tidak = false;
    for i in i..buf.len() {
        match buf[i].as_str(){
            "!"=>{ tidak = true }
            "benar"=>{
                kirim.send(
                    perintah::boolean(
                        if tidak {
                            tidak = false;
                            false
                        } else {
                            true
                        }
                    )
                ).unwrap();
            }
            "salah"=>{
                kirim.send(
                    perintah::boolean(
                        if tidak {
                            tidak = false;
                            true
                        } else {
                            false
                        }
                    )
                ).unwrap()
            }
            _=>{}
        }
    }
    kirim.send(
        perintah::jika_
    ).unwrap()
}
*/
use crate::parsing::perintah;
use crate::parsing::Tipe;
//use crate::parsing::tipe;
/*
fn glob_tipe(nama:&String,_mut:bool,tipe:Tipe,kirim:&std::sync::mpsc::Sender<perintah>){
    kirim.send(
        perintah::glob_var(nama.to_string(),_mut,tipe)
    ).unwrap()
}
*/
fn let_(nama: &String, _mut: bool, tipe: Tipe, kirim: &std::sync::mpsc::Sender<perintah>) {
    kirim
        .send(perintah::_let(nama.to_string(), _mut, tipe))
        .unwrap()
}
fn cek_jika_nama_var_sesuai(nama: String, i: &mut usize) -> String {
    *i += 1;
    nama
}
fn cek_tipe_var(buf: &Vec<String>, i: &mut usize) -> Tipe {
    fn array(buf: &Vec<String>, i: &mut usize) -> Tipe {
        //error ???
        panic!();
        /*
        match buf[*i].as_str() {
            "u8" => {
                //panic!();
                *i += 1;
                if buf[*i].as_str() == ";" {
                    *i += 1;
                    if let Ok(total) = buf[*i].parse::<usize>() {
                        if total == 0 {
                            panic!()
                        }
                        if buf[*i + 1] != "]" {
                            panic!()
                        }
                        if buf[*i + 2] != "=" {
                            panic!()
                        }
                        *i += 3;
                        match buf[*i].as_str() {
                            "[" => {
                                panic!()
                            }
                            "?" => {
                                let mut v: Vec<String> = Vec::with_capacity(total * 2 + 1);
                                v.push("[".to_string());
                                let x = v.len() - 2;
                                let mut i = 0;
                                loop {
                                    v.push("0".to_string());
                                    if i == x {
                                        break;
                                    }
                                    v.push(",".to_string());
                                    i += 1
                                }
                                v.push("[".to_string());
                                Tipe::_u8_ar(total, v)
                            }
                            _ => {
                                panic!()
                            }
                        }
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
            }
            _ => {
                panic!()
            }
        }
        */
    }
    match buf[*i].as_str() {
        /*
        "u8" => Tipe::_u8({
            *i += 1;
            if buf[*i] == "=" {
                if let Ok(o) = buf[*i + 1].parse::<u8>() {
                    Some(o)
                } else {
                    panic!()
                }
            } else {
                None
            }
        }),
        */
        "u8" => {
            let mut v = Vec::new();
            *i += 1;
            if buf[*i] == ":" {
                *i += 1;
                let _len_: usize = buf[*i].parse().unwrap();
                //contoh
                v.reserve_exact(_len_);
                *i += 1;
                if buf[*i] == "=" {
                    *i += 1;
                    if buf[*i] != "[" {
                        panic!()
                    }
                } else {
                    panic!()
                }
                loop {
                    *i += 1;
                    if let Ok(ok) = buf[*i].parse() {
                        v.push(Some(ok))
                    } else if buf[*i] == "]" {
                        break;
                    }
                    /*
                    if let Ok(ok) = buf[*i].parse() {
                        v.push(Some(ok))
                    } else if buf[*i] != "," {
                        if buf[*i] == "]" {
                            if  v.len() != _len_ {
                                panic!()
                            }
                            break
                        } else {
                            panic!()
                        }
                    }
                    */
                }
                //println!("testing -> {:?}" ,v);

                /*
                for _ in 0.._len_ {
                    loop{

                    }
                }
                */

                /*
                *i += 1 ;
                if let Ok(o) = buf[*i].parse::<usize>() {
                    *i += 1 ;
                    if buf[*i] == "=" {
                        *i += 1 ;
                        v.reserve_exact(o);
                        match buf[*i].as_str() {
                            "?" => {
                                for _ in 0..o {
                                    v.push(Some(0))
                                }
                            }
                            "??" => {
                                panic!()
                            }
                            "[" =>{
                                panic!()
                            }
                            _=>{panic!()}
                        }
                    } else {
                        panic!();
                    }
                } else {
                    panic!()
                }
                */
            } else if buf[*i] == "=" {
                *i += 1;
                if let Ok(o) = buf[*i].parse::<u8>() {
                    v.reserve_exact(1);
                    v.push(Some(o))
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
            Tipe::_u8(false, v)
        }
        "Vec" => {
            //panic!();
            *i += 1;
            if buf[*i] != ":" {
                panic!()
            }
            *i += 1;
            match buf[*i].as_str() {
                "u8" => {
                    *i += 1;
                    if buf[*i] != "=" {
                        panic!()
                    }
                    *i += 1;
                    if buf[*i] != "Vec" {
                        panic!()
                    }
                    let mut v = Vec::new();
                    *i += 1;
                    if let Ok(ok) = buf[*i].parse() {
                        v.reserve(ok);
                        *i += 1;
                        if buf.len() <= *i {
                            if buf[*i] != "[" {
                                panic!()
                            }
                            loop {
                                *i += 1;
                                if buf.len() == *i {
                                    panic!()
                                } else if buf[*i] == "]" {
                                    break;
                                } else if buf[*i] == "," {
                                    continue;
                                } else if let Ok(ok) = buf[*i].parse() {
                                    v.push(Some(ok));
                                }
                            }
                            if buf.len() < ok {
                                panic!()
                            }
                        }
                    } else if buf[*i] == "[" {
                        loop {
                            *i += 1;
                            if buf.len() == *i {
                                panic!()
                            } else if buf[*i] == "]" {
                                break;
                            } else if buf[*i] == "," {
                                continue;
                            } else if let Ok(ok) = buf[*i].parse() {
                                v.push(Some(ok));
                            } else {
                                panic!()
                            }
                        }
                        v.shrink_to_fit()
                    } else {
                        panic!()
                    }
                    /*
                    *i += 1 ;
                    if buf[*i] != ":" { panic!()}
                    *i += 1 ;
                    let mut v = Vec::with_capacity(buf[*i].parse().unwrap());
                    *i += 1 ;
                    if buf[*i] == "=" {
                        //println!("testing 0");
                        *i += 1 ;
                        if buf[*i] != "Vec" { panic!() }
                        *i += 1 ;
                        if buf[*i] == "[" {
                            loop {
                                *i += 1 ;
                                if buf.len() == *i { panic!() }
                                else if buf[*i] == "]" {
                                    break
                                } else if buf[*i] == "," {
                                    continue
                                } else if let Ok(ok) = buf[*i].parse() {
                                    v.push(Some(ok));
                                } else {
                                    panic!()
                                }
                            }
                            v.shrink_to_fit();
                        } else if let Ok(ok) = buf[*i].parse() {
                            v.reserve(ok);
                            println!("testing 1");
                            *i += 1 ;
                            if buf.len() <= *i {
                                if buf[*i] != "[" { panic!() }
                                loop {
                                    *i += 1 ;
                                    if buf.len() == *i { panic!() }
                                    else if buf[*i] == "]" {
                                        break
                                    } else if buf[*i] == "," {
                                        continue
                                    } else if let Ok(ok) = buf[*i].parse() {
                                        v.push(Some(ok));
                                    }
                                }
                                if buf.len() < ok {
                                    panic!()
                                }
                            }

                        } else {
                            panic!()
                        }
                    } else {
                        panic!()
                    }
                    println!("testing");
                    */
                    Tipe::_u8(true, v)
                }
                _ => {
                    panic!()
                }
            }
        }
        "str" => Tipe::_string({
            *i += 1;
            if buf[*i] == "=" {
                let mut v = buf[*i + 1].clone();
                if v.starts_with('"') && v.ends_with('"') {
                    v.remove(0);
                    v.pop();
                } else {
                    panic!()
                }
                crate::parsing::Str::Some(v)
            } else {
                crate::parsing::Str::None
            }
        }),
        //"[" => array(buf, i),
        "string" | "String" => {
            panic!()
        }
        _ => {
            panic!()
        }
    }
}
fn token(
    mut buf: Vec<String>,
    jika_br: &mut bool,
    terima: &std::sync::mpsc::Receiver<Vec<String>>,
    kirim: &std::sync::mpsc::Sender<perintah>,
    dalam_fn: &mut bool,
    perintah_terakhir: &mut perintah,
) {
    //let mut var = Vec::new();
    //let mut lapisan = 0 ;
    loop {
        match buf[0].as_str() {
            /*
            "konst"=>{

                /*
                if buf[1] != "mut"{
                    panic!()
                }
                if buf[4] != "=" {
                    panic!()
                }
                if !buf[5].ends_with("\"") || !buf[5].starts_with("\""){
                    panic!()
                }
                */
                /*kirim.send(
                    perintah::konst(buf[1].clone(),tipe::_string(buf[5].clone()))
                ).unwrap()*/
                match buf[2].as_str(){
                    "str"=>{
                        //print!("testing!!!");
                        kirim.send(
                            perintah::konst(buf[1].clone(),Tipe::_string( Some( buf[4].clone() ) ))
                        ).unwrap()
                    }
                    "nomer"=>{
                        kirim.send(
                            perintah::konst(buf[1].clone(),Tipe::nomer(buf[4].clone()))
                        ).unwrap()
                    }
                    "i8"=>{}
                    _=>{panic!()}
                }
            }*/
            "let" => {
                let (mut i, _mut) = if buf[1] == "mut" {
                    (2, true)
                } else {
                    (1, false)
                };
                kirim
                    .send(perintah::_let(
                        cek_jika_nama_var_sesuai(buf[i].clone(), &mut i),
                        _mut,
                        cek_tipe_var(&buf, &mut i),
                    ))
                    .unwrap()
                /*
                let mut i = 1 ;
                let _mut = if buf[i] == "mut" {
                    i += 1 ;
                    true
                } else {
                    false
                };
                let nama = if true {
                    buf[i].clone()
                }else {
                    panic!()
                };
                i += 1;
                let tipe = match buf[i].as_str() {
                    "u8" =>{
                        Tipe::_u8({
                            i += 1 ;
                            if buf[i] == "="{
                                if let Ok(o) = buf[i+1].parse::<u8>() {
                                    Some(o)
                                } else {
                                    panic!()
                                }
                            } else {
                                None
                            }
                        })
                    }
                    "str" =>{
                        Tipe::_string({
                            i += 1 ;
                            if buf[i] == "="{
                                let mut v = buf[i+1].clone();
                                if v.starts_with('"') && v.ends_with('"') {
                                    v.remove(0);
                                    v.pop();
                                } else {
                                    panic!()
                                }
                                crate::parsing::Str::Some(v)
                            } else {
                                crate::parsing::Str::None
                            }
                        })

                    }
                    _=>{panic!()}
                };
                kirim.send(
                    perintah::_let(nama,_mut,tipe)
                ).unwrap()
                */
            }
            /*
            "global"=>{
                //(local $index i32)
                // 0 = String
                // 1 = i8
                // 2 = u8
                // 3 = i16
                // 4 = u16
                // 5 = i32
                // 6 = u32
                // 7 = i64
                // 8 = u64
                if buf[1] == "mut" {
                    //index
                    //mut = true
                    //nama = buf[2]
                    match buf[3].as_str(){
                        "str"=>{
                            //???
                            /*aturan ( prototipe )
                                str tidak dapat diubah
                                maka jika str mut maka akan error
                            */
                            panic!()
                        }
                        "i8"=>{
                            glob_tipe(&buf[2], true , Tipe::_i8(None) , kirim)
                        }
                        "u8"=>{
                            glob_tipe(&buf[2], true , Tipe::_u8(None) , kirim)
                        }
                        "i16"=>{
                            glob_tipe(&buf[2], true , Tipe::_i16(None) , kirim)

                        }
                        "u16"=>{
                            glob_tipe(&buf[2], true , Tipe::_u16(None) , kirim)

                        }
                        "i32"=>{
                            glob_tipe(&buf[2], true , Tipe::_i32(None) , kirim)

                        }
                        "u32"=>{
                            glob_tipe(&buf[2], true , Tipe::_u32(None) , kirim)

                        }
                        "i64"=>{
                            glob_tipe(&buf[2], true , Tipe::_i64(None) , kirim)

                        }
                        "u64"=>{
                            glob_tipe(&buf[2], true , Tipe::_u64(None) , kirim)
                        }
                        _=>{panic!()}
                    }
                } else {
                    //??
                }
            }
            "jika"=>{
                kirim.send(
                    perintah::jika_b(lapisan)
                ).unwrap();
                jika(1,buf,kirim);
            }
            "jika_"=>{
                kirim.send(
                    perintah::jika_tutup
                ).unwrap();
                kirim.send(
                    perintah::blok_tutup
                ).unwrap()
            }
            "lalu"=>{
                if buf.len() > 1 {
                    match buf[1].as_str(){
                        "jika"=>{
                            kirim.send(
                                perintah::lalu_jika
                            ).unwrap();
                            jika(2,buf,kirim);
                        }
                        "jika_"=>{
                            kirim.send(
                                perintah::lalu_jika_
                            ).unwrap();
                        }
                        _=>{}
                    }
                } else {
                    kirim.send(
                        perintah::lalu
                    ).unwrap();
                }
            }
            "lalu_"=>{
                kirim.send(
                    perintah::lalu_
                ).unwrap()
            }
            */
            "swict" => {
                let mut v = buf.clone();
                v.remove(0);
                kirim.send(perintah::swict(0, v)).unwrap();
            }
            "bandingkan" => {
                let mut v = buf.clone();
                v.remove(0);
                kirim.send(perintah::swict(1, v)).unwrap();
            }
            "jika" => {
                let mut v = buf.clone();
                v.remove(0);
                kirim.send(perintah::swict(2, v)).unwrap();
            }
            "?" => {
                let mut v = buf.clone();
                v.remove(0);
                kirim
                    .send(perintah::kasus(if v[0] == "_" {
                        //println!("kasus lalu");
                        None
                    } else {
                        //println!("kasus'{:?}'",v);
                        Some(v)
                    }))
                    .unwrap()
            }
            "swict_" => {
                kirim.send(perintah::swict_(0)).unwrap();
            }
            "bandingkan_" => {
                kirim.send(perintah::swict_(1)).unwrap();
            }
            "jika_" => {
                kirim.send(perintah::swict_(2)).unwrap();
            }
            /* prototipe
            "mencocokan"=>{

            }
            "mencocokan_"=>{

            }
            */
            "benar" => {
                kirim.send(perintah::boolean(true)).unwrap();
                if buf.len() != 1 {
                    buf.remove(0);
                    continue;
                }
            }
            "salah" => {
                kirim.send(perintah::boolean(false)).unwrap();
                if buf.len() != 1 {
                    buf.remove(0);
                    continue;
                }
            }
            /*
            "jika"=>{
                kirim.send(
                    perintah::jika
                ).unwrap();
                /*
                kirim.send(
                    perintah::blok("then".to_string())
                ).unwrap();
                */
                rv(buf,kirim,1);
                //
                if terima.recv().unwrap()[0] == "{"{
                    kirim.send(
                        perintah::jika_
                    ).unwrap();
                } else {
                    panic!()
                }
                *jika_br = true;
                //test
            }
            "lalu"=>{
                if buf.len() == 1 {
                    kirim.send(
                        perintah::lalu
                    ).unwrap()
                } else if buf[1] == "jika" {
                    *jika_br = true;
                    kirim.send(
                        perintah::lalu_jika
                    ).unwrap();
                    //sementara
                    rv(buf,kirim,2);
                    if terima.recv().unwrap()[0] == "{"{
                        kirim.send(
                            perintah::jika_
                        ).unwrap()
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
            }
            */
            /*
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
                var.push(perintah::variabel_null(buf[indx].clone(),nama.clone()));
                //if indx + 2 < buf.len()-1{
                if indx + 3 < buf.len(){
                    let mut v = Vec::new();
                    for i in indx+4..buf.len() {
                        v.push(buf[i].clone())
                    }
                    kirim.send(
                        perintah::tulis(nama,v)
                    ).unwrap()
                }
                *jika_br = true;
                //pengaman aktiv
            }
            */
            "putar" => {
                kirim.send(perintah::putar).unwrap();
            }
            "putar_" => {
                kirim.send(perintah::batas).unwrap();
            }
            "lanjut" => kirim
                .send(if !*dalam_fn {
                    panic!()
                } else {
                    perintah::lanjut
                })
                .unwrap(),
            "putus" => kirim
                .send(if !*dalam_fn {
                    panic!()
                } else {
                    perintah::putus
                })
                .unwrap(),
            /*
            "$putus"|"$lanjut"=>{
                kirim.send(
                    perintah::br(buf[1].clone())
                ).unwrap()
            }
            */
            "cetak" => kirim
                .send(if !*dalam_fn {
                    panic!()
                } else {
                    
                    let mut v = Vec::with_capacity(buf.len() - 1) ;
                    (1..buf.len()).for_each(|f|{
                        v.push(buf[f].clone())
                    });
                    perintah::cetak(v)
                })
                .unwrap(),
            "halaman" => kirim.send(perintah::halaman(buf[1].clone())).unwrap(),
            "." => {
                match perintah_terakhir {
                    perintah::navbar(id) => match buf[1].as_str() {
                        "tombol" => {
                            if buf[2] == "(" {
                                if buf[3].starts_with('"') && buf[3].ends_with('"') {
                                    if buf[4] == "," {
                                        let mut _fungsi = Vec::with_capacity(2);
                                        for i in 5.. {
                                            match buf[i].as_str() {
                                                ":" => {}
                                                "(" => break,
                                                _ => _fungsi.push(buf[i].clone()),
                                            }
                                        }
                                        kirim
                                            .send(perintah::navbar_tombol(
                                                {
                                                    buf[3].pop();
                                                    buf[3].remove(0);
                                                    buf[3].clone()
                                                },
                                                id.to_string(),
                                                _fungsi,
                                            ))
                                            .unwrap()
                                    } else {
                                        panic!()
                                    }
                                } else {
                                    panic!()
                                }
                            } else {
                                panic!()
                            }
                        }
                        _ => {
                            panic!()
                        }
                    },
                    perintah::tombol(o) => {
                        match buf[1].as_str() {
                            "isi" => {
                                if buf[2] == "=" {
                                    if buf[3].starts_with("\"") && buf[3].ends_with("\"") {
                                        buf[3].remove(0);
                                        buf[3].pop();
                                        kirim
                                            .send(perintah::isi(o.clone(), buf[3].clone()))
                                            .unwrap()
                                    } else {
                                        panic!()
                                    }
                                } else {
                                    panic!("{:?}", buf)
                                }
                            }
                            "warna" => {
                                //document.getElementById("button").style.background='#000000';
                                if buf[2] == "=" {
                                    if buf[3].starts_with("\"") && buf[3].ends_with("\"") {
                                        buf[3].remove(0);
                                        buf[3].pop();
                                        //
                                        kirim
                                            .send(perintah::warna(o.clone(), buf[3].clone()))
                                            .unwrap()
                                    } else {
                                        panic!()
                                    }
                                } else {
                                    panic!("{:?}", buf)
                                }
                            }
                            "warnalatarbelakang" => {
                                if buf[2] == "=" {
                                    if buf[3].starts_with("\"") && buf[3].ends_with("\"") {
                                        buf[3].remove(0);
                                        buf[3].pop();
                                        //
                                        kirim
                                            .send(perintah::warnalatarbelakangid(
                                                o.clone(),
                                                buf[3].clone(),
                                            ))
                                            .unwrap()
                                    } else {
                                        panic!()
                                    }
                                } else {
                                    panic!("{:?}", buf)
                                }
                            }
                            "ukurankata" => {
                                if buf[2] == "=" {
                                    if buf[3].starts_with("\"") && buf[3].ends_with("\"") {
                                        buf[3].remove(0);
                                        buf[3].pop();
                                        //
                                        kirim
                                            .send(perintah::ukurankata(o.clone(), buf[3].clone()))
                                            .unwrap()
                                    } else {
                                        panic!()
                                    }
                                } else {
                                    panic!("{:?}", buf)
                                }
                            }
                            "klik" => {
                                if buf[2] == "=" {
                                    let mut nama_modul_fn = Vec::with_capacity(2);
                                    let mut i = 3;
                                    loop {
                                        match buf[i].as_str() {
                                            ":" => {
                                                i += 1;
                                                if buf[i] == ":" {
                                                    i += 1;
                                                    nama_modul_fn.push(buf[i].clone());
                                                    i += 1;
                                                } else {
                                                    panic!()
                                                }
                                            }
                                            "(" => {
                                                i += 1;
                                                if buf[i] == ")" {
                                                    break;
                                                }
                                            }
                                            _ => {
                                                nama_modul_fn.push(buf[i].clone());
                                                i += 1;
                                            }
                                        }
                                    }
                                    kirim
                                        .send(perintah::klik(o.to_string(), nama_modul_fn))
                                        .unwrap();
                                } else {
                                    panic!()
                                }
                            }
                            _ => {
                                panic!()
                            }
                        }
                        println!("{:?}", buf)
                    }
                    _ => {
                        panic!()
                    }
                }
            }
            "app" => {
                if buf[1] == "." {
                    match buf[2].as_str() {
                        "navbar" => {
                            if buf[3] == "(" {
                                if buf[4].starts_with('"') && buf[4].ends_with('"') {
                                    if buf[5] == ")" {
                                        buf[4].pop();
                                        buf[4].remove(0);
                                        *perintah_terakhir = perintah::navbar(buf[4].clone());
                                        kirim.send(perintah::navbar(buf[4].clone())).unwrap()
                                    } else {
                                        panic!()
                                    }
                                } else {
                                    panic!()
                                }
                            } else {
                                panic!()
                            }
                        }
                        "gambarlatarbelakang" => {
                            if buf[3] == "=" {
                                if buf[4].ends_with('"') && buf[4][0..1] == *"\"" {
                                    kirim
                                        .send(perintah::gambarlatarbelakang({
                                            let mut v = buf[4].clone();
                                            v.remove(0);
                                            v.pop();
                                            v
                                        }))
                                        .unwrap()
                                } else {
                                    panic!()
                                }
                            } else {
                                panic!()
                            }
                        }
                        "warnalatarbelakang" => {
                            if buf[3] == "=" {
                                kirim
                                    .send(match buf[4].as_str() {
                                        x if x.starts_with('"') && x.ends_with('"') => {
                                            perintah::warnalatarbelakang({
                                                let mut v = buf[4].clone();
                                                v.remove(0);
                                                v.pop();
                                                v
                                            })
                                        }
                                        _ => {
                                            panic!()
                                        }
                                    })
                                    .unwrap();
                            } else {
                                panic!()
                            }
                        }
                        "judul" => {
                            if buf[3] == "=" {
                                if buf[4].ends_with('"') && buf[4][0..1] == *"\"" {
                                    kirim
                                        .send(perintah::judul({
                                            let mut v = buf[4].clone();
                                            v.remove(0);
                                            v.pop();
                                            v
                                        }))
                                        .unwrap()
                                } else {
                                    panic!()
                                }
                            } else {
                                panic!()
                            }
                        }
                        "tombol" => {
                            if buf[3] == "(" {
                                let mut i = 4;
                                if buf[i].ends_with('"') && buf[i][0..1] == *"\"" {
                                    buf[i].remove(0);
                                    buf[i].pop();
                                    *perintah_terakhir = perintah::tombol(buf[i].clone());
                                    kirim.send(perintah::tombol(buf[i].clone())).unwrap();
                                    i += 1
                                } else {
                                    panic!()
                                }
                                //???
                            } else {
                                panic!()
                            }
                        }
                        "id" => {
                            if buf[3] == "(" {
                                if buf[4].ends_with("\"") && buf[4].starts_with("\"") {
                                    buf[4].remove(0);
                                    buf[4].pop();
                                    if buf[5] == ")" {
                                        if buf[6] == "." {
                                            match buf[7].as_str() {
                                                "isi" => {
                                                    if buf[8] == "=" {
                                                        if buf[9].ends_with("\"")
                                                            && buf[9].starts_with("\"")
                                                        {
                                                            buf[9].remove(0);
                                                            buf[9].pop();
                                                            kirim
                                                                .send(perintah::isi(
                                                                    buf[4].clone(),
                                                                    buf[9].clone(),
                                                                ))
                                                                .unwrap()
                                                        } else {
                                                            panic!()
                                                        }
                                                    } else {
                                                        panic!()
                                                    }
                                                }
                                                _ => {
                                                    panic!()
                                                }
                                            }
                                        } else {
                                            panic!()
                                        }
                                    } else {
                                        panic!()
                                    }
                                } else {
                                    panic!()
                                }
                            } else {
                                panic!()
                            }
                        }
                        _ => {
                            panic!()
                        }
                    }
                } else {
                    panic!()
                }
                /*
                loop{
                    match buf[i].as_str(){
                        "."=>{}
                        "gambarlatarbelakang"=>{
                            i += 1 ;
                            if buf[i] == "=" {
                                i += 1 ;
                                if buf[i].ends_with('"') && buf[i][0..1] == *"\"" {
                                    //testing
                                    kirim.send(
                                        perintah::gambarlatarbelakang(
                                            {
                                                let mut v = buf[i].clone();
                                                v.remove(0);
                                                v.pop();
                                                v
                                            }
                                        )
                                    ).unwrap()
                                } else {
                                    panic!()
                                }
                            } else {
                                panic!()
                            }
                            break
                        }
                        "warnalatarbelakang" =>{
                            // document.body.style.backgroundColor = "red";
                            i += 1 ;
                            if buf[i] == "=" {
                                i += 1 ;
                                if buf[i].ends_with('"') && buf[i][0..1] == *"\"" {
                                    //testing
                                    kirim.send(
                                        perintah::warnalatarbelakang(
                                            {
                                                let mut v = buf[i].clone();
                                                v.remove(0);
                                                v.pop();
                                                v
                                            }
                                        )
                                    ).unwrap()
                                } else {
                                    panic!()
                                }
                            } else {
                                panic!()
                            }
                            break
                        }
                        "judul"=>{
                            i += 1 ;
                            if buf[i] == "=" {
                                i += 1 ;
                                if buf[i].ends_with('"') && buf[i][0..1] == *"\"" {
                                    //testing
                                    kirim.send(
                                        perintah::judul(
                                            {
                                                let mut v = buf[i].clone();
                                                v.remove(0);
                                                v.pop();
                                                v
                                            }
                                        )
                                    ).unwrap()
                                } else {
                                    panic!()
                                }
                            } else {
                                panic!()
                            }
                            break
                        }
                        "tombol"=>{
                            i += 1 ;
                            if buf[i] == "("{
                                kirim.send(
                                    perintah::tombol(
                                        if buf[i].ends_with('"') && buf[i][0..1] == *"\"" {
                                            buf[i].pop();
                                            buf[i].remove(0);
                                            i += 1 ;
                                            buf[i].clone()
                                        } else {
                                            panic!()
                                        }
                                        ,
                                        if buf[i] == ","{
                                            i += 1 ;
                                            if let Ok(o) = buf[i].parse::<u64>() {
                                                o
                                            } else {
                                                panic!()
                                            }
                                        } else {
                                            panic!()
                                        }
                                    )
                                ).unwrap();
                                break
                            }
                        }
                        _=>{
                            panic!()
                        }
                    }
                    i += 1 ;
                    if i >= buf.len() { break }
                }
                */
            }
            "fn" => {
                let (nama, publik) = if buf[0].as_str() == "pub" {
                    (buf[2].clone(), true)
                } else {
                    (buf[1].clone(), false)
                };
                *dalam_fn = true;
                kirim.send(perintah::cpu(nama, publik)).unwrap()
            }
            "{" => kirim.send(perintah::blok("blok".to_string())).unwrap(),
            "}" => {
                kirim.send(perintah::blok_).unwrap()
                /*
                //sementara jika
                if *jika_br {
                    //keluar dari if
                    /*
                    kirim.send(
                        perintah::br("$if".to_string())
                    ).unwrap();
                    kirim.send(perintah::blok_tutup).unwrap();
                    */
                    //
                    //kirim.send(perintah::blok_tutup).unwrap();
                    //
                    kirim.send(perintah::jika_tutup).unwrap();
                    //
                    *jika_br = false;
                } else {
                    kirim.send(perintah::blok_tutup).unwrap();
                }
                */
                /*
                kirim.send(
                    perintah::blok_tutup
                ).unwrap();
                lapisan -= 1 ;
                */
            }
            /*
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
            */
            /*
            _ if buf[0] == "!" && buf[1] == "("=> {
                //belum siap
                //fungsi arg
                if !*dalam_fn {
                    println!("token tidak sesuai");
                    std::process::exit(1);
                }
            }
            */
            "modul" => kirim
                .send(if buf.len() == 2 {
                    perintah::modul_masuk(buf[1].clone())
                } else {
                    panic!()
                })
                .unwrap(),
            "modul_" => kirim.send(perintah::modul_keluar).unwrap(),
            /*
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
            */
            _ if buf[1] == "=" => {
                kirim
                    .send(perintah::tulis(
                        if !buf[0].contains("\"") {
                            buf[0].clone()
                        } else {
                            panic!()
                        },
                        {
                            let mut v = Vec::new();
                            for i in 2..buf.len() {
                                v.push(buf[i].clone())
                            }
                            if v.is_empty() {
                                panic!()
                            }
                            v
                        },
                    ))
                    .unwrap();
            }
            _ => {
                //memanggil fungsi
                if buf[1] == "(" {
                    kirim
                        .send(perintah::panggil_fn(
                            ["main".to_string(), buf[0].clone()].to_vec(),
                            Vec::new(),
                        ))
                        .unwrap();
                } else if buf.len() > 3 {
                    if buf[1] == ":" && buf[2] == ":" {
                        kirim
                            .send(perintah::panggil_fn(
                                {
                                    let mut v = Vec::new();
                                    v.push(buf[0].clone());
                                    v.push(buf[3].clone());
                                    v
                                },
                                {
                                    //uji coba
                                    //argumen
                                    let v = Vec::new();

                                    v
                                },
                            ))
                            .unwrap();
                    }
                } else {
                    println!("salah token {:?}", buf);
                    std::process::exit(1);
                }
            }
        }
        break;
    }
}
//belum selesai
fn duplikat(
    terima: &std::sync::mpsc::Receiver<Vec<String>>,
    kirim: &std::sync::mpsc::Sender<perintah>,
    jumlah: u64,
) {
    let mut buf: Vec<Vec<String>> = Vec::with_capacity(2);
    loop {
        let v = terima.recv().unwrap();
        if v[0] == "duplikat_batas" {
            break;
        }
        //token(&,&mut jika_br, &terima,&kirim, &mut dalam_fn);
        buf.push(v);
    }
    for _ in 0..jumlah {
        for x in &buf {
            //token()
        }
    }
}
//parse
pub fn parse(
    terima: std::sync::mpsc::Receiver<Vec<String>>,
    kirim: std::sync::mpsc::Sender<perintah>,
) {
    //let mut buf = terima.recv().expect("");
    let mut dalam_fn = false;
    let mut jika_br = false;
    let mut perintah_terakhir = perintah::selesai;
    terima.iter().for_each(|buff| {
        if !buff.is_empty() {
            if buff[0] != "duplikat" {
                token(
                    buff,
                    &mut jika_br,
                    &terima,
                    &kirim,
                    &mut dalam_fn,
                    &mut perintah_terakhir,
                );
            } else {
                duplikat(&terima, &kirim, buff[1].parse::<u64>().unwrap());
            }
        } else {
            kirim.send(perintah::selesai).expect("parse gagal selesai");
            return;
        }
    });
}
