use rust_embed::RustEmbed;
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{Read, Seek, SeekFrom, Write};
#[derive(RustEmbed)]
#[folder = "asset/"]
#[prefix = "prefix/"]
struct Asset;
struct Manifest {
    nama: String,
    nama_pendek: String,
    warna_latarbelakan: String,
    warna_tema: String,
    display: String,
    description: String,
    url_mulai: String,
    icons: Vec<[String; 3]>,
    screenshots: Vec<[String; 3]>,
}
struct Perpus {
    cetak_data: bool,
    cetak_angka: bool,
    cetak_angka_float: bool,
    tidur: bool,
    kalender: bool,
    error: bool,
}
impl Perpus {
    pub fn baru() -> Perpus {
        Perpus {
            cetak_data: false,
            cetak_angka: false,
            cetak_angka_float: false,
            tidur: false,
            kalender: false,
            error: false,
        }
    }
    pub fn perpus(&self, buf: &mut String, html: &mut File) {
        //htmal
        let offset = buf.len();
        buf.push_str("<!DOCTYPE html><head><meta name=\"viewport\" content=\"width=device-width\" /><link rel=\"apple-touch-icon\" href=\"aset/icon512.png\" /><link rel=\"manifest\"href=\"./manifest.json\"/><script>if ('serviceWorker' in navigator) {const o_={import: {woker_in:(o,l)=>navigator.serviceWorker.register(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer, o, l))).then((r) => console.log('Service worker registration succeeded:', r),(e) => console.log('Service worker registration failed:', e)),");
        if self.cetak_data {
            buf.push_str("log_:(x,y)=>console.log(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,x,y))),")
        }
        if self.cetak_angka {
            buf.push_str("log:(x)=>console.log(x),")
        }
        if self.cetak_angka_float {
            buf.push_str("log_f:(x)=>console.log(x),")
        }
        if self.kalender {
            buf.push_str("kalender:()=>{return Date.now()},");
        }
        if self.tidur {
            buf.push_str("tidur:async(x)=>{await new promise(()=>setTimeout(x))},")
        }
        if false {
            //timer
            buf.push_str("jam=>[0],")
        }
        if false {
            //fps
            buf.push_str("fps:(x)=>{let y = Date.now();if (jam[0] != 0) do {y = Date.now();} while (y - jam[0] < x);}jam[0] = y;},")
        }
        if self.error {
            ("Error:(x,y)=>throw new WebAssembly.RuntimeError(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,x,y))),");
        }
        buf.push_str("mem:new WebAssembly.Memory({ initial: 1 }),},};fetch('main.wasm').then((r) => r.arrayBuffer()).then((b) => WebAssembly.instantiate(b, o_)).then((r) => {r.instance.exports.main()}).catch(console.error);} else {console.log('Service workers are not supported.');}</script></head>");
        html.write_all(buf[offset..buf.len()].as_bytes()).unwrap();
        //wasm
        buf.truncate(offset);
        if self.cetak_data {
            buf.push_str("(import\"import\"\"log_\"(func $log_(param i32 i32)))\n");
        }
        if self.cetak_angka {
            buf.push_str("(import\"import\"\"log\"(func $log(param i32)))\n");
        }
        if self.cetak_angka_float {
            buf.push_str("(import\"import\"\"log_f\"(func $log_f(param f32)))\n");
        }
        if self.kalender {
            buf.push_str("(import\"import\"\"kalender\"(func $kalender(result i32)))\n")
        }
        if self.tidur {
            buf.push_str("(import\"import\"\"tidur\"(func $tidur(param i32)))\n")
        }
        if self.error {
            buf.push_str("(import\"import\"\"Error\"(func $Error(param i32 i32)))\n")
        }
    }
}
struct App {
    data: String,
    buf: String,
    instruksi: Vec<(
        File,
        File,
        crate::parsing::Arrmap<(bool, usize), Vec<String>>,
        String,
        bool,
        Vec<u8>,
        Vec<u8>,
    )>,
    //alokasi: Vec<File>,
    alokasi_offset: u64,
    alokasi_penunjuk: HashMap<
        u64,
        (
            crate::parsing::mem_tipe,
            Vec<(/*offset*/ u64, /*Bytes*/ u64)>,
        ),
    >,
    perpus: Perpus,
    manifes: Option<Manifest>,
}
impl App {
    fn _let(&mut self, _let: &crate::parsing::Let_) {
        match &_let.tipe {
            crate::parsing::Tipe::int(unsigh, bit) => {
                //println!("-> {:?}",nilai);
                self.buf.clear();
                let mut var_offset: Vec<(u64, u64)> = Vec::new();
                match unsigh {
                    crate::parsing::mem_tipe::int(v) => {
                        var_offset.reserve_exact(v.as_ref().unwrap().len());
                        v.as_ref().unwrap().into_iter().for_each(|x| {
                            self.buf.push_str("i32.const ");
                            self.buf.push_str(&format!("{}", self.alokasi_offset));
                            self.buf.push_str("\ni32.const ");
                            if let Some(y) = x {
                                self.buf.push_str(&format!("{}", y));
                            } else {
                                self.buf.push_str("0");
                            }
                            var_offset.push((self.alokasi_offset, {
                                if *bit >= 8 {
                                    self.buf.push_str("\ni32.store8\n");
                                    8
                                } else {
                                    panic!()
                                }
                            }));
                            self.alokasi_offset += 1;
                        });
                    }
                    crate::parsing::mem_tipe::unint(v) => {
                        var_offset.reserve_exact(v.as_ref().unwrap().len());
                        v.as_ref().unwrap().into_iter().for_each(|x| {
                            self.buf.push_str("i32.const ");
                            self.buf.push_str(&format!("{}", self.alokasi_offset));
                            self.buf.push_str("\ni32.const ");
                            if let Some(y) = x {
                                self.buf.push_str(&format!("{}", y));
                            } else {
                                self.buf.push_str("0");
                            }
                            var_offset.push((self.alokasi_offset, {
                                if *bit >= 8 {
                                    self.buf.push_str("\ni32.store8\n");
                                    8
                                } else {
                                    panic!()
                                }
                            }));
                            self.alokasi_offset += 1;
                        });
                    }
                    crate::parsing::mem_tipe::float(v) => {
                        var_offset.reserve_exact(v.as_ref().unwrap().len());
                        v.as_ref().unwrap().into_iter().for_each(|x| {
                            self.buf.push_str("i32.const ");
                            self.buf.push_str(&format!("{}", self.alokasi_offset));
                            self.buf.push_str("\ni32.const ");
                            if let Some(y) = x {
                                self.buf.push_str(&format!("{}", y));
                            } else {
                                self.buf.push_str("0");
                            }
                            var_offset.push((self.alokasi_offset, {
                                if *bit >= 8 {
                                    self.buf.push_str("\ni32.store8\n");
                                    8
                                } else {
                                    panic!()
                                }
                            }));
                            self.alokasi_offset += 1;
                        });
                    }
                }
                self.instruksi
                    .last_mut()
                    .unwrap()
                    .1
                    .write_all(self.buf.as_bytes())
                    .unwrap();
                self.alokasi_penunjuk
                    .insert(_let.id, (unsigh.clone(), var_offset));
            }
            _ => {
                panic!()
            }
        }
    }
    fn tambah_data(&mut self, str_: &String) -> usize {
        let offset;
        if let Some(data_offset) = self.data.find(str_) {
            offset = data_offset
        } else {
            offset = self.data.len();
            self.data.push_str(&str_);
        }
        offset
    }
    fn cetak(&mut self, data: &crate::parsing::args) {
        match data {
            crate::parsing::args::Str_Lansung(str_) => {
                self.buf.clear();
                self.buf.push_str("i32.const ");
                let off = self.tambah_data(str_);
                self.buf.push_str(&format!("{}", off));
                self.buf.push_str("\ni32.const ");
                self.buf.push_str(&format!("{}", str_.len()));
                self.buf.push_str("\ncall $log_\n");
                self.instruksi
                    .last_mut()
                    .unwrap()
                    .0
                    .write_all(self.buf.as_bytes())
                    .unwrap();
                self.perpus.cetak_data = true
            }
            crate::parsing::args::Int(bit, _, _, _, nama, nilai) => {}
            crate::parsing::args::Str_int(nilai) => self.perpus.cetak_angka = true,
            crate::parsing::args::penunjuk(id) => {
                let data = self.alokasi_penunjuk.get(id).unwrap();
                self.buf.clear();
                match &data.0 {
                    crate::parsing::mem_tipe::unint(_) => {
                        self.buf.push_str("i32.const ");
                        self.buf.push_str(&format!("{}", data.1[0].0));
                        if data.1[0].1 >= 8 {
                            self.buf.push_str("\ni32.load8_u\n");
                        }
                        self.buf.push_str("call $log\n");
                        self.perpus.cetak_angka = true;
                    }
                    crate::parsing::mem_tipe::int(_) => {
                        self.buf.push_str("i32.const ");
                        self.buf.push_str(&format!("{}", data.1[0].0));
                        if data.1[0].1 >= 8 {
                            self.buf.push_str("\ni32.load8_i\n");
                        }
                        self.buf.push_str("\ncall $log\n");
                        self.perpus.cetak_angka = true;
                    }
                    crate::parsing::mem_tipe::float(_) => {
                        self.buf.push_str("i32.const ");
                        self.buf.push_str(&format!("{}", data.1[0].0));
                        if data.1[0].1 >= 8 {
                            self.buf.push_str("\ni32.load8_f\n");
                        }
                        self.buf.push_str("\ncall $log_f\n");
                        self.perpus.cetak_angka_float = true;
                    }
                }
                self.instruksi
                    .last_mut()
                    .unwrap()
                    .0
                    .write_all(self.buf.as_bytes())
                    .unwrap();
            }
            crate::parsing::args::penunjuk_nama(_) => {
                panic!()
            }
            crate::parsing::args::null => {
                panic!()
            }
            crate::parsing::args::internar_memory(nama) => {
                self.buf.push_str("local.get $");
                self.buf.push_str(&nama);
                self.buf.push_str("\ncall $log\n");
                self.instruksi
                    .last_mut()
                    .unwrap()
                    .0
                    .write_all(self.buf.as_bytes())
                    .unwrap();
                self.perpus.cetak_angka = true;
            }
        }
    }
    fn tidur(&mut self, data: &crate::parsing::args) {
        self.buf.clear();
        match data {
            crate::parsing::args::Str_int(i) => {
                self.buf.push_str("i32.const ");
                self.buf.push_str(&format!("{}", i));
            }
            _ => {
                panic!()
            }
        }
        self.buf.push_str("\ncall $tidur\n");
        self.instruksi
            .last_mut()
            .unwrap()
            .0
            .write_all(self.buf.as_bytes())
            .unwrap();
        self.perpus.tidur = true;
        //self.perpus.kalender = true
    }
    fn br(&mut self, nama: &String) {
        self.buf.clear();
        self.buf.push_str("br ");
        self.buf.push_str(nama);
        self.buf.push('\n');
        self.instruksi
            .last_mut()
            .unwrap()
            .0
            .write_all(self.buf.as_bytes())
            .unwrap();
    }
    fn blok(&mut self, iter_: &crate::parsing::eterator) {
        self.buf.clear();
        match iter_ {
            crate::parsing::eterator::Blok(nama) => {
                self.buf.push_str("block $");
                self.buf.push_str(&nama);
                self.buf.push_str("\n");
            }
            crate::parsing::eterator::Putar(nama) => {
                self.buf.push_str("loop $");
                self.buf.push_str(&nama);
                self.buf.push_str("\n");
            }
            crate::parsing::eterator::Iter(nama, min, max) => match (max, min) {
                (
                    crate::parsing::args::Str_int(maximum),
                    crate::parsing::args::Str_int(minimum),
                ) => {
                    panic!()
                }
                (crate::parsing::args::null, crate::parsing::args::Str_int(putarkan)) => {
                    if putarkan <= &0 {
                        self.buf.push_str("loop $");
                        self.buf.push_str(&nama);
                        self.buf.push_str("\ni32.const 0\nbr_if 1\n");
                    } else if let Some(t) = self
                        .instruksi
                        .last_mut()
                        .unwrap()
                        .2
                        .get_all_mut(&(false, 32))
                    {
                        t.0 .0 = true;
                        self.buf.push_str("i32.const ");
                        self.buf.push_str(&format!("{}\n", putarkan));
                        self.buf.push_str("local.set $");
                        self.buf.push_str(&format!("{}\n", t.1[0]));
                        self.buf.push_str("loop $");
                        self.buf.push_str(&nama[1..]);
                        self.buf.push_str("\n");
                        self.buf
                                .push_str(&format!("local.get ${0}\ni32.eqz\nbr_if 1\nlocal.get ${0}\ni32.const 1\ni32.sub\nlocal.set ${0}\n", t.1[0]));
                    } else {
                        self.buf.push_str("i32.const ");
                        self.buf.push_str(&format!("{}\n", putarkan));
                        self.buf.push_str("local.set $");
                        self.buf.push_str(&format!("{}\n", nama));
                        self.buf.push_str("loop $");
                        self.buf.push_str(&nama[1..]);
                        self.buf.push_str("\n");
                        self.instruksi
                            .last_mut()
                            .unwrap()
                            .2
                            .insert((true, 32), vec![format!("{}", nama)]);
                        self.buf
                                .push_str(&format!("local.get ${0}\ni32.eqz\nbr_if 1\nlocal.get ${0}\ni32.const 1\ni32.sub\nlocal.set ${0}\n", nama));
                    }
                }
                _ => {
                    panic!()
                }
            },
        }
        /*
        match nama {
            crate::parsing::eterator::Blok(nama) => {
                self.buf.push_str("block $");
                self.buf.push_str(&nama);
                self.buf.push_str("\n");
            }
            crate::parsing::eterator::Putar(nama) => {
                self.buf.push_str("loop $");
                self.buf.push_str(&nama);
                self.buf.push_str("\n");
            }
            crate::parsing::eterator::Iter(nama, min, max) => match max {
                crate::parsing::args::Int(..) => {
                    panic!()
                }
                crate::parsing::args::penunjuk(..) => {
                    panic!()
                }
                crate::parsing::args::penunjuk_nama(_) =>{
                    panic!()
                }
                crate::parsing::args::Str_int(maximum) => {
                    match (&min, self.instruksi.last_mut().unwrap()) {
                        _ if maximum <= &0 => {}
                        (crate::parsing::args::Str_int(minimum), x) => {
                            if let Some(t) = x.2.get_all_mut(&(false, 32)) {
                                panic!()
                            } else if !(minimum <= &0 || minimum <= maximum) {
                                self.buf.push_str(&format!(
                                    "i32.const {}\nlocal.set ${}\n",
                                    minimum, nama
                                ));
                                self.buf.push_str("loop $");
                                self.buf.push_str(&nama);
                                self.buf.push_str(&format!(
                                    "\nlocal.get ${0}\ni32.const {1}\ni32.eq\ni32.br_if 1\nlocal.get ${0}\n",nama,maximum
                                ));
                                //penghitum putaran harus ditambah diakhir loop
                                x.2.insert((true, 32), vec![format!("{}", nama)]);
                            } else {
                                self.buf.push_str("loop $");
                                self.buf.push_str(&nama);
                                self.buf.push_str("\ni32.const 0\nbr_if 1\n");
                            }
                        }
                        _ => {
                            panic!()
                        }
                    }
                }
                crate::parsing::args::null => match min {
                    crate::parsing::args::Int(..) => {
                        panic!()
                    }
                    crate::parsing::args::penunjuk(..) => {
                        panic!()
                    }
                    crate::parsing::args::penunjuk_nama(_) =>{
                        panic!()
                    }
                    crate::parsing::args::Str_int(putarkan) => {
                        //let id = 0;
                        if putarkan <= &0 {
                            self.buf.push_str("loop $");
                            self.buf.push_str(&nama);
                            self.buf.push_str("\ni32.const 0\nbr_if 1\n");
                        } else {
                            let x = self.instruksi.last_mut().unwrap();
                            if let Some(t) = x.2.get_all_mut(&(false, 32)) {
                                t.0 .0 = true;
                                self.buf.push_str(&format!(
                                    "i32.const {}\nlocal.set ${}\n",
                                    putarkan, t.1[0]
                                ));
                                self.buf.push_str("loop $");
                                self.buf.push_str(&nama);
                                self.buf.push_str("\n");
                                self.buf
                                        .push_str(&format!("local.get ${0}\ni32.eqz\nbr_if 1\nlocal.get ${0}\ni32.const 1\ni32.sub\nlocal.set ${0}\n", t.1[0]));
                            } else {
                                self.buf.push_str(&format!(
                                    "i32.const {}\nlocal.set ${}\n",
                                    putarkan, nama
                                ));
                                self.buf.push_str("loop $");
                                self.buf.push_str(&nama);
                                self.buf.push_str("\n");
                                x.2.insert((true, 32), vec![format!("{}", nama)]);
                                self.buf
                                            .push_str(&format!("local.get ${0}\ni32.eqz\nbr_if 1\nlocal.get ${0}\ni32.const 1\ni32.sub\nlocal.set ${0}\n", nama));
                            }
                        }
                    }
                    crate::parsing::args::Str_Lansung(..) => {
                        panic!()
                    }
                    crate::parsing::args::null => {
                        panic!()
                    }
                    crate::parsing::args::internar_memory(nama) =>{
                        panic!()
                    }
                },
                crate::parsing::args::Str_Lansung(..) => {
                    panic!()
                }
                crate::parsing::args::internar_memory(nama) =>{
                    panic!()
                }
            },
        }
        */
        self.instruksi
            .last_mut()
            .unwrap()
            .0
            .write(self.buf.as_bytes())
            .unwrap();
    }
    fn blok_(&mut self) {
        self.instruksi
            .last_mut()
            .unwrap()
            .0
            .write(b"end\n")
            .unwrap();
    }
    fn error(&mut self, teks: Option<std::string::String>) {}
    pub fn baru(path: &String) -> App {
        //let mut buf = format! {"{}\\parsing\\pwa\\main.wat",path};
        //let mut wat = std::fs::File::with_options().create(true).truncate(true).read(true).write(true).open(&buf).unwrap();
        //(0..8).into_iter().for_each(|_|{
        //    buf.pop();
        //});
        let mut buf = format! {"{}\\parsing\\pwa\\instruksi\\main",path};
        let mut instruksi = std::fs::File::options()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(&buf)
            .unwrap();
        buf.truncate(buf.len() - 14);
        buf.push_str("alokasi\\main");
        let alokasi = std::fs::File::options()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(&buf)
            .unwrap();
        //alokasi.write(b"(func(export\"main\")\n").unwrap();
        instruksi
            .write(b"i32.const 0\ni32.const 5\ncall $woker_in\n")
            .unwrap();
        //wat.write(b"(modul\n").unwrap();
        App {
            data: String::from("sw.js"),
            buf,
            //wat:wat,
            instruksi: vec![(
                instruksi,
                alokasi,
                crate::parsing::Arrmap::new(),
                "main".to_string(),
                true,
                Vec::new(),
                Vec::new(),
            )],
            //alokasi: z,
            alokasi_offset: 0,
            alokasi_penunjuk: HashMap::with_capacity(1),
            perpus: Perpus::baru(),
            manifes: None,
        }
    }
    pub fn drop(mut self, path: &String) {
        //self.buf.clear();
        //self.buf.push_str(path.as_str());
        //self.buf.push_str("/target/pwa/debug/main.wasm");
        //println!(" test-> {}",self.buf);
        //let mut was : File = std::fs::File::with_options().create(true).truncate(true).read(true).write(true).open(&self.buf).unwrap();
        self.buf.clear();
        self.buf.push_str("(module\n(import\"import\"\"woker_in\"(func $woker_in(param i32 i32)))\n(import\"import\"\"mem\"(memory 1))\n");
        let offset = self.buf.len();
        self.buf.push_str(&path);
        self.buf.push_str("\\target\\debug\\pwa\\index.html");
        let mut html = File::create(format!("{}\\target\\debug\\pwa\\index.html", path)).unwrap();
        self.buf.truncate(offset);
        self.perpus.perpus(&mut self.buf, &mut html);
        /*
        for i in 1..self.instruksi.len() {
            self.instruksi[i].0.seek(SeekFrom::Start(0)).unwrap();
            self.instruksi[i].1.seek(SeekFrom::Start(0)).unwrap();
            self.instruksi[i].1.read_to_string(&mut self.buf).unwrap();
            self.instruksi[i].0.read_to_string(&mut self.buf).unwrap();
            self.buf.push(')');
        }
        */
        self.buf.push_str("(data(i32.const 0)\"");
        self.buf.push_str(&self.data);
        self.buf.push_str("\")\n");
        for i in &mut self.instruksi {
            if i.4 {
                self.buf.push_str("(func(export\"");
                self.buf.push_str(&i.3);
                self.buf.push_str("\")\n");
            } else {
                self.buf.push_str("(func");
                panic!()
            }
            i.0.seek(SeekFrom::Start(0)).unwrap();
            i.1.seek(SeekFrom::Start(0)).unwrap();
            for i in i.2.iter() {
                self.buf.push_str("(local $");
                self.buf.push_str(&i.1[0]);
                self.buf.push_str(&format!(" i{}", i.0 .1));
                self.buf.push_str(")\n");
            }
            i.1.read_to_string(&mut self.buf).unwrap();
            i.0.read_to_string(&mut self.buf).unwrap();
            self.buf.push(')');
        }
        /*
        self.buf.push_str("(func(export\"main\")\n");
        self.instruksi[0].0.seek(SeekFrom::Start(0)).unwrap();
        self.instruksi[0].1.seek(SeekFrom::Start(0)).unwrap();
        //println!("{:?}", self.instruksi[0].2);
        for i in self.instruksi[0].2.iter() {
            self.buf.push_str("(local $");
            self.buf.push_str(&i.1[0]);
            self.buf.push_str(&format!(" i{}", i.0 .1));
            self.buf.push_str(")\n");
        }
        self.instruksi[0].1.read_to_string(&mut self.buf).unwrap();
        self.instruksi[0].0.read_to_string(&mut self.buf).unwrap();
        self.buf.push(')');
        */
        /*
        for i in read_dir(format! {"{}\\parsing\\pwa\\instruksi\\",path}).unwrap(){
            let nama = &i.unwrap().path().display().to_string();
            //let nama_fungsi  = nama.split("\\").collect::<Vec<&str>>()[];
            self.instruksi = File::open(nama).unwrap();
            self.instruksi.read_to_string(&mut self.buf).unwrap();
            self.buf.push(')')
        }
        */
        /*
        self.buf.push_str("(data(i32.const 0)\"");
        self.buf.push_str(&self.data);
        self.buf.push_str("\"))");
        */
        self.buf.push_str(")");
        let mut was = File::create(format!("{}\\target\\debug\\pwa\\main.wasm", path)).unwrap();
        std::fs::write(format!("{}\\parsing\\wat.debug", path), &self.buf).unwrap();
        was.write_all(&wat::parse_bytes(self.buf.as_bytes()).unwrap())
            .unwrap();
        //sw.js
        std::fs::write(format!("{}\\target\\debug\\pwa\\sw.js", path), b"this.addEventListener('install',(e)=>{console.log('Installed service worker');e.waitUntil(caches.open('static').then((c)=>{c.addAll(['./','./index.html','./main.wasm','./aset/icon192.png','./aset/icon192.png'])}))});self.addEventListener('activate',()=>console.log('SW Activated'));self.addEventListener('fetch',(e)=>e.respondWith(caches.match(e.request).then((r)=>r|fetch(e.request))));").unwrap();
        let mut manifest =
            std::fs::File::create(format!("{}\\target\\debug\\pwa\\manifest.json", path)).unwrap();
        if let Some(_manifest) = self.manifes {
            panic!()
        } else {
            manifest.write_all(
                b"{\"name\":\"app\",\"short-name\":\"app\",\"start_url\":\"./\",\"scope\":\".\",\"display\":\"standalone\",\"background_color\":\"#000000\",\"theme_color\":\"#000000\",\"description\":\"app saya\",\"icons\":[{\"src\":\"aset/icon192.png\",\"type\":\"image/png\",\"size\":\"192x192\"},{\"src\":\"aset/icon512.png\",\"type\":\"image/png\",\"size\":\"512x512\"}]}"
            ).unwrap();
            if let Err(_) = std::fs::write(
                format!("{}\\target\\debug\\pwa\\aset\\sancaicon192.png", path),
                Asset::get("prefix/sancaicon192.png").unwrap().data,
            ) {
                std::fs::create_dir_all(format!("{}\\target\\debug\\pwa\\aset", path)).unwrap();
                std::fs::write(
                    format!("{}\\target\\debug\\pwa\\aset\\sancaicon192.png", path),
                    Asset::get("prefix/sancaicon192.png").unwrap().data,
                )
                .unwrap();
            }
            std::fs::write(
                format!("{}\\target\\debug\\pwa\\aset\\sancaicon512.png", path),
                Asset::get("prefix/sancaicon512.png").unwrap().data,
            )
            .unwrap();
        }
    }
    fn tulis(
        &mut self,
        nama: &String,
        index: &usize,
        nilai: &crate::parsing::nilai,
        _var: &crate::parsing::Arrmap<String, Vec<crate::parsing::Let_>>,
    ) {
        if let Some(s) = _var.get(&nama) {
            let var = &s[*index];
            match nilai {
                crate::parsing::nilai::aritmatik(x) => {
                    self.buf.clear();
                    if let Some(m) = self.alokasi_penunjuk.get(&var.id) {
                        self.buf.push_str("i32.const ");
                        self.buf.push_str(&format!("{}\n", m.1[0].0));
                        /*
                        match &m.0 {
                            crate::parsing::mem_tipe::int(i)=>{panic!()}
                            crate::parsing::mem_tipe::unint(i)=>{
                            }
                            crate::parsing::mem_tipe::float(i)=>{panic!()}
                        }
                        */
                        let mut p: Vec<(usize, &crate::parsing::aritmatik)> = Vec::with_capacity(2);
                        p.push((0, &x));
                        while let Some(k) = p.last_mut() {
                            match k.1 {
                                crate::parsing::aritmatik::tambah(a, b) => {
                                    let nilai_ = if k.0 == 0 {
                                        self.buf.push_str("(i32.add(\n");
                                        a
                                    } else {
                                        b
                                    };
                                    match nilai_ {
                                        crate::parsing::nilai::angka(i) => {
                                            self.buf.push_str(&format!("i32.const {}\n", i));
                                        }
                                        crate::parsing::nilai::penunjuk(i) => {
                                            let variabel = self
                                                .alokasi_penunjuk
                                                .get(&_var.get(&i).unwrap()[0].id)
                                                .unwrap();
                                            match variabel.0 {
                                                crate::parsing::mem_tipe::unint(_) => {
                                                    if variabel.1[0].1 >= 8 {
                                                        self.buf.push_str(&format!(
                                                            "i32.const {}\ni32.load8_u\n",
                                                            variabel.1[0].0
                                                        ));
                                                    }
                                                }
                                                _ => {
                                                    panic!()
                                                }
                                            }
                                        }
                                        _ => {
                                            panic!()
                                        }
                                    }
                                    if k.0 == 1 {
                                        self.buf.push_str("))\n");
                                        p.pop();
                                    } else {
                                        k.0 += 1;
                                        self.buf.push_str(")(\n");
                                    }
                                }
                                _ => {
                                    panic!()
                                }
                            }
                        }
                        if m.1[0].1 >= 8 {
                            self.buf.push_str("i32.store8\n");
                        } else {
                            panic!()
                        }
                        //println!("testing ->\n{}",app.buf);
                        self.instruksi
                            .last_mut()
                            .unwrap()
                            .0
                            .write_all(self.buf.as_bytes())
                            .unwrap();
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
    }
}
pub fn konversi(
    _var: &crate::parsing::Arrmap<String, Vec<crate::parsing::Let_>>,
    terima: std::sync::mpsc::Receiver<crate::parsing::Pohon>,
    path: &String,
) -> bool {
    let mut app: App = App::baru(path);
    loop {
        match terima.recv().unwrap() {
            crate::parsing::Pohon::Let(nama, index) => app._let(&_var.get(&nama).unwrap()[index]),
            crate::parsing::Pohon::Cetak(arg) => app.cetak(&arg),
            crate::parsing::Pohon::tidur(arg) => app.tidur(&arg),
            crate::parsing::Pohon::tulis(nama, index, nilai) => {
                app.tulis(&nama, &index, &nilai, &_var)
            }
            crate::parsing::Pohon::blok(nama) => app.blok(&nama),
            crate::parsing::Pohon::blok_ => app.blok_(),
            crate::parsing::Pohon::br(nama) => app.br(&nama),
            crate::parsing::Pohon::Error => {
                return false
            },
            crate::parsing::Pohon::panic(pesan) => app.error(pesan),
            crate::parsing::Pohon::Selesai => break,
        }
    }
    app.drop(path);
    true
}
