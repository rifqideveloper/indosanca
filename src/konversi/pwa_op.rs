use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::BufRead;
use std::io::{Read, Write};
use rust_embed::RustEmbed;
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
enum Tipe_data {
    u8_(u8)
}
struct lib {
    alokasi: HashMap<u64,(usize,Tipe_data)>,
    data: String,
    cetak_data: bool,
    cetak: bool,
    cetak_f: bool,
}
impl lib {
    fn tambah_alokasi(
        &mut self,
        fungsi_buf_file: &mut File,
        buf: &mut String,
        nilai: &crate::parsing::parse_3_1::let_,
    ) {
        match &nilai.tipe {
            crate::parsing::Tipe::_string(s) => {}
            crate::parsing::Tipe::nomer(n) => {}
            crate::parsing::Tipe::uint(bit, u) => {
                let bytes = bit / 8 ;
                if u.len() == 1 {
                    match bytes {
                        1 =>{
                            let v = (self.alokasi.len(),u[0].unwrap() as u8);
                            buf.clear();
                            buf.push_str("i32.const ");
                            buf.push_str(&format!("{}",v.0));
                            buf.push_str("\ni32.const ");
                            buf.push_str(&format!("{}",v.1));
                            buf.push_str("\ni32.store8\n");
                            fungsi_buf_file.write(buf.as_bytes()).unwrap();
                            let v = (v.0,Tipe_data::u8_(v.1));
                            self.alokasi.insert(
                                nilai.id,
                                v
                            );
                        }
                        _ =>{}
                    }
                }
            }
            crate::parsing::Tipe::int(bit, i) => {}
            crate::parsing::Tipe::penujuk_(bit, p) => {}
            crate::parsing::Tipe::minta(bit, m) => {}
            crate::parsing::Tipe::None => {}
        }
    }
    fn ada_data_yn_sama(&mut self, input: &String) -> Option<usize> {
        //potimalisasi
        self.data.find(input)
    }
    fn offset(&self) -> usize {
        self.data.len()
    }
    fn cetak(
        &mut self,
        nilai: crate::parsing::parse_3_1::Nilai,
        fungsi_buf_file: &mut File,
        buf: &mut String,
    ) {
        match nilai {
            crate::parsing::parse_3_1::Nilai::langsung_float(f) => {
                buf.clear();
                buf.push_str("i32.const ");
                buf.push_str(&format!("{}", f));
                buf.push_str("\ncall $log_f\n");
                self.cetak_f = true
            }
            crate::parsing::parse_3_1::Nilai::langsung_int(i) => {
                buf.clear();
                buf.push_str("i32.const ");
                buf.push_str(&format!("{}", i));
                buf.push_str("\ncall $log\n");
                self.cetak = true
            }
            crate::parsing::parse_3_1::Nilai::langsung_str(s) => {
                buf.clear();
                buf.push_str("i32.const ");
                if let Some(offset) = self.ada_data_yn_sama(&s) {
                    buf.push_str(&format!("{}", offset));
                } else {
                    buf.push_str(&format!("{}", self.offset()));
                    self.data.push_str(s.as_str());
                }
                buf.push_str("\ni32.const ");
                buf.push_str(&format!("{}", s.len()));
                buf.push_str("\ncall $log_\n");
                fungsi_buf_file.write(buf.as_bytes()).unwrap();
                self.cetak_data = true
            }
            crate::parsing::parse_3_1::Nilai::minta(m) => {}
            crate::parsing::parse_3_1::Nilai::penujuk_(p) => {

            }
            crate::parsing::parse_3_1::Nilai::None => {}
        }
    }
    fn drop(self, wat_file: &mut File, path: &String, buf: &mut String) {
        let mut html = File::create(format!("{}\\target\\debug\\pwa\\index.html", path)).unwrap();
        html.write(b"<!DOCTYPE html><head><meta name=\"viewport\"content=\"width=device-width\"/><link rel=\"apple-touch-icon\"href=\"aset/icon512.png\"/><link rel=\"manifest\"href=\"./manifest.json\"/><script>if('serviceWorker' in navigator){const o_ ={import:{").unwrap();
        wat_file
            .write(b"(import\"import\"\"woker_in\"(func $woker_in(param i32 i32)))\n")
            .unwrap();
        wat_file
            .write(b"(import\"import\"\"mem\"(memory 1))\n")
            .unwrap();
        if self.cetak_data {
            html.write(b"log_:(x,y)=>console.log(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,x,y))),").unwrap();
            wat_file
                .write(b"(import\"import\"\"log_\"(func $log_(param i32 i32)))\n")
                .unwrap();
        }
        if self.cetak {
            html.write(b"log:(i)=>console.log(i),").unwrap();
            wat_file
                .write(b"(import\"import\"\"log\"(func $log(param i32)))\n")
                .unwrap();
        }
        html.write(b"mem:new WebAssembly.Memory({initial:1}),woker_in:(o,l)=>navigator.serviceWorker.register(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))).then((r)=>console.log('Service worker registration succeeded:',r),(e) =>console.log('Service worker registration failed:',e))}};").unwrap();
        for _fungsi in std::fs::read_dir(format!("{}\\parsing\\pwa\\instruksi", path)).unwrap() {
            let nama = _fungsi.unwrap().path().display().to_string();
            let alokasi_nama = nama.split("\\").last().unwrap();
            let mut alokasi =
                File::open(format!("{}\\parsing\\pwa\\alokasi\\{}", path, alokasi_nama,)).unwrap();
            let mut fungsi_intruksi = std::io::BufReader::new(File::open(nama).unwrap());
            buf.clear();
            fungsi_intruksi.read_line(buf).unwrap();
            alokasi.read_to_string(buf).unwrap();
            while fungsi_intruksi.read_line(buf).unwrap() != 0 {
                wat_file.write(buf.as_bytes()).unwrap();
                buf.clear()
            }
            buf.push(')');
            wat_file.write(buf.as_bytes()).unwrap();
        }
        html.write_all(b"fetch('main.wasm').then((r) => r.arrayBuffer()).then((b) => WebAssembly.instantiate(b, o_)).then((r) => {r.instance.exports.main()}).catch(console.error)}else{console.log('Service workers are not supported.')}</script></head>").unwrap();
        wat_file.write(b"(data(i32.const 0)\"").unwrap();
        wat_file.write(self.data.as_bytes()).unwrap();
        wat_file.write(b"\")").unwrap();
    }
}
#[derive(Clone)]
struct Fungsi {
    nama: String,
    arg: Vec<String>,
    kembali: Vec<String>,
    alokasi: Vec<String>,
}
impl Fungsi {
    fn _let(&mut self,buf:&mut String,buf_file:&mut File , lib: &mut lib, alokasi: &mut File, var:Box< crate::parsing::parse_3_1::let_>,) {
        lib.tambah_alokasi(buf_file,buf,&var)
    }
    fn drop(self, wat_file: &mut File) {}
}
pub fn konversi(
    terima: std::sync::mpsc::Receiver<Box<crate::parsing::parse_3_1::Pohon>>,
    path: &String,
) {
    let mut buf = String::with_capacity(200);
    let mut manifest: Option<Manifest> = None;
    let mut perpus = lib {
        alokasi: HashMap::with_capacity(5),
        data: String::from("./sw.js"),
        cetak: false,
        cetak_f: false,
        cetak_data: false,
    };
    buf.push_str(path.as_str());
    buf.push_str("\\parsing\\pwa\\instruksi");
    std::fs::remove_dir_all(&buf).is_ok();
    std::fs::create_dir_all(&buf).unwrap();
    buf.push_str("\\main");
    let mut fungsi_buf_file = File::create(&buf).unwrap();
    buf.clear();
    buf.push_str(path.as_str());
    buf.push_str("\\parsing\\pwa\\alokasi");
    std::fs::remove_dir_all(&buf).is_ok();
    std::fs::create_dir_all(&buf).unwrap();
    buf.push_str("\\main");
    let mut alokasi = File::create(&buf).unwrap();
    fungsi_buf_file
        .write(b"(func(export\"main\")\ni32.const 0\ni32.const 7\ncall $woker_in\n")
        .unwrap();
    let mut fungsi: Fungsi = Fungsi {
        nama: "main".to_string(),
        arg: Vec::new(),
        kembali: Vec::new(),
        alokasi: Vec::new(),
    };
    loop {
        match *terima.recv().unwrap() {
            crate::parsing::parse_3_1::Pohon::_let(var) => {
                fungsi._let(&mut buf,&mut fungsi_buf_file,&mut perpus, &mut alokasi, var)
            }
            crate::parsing::parse_3_1::Pohon::cetak(nilai) => {
                perpus.cetak(nilai, &mut fungsi_buf_file, &mut buf)
            }
            crate::parsing::parse_3_1::Pohon::tambah_fungsi(nama) => {}
            crate::parsing::parse_3_1::Pohon::selesai => break,
            crate::parsing::parse_3_1::Pohon::kopilasi_error => return,
            crate::parsing::parse_3_1::Pohon::blok(nama) => {}
            crate::parsing::parse_3_1::Pohon::blok_ => {}
            _ => {
                panic!()
            }
        }
    }
    buf.clear();
    buf.push_str(path.as_str());
    buf.push_str("\\parsing\\pwa\\main.wat");
    let mut wat_file = File::create(&buf).unwrap();
    wat_file.write(b"(module\n").unwrap();
    drop(alokasi);
    fungsi.drop(&mut wat_file);
    perpus.drop(&mut wat_file, path, &mut buf);
    { wat_file }.write(b")").unwrap();
    //wasm
    std::fs::write(
        format!("{}\\target\\debug\\pwa\\main.wasm", path),
    )
    .unwrap();
    //sw.js
    std::fs::write(
        format!("{}\\target\\debug\\pwa\\sw.js", path)
        ,b"this.addEventListener('install',(e)=>{console.log('Installed service worker');e.waitUntil(caches.open('static').then((c)=>{c.addAll(['./','./index.html','./main.wasm','./aset/icon192.png','./aset/icon192.png'])}))});self.addEventListener('activate',()=>console.log('SW Activated'));self.addEventListener('fetch',(e)=>e.respondWith(caches.match(e.request).then((r)=>r|fetch(e.request))));"
    ).unwrap();
    //manifes
    if let Some(manifest) = manifest {
    } else {
        //manifes bawaan
        std::fs::write(
            format!("{}\\target\\debug\\pwa\\manifest.json", path),
            b"{\"name\":\"app\",\"short-name\":\"app\",\"start_url\":\"./\",\"scope\":\".\",\"display\":\"standalone\",\"background_color\":\"#000000\",\"theme_color\":\"#000000\",\"description\":\"app saya\",\"icons\":\"{{\"src\":\"aset/icon192.png\",\"type\"\"image/png\",\"size\":\"192x192\"}{\"src\":\"aset/icon192.png\",\"type\"\"image/png\",\"size\":\"512x512\"}}\"}"
        ).unwrap();
        //icon

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

/* versi lama
pub fn konversi(
    terima: std::sync::mpsc::Receiver<crate::parsing::parse_3_1::Pohon>,
    path: &String,
) {
    let mut perpus = lib {
        data: String::from(""),
        cetak: false,
        cetak_data: false,
    };
    let mut manifes :Option<Manifest> = None;
    let mut buf = String::with_capacity(100);
    let mut v = terima.recv().unwrap();
    let mut js = if let Ok(ok) = std::fs::File::create(format!("{}\\parsing\\pwa\\main.html", path)){
        ok
    } else{
        std::fs::create_dir_all(format!("{}\\parsing\\pwa", path)).unwrap();
        std::fs::File::create(format!("{}\\parsing\\pwa\\main.html", path)).unwrap()
    };
    let mut wat = std::fs::File::create(format!("{}\\parsing\\pwa\\main.wat", path)).unwrap();
    let mut data = std::fs::File::create(format!("{}\\parsing\\pwa\\data", path)).unwrap();
    let mut instruksi =
        if let Ok(ok) = std::fs::File::create(format!("{}\\parsing\\pwa\\instruksi\\main", path)) {
            ok
        } else {
            std::fs::create_dir_all(format!("{}\\parsing\\pwa\\instruksi", path)).unwrap();
            std::fs::File::create(format!("{}\\parsing\\pwa\\instruksi\\main", path)).unwrap()
        };
    let mut fungsi = [Fungsi {
        nama: "main".to_string(),
        arg: Vec::new(),
        kembali: Vec::new(),
        alokasi: Vec::new(),
    }]
    .to_vec();
    //wat.write(b"(module\n(import\n\"import\"\"woker_in\"(func $woker_in(param i32 i32)))\n(import\"import\"\"mem\"(memory 1))\n").unwrap();
    wat.write(b"(module\n(import\"import\"\"woker_in\"(func $woker_in(param i32 i32)))\n")
        .unwrap();
    js.write(format!("<!DOCTYPE html><head><meta name=\"viewport\" content=\"width=device-width\" /><link rel=\"apple-touch-icon\"href=\"{}\"/><link rel=\"manifest\"href=\"{}\"/><script>","aset/icon512.png","./manifest.json").as_bytes()).unwrap();
    js.write(b"if('serviceWorker' in navigator){const o_ = {import: {")
        .unwrap();
    'luar: loop {
        match v {
            crate::parsing::parse_3_1::Pohon::_let(var) => {
                match var.tipe {
                    crate::parsing::Tipe::_u8(nilai) => {}
                    crate::parsing::Tipe::_string(str_) => match str_ {
                        crate::parsing::Str::Some(konst) => {}
                        _ => {}
                    },
                    _ => {}
                }
                //panic!()
            }
            crate::parsing::parse_3_1::Pohon::cetak(nilai) => match nilai {
                crate::parsing::parse_3_1::Nilai::langsung_int(int) => {
                    instruksi
                        .write({ format!("i32.const {}\ncall $log\n", int) }.as_bytes())
                        .unwrap();
                    perpus.cetak = true;
                }
                _ => {}
            },
            crate::parsing::parse_3_1::Pohon::fungsi(nama) => {}
            crate::parsing::parse_3_1::Pohon::tambah_fungsi(nama) => fungsi.push(Fungsi {
                nama: nama,
                arg: Vec::new(),
                kembali: Vec::new(),
                alokasi: Vec::new(),
            }),
            crate::parsing::parse_3_1::Pohon::selesai => {
                loop {
                    match terima.recv().unwrap() {
                        crate::parsing::parse_3_1::Pohon::_let(var) => {}
                        crate::parsing::parse_3_1::Pohon::selesai => {
                            v = terima.recv().unwrap();
                            //jika selesai dua kali maka app sudah jadi
                            if let crate::parsing::parse_3_1::Pohon::selesai = v {
                                break 'luar;
                            }
                            continue 'luar;
                        }
                        _ => {
                            panic!()
                        }
                    }
                }
            }

            crate::parsing::parse_3_1::Pohon::kopilasi_error => return,
            _ => {}
        }
        v = terima.recv().unwrap();
    }
    let mem = if perpus.data {
        panic!();
        //
        buf.clear();
        wat.write(format!("(import\"import\"\"mem\"(memory {}))\n", 1).as_bytes())
            .unwrap();
    } else {
        wat.write(
            b"(import\"import\"\"mem\"(memory 1))\n(data(i32.const 0)\"./sw.jstestingfn\")\n",
        )
        .unwrap();
        1
    };
    if perpus.cetak {
        wat.write(b"(import\"import\"\"log\"(func $log(param i32)))\n")
            .unwrap();
        js.write(b"log:i=>console.log(i),").unwrap();
    }
    js.write(format!("mem:new WebAssembly.Memory({{initial:{}}}),woker_in: (o, l) =>navigator.serviceWorker.register(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer, o, l))).then((r) => console.log('Service worker registration succeeded:', r),(e) => console.log('Service worker registration failed:', e)),}},}};fetch('main.wasm').then((r) => r.arrayBuffer()).then((b) => WebAssembly.instantiate(b, o_)).then((r) => {{r.instance.exports.main()}}).catch(console.error)",mem).as_bytes()).unwrap();
    for i in 1..fungsi.len() {
        wat.write(format!("(func ${} ", fungsi[i].nama).as_bytes())
            .unwrap();
        for i in &fungsi[i].arg {
            wat.write(format!("(param ${} i32)", i).as_bytes()).unwrap();
        }
        if !fungsi[i].kembali.is_empty() {
            wat.write(format!("(result ").as_bytes()).unwrap();
            for i in &fungsi[i].kembali {
                wat.write(format!("{}", i).as_bytes()).unwrap();
            }
            wat.write(format!(")\n").as_bytes()).unwrap();
        }
        for i in &fungsi[i].alokasi {
            wat.write(format!("(local ${} i32)\n", i).as_bytes())
                .unwrap();
        }
        if let Ok(mut f) = std::fs::File::open(format!(
            "{}\\parsing\\pwa\\instruksi\\{}",
            path, fungsi[i].nama
        )) {
            buf.clear();
            f.read_to_string(&mut buf).unwrap();
            wat.write(buf.as_bytes()).unwrap();
        } else {
            return;
        }
        wat.write(b")\n").unwrap();
    }
    //main
    if let Ok(mut f) = std::fs::File::open(format!("{}\\parsing\\pwa\\instruksi\\main", path)) {
        buf.clear();
        buf.push_str("(func(export\"main\")\n");
        for i in &fungsi[0].alokasi {
            buf.push_str(&format!("(local ${} i32)\n", i));
        }
        buf.push_str("i32.const 0\ni32.const 7\ncall $woker_in\n");
        f.read_to_string(&mut buf ).unwrap();
        buf.push_str("))\n");
        wat.write(buf.as_bytes()).unwrap();
    } else {
        return;
    }
    js.write(b"}else{console.log('Service workers are not supported.')}</script></head>")
        .unwrap();
    drop(js);
    drop(wat);
    std::io::copy(
        &mut std::fs::File::open(format!("{}\\parsing\\pwa\\main.html", path)).unwrap(),
        &mut if let Ok(ok) = std::fs::File::create(format!("{}\\target\\debug\\pwa\\index.html", path)){
            ok
        }else{
            std::fs::create_dir_all(format!("{}\\target\\debug\\pwa", path)).unwrap();
            std::fs::File::create(format!("{}\\target\\debug\\pwa\\index.html", path)).unwrap()
        },
    )
    .unwrap();
    //kompilasi wat ke wasm
    std::fs::write(
        format!("{}\\target\\debug\\pwa\\main.wasm", path),
        wat::parse_file(format!("{}\\parsing\\pwa\\main.wat", path)).unwrap(),
    )
    .unwrap();
    std::fs::write(
        format!("{}\\parsing\\pwa\\sw.js", path),
        b"this.addEventListener('install',(e)=>{console.log('Installed service worker');e.waitUntil(caches.open('static').then((c)=>{c.addAll(['./','./index.html','./main.wasm','./aset/icon192.png','./aset/icon192.png'])}))});self.addEventListener('activate',()=>console.log('SW Activated'));self.addEventListener('fetch',(e)=>e.respondWith(caches.match(e.request).then((r)=>r|fetch(e.request))));",
    ).unwrap();
    if let Some(_kastum_manifes) = manifes {

    } else {
        //manifes bawaan
        std::fs::write(
            format!("{}\\target\\debug\\pwa\\manifest.json", path),
            b"{\"name\":\"app\",\"short-name\":\"app\",\"start_url\":\"./\",\"scope\":\".\",\"display\":\"standalone\",\"background_color\":\"#000000\",\"theme_color\":\"#000000\",\"description\":\"app saya\",\"icons\":\"{{\"src\":\"aset/icon192.png\",\"type\"\"image/png\",\"size\":\"192x192\"}{\"src\":\"aset/icon192.png\",\"type\"\"image/png\",\"size\":\"512x512\"}}\"}"
        ).unwrap();
        //icon
        if let Err(_) = std::fs::write(
            format!("{}\\aset\\sancaicon192.png", path),
            Asset::get("prefix/sancaicon192.png").unwrap().data
        ) {
            std::fs::create_dir_all(format!("{}\\aset", path),).unwrap();
            std::fs::write(
                format!("{}\\aset\\sancaicon192.png", path),
                Asset::get("prefix/sancaicon192.png").unwrap().data
            ).unwrap();
        }
        std::fs::write(
            format!("{}\\aset\\sancaicon512.png", path),
            Asset::get("prefix/sancaicon512.png").unwrap().data
        ).unwrap();
    }
}
*/
