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
struct lib {
    data: bool,
    cetak_data: bool,
    cetak: bool,
}
#[derive(Clone)]
struct Fungsi {
    nama: String,
    arg: Vec<String>,
    kembali: Vec<String>,
    alokasi: Vec<String>,
}
pub fn konversi(
    terima: std::sync::mpsc::Receiver<crate::parsing::parse_3_1::Pohon>,
    path: &String,
) {
    let mut perpus = lib {
        data: false,
        cetak: false,
        cetak_data: false,
    };
    let mut manifes :Option<Manifest> = None;
    let mut buf = Vec::with_capacity(100);
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
            f.read_to_end(&mut buf).unwrap();
            wat.write(&buf).unwrap();
        } else {
            return;
        }
        wat.write(b")\n").unwrap();
    }
    wat.write(b"(func(export\"main\")\n").unwrap();
    for i in &fungsi[0].alokasi {
        wat.write(format!("(local ${} i32)\n", i).as_bytes())
            .unwrap();
    }
    wat.write(b"i32.const 0\ni32.const 7\ncall $woker_in\n")
        .unwrap();
    if let Ok(mut f) = std::fs::File::open(format!("{}\\parsing\\pwa\\instruksi\\main", path)) {
        buf.clear();
        f.read_to_end(&mut buf).unwrap();
        wat.write(&buf).unwrap();
    } else {
        return;
    }
    wat.write(b"))\n").unwrap();
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
