use crate::parsing::parse_3::{Pohon,Nilai,Variabel,Tipe,Arit};
use std::io::Write;
/*
struct WEB{
    path:String,
    offset:usize,
    import_lib:[bool;3]
}
impl WEB{
    fn was(&self){
        let mut was = std::io::BufWriter::new( match std::fs::File::create(format!("{}\\target\\www\\main.wasm",self.path)){
            Ok(o)=>{o}
            Err(_)=>{
                std::fs::create_dir_all(format!("{}\\target\\www",self.path)).expect("tidak dapat membuat target direktori (target)");
                std::fs::File::create(format!("{}\\target\\www\\main.wasm",self.path)).unwrap()
            }
        });
        was.write_all(
            &wat::parse_file(format!("{}\\parsing\\www\\main.wat",self.path)).unwrap()
        ).unwrap();
    }
    fn import_log(&mut self) -> (&str,&str){
        self.import_lib[0] = true;
        (
            "(import \"import\"\"log\"(func $log (param i32 i32)))\n",
            "\"log\":(offset,length)=>{\
                console.log(new TextDecoder('utf8').decode(new Uint8Array(import_object.import.mem.buffer,offset,length)))},"
        )
    }
    fn data(&mut self,p:&str,length:usize,d:String) -> String {
        let t = if self.import_lib[2] {
            format!("{}",d)
        } else {
            self.import_lib[2] = true;
            format!("{}",d)
        };
        self.offset += length;
        t
    }
}

pub fn app(
    pohon:&std::vec::Vec<crate::parsing::parse_3::Pohon>,
    path:&String,
){
    let mut _web = WEB{
        path:path.to_string(),
        offset:0usize,
        import_lib:[false,false,false],
    };
    let mut wat = std::io::BufWriter::new( match std::fs::File::create(format!("{}\\parsing\\www\\main.wat",path)){
        Ok(o)=>{o}
        Err(_)=>{
            std::fs::create_dir_all(format!("{}\\parsing\\www",path)).expect("tidak dapat membuat target direktori (target)");
            std::fs::File::create(format!("{}\\parsing\\www\\main.wat",path)).unwrap()
        }
    });
    
    let mut imp = std::io::BufWriter::new( match std::fs::File::create(format!("{}\\parsing\\www\\import",path)){
        Ok(o)=>{o}
        Err(_)=>{
            std::fs::create_dir_all(format!("{}\\parsing\\www",path)).expect("tidak dapat membuat target direktori (target)");
            std::fs::File::create(format!("{}\\parsing\\www\\import",path)).unwrap()
        }
    });
    let mut _main = std::io::BufWriter::new( match std::fs::File::create(format!("{}\\parsing\\www\\main",path)){
        Ok(o)=>{o}
        Err(_)=>{
            std::fs::create_dir_all(format!("{}\\parsing\\www",path)).expect("tidak dapat membuat target direktori (target)");
            std::fs::File::create(format!("{}\\parsing\\www\\main",path)).unwrap()
        }
    });
    
    let mut _data = std::io::BufWriter::new( match std::fs::File::create(format!("{}\\parsing\\www\\data",path)){
        Ok(o)=>{o}
        Err(_)=>{
            std::fs::create_dir_all(format!("{}\\parsing\\www",path)).expect("tidak dapat membuat target direktori (target)");
            std::fs::File::create(format!("{}\\parsing\\www\\data",path)).unwrap()
        }
    });
    //let mut v_num = 0;
    wat.write(b"(module\n").unwrap();
    for i in pohon{
        match i {
            Pohon::cetak(o)=>{
                /*
                if !_web.import_lib[1]{
                    _data.write(b"(import\"import\"\"mem\"(memory 1))\n").unwrap();
                    imp.write(b"\"mem\":new WebAssembly.Memory({initial:1}),").unwrap();
                    _web.import_lib[1] = true;
                }*/
                if !_web.import_lib[0]{
                    let t = _web.import_log();
                    wat.write(t.0.as_bytes()).unwrap();
                    imp.write(t.1.as_bytes()).unwrap();
                    _web.import_lib[0] = true;
                }
                match o {
                    Nilai::lansung(o)=>{
                        let t = format!("i32.const {}\ni32.const {}\ncall $log\n",_web.offset,o.len());
                        _main.write(
                            t.as_bytes()
                        ).unwrap();
                        _data.write(_web.data("32",o.len(),format!("{}" , o.as_str() ) ).as_bytes()).unwrap();
                    }
                    Nilai::minta(o)=>{}
                    Nilai::penujuk(o)=>{}
                }
                
            }
            Pohon::var(o)=>{
                let v = format!("(local $v_{0} i32)\n",o.id);
                _main.write(v.as_bytes()).unwrap();
            }
        }
    }
    if _web.import_lib[2] {
        _data.write(b"\")\n").unwrap();
    }
    drop((_main,_data,imp));
    {wat}.write({
        let mut t = std::fs::read_to_string(format!("{}\\parsing\\www\\data",path)).unwrap();
        t.push_str("(func(export\"main\")\n");
        t.push_str(std::fs::read_to_string(format!("{}\\parsing\\www\\main",path)).unwrap().as_str()  );
        t.push_str(")\n)");
        t
    }.as_bytes()).unwrap();
    std::fs::write(format!("{}\\target\\www\\main.wasm",path), &wat::parse_file(format!("{}\\parsing\\www\\main.wat",path)).unwrap()).unwrap();
    //
    let mut html = std::io::BufWriter::new( match std::fs::File::create(format!("{}\\target\\www\\index.html",path)){
        Ok(o)=>{o}
        Err(_)=>{
            std::fs::create_dir_all(format!("{}\\target\\www",path)).expect("tidak dapat membuat target direktori (target)");
            std::fs::File::create(format!("{}\\target\\www\\index.html",path)).unwrap()
        }
    });
    html.write(b"<head>").unwrap();
    let import = std::fs::read(format!("{}\\parsing\\www\\import",path)).unwrap();
    if !import.is_empty(){
        html.write(b"<script>var import_object={\"import\":{").unwrap();
        html.write(&import).unwrap();
        html.write(b"}};fetch('main.wasm').then(response=>response.arrayBuffer()).then(bytes=>WebAssembly.instantiate(bytes,import_object)).then(results=>results.instance.exports.main()).catch(console.error);</script></head>").unwrap();
    } else {
        html.write(b"<script>fetch('main.wasm').then(response=>response.arrayBuffer()).then(bytes=>WebAssembly.instantiate(bytes)).then(results=>results.instance.exports.main()).catch(console.error);</script></head>").unwrap();
    }
}
*/
struct web {
    //path:String,
    offset:usize,
    import_lib:[bool;3]
}

impl web {
    fn print(&mut self,_in:&String) -> String{
        let t = format!("i32.const {}\ni32.const {}\ncall $log\n",self.offset,_in.len());
        self.offset =_in.len();
        t
    }
}
pub fn  app_2(
    pohon:&std::vec::Vec<crate::parsing::parse_3::Pohon>,
    path:&String,
){
    {
        let mut web = web {
            //path:path.to_string(),
            offset:0usize,
            import_lib:[false,false,false],
        };
        let mut _main = std::io::BufWriter::new( match std::fs::File::create(format!("{}\\parsing\\www\\main",path)){
            Ok(o)=>{o}
            Err(_)=>{
                std::fs::create_dir_all(format!("{}\\parsing\\www",path)).expect("tidak dapat membuat target direktori (target)");
                std::fs::File::create(format!("{}\\parsing\\www\\main",path)).unwrap()
            }
        });
        let mut _data = std::io::BufWriter::new( match std::fs::File::create(format!("{}\\parsing\\www\\data",path)){
            Ok(o)=>{o}
            Err(_)=>{
                std::fs::create_dir_all(format!("{}\\parsing\\www",path)).expect("tidak dapat membuat target direktori (target)");
                std::fs::File::create(format!("{}\\parsing\\www\\data",path)).unwrap()
            }
        });
        let mut imp = std::io::BufWriter::new( match std::fs::File::create(format!("{}\\parsing\\www\\impor",path)){
            Ok(o)=>{o}
            Err(_)=>{
                std::fs::create_dir_all(format!("{}\\parsing\\www",path)).expect("tidak dapat membuat target direktori (target)");
                std::fs::File::create(format!("{}\\parsing\\www\\impor",path)).unwrap()
            }
        });
        let mut imp_wat = std::io::BufWriter::new( match std::fs::File::create(format!("{}\\parsing\\www\\impor_wat",path)){
            Ok(o)=>{o}
            Err(_)=>{
                std::fs::create_dir_all(format!("{}\\parsing\\www",path)).expect("tidak dapat membuat target direktori (target)");
                std::fs::File::create(format!("{}\\parsing\\www\\impor_wat",path)).unwrap()
            }
        });
        let mut local = std::io::BufWriter::new( match std::fs::File::create(format!("{}\\parsing\\www\\local",path)){
            Ok(o)=>{o}
            Err(_)=>{
                std::fs::create_dir_all(format!("{}\\parsing\\www",path)).expect("tidak dapat membuat target direktori (target)");
                std::fs::File::create(format!("{}\\parsing\\www\\local",path)).unwrap()
            }
        });
        //wat.write(b"(module\n").unwrap();
        let mut imp_koma = false;
        for i in pohon{
            match i {
                Pohon::cetak(o)=>{
                    match o {
                        Nilai::lansung(o)=>{
                            if o.is_empty(){continue}
                            if !web.import_lib[0]{
                                if imp_koma {
                                    imp.write(b",").unwrap();
                                } else {
                                    imp_koma = true;
                                }
                                imp.write(b"\"mem\":new WebAssembly.Memory({initial:1})").unwrap();
                                web.import_lib[0] = true
                            }
                            if !web.import_lib[1]{
                                if imp_koma {
                                    imp.write(b",").unwrap();
                                } else {
                                    imp_koma = true;
                                }
                                imp_wat.write(b"(import\"import\"\"log\"(func $log(param i32 i32)))\n").unwrap();
                                imp.write(b"\"log\":(offset,length)=>{console.log(new TextDecoder('utf8').decode(new Uint8Array(import_object.import.mem.buffer,offset,length)))").unwrap();
                                web.import_lib[1] = true
                            }
                            _main.write(web.print(o).as_bytes()).unwrap();
                            _data.write(o.as_bytes()).unwrap();
                        }
                        Nilai::minta(o)=>{}
                        Nilai::penujuk(o)=>{}
                        Nilai::lansung_int(o)=>{}
                        Nilai::lansung_float(o)=>{}
                    }
                }
                Pohon::tulis(a,b)=>{
                    match b {
                        Tipe::_u8(o)=>{
                            if let Some(o) = o {
                                let v = &format!("(local.set ${} (i32.const {}))\n",a,o);
                                _main.write(v.as_bytes()).unwrap();
                            }
                        }
                        Tipe::_String(_)=>{

                        }
                    }
                    
                }
                Pohon::var(a,b)=>{
                    /*
                    match &o.nilai {
                        Tipe::_u8(x)=>{
                            let mut v = format!("(local ${} i32)\n",o.id);
                            local.write(v.as_bytes()).unwrap();
                            if let Some(x) = x {
                                v.clear();
                                v.push_str(
                                    &format!("(local.set ${} (i32.const {}))\n",o.id,x)
                                );
                                _main.write(v.as_bytes()).unwrap();
                            }
                            
                        }
                        Tipe::_String(o)=>{
                            
                        }
                    }*/
                    match b {
                        Tipe::_u8(_)=>{
                            let v = format!("(local ${} i32)\n",a);
                            local.write(v.as_bytes()).unwrap();
                        }
                        Tipe::_String(_)=>{

                        }
                    }
                }
                Pohon::arit(o)=>{
                    match o{
                        Arit::bagi(a,b,c)=>{

                        }
                        Arit::kali(a,b,c)=>{

                        }
                        Arit::tambah(a,b,c)=>{

                        }
                        Arit::kurang(a,b,c)=>{

                        }
                        Arit::modus(a,b,c)=>{

                        }
                        //trigonometri
                        Arit::sin(a,b)=>{

                        }
                        Arit::cos(a,b)=>{

                        }
                        Arit::Tan(a,b)=>{
                            
                        }
                    }
                }
            }
        }
    }
    match std::fs::read_to_string(format!("{}\\parsing\\www\\impor",path)) {
        Ok(import)=>{
            std::fs::write(format!("{}\\target\\debug\\www\\index.html",path), 
                {
                    if !import.is_empty(){
                        let t = format!("<head><script>let import_object={{\"import\":{{{0}}}}}}};fetch('main.wasm').then(r=>r.arrayBuffer()).then(b=>WebAssembly.instantiate(b,import_object)).then(r=>r.instance.exports.main()).catch(console.error);</script></head>",import.as_str());
                        t
                    } else {
                        "<head><script>fetch('main.wasm').then(r=>r.arrayBuffer()).then(b=>WebAssembly.instantiate(b)).then(r=>r.instance.exports.main()).catch(console.error);</script></head>".to_string()
                    }
                }.as_bytes()
                
            ).unwrap();

        }
        Err(_)=>{panic!()}
    }
    {
        let mut wat = std::io::BufWriter::new( match std::fs::File::create(format!("{}\\parsing\\www\\main.wat",path)){
            Ok(o)=>{o}
            Err(_)=>{
                std::fs::create_dir_all(format!("{}\\parsing\\www",path)).expect("tidak dapat membuat target direktori (target)");
                std::fs::File::create(format!("{}\\parsing\\www\\main.wat",path)).unwrap()
            }
        });
        wat.write(b"(module\n").unwrap();
        //
        wat.write(&std::fs::read(format!("{}\\parsing\\www\\impor_wat",path)).unwrap()).unwrap();
        //string str data
        match std::fs::read_to_string(format!("{}\\parsing\\www\\data",path)) {
            Ok(o)=>{
                if !o.is_empty(){
                    wat.write({format!("(import\"import\"\"mem\"(memory {}))\n(data(i32.const 0)\"{}\")\n",1,o)}.as_bytes()).unwrap();
                }
            }
            Err(_)=>{}
        }
        //main
        wat.write(b"(func(export\"main\")\n").unwrap();
        //variabel
        match std::fs::read(format!("{}\\parsing\\www\\local",path)) {
            Ok(local)=>{
                if !local.is_empty(){
                    wat.write(&local).unwrap();
                }
            }
            Err(_)=>{panic!()}
        }
        wat.write(&std::fs::read(format!("{}\\parsing\\www\\main",path)).unwrap()).unwrap();
        wat.write(b")\n)").unwrap();
    }
    std::fs::write(format!("{}\\target\\debug\\www\\main.wasm",path), &wat::parse_file(format!("{}\\parsing\\www\\main.wat",path)).unwrap()).unwrap();

}