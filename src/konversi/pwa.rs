use crate::parsing::parse_3::{Nilai, Pohon, Variabel};
use crate::parsing::Tipe;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;

struct manifest {
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
struct AppObjek {
    manifest: manifest,
    offset: usize,
    main_selesai: bool,
    alokasi_mem : Vec<(String,String)>,
    import: (
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
    ),
    chace: Vec<String>,
    var_: HashMap<u64, Tipe>,
    data: std::fs::File,
    was_main: std::io::BufWriter<std::fs::File>,
    html: std::io::BufWriter<std::fs::File>,
    swj: std::io::BufWriter<std::fs::File>,
    wat: std::io::BufWriter<std::fs::File>,
    lib: std::io::BufWriter<std::fs::File>,
}
impl AppObjek {
    fn awal(&mut self) {
        self.tambah_data(&"./sw.js".to_string());
        self.html.write(b"<!DOCTYPE html><head><meta name=\"viewport\"content=\"width=device-width\"><link rel=\"apple-touch-icon\" href=\"aset/icon512.png\"><link rel=\"manifest\"href=\"./manifest.json\"><script>if('serviceWorker' in navigator){").unwrap();
    }
    fn akhir_(mut self, path: &String) -> (std::io::BufWriter<std::fs::File>,std::vec::Vec<(String,String)> ) {
        //self.html.write(b"}else{}</script>").unwrap();
        std::fs::write(format!("{}\\target\\debug\\pwa\\manifest.json", path), {format!("{{\"name\":{:?},\"short-name\":{:?},\"start_url\":{:?},\"scope\":\".\",\"display\":{:?},\"background_color\":{:?},\"theme_color\":{:?},\"description\":{:?},\"icons\":{:?}}}",self.manifest.nama,self.manifest.nama_pendek,self.manifest.url_mulai,self.manifest.display,self.manifest.warna_latarbelakan,self.manifest.warna_tema,self.manifest.description,{
            let mut v = "{".to_string();
            if self.manifest.icons.is_empty() {
                std::fs::write(
                    format!("{}\\target\\debug\\pwa\\aset\\icon192.png", path),
                    crate::dok::Asset::get("prefix/sancaicon192.png").unwrap(),
                )
                .unwrap();
                std::fs::write(
                    format!("{}\\target\\debug\\pwa\\aset\\icon512.png", path),
                    crate::dok::Asset::get("prefix/sancaicon512.png").unwrap(),
                )
                .unwrap();
                /*
                v.push_str("{\"src\":\"aset\\icon192.png\",\"type\"image/png\"\",\"size\":\"192x192\"},{\"src\":\"aset\\icon512.png\",\"type\"image/png\",\"size\":\"512x512\"}}");
                */
                self.manifest.icons.push(
                    ["aset/icon192.png".to_string(),"image/png".to_string(),"192x192".to_string()]
                );
                self.manifest.icons.push(
                    ["aset/icon192.png".to_string(),"image/png".to_string(),"512x512".to_string()]
                );

            } 
                self.swj.write(b"this.addEventListener('install',(e)=>{console.log('Installed service worker');e.waitUntil(caches.open('static').then((c)=>{c.addAll(['./','./index.html','./main.wasm'").unwrap();
                for i in self.manifest.icons {
                    v.push_str(
                        &format!("{{\"src\":\"{}\",\"type\"\"{}\",\"size\":\"{}\"}}",
                        i[0],i[1],i[2]
                        )
                    );
                    self.swj.write(
                        format!(",'./{}'",i[0]).as_bytes()
                    ).unwrap();
                }
                v.push('}');
            
            v
        })}.as_bytes()).unwrap();
        for i in &self.chace {
            let mut x = String::from(",'");
            x.push_str(i.clone().as_str());
            x.push('\'');
            self.swj.write(x.as_bytes()).unwrap();
        }
        //
        self.swj.write(b"])}))});self.addEventListener('activate',()=>console.log('SW Activated'));self.addEventListener('fetch',(e)=>e.respondWith(caches.match(e.request).then((r)=>r|fetch(e.request))));").unwrap();
        //
        self.html.write({format!("const o_={{import:{{mem: new WebAssembly.Memory({{initial:{}}}),woker_in:(o, l)=>navigator.serviceWorker.register(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))).then((r)=>console.log('Service worker registration succeeded:', r),(e)=>console.log('Service worker registration failed:',e))",1,)}.as_bytes()).unwrap();
        //lib
        self.wat.write(b"(module\n(import\"import\"\"woker_in\"(func $woker_in(param i32 i32)))\n(import\"import\"\"mem\"(memory 1))\n").unwrap();
        self.wat.write({format!("(data(i32.const 0)\"{}\")\n",{
            //self.data.flush().unwrap();
            self.data = std::fs::File::open(format!("{}\\parsing\\pwa\\data", path)).unwrap();
            let mut buf = String::with_capacity(30);
            self.data.read_to_string(&mut buf).unwrap();
            buf
        })}.as_bytes()).unwrap();
        if self.import.0 {
            self.html.write(b",\"log\":(o,l)=>console.log(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l)))").unwrap();
            self.wat.write(b"(import\"import\"\"log\"(func $log(param i32 i32)))\n")
            .unwrap();
        }
        if self.import.1 {
            self.html.write(b",\"log_\":(o)=>console.log(o)").unwrap();
            self.wat.write(b"(import\"import\"\"log_\"(func $log_(param i32)))\n")
            .unwrap();
        }
        if self.import.2 {
            self.html.write(b",\"body\":(o,l)=>{document.write(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l)))}").unwrap();
            self.wat.write(b"(import\"import\"\"body\"(func $body(param i32 i32)))\n")
            .unwrap();
        }
        if self.import.3 {
            self.html.write(b",\"body_\":(o)=>{document.write(o)}").unwrap();
            self.wat.write(b"(import\"import\"\"body_\"(func $body_(param i32)))\n")
            .unwrap();
        }
        if self.import.4 {
            self.html.write(b",\"warnalatarbelakang\":(o,l)=>{document.body.style.backgroundColor=new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))}").unwrap();
        }
        if self.import.5 {
            self.html.write(b",\"gambarlatarbelakang\":(o,l)=>{document.body.style.backgroundImage=new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))}").unwrap();
        }
        if self.import.6 {
            self.html.write(b",\"judul\":(o,l)=>{document.title=new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))}").unwrap();
        }
        if self.import.7 {
            self.html.write(b",\"tombol\":(o,l)=>{let v =document.createElement(\"button\");v.id =new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l));document.body.appendChild(v)}").unwrap();
        }
        if self.import.8 {
            self.html.write(b",\"isi\":(o,l,x,y)=>{document.getElementById(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))).innerHTML =new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,x,y))}").unwrap();
        }
        if self.import.9 {
            self.html.write(b",\"warna\":(o,l,x,y)=>{document.getElementById(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))).style.color =new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,x,y))}").unwrap();
        }
        if self.import.10 {
            self.html.write(b",\"warnalatarbelakangid\":(o,l,x,y)=>{document.getElementById(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))).style.backgroundColor =new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,x,y))}").unwrap();
        }
        if self.import.11 {
            self.html.write(b",\"ukurankata\":(o,l,x,y)=>{document.getElementById(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))).style.fontSize =new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,x,y))}").unwrap();
        }
        if self.import.12 {
            self.html.write(b",\"kelas\":(o,l,x,y)=>{document.getElementById(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))).classList.add(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,x,y)))}").unwrap();
        }
        if self.import.13 {
            self.html.write(b",\"str_sama_str\":(o,l,x,y)=>{return new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l)===new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,x,y)?1:0})))}").unwrap();
        }
        /*
        //main
        self.wat.write(b"(func(export\"main\")\n").unwrap();

        self.wat.write(b"\n)\n)").unwrap();
        //
        */
        self.html.write(b"}};fetch('main.wasm').then((r)=>r.arrayBuffer()).then((b)=>WebAssembly.instantiate(b,o_)).then((r)=>{r.instance.exports.main();").unwrap();
        //iven
        //
        self.html.write(b"}).catch(console.error)}else{console.log('Service workers are not supported.')}</script>").unwrap();
        //
        (self.wat,self.alokasi_mem)
    }
    /*
    fn akhir(
        mut self,
        path: &String,
        manifes: &mut manifest,
    ) -> (
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
    ) {
        self.swj.write(b"this.addEventListener('install',(e)=>{console.log('Installed service worker');e.waitUntil(caches.open('static').then((c)=>{c.addAll(['./','./index.html','./main.wasm'").unwrap();
        //chace
        if manifes.icons.is_empty() {
            std::fs::write(format!("{}\\target\\debug\\pwa\\aset\\sancaicon192.png", path),crate::dok::Asset::get("prefix/sancaicon192.png").unwrap() /*include_bytes!(".\\sancaicon192.png")*/).unwrap();
            std::fs::write(
                format!("{}\\target\\debug\\pwa\\aset\\sancaicon512.png", path),
                crate::dok::Asset::get("prefix/sancaicon512.png").unwrap(),
            )
            .unwrap();
            manifes.icons.push([
                "./aset/sancaicon192.png".to_string(),
                "image/png".to_string(),
                "192x192".to_string(),
            ]);
            manifes.icons.push([
                "./aset/sancaicon512.png".to_string(),
                "image/png".to_string(),
                "512x512".to_string(),
            ]);
        }
        manifes.icons.iter().for_each(|i| {
            let mut x = String::from(",'");
            x.push_str(i[0].clone().as_str());
            x.push('\'');
            self.swj.write(x.as_bytes()).unwrap();
        });
        for i in &self.chace {
            let mut x = String::from(",'");
            x.push_str(i.clone().as_str());
            x.push('\'');
            self.swj.write(x.as_bytes()).unwrap();
        }
        //
        self.swj.write(b"])}))});self.addEventListener('activate',()=>console.log('SW Activated')});self.addEventListener('fetch',(e)=>{e.respondWith(caches.match(e.request).then((r)=>r|fetch(e.request))));").unwrap();
        //
        self.html.write({format!("const o_={{import:{{mem: new WebAssembly.Memory({{initial:{}}}),woker_in:(o, l)=>navigator.serviceWorker.register(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))).then((r)=>console.log('Service worker registration succeeded:', r),(e)=>console.log('Service worker registration failed:',e))",1,)}.as_bytes()).unwrap();
        //lib
        if self.import.0 {
            self.html.write(b",\"log\":(o,l)=>console.log(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l)))").unwrap();
            
        }
        if self.import.1 {
            self.html.write(b",\"log_\":(o)=>console.log(o)").unwrap();
            
        }
        if self.import.2 {
            self.html.write(b",\"body\":(o,l)=>{document.write(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l)))}").unwrap();
            
        }
        if self.import.3 {
            self.html.write(b",\"warnalatarbelakang\":(o,l)=>{document.body.style.backgroundColor=new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))}").unwrap();
        }
        if self.import.4 {
            self.html.write(b",\"gambarlatarbelakang\":(o,l)=>{document.body.style.backgroundImage=new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))}").unwrap();
        }
        if self.import.5 {
            self.html.write(b",\"judul\":(o,l)=>{document.title=new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))}").unwrap();
        }
        if self.import.6 {
            self.html.write(b",\"tombol\":(o,l)=>{let v =document.createElement(\"button\");v.id =new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l));document.body.appendChild(v)}").unwrap();
        }
        if self.import.7 {
            self.html.write(b",\"isi\":(o,l,x,y)=>{document.getElementById(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))).innerHTML =new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,x,y))}").unwrap();
        }
        if self.import.8 {
            self.html.write(b",\"warna\":(o,l,x,y)=>{document.getElementById(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))).style.color =new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,x,y))}").unwrap();
        }
        if self.import.9 {
            self.html.write(b",\"warnalatarbelakangid\":(o,l,x,y)=>{document.getElementById(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))).style.backgroundColor =new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,x,y))}").unwrap();
        }
        if self.import.10 {
            self.html.write(b",\"ukurankata\":(o,l,x,y)=>{document.getElementById(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))).style.fontSize =new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,x,y))}").unwrap();
        }
        if self.import.11 {
            self.html.write(b",\"kelas\":(o,l,x,y)=>{document.getElementById(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l))).classList.add(new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,x,y)))}").unwrap();
        }
        if self.import.12 {
            self.html.write(b",\"str_sama_str\":(o,l,x,y)=>{return new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,o,l)===new TextDecoder('utf8').decode(new Uint8Array(o_.import.mem.buffer,x,y)?1:0})))}").unwrap();
        }
        self.wat.write(b"\n)").unwrap();
        //
        self.html.write(b"}};fetch('main.wasm').then((r)=>r.arrayBuffer()).then((b)=>WebAssembly.instantiate(b,o_)).then((r)=>{r.instance.exports.main();").unwrap();
        //iven

        self.html.write(b"}).catch(console.error)}else{console.log('Service workers are not supported.')}</script>").unwrap();
        //
        std::fs::write(format!("{}\\target\\debug\\pwa\\manifest.json", path), format!("{{\"name\":\"{}\",\"short_name\":\"{}\",\"background_color\":\"{}\",\"theme_color\":\"{}\",\"display\":\"{}\",\"start_url\":\"{}\",\"scope\": \".\",\"description\":\"{}\",\"icons\":[{}]}}",manifes.nama,manifes.nama_pendek,manifes.warna_latarbelakan,manifes.warna_tema,manifes.display,manifes.url_mulai,manifes.description,{
            let mut v = String::with_capacity(400);
            if !manifes.icons.is_empty(){
                let mut x = false;
                for i in &manifes.icons{
                    v.push_str({format!("{}{{\"src\":\"{}\",\"type\":\"{}\",\"sizes\":\"{}\"}}",if x {","} else {x = true;""},i[0].clone(),i[1].clone(),i[2].clone())}.as_str())
                }
            }
            v
        })).unwrap();
        self.import
    }
    */
    fn tambah_data(&mut self, kata: &String) -> usize {
        self.data.write(kata.as_bytes()).unwrap();
        let v = self.offset.clone();
        self.offset += kata.len();
        v
    }
    fn tambah_var(&mut self, data: &Tipe, id: u64) {
        match data {
            Tipe::_string(data) => {
                match data {
                    crate::parsing::Str::Some(kata)=>{
                        let x = self.tambah_data(&kata);
                        self.var_
                            .insert(id, Tipe::_string(crate::parsing::Str::arr([x, kata.len()])));
                    }
                    crate::parsing::Str::arr(o)=>{

                    }
                    crate::parsing::Str::None=>{}
                }
                /*
                if let crate::parsing::Str::Some(kata) = data {
                    let x = self.tambah_data(&kata);
                    self.var_
                        .insert(id, Tipe::_string(crate::parsing::Str::arr([x, kata.len()])));
                } else {
                    self.var_
                        .insert(id, Tipe::_string(crate::parsing::Str::None));
                }
                */
            }
            Tipe::_u8(data_) => {
                if let Some(data_) = data_ {
                    self.was_main.write(
                        format!("i32.const {:?}\nset_local $_{:?}\n",data_,id).as_bytes()
                    ).unwrap();
                    self.alokasi_mem.push((
                        "i32".to_string(),
                        id.to_string()
                    ));
                    self.var_
                            .insert(id, Tipe::_u8(Some(*data_)));
                } else {

                }
                /*
                self.var_
                    .insert(id, Tipe::_u8(if let Some(_) = data { *data } else { None }));
                */
                }
            Tipe::nomer(data) => {}
            _ => {}
        }
        //self.var_.insert(id,data);
    }
    /*
    fn tambah_data_var(&mut self, kata: &String, id: u64) {
        //use std::convert::TryInto;
        //let x = self.tambah_data(kata).try_into().unwrap();
    }
    */
    fn print_str(&mut self, kata: &String) {
        let off = self.tambah_data(kata);
        self.was_main
            .write(format!("i32.const {}\ni32.const {}\ncall $log\n", off, kata.len()).as_bytes())
            .unwrap();
        self.import.0 = true
    }
    /*
    fn halaman(&mut self, kata: &String) {
        let off = self.tambah_data(kata);
        self.was_main
            .write(format!("i32.const {}\ni32.const {}\ncall $body\n", off, kata.len()).as_bytes())
            .unwrap();
        self.import.2 = true
        //panic!()
    }
    fn halaman_penujuk(&mut self, data: [usize; 2]) {
        
    }
    */
    fn halaman_(&mut self , in_:&crate::parsing::parse_3::Nilai) {
        match in_ {
            crate::parsing::parse_3::Nilai::lansung_str(kata)=>{
                let off = self.tambah_data(kata);
                self.was_main
                    .write(format!("i32.const {}\ni32.const {}\ncall $body\n", off, kata.len()).as_bytes())
                    .unwrap();
                self.import.2 = true
            }
            crate::parsing::parse_3::Nilai::penujuk_(k)=>{
                if let Some(o) = self.var_.get(k) {
                    match o {
                        crate::parsing::Tipe::_string(o)=>{
                            match o {
                                crate::parsing::Str::arr(data)=>{
                                    self.was_main
                                        .write(format!("i32.const {}\ni32.const {}\ncall $body\n", data[0], data[1]).as_bytes())
                                        .unwrap();
                                    self.import.2 = true;
                                    //app.halaman_penujuk(*o);
                                }
                                _=>{}
                            }
                        }
                        crate::parsing::Tipe::_u8(o)=>{
                            if let Some(_) = o {
                                self.was_main
                                    .write(
                                        format!("local.get $_{}\ncall $body_\n",k).as_bytes()
                                    ).unwrap();
                                    self.import.3 = true;
                            } else {

                            }
                        }
                        _=>{}
                    }
                } else {
                    panic!()
                }
            }
            _=>{}
        }
    }
    fn tambah_ke_stak(&mut self,nama:&crate::parsing::Tipe,konst:bool) {
        match nama {
            Tipe::_i32(nomer) =>{
                if let Some(nomer) = nomer {
                    self.was_main.write(
                        format!("local ${:?} i32\n(local.set ${:?} i32.const {})\n",nama,nama,nomer).as_bytes()
                    ).unwrap();
                } else {
                    self.was_main.write(
                        format!("local ${:?} i32\n",nama).as_bytes()
                    ).unwrap();
                }
            }
            _=>{panic!()}
        }
    }
    fn set_stak(&mut self,nama:&crate::parsing::Tipe) {
        match nama {
            Tipe::_i32(nomer) =>{
                if let Some(nomer) = nomer {
                    self.was_main.write(
                        format!("(local.set ${:?} i32.const {})\n",nama,nomer).as_bytes()
                    ).unwrap();
                } else {
                    panic!()
                }
            }
            _=>{panic!()}
        }
    }
    fn tambah_fn(&mut self, nama: &String, path: &String) {
        if !self.main_selesai {
            self.was_main = std::io::BufWriter::new(
                match std::fs::File::create(format!("{}\\parsing\\pwa\\lib", path)) {
                    Ok(o) => o,
                    Err(_) => {
                        std::fs::create_dir_all(format!("{}\\parsing\\pwa", path)).unwrap();
                        std::fs::File::create(format!("{}\\parsing\\pwa\\lib", path)).unwrap()
                    }
                },
            );
            self.main_selesai = true;
        }
        self.was_main
            .write({ format!(":fn:{}\n", nama) }.as_bytes())
            .unwrap();
    }
}
pub fn app(pohon: &std::vec::Vec<Pohon>, path: &String, nama: String) -> crate::error::ErrorKode {
    let mut app = AppObjek {
        manifest: manifest {
            nama: nama.clone(),
            nama_pendek: nama,
            warna_latarbelakan: "#000000".to_string(),
            warna_tema: "#000000".to_string(),
            display: "standalone".to_string(),
            description: "app saya".to_string(),
            url_mulai: "./".to_string(),
            icons: Vec::with_capacity(3),
            screenshots: Vec::with_capacity(3),
        },
        offset: 0,
        main_selesai: false,
        alokasi_mem : Vec::new(),
        import: (
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false
        ),
        chace: Vec::with_capacity(2),
        var_: HashMap::with_capacity(3),
        data: match std::fs::File::with_options().read(true).write(true).create(true).open(format!("{}\\parsing\\pwa\\data", path)) {
            Ok(o) => o,
            Err(_) => {
                std::fs::create_dir_all(format!("{}\\parsing\\pwa", path)).unwrap();
                std::fs::File::create(format!("{}\\parsing\\pwa\\data", path)).unwrap()
            }
        },
        was_main: std::io::BufWriter::new(
            match std::fs::File::create(format!("{}\\parsing\\pwa\\main", path)) {
                Ok(o) => o,
                Err(_) => {
                    std::fs::create_dir_all(format!("{}\\parsing\\pwa", path)).unwrap();
                    std::fs::File::create(format!("{}\\parsing\\pwa\\main", path)).unwrap()
                }
            },
        ),
        html: std::io::BufWriter::new(
            match std::fs::File::create(format!("{}\\target\\debug\\pwa\\index.html", path)) {
                Ok(o) => o,
                Err(_) => {
                    std::fs::create_dir_all(format!("{}\\target\\debug\\pwa\\aset", path)).unwrap();
                    std::fs::File::create(format!("{}\\target\\debug\\pwa\\index.html", path))
                        .unwrap()
                }
            },
        ),
        swj: std::io::BufWriter::new(
            match std::fs::File::create(format!("{}\\target\\debug\\pwa\\sw.js", path)) {
                Ok(o) => o,
                Err(_) => {
                    std::fs::File::create(format!("{}\\target\\debug\\pwa\\sw.js", path)).unwrap()
                }
            },
        ),
        wat: std::io::BufWriter::new(
            std::fs::File::create(format!("{}\\parsing\\pwa\\main.wat", path)).unwrap()
        ),
        lib: std::io::BufWriter::new(
            std::fs::File::create(format!("{}\\parsing\\pwa\\lib", path)).unwrap()
        )
    };
    app.awal();
    for i in pohon {
        match i {
            Pohon::_let(id,data,_,_) =>{
                app.tambah_var(&data, *id );
            }
            Pohon::halaman(o)=>{
                app.halaman_(o);
            }
            _ => {}
        }
    }
    use std::io::BufRead;
    let (mut wat,alokasi_mem) = app.akhir_(path) ;
    let mut _main = std::io::BufReader::new( std::fs::File::open( format!("{}\\parsing\\pwa\\main", path) ).unwrap());
    let mut _data = std::io::BufReader::new( std::fs::File::open( format!("{}\\parsing\\pwa\\data", path) ).unwrap());
    let mut _lib = std::io::BufReader::new( std::fs::File::open( format!("{}\\parsing\\pwa\\lib", path) ).unwrap());
    let mut buf = String::with_capacity(100);
    //lib
    while _lib.read_line(&mut buf ).unwrap() != 0 {
        buf.clear()
    }
    //main
    wat.write(b"(func(export\"main\")\n").unwrap();
    for i in alokasi_mem {
        wat.write(
            {format!("(local $_{} {})\n",i.1,i.0)}.as_bytes()
        ).unwrap();
    }
    wat.write(b"i32.const 0\ni32.const 7\ncall $woker_in\n").unwrap();
    while _main.read_line(&mut buf ).unwrap() != 0 {
        wat.write(buf.as_bytes()).unwrap();
        buf.clear()
    }
    {wat}.write(b"\n)\n)").unwrap();
    drop((_main,_data,_lib));
    std::fs::write(
        format!("{}\\target\\debug\\pwa\\main.wasm", path),
        wat::parse_file(format!("{}\\parsing\\pwa\\main.wat", path)).unwrap(),
    ).unwrap();
    crate::error::ErrorKode::Oke
}
/*
pub fn app(pohon: &std::vec::Vec<Pohon>, path: &String, nama: String) -> crate::error::ErrorKode {
    let mut manifes = manifest {
        nama: &nama,
        nama_pendek: &nama,
        warna_latarbelakan: "#000000".to_string(),
        warna_tema: "#000000".to_string(),
        display: "standalone".to_string(),
        description: "app saya".to_string(),
        url_mulai: "./".to_string(),
        icons: Vec::with_capacity(3),
        screenshots: [
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ],
    };
    /*
    let mut lib = std::io::BufWriter::new(
        match std::fs::File::create(format!("{}\\target\\parsing\\pwa\\lib", path)) {
            Ok(o) => o,
            Err(_) => {
                std::fs::create_dir_all(format!("{}\\target\\parsing\\pwa", path)).unwrap();
                std::fs::File::create(format!("{}\\target\\parsing\\pwa\\lib", path)).unwrap()
            }
        },
    );
    */
    let mut appobjek = AppObjek {
        offset: 0,
        main_selesai: false,
        import: (
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false,
        ),
        chace: Vec::with_capacity(2),
        var_: HashMap::with_capacity(3),
        data: std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(format!("{}\\parsing\\pwa\\data", path))
            .unwrap(),
        was_main: std::io::BufWriter::new(
            match std::fs::File::create(format!("{}\\parsing\\pwa\\main", path)) {
                Ok(o) => o,
                Err(_) => {
                    std::fs::create_dir_all(format!("{}\\parsing\\pwa", path)).unwrap();
                    std::fs::File::create(format!("{}\\parsing\\pwa\\main", path)).unwrap()
                }
            },
        ),
        html: std::io::BufWriter::new(
            match std::fs::File::create(format!("{}\\target\\debug\\pwa\\index.html", path)) {
                Ok(o) => o,
                Err(_) => {
                    std::fs::create_dir_all(format!("{}\\target\\debug\\pwa\\aset", path)).unwrap();
                    std::fs::File::create(format!("{}\\target\\debug\\pwa\\index.html", path))
                        .unwrap()
                }
            },
        ),
        swj: std::io::BufWriter::new(
            match std::fs::File::create(format!("{}\\target\\debug\\pwa\\sw.js", path)) {
                Ok(o) => o,
                Err(_) => {
                    std::fs::File::create(format!("{}\\target\\debug\\pwa\\sw.js", path)).unwrap()
                }
            },
        ),
    };
    appobjek.awal();
    //swj.write(b"const {dataCacheName,cacheName,filesToCache}='pwa-data','pwa-',['./','./index.html'").unwrap();
    for i in 0..pohon.len() {
        match &pohon[i] {
            Pohon::_let(id, tipe) => appobjek.tambah_var(&tipe, *id),
            //Pohon::konst_str(id,kata)=>appobjek.tambah_data_var(kata,*id),
            Pohon::cetak(o) => match o {
                Nilai::lansung_str(kata) => appobjek.print_str(&kata),
                Nilai::lansung_int(angka) => appobjek.print_str(&format!("{}", angka)),
                Nilai::lansung_float(angka) => appobjek.print_str(&format!("{}", angka)),
                Nilai::None => appobjek.print_str(&"\n".to_string()),
                _ => {
                    panic!()
                }
            },
            Pohon::halaman(nilai) => match nilai {
                Nilai::lansung_str(kata) => {
                    appobjek.halaman(&kata);
                }
                Nilai::penujuk_(var) => {
                    if let Some(o) = appobjek.var_.get(&var) {
                        if let Tipe::_string(o) = o {
                            match o {
                                crate::parsing::Str::arr(o) => appobjek.halaman_penujuk(*o),
                                crate::parsing::Str::None => {
                                    appobjek.halaman(&"\n".to_string());
                                }
                                _ => {
                                    panic!()
                                }
                            }
                        }
                    } else {
                        return crate::error::ErrorKode::ErrVariabelTidakDitemukan
                    }
                }
                Nilai::lansung_int(o) => {
                    appobjek.halaman(&format!("{}", o));
                }
                Nilai::lansung_float(o) => {
                    appobjek.halaman(&format!("{}", o));
                }
                Nilai::minta(_) => {
                    panic!()
                }
                Nilai::None => appobjek.halaman(&"\n".to_string()),
            },
            Pohon::fungsi(nama) => appobjek.tambah_fn(&nama, path),
            _ => {}
        }
    }
    let imp = appobjek.akhir(path, &mut manifes);
    let mut wat = std::io::BufWriter::new(
        std::fs::File::create(format!("{}\\parsing\\pwa\\main.wat", path)).unwrap(),
    );
    wat.write(b"(module\n(import\"import\"\"mem\"(memory 1))\n(import\"import\"\"woker_in\"(func $woker_in(param i32 i32)))\n(data(i32.const 0)\"").unwrap();
    wat.write(
        { std::fs::read_to_string(format!("{}\\parsing\\pwa\\data", path)).unwrap() + "\")\n" }
            .as_bytes(),
    )
    .unwrap();
    //lib
    if imp.0 {
        wat.write(b"(import\"import\"\"log\"(func $log(param i32 i32)))\n")
            .unwrap();
    }
    if imp.1 {
        wat.write(b"(import\"import\"\"log_\"(func $log_(param i32)))\n")
            .unwrap();
    }
    if imp.2 {
        wat.write(b"(import\"import\"\"body\"(func $body(param i32 i32)))\n")
            .unwrap();
    }
    //lib fn
    use std::io::BufRead;
    let mut fn_lib = std::io::BufReader::new(
        std::fs::File::open(format!("{}\\parsing\\pwa\\lib", path)).unwrap(),
    );
    let mut buf = String::with_capacity(100);
    if fn_lib.read_line(&mut buf).unwrap() != 0 {
        let mut bukan_pertama = false;
        loop {
            if !buf.starts_with(":fn:") {
                wat.write(buf.as_bytes()).unwrap();
            } else {
                let b: Vec<&str> = buf.split(':').collect();
                wat.write(
                    format!(
                        "{0}(func ${1}(export\"{1}\")\n",
                        if bukan_pertama {
                            ")\n".to_string()
                        } else {
                            bukan_pertama = true;
                            "".to_string()
                        },
                        { b[2].trim_end() }
                    )
                    .as_bytes(),
                )
                .unwrap();
            }
            buf.clear();
            if fn_lib.read_line(&mut buf).unwrap() == 0 {
                wat.write(b")\n").unwrap();
                break;
            }
        }
    }
    //main
    wat.write(b"(func(export\"main\")\ni32.const 0\ni32.const 7\ncall $woker_in\n")
        .unwrap();
    //read & drop
    { wat }
        .write(
            {
                std::fs::read_to_string(format!("{}\\parsing\\pwa\\main", path)).unwrap() + ")\n)\n"
            }
            .as_bytes(),
        )
        .unwrap();
    std::fs::write(
        format!("{}\\target\\debug\\pwa\\main.wasm", path),
        wat::parse_file(format!("{}\\parsing\\pwa\\main.wat", path)).unwrap(),
    )
    .unwrap();
    //
    crate::error::ErrorKode::Oke
}
*/
