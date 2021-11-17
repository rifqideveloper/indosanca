use crate::parsing::parse_3::{Nilai, Pohon, Tipe, Variabel};
use std::io::BufRead;
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
    offset: usize,
    import_lib: [bool; 14],
    Klik: Vec<[String; 2]>,
}

impl web {
    fn body(&mut self, _in: &String) -> String {
        let v = format!(
            "i32.const {}\ni32.const {}\ncall $body\n",
            self.offset,
            _in.len()
        );
        self.offset += _in.len();
        self.import_lib[0] = true;
        self.import_lib[3] = true;
        v
    }
    fn print(&mut self, _in: &String) -> String {
        let t = format!(
            "i32.const {}\ni32.const {}\ncall $log\n",
            self.offset,
            _in.len()
        );
        self.offset += _in.len();
        self.import_lib[0] = true;
        t
    }
    fn print_(&mut self, index: usize, o: String) -> String {
        self.import_lib[index] = true;
        o
    }
    fn print_var(&mut self, id: u64, tipe: usize) -> String {
        match tipe {
            1 => {
                self.import_lib[1] = true;
                format!("local.get ${}\ncall $log_\n", id)
            }
            2 => {
                self.import_lib[2] = true;
                format!("local.get ${}\ncall $log_\n", id)
            }
            _ => {
                format!("local.get ${}\ncall $log_\n", id)
            }
        }
    }
    fn warnalatarbelakang(&mut self, _in: &String) -> String {
        let v = format!(
            "i32.const {}\ni32.const {}\ncall $warnalatarbelakang\n",
            self.offset,
            _in.len()
        );
        self.offset += _in.len();
        self.import_lib[0] = true;
        self.import_lib[4] = true;
        v
    }
    fn gambarlatarbelakang(&mut self, _in: &String) -> String {
        let v = format!(
            "i32.const {}\ni32.const {}\ncall $gambarlatarbelakang\n",
            self.offset,
            _in.len()
        );
        self.offset += _in.len();
        self.import_lib[0] = true;
        self.import_lib[5] = true;
        v
    }
    fn judul(&mut self, _in: &String) -> String {
        let v = format!(
            "i32.const {}\ni32.const {}\ncall $judul\n",
            self.offset,
            _in.len()
        );
        self.offset += _in.len();
        self.import_lib[0] = true;
        self.import_lib[6] = true;
        v
    }
    fn tombol(&mut self, _in: &String) -> String {
        let v = format!(
            "i32.const {}\ni32.const {}\ncall $tombol\n",
            self.offset,
            _in.len()
        );
        self.offset += _in.len();
        self.import_lib[0] = true;
        self.import_lib[7] = true;
        v
    }
    fn isi(
        &mut self,
        x: &String,
        y: &String,
        o: &mut std::io::BufWriter<std::fs::File>,
        l: &mut std::io::BufWriter<std::fs::File>,
    ) {
        let mut v = String::with_capacity(20);
        v.push_str({ format!("i32.const {}\ni32.const {}\n", self.offset, x.len()) }.as_str());
        self.offset += x.len();
        l.write(x.as_bytes()).unwrap();
        v.push_str({ format!("i32.const {}\ni32.const {}\n", self.offset, y.len()) }.as_str());
        self.offset += y.len();
        l.write(y.as_bytes()).unwrap();
        v.push_str("call $isi\n");
        o.write(v.as_bytes()).unwrap();
        self.import_lib[0] = true;
        self.import_lib[8] = true;
    }
    fn warna(
        &mut self,
        x: &String,
        y: &String,
        o: &mut std::io::BufWriter<std::fs::File>,
        l: &mut std::io::BufWriter<std::fs::File>,
    ) {
        let mut v = String::with_capacity(20);
        v.push_str({ format!("i32.const {}\ni32.const {}\n", self.offset, x.len()) }.as_str());
        self.offset += x.len();
        l.write(x.as_bytes()).unwrap();
        v.push_str({ format!("i32.const {}\ni32.const {}\n", self.offset, y.len()) }.as_str());
        self.offset += y.len();
        l.write(y.as_bytes()).unwrap();
        v.push_str("call $warna\n");
        o.write(v.as_bytes()).unwrap();
        self.import_lib[0] = true;
        self.import_lib[9] = true;
    }
    fn warnalatarbelakangid(
        &mut self,
        x: &String,
        y: &String,
        o: &mut std::io::BufWriter<std::fs::File>,
        l: &mut std::io::BufWriter<std::fs::File>,
    ) {
        let mut v = String::with_capacity(20);
        v.push_str({ format!("i32.const {}\ni32.const {}\n", self.offset, x.len()) }.as_str());
        self.offset += x.len();
        l.write(x.as_bytes()).unwrap();
        v.push_str({ format!("i32.const {}\ni32.const {}\n", self.offset, y.len()) }.as_str());
        self.offset += y.len();
        l.write(y.as_bytes()).unwrap();
        v.push_str("call $warnalatarbelakangid\n");
        o.write(v.as_bytes()).unwrap();
        self.import_lib[0] = true;
        self.import_lib[10] = true;
    }
    fn ukurankata(
        &mut self,
        x: &String,
        y: &String,
        o: &mut std::io::BufWriter<std::fs::File>,
        l: &mut std::io::BufWriter<std::fs::File>,
    ) {
        let mut v = String::with_capacity(20);
        v.push_str({ format!("i32.const {}\ni32.const {}\n", self.offset, x.len()) }.as_str());
        self.offset += x.len();
        l.write(x.as_bytes()).unwrap();
        v.push_str({ format!("i32.const {}\ni32.const {}\n", self.offset, y.len()) }.as_str());
        self.offset += y.len();
        l.write(y.as_bytes()).unwrap();
        v.push_str("call $ukurankata\n");
        o.write(v.as_bytes()).unwrap();
        self.import_lib[0] = true;
        self.import_lib[11] = true;
    }
    fn id_kelas(
        &mut self,
        id: &String,
        kelas: &String,
        o: &mut std::io::BufWriter<std::fs::File>,
        l: &mut std::io::BufWriter<std::fs::File>,
    ) {
        let mut v = String::with_capacity(30);
        v.push_str({ format!("i32.const {}\ni32.const {}\n", self.offset, id.len()) }.as_str());
        self.offset += id.len();
        l.write(id.as_bytes()).unwrap();
        v.push_str({ format!("i32.const {}\ni32.const {}\n", self.offset, kelas.len()) }.as_str());
        self.offset += kelas.len();
        l.write(kelas.as_bytes()).unwrap();
        v.push_str("call $kelas\n");
        o.write(v.as_bytes()).unwrap();
        self.import_lib[0] = true;
        self.import_lib[12] = true;
    }
    fn klik(&mut self, kelas: &String, fn_: &String) {
        self.Klik.push([kelas.to_string(), fn_.to_string()]);
    }
    fn str_sama_str(&mut self,x:u64,y:u64) {
        
        self.import_lib[0] = true;
        self.import_lib[13] = true;
    }
}
use std::collections::HashMap;

//
pub fn app_3(pohon: &std::vec::Vec<crate::parsing::parse_3::Pohon>, path: &String) {
    let mut web = web {
        //path:path.to_string(),
        offset: 0usize,
        import_lib: [
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false
        ],
        Klik: Vec::new(), /*prototipe
                           css:[]
                          */
    };
    let mut konst_str: HashMap<u64, [u64; 2]> = HashMap::new();
    let mut _main = std::io::BufWriter::new(
        match std::fs::File::create(format!("{}\\parsing\\www\\main", path)) {
            Ok(o) => o,
            Err(_) => {
                std::fs::create_dir_all(format!("{}\\parsing\\www", path))
                    .expect("tidak dapat membuat target direktori (target)");
                std::fs::File::create(format!("{}\\parsing\\www\\main", path)).unwrap()
            }
        },
    );
    let mut _data = std::io::BufWriter::new(
        match std::fs::File::create(format!("{}\\parsing\\www\\data", path)) {
            Ok(o) => o,
            Err(_) => {
                std::fs::create_dir_all(format!("{}\\parsing\\www", path))
                    .expect("tidak dapat membuat target direktori (target)");
                std::fs::File::create(format!("{}\\parsing\\www\\data", path)).unwrap()
            }
        },
    );
    let mut local = std::io::BufWriter::new(
        match std::fs::File::create(format!("{}\\parsing\\www\\local", path)) {
            Ok(o) => o,
            Err(_) => {
                std::fs::create_dir_all(format!("{}\\parsing\\www", path))
                    .expect("tidak dapat membuat target direktori (target)");
                std::fs::File::create(format!("{}\\parsing\\www\\local", path)).unwrap()
            }
        },
    );
    let mut main_selesai = false;
    println!("{:?}", pohon);
    pohon.iter().for_each(|i|{
        match i {
            Pohon::main(o) => {
                _main.write(o.as_bytes()).unwrap();
            }
            Pohon::fungsi(o) => {
                if !main_selesai {
                    _main = std::io::BufWriter::new(
                        match std::fs::File::create(format!("{}\\parsing\\www\\lib", path)) {
                            Ok(o) => o,
                            Err(_) => {
                                std::fs::create_dir_all(format!("{}\\parsing\\www", path))
                                    .expect("tidak dapat membuat target direktori (target)");
                                std::fs::File::create(format!("{}\\parsing\\www\\lib", path))
                                    .unwrap()
                            }
                        },
                    );
                    main_selesai = true
                }
                _main.write({ format!(":fn:{}\n", o) }.as_bytes()).unwrap();
            }
            Pohon::panggil_fn(o) => {
                //memanggil fungsi
                _main
                    .write({ format!("call ${}\n", o) }.as_bytes())
                    .unwrap();
            }
            Pohon::konst_str(id, l) => {
                konst_str.insert(*id, [web.offset as u64, l.len() as u64]);
                web.offset += l.len();
                _data.write(l.as_bytes()).unwrap();
            }
            Pohon::_let(id,tipe)=>{
                _main.write(
                    {
                        format!("i32.store i32.const 1 i32.const 0\n")
                    }.as_bytes()
                ).unwrap();
            }
            Pohon::r#if => {
                //_main.write(b"(else\n").unwrap();
                _main.write(b"(block\n").unwrap();
            }

            Pohon::lalu_jika => {
                _main.write(b"(block $then\n").unwrap();
            }
            Pohon::putar => {
                _main.write({ format!("(loop\n") }.as_bytes()).unwrap();
            }
            Pohon::lanjut => {
                _main.write(b"br 0\n").unwrap();
            }
            Pohon::putus => {
                // ???
            }
            Pohon::batas | Pohon::if_ | Pohon::blok_ | Pohon::jika_tutup => {
                _main.write(b")\n").unwrap();
            }
            Pohon::const_32(a) => {
                _main
                    .write({ format!("i32.const {}\n", a) }.as_bytes())
                    .unwrap();
            }
            Pohon::i32_eqz => {
                _main.write(b"\ni32.eqz\n").unwrap();
            }
            Pohon::i32_eq => {
                _main.write(b"\ni32.eq\n").unwrap();
            }
            Pohon::halaman(o) => {
                match o {
                    Nilai::lansung_float(_) => {}
                    Nilai::lansung_int(_) => {}
                    Nilai::lansung_str(o) => {
                        _main.write(web.body(&o).as_bytes()).unwrap();
                        //menyimpan data
                        _data.write(o.as_bytes()).unwrap();
                    }
                    Nilai::penujuk_(o) => {
                        if let Some(o) = konst_str.get(&o) {
                            _main
                                .write(
                                    {
                                        format!(
                                            "i32.const {}\ni32.const {}\ncall $body\n",
                                            o[0], o[1]
                                        )
                                    }
                                    .as_bytes(),
                                )
                                .unwrap();
                            web.import_lib[3] = true;
                        } else {
                            panic!()
                        }
                    }
                    _ => {}
                }
                //println!("testing halaman ({})",o)
            }
            Pohon::warnalatarbelakang(o) => {
                _main.write(web.warnalatarbelakang(o).as_bytes()).unwrap();
                _data.write(o.as_bytes()).unwrap();
            }
            Pohon::gambarlatarbelakang(o) => {
                _main.write(web.gambarlatarbelakang(o).as_bytes()).unwrap();
                _data.write(o.as_bytes()).unwrap();
            }
            Pohon::judul(o) => {
                _main.write(web.judul(o).as_bytes()).unwrap();
                _data.write(o.as_bytes()).unwrap();
            }
            Pohon::tombol(o) => {
                _main.write(web.tombol(o).as_bytes()).unwrap();
                _data.write(o.as_bytes()).unwrap();
            }
            Pohon::klik(id, _fn) => {
                let mut fn_ = String::with_capacity(10);
                for i in 0.. {
                    fn_.push_str(&_fn[i]);
                    if i + 1 >= _fn.len() {
                        break;
                    }
                    fn_.push('_');
                }
                //jika klik fn pernah digunakan maka error
                //jika belum tambahkan
                let mut t = false;
                for i in &web.Klik {
                    if i[1] == fn_ {
                        t = false;
                        break;
                    }
                }
                if !t {
                    web.klik(&fn_, &fn_);
                }
                web.id_kelas(&id, &fn_, &mut _main, &mut _data);
            }
            Pohon::isi(o, l) => web.isi(o, l, &mut _main, &mut _data),
            Pohon::warna(o, l) => web.warna(o, l, &mut _main, &mut _data),
            Pohon::warnalatarbelakangid(o, l) => {
                web.warnalatarbelakangid(o, l, &mut _main, &mut _data)
            }
            Pohon::ukurankata(o, l) => web.ukurankata(o, l, &mut _main, &mut _data),
            Pohon::add => {
                _main.write({ "i32.add\n" }.as_bytes()).unwrap();
            }
            Pohon::sub => {
                _main.write({ "i32.sub\n" }.as_bytes()).unwrap();
            }
            Pohon::div_s => {
                _main.write({ "i32.div_s\n" }.as_bytes()).unwrap();
            }
            Pohon::div_u => {
                _main.write({ "i32.div_u\n" }.as_bytes()).unwrap();
            }
            Pohon::mul => {
                _main.write({ "i32.shl\n" }.as_bytes()).unwrap();
            }
            Pohon::bandingkan_str_str(x,y)=>{

            }
            Pohon::local_set(a) => {
                _main
                    .write({ format!("local.set ${}\n", a) }.as_bytes())
                    .unwrap();
            }
            Pohon::tulis(a, b) => match b {
                Tipe::_u8(o) => {
                    if let Some(o) = o {
                        _main
                            .write({ format!("(local.set ${} (i32.const {}))\n", a, o) }.as_bytes())
                            .unwrap();
                    } else {
                        panic!()
                    }
                }
                Tipe::_String(_) => {}
            },
            Pohon::var(a, b) => match b {
                Tipe::_u8(_) => {
                    local
                        .write({ format!("(local ${} i32)\n", a) }.as_bytes())
                        .unwrap();
                }
                Tipe::_String(_) => {}
            },
            Pohon::blok(o) => {
                _main
                    .write({ format!("(block ${}\n", o) }.as_bytes())
                    .unwrap();
            }
            Pohon::br(o) => {
                _main.write({ format!("br {}\n", o) }.as_bytes()).unwrap();
            }
            Pohon::if_br(o, a) => {
                _main
                    .write({ format!("{}\nbr_if {}\n", a, o) }.as_bytes())
                    .unwrap();
            }
            Pohon::cetak(o) => {
                match o {
                    Nilai::lansung_str(o) => {
                        _main.write(web.print(&o).as_bytes()).unwrap();
                        _data.write(o.as_bytes()).unwrap();
                    }
                    Nilai::lansung_int(o) => {
                        _main
                            .write(
                                web.print_(1, format!("i32.const {}\ncall $log_\n", o))
                                    .as_bytes(),
                            )
                            .unwrap();
                    }
                    Nilai::lansung_float(o) => {
                        _main
                            .write(
                                web.print_(2, format!("f32.const {}\ncall $log_f\n", o))
                                    .as_bytes(),
                            )
                            .unwrap();
                    }
                    Nilai::minta(o) | Nilai::penujuk(o) => {
                        _main
                            .write(
                                web.print_var(o.id, if let Tipe::_u8(_) = o.nilai { 1 } else { 2 })
                                    .as_bytes(),
                            )
                            .unwrap();
                    }
                    Nilai::penujuk_(O) => {}
                    Nilai::None => {
                        //uji coba
                        _main.write(b"\n").unwrap();
                    }
                }
            }
        }
    });
    drop((_main, _data, local));
    let mut wat = std::io::BufWriter::new(
        match std::fs::File::create(format!("{}\\parsing\\www\\main.wat", path)) {
            Ok(o) => o,
            Err(_) => {
                std::fs::create_dir_all(format!("{}\\parsing\\www", path))
                    .expect("tidak dapat membuat target direktori (target)");
                std::fs::File::create(format!("{}\\parsing\\www\\main.wat", path)).unwrap()
            }
        },
    );
    let mut html = std::io::BufWriter::new(
        match std::fs::File::create(format!("{}\\target\\debug\\www\\index.html", path)) {
            Ok(o) => o,
            Err(_) => {
                std::fs::create_dir_all(format!("{}\\parsing\\www", path))
                    .expect("tidak dapat membuat target direktori (target)");
                std::fs::File::create(format!("{}\\parsing\\www\\index.html", path)).unwrap()
            }
        },
    );
    //wat library
    /*
    if web.import_lib[1]{
        //wat.write(b"").unwrap();
        wat.write(b"(import\"import\"\"log_\"(func $log_(param i32)))\n").unwrap();
    }
    if web.import_lib[2]{
        wat.write(b"(import\"import\"\"log_\"(func $log_f(param f32)))\n").unwrap();
    }
    if web.import_lib[3]{
        wat.write(b"(import\"import\"\"body\"(func $body(param i32 i32)))\n").unwrap();
    }
    if web.import_lib[0]{
        //wat.write(b"\"mem\":new WebAssembly.Memory({initial:1})\n").unwrap();
        //(import "import" "mem" (memory 1))
        wat.write({format!("(import\"import\"\"log\"(func $log(param i32 i32)))\n(import\"import\"\"mem\"(memory 1))\n(data (i32.const 0)\"{}\")\n",std::fs::read_to_string(format!("{}\\parsing\\www\\data",path)).unwrap())}.as_bytes()).unwrap();
    }
    */
    wat.write(b"(module\n").unwrap();
    html.write(b"<head><script>let i_o={\"import\":{").unwrap();
    if web.import_lib[0] {
        wat.write({format!("(import\"import\"\"log\"(func $log(param i32 i32)))\n(import\"import\"\"mem\"(memory 1))\n(data (i32.const 0)\"{}\")\n",std::fs::read_to_string(format!("{}\\parsing\\www\\data",path)).unwrap())}.as_bytes()).unwrap();
        html.write(b"\"mem\":new WebAssembly.Memory({initial:1}),\"log\":(o,l)=>{console.log(new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,o,l)))}").unwrap();
    }
    if web.import_lib[1] {
        wat.write(b"(import\"import\"\"log_\"(func $log_(param i32)))\n")
            .unwrap();
        html.write(b",\"log_\":o=>console.log(o)").unwrap();
    }
    if web.import_lib[3] {
        wat.write(b"(import\"import\"\"body\"(func $body(param i32 i32)))\n")
            .unwrap();
        html.write(b",\"body\":(o,l)=>{document.write(new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,o,l)))}").unwrap();
    }
    if web.import_lib[4] {
        wat.write(
            b"(import\"import\"\"warnalatarbelakang\"(func $warnalatarbelakang(param i32 i32)))\n",
        )
        .unwrap();
        html.write(b",\"warnalatarbelakang\":(o,l)=>{document.body.style.backgroundColor=new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,o,l))}").unwrap();
    }
    if web.import_lib[5] {
        wat.write(b"(import\"import\"\"gambarlatarbelakang\"(func $gambarlatarbelakang(param i32 i32)))\n").unwrap();
        html.write(b",\"gambarlatarbelakang\":(o,l)=>{document.body.style.backgroundImage=new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,o,l))}").unwrap();
    }
    if web.import_lib[6] {
        wat.write(b"(import\"import\"\"judul\"(func $judul(param i32 i32)))\n")
            .unwrap();
        html.write(b",\"judul\":(o,l)=>{document.title=new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,o,l))}").unwrap();
    }
    if web.import_lib[7] {
        wat.write(b"(import\"import\"\"tombol\"(func $tombol(param i32 i32)))\n")
            .unwrap();
        html.write(b",\"tombol\":(o,l)=>{let v =document.createElement(\"button\");v.id =new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,o,l));document.body.appendChild(v)}").unwrap();
    }
    if web.import_lib[8] {
        wat.write(b"(import\"import\"\"isi\"(func $isi(param i32 i32 i32 i32)))\n")
            .unwrap();
        html.write(b",\"isi\":(o,l,x,y)=>{document.getElementById(new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,o,l))).innerHTML =new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,x,y))}").unwrap();
    }
    if web.import_lib[9] {
        wat.write(b"(import\"import\"\"warna\"(func $warna(param i32 i32 i32 i32)))\n")
            .unwrap();
        html.write(b",\"warna\":(o,l,x,y)=>{document.getElementById(new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,o,l))).style.color =new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,x,y))}").unwrap();
    }
    if web.import_lib[10] {
        wat.write(b"(import\"import\"\"warnalatarbelakangid\"(func $warnalatarbelakangid(param i32 i32 i32 i32)))\n").unwrap();
        html.write(b",\"warnalatarbelakangid\":(o,l,x,y)=>{document.getElementById(new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,o,l))).style.backgroundColor =new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,x,y))}").unwrap();
    }
    if web.import_lib[11] {
        wat.write(b"(import\"import\"\"ukurankata\"(func $ukurankata(param i32 i32 i32 i32)))\n")
            .unwrap();
        html.write(b",\"ukurankata\":(o,l,x,y)=>{document.getElementById(new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,o,l))).style.fontSize =new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,x,y))}").unwrap();
    }
    if web.import_lib[12] {
        wat.write(b"(import\"import\"\"kelas\"(func $kelas(param i32 i32 i32 i32)))\n")
            .unwrap();
        html.write(b",\"kelas\":(o,l,x,y)=>{document.getElementById(new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,o,l))).classList.add(new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,x,y)))}").unwrap();
    }
    if web.import_lib[13] {
        wat.write(b"(import\"import\"\"str_sama_str\"(func $str_sama_str(param i32 i32 i32 i32))(result i32))\n")
            .unwrap();
        html.write(b",\"str_sama_str\":(o,l,x,y)=>{return new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,o,l)===new TextDecoder('utf8').decode(new Uint8Array(i_o.import.mem.buffer,x,y)?1:0})))}").unwrap();

    }
    //
    html.write(b"}};fetch('main.wasm').then(r=>r.arrayBuffer()).then(b=>WebAssembly.instantiate(b,i_o)).then(r=>{r.instance.exports.main();").unwrap();
    for i in web.Klik {
        //html.write({format!("let v=document.getElementsByClassName('{}');let i=v.length;while(i--)v[i].addEventListener(\"click\",r.instance.exports.{});",i[0],i[1])}.as_bytes()).unwrap();
        //html.write({format!("document.getElementsByClassName('{}').forEach(function (i){{i.addEventListener(\"click\",r.instance.exports.{})}});",i[0],i[1])}.as_bytes()).unwrap();
        html.write({format!("[].forEach.call(document.getElementsByClassName('{}'),function(i){{i.addEventListener(\"click\",r.instance.exports.{})}});",i[0],i[1])}.as_bytes()).unwrap();
    }
    html.write(b"}).catch(console.error);</script></head>")
        .unwrap();
    //html.write(b"}};WebAssembly.instantiateStreaming(fetch('./main.wasm'),i_o).then(r=>r.instance.exports.main()).catch(console.error);</script></head>").unwrap();
    match std::fs::read_to_string(format!("{}\\parsing\\www\\local", path)) {
        Ok(local) => {
            //app memiliki lib
            if main_selesai {
                match std::fs::File::open(format!("{}\\parsing\\www\\lib", path)) {
                    Ok(o) => {
                        let mut fn_pertama = false;
                        let mut f = std::io::BufReader::new(o);
                        let mut buff = String::with_capacity(200);
                        /*for i in f.lines() {

                        }*/
                        while f.read_line(&mut buff).unwrap() != 0 {
                            if buff.starts_with(":fn:") {
                                let t: Vec<&str> = buff.split(":").collect();
                                wat.write(
                                    {
                                        format!(
                                            "{0}(func ${1}(export\"{1}\")\n",
                                            if fn_pertama {
                                                ")\n"
                                            } else {
                                                fn_pertama = true;
                                                ""
                                            },
                                            t[2].trim_end()
                                        )
                                    }
                                    .as_bytes(),
                                )
                                .unwrap();
                            } else {
                                wat.write(buff.as_bytes()).unwrap();
                            }
                            buff.clear()
                        }
                        wat.write(b")\n").unwrap();
                    }
                    Err(_) => {
                        panic!()
                    }
                }
            }
            //write&drop
            { wat }
                .write(
                    {
                        format!(
                            "(func(export\"main\")\n{}{}))",
                            if !local.is_empty() {
                                local
                            } else {
                                "".to_string()
                            },
                            std::fs::read_to_string(format!("{}\\parsing\\www\\main", path))
                                .unwrap()
                        )
                    }
                    .as_bytes(),
                )
                .unwrap();
            std::fs::write(
                format!("{}\\target\\debug\\www\\main.wasm", path),
                &wat::parse_file(format!("{}\\parsing\\www\\main.wat", path)).unwrap(),
            )
            .unwrap();
        }
        Err(_) => {
            panic!()
        }
    }
    //css
    //
    //import
    /*versi lama
    let mut coma = false;
    if web.import_lib[0]{
        coma = true;
        html.write(b"\"mem\":new WebAssembly.Memory({initial:1}),\"log\":(offset,length)=>{console.log(new TextDecoder('utf8').decode(new Uint8Array(import_object.import.mem.buffer,offset,length)))}").unwrap();
    }
    if web.import_lib[1]{
        if coma {
            html.write(b",\"log_\":o=>console.log(o)").unwrap();
        } else {
            coma = true;
            html.write(b"\"log_\":o=>console.log(o)").unwrap();
        }
    }
    if web.import_lib[3]{
        //document.body = "";
        if coma {
            html.write(b",\"body_\":o=>{document.body = o}").unwrap();
        } else {
            coma = true;
            html.write(b"\"body_\":o=>{document.body = o}").unwrap();
        }
    }*/
    //optimalisasi

    /*gagal
    html.write(b"<script>fetch('main.wasm').then(r=>r.arrayBuffer()).then(b=>WebAssembly.instantiate(b,{\"import\":{").unwrap();
    let mut coma = false;
    if web.import_lib[0]{
        coma = true;
        html.write(b"\"mem\":new WebAssembly.Memory({initial:1}),\"log\":(offset,length)=>{console.log(new TextDecoder('utf8').decode(new Uint8Array(import_object.import.mem.buffer,offset,length)))}").unwrap();
    }
    if web.import_lib[1]{
        if coma {
            html.write(b",\"log_\":o=>console.log(o)").unwrap();
        } else {
            coma = true;
            html.write(b"\"log_\":o=>console.log(o)").unwrap();
        }
    }
    html.write(b"}})).then(r=>r.instance.exports.main()).catch(console.error);</script></head>").unwrap();
    */
}
/*
// versi lama
pub fn app_2(
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
                        Nilai::None=>{
                            if !web.import_lib[2]{
                                if imp_koma {
                                    imp.write(b",").unwrap();
                                } else {
                                    imp_koma = true;
                                }
                                imp_wat.write(b"(import\"import\"\"log_\"(func $log_(param i32 i32)))\n").unwrap();
                                imp.write(b"\"log_\":o=>console.log(o)").unwrap();
                                web.import_lib[2] = true;
                            }
                        }
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
                        Nilai::minta(o)|Nilai::penujuk(o)=>{
                            //
                            if !web.import_lib[2]{
                                if imp_koma {
                                    imp.write(b",").unwrap();
                                } else {
                                    imp_koma = true;
                                }
                                imp_wat.write(b"(import\"import\"\"log_\"(func $log_(param i32)))\n").unwrap();
                                imp.write(b"\"log_\":o=>console.log(o)").unwrap();
                                web.import_lib[2] = true;
                            }
                            _main.write({
                                format!("local.get ${}\ncall $log_\n",o.id)
                            }.as_bytes()).unwrap();
                        }
                        Nilai::lansung_int(o)=>{
                            if web.import_lib[2]{
                                if imp_koma {
                                    imp.write(b",").unwrap();
                                } else {
                                    imp_koma = true;
                                }
                                imp_wat.write(b"(import\"import\"\"log_\"(func $log_(param i32 i32)))\n").unwrap();
                                imp.write(b"\"log_\":o=>console.log(o)").unwrap();
                                web.import_lib[2] = true;
                            }
                            _main.write({
                                format!("i32.const {}\ncall $log_\n",o)
                            }.as_bytes()).unwrap();
                        }
                        Nilai::lansung_float(o)=>{
                            if web.import_lib[2]{
                                if imp_koma {
                                    imp.write(b",").unwrap();
                                } else {
                                    imp_koma = true;
                                }
                                imp_wat.write(b"(import\"import\"\"log_\"(func $log_(param i32 i32)))\n").unwrap();
                                imp.write(b"\"log_\":o=>console.log(o)").unwrap();
                                web.import_lib[2] = true;
                            }
                            _main.write({
                                format!("i32.const {}\ncall $log_\n",o)
                            }.as_bytes()).unwrap();
                        }
                    }
                }
                Pohon::const_32(a)=>{
                     _main.write({
                         format!("i32.const {}\n",a)
                     }.as_bytes()).unwrap();
                }
                Pohon::add=>{
                    _main.write({
                        "i32.add\n"
                    }.as_bytes()).unwrap();
                }
                Pohon::sub=>{
                    _main.write({
                        "i32.sub\n"
                    }.as_bytes()).unwrap();
                }
                Pohon::div_s=>{
                    _main.write({
                        "i32.div_s\n"
                    }.as_bytes()).unwrap();
                }
                Pohon::div_u=>{
                    _main.write({
                        "i32.div_u\n"
                    }.as_bytes()).unwrap();
                }
                Pohon::mul=>{
                    _main.write({
                        "i32.shl\n"
                    }.as_bytes()).unwrap();
                }
                Pohon::local_set(a)=>{
                    _main.write({
                        format!("local.set ${}\n",a)
                    }.as_bytes()).unwrap();
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
*/
