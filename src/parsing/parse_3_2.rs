use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::Deref;
use std::sync::Mutex;
pub struct Derefex<T> {
    value: T,
}
impl<T> Deref for Derefex<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
struct App {
    fungsi: Vec<([String; 3], bool)>,
    id: u64,
    terima: std::sync::mpsc::Receiver<std::option::Option<serde_json::Value>>,
    kirim_pwa: std::sync::mpsc::Sender<crate::parsing::Pohon>,
    kirim_cpp: std::sync::mpsc::Sender<crate::parsing::Pohon>,
    kirim_json: std::sync::mpsc::Sender<([String; 3], bool)>,
    kom: (bool, bool,bool),
    blok_id: u64,
}
impl App {
    fn kirim(&mut self, nilai: crate::parsing::Pohon) {
        if self.kom.0 {panic!()}
        if self.kom.1 {
            self.kirim_pwa.send(nilai.clone()).unwrap();
        }
        if self.kom.2 {
            self.kirim_cpp.send(nilai).unwrap();
        }
    }
    fn var(
        &mut self,
        json: &serde_json::Value,
        var: &mut crate::parsing::Arrmap<String, Vec<crate::parsing::Let_>>,
    ) {
        /* contoh
        self.kirim_pwa
            .send(crate::parsing::Pohon::_let(Box::new(
                crate::parsing::Let_::baru(
                    &mut 0,
                    None,
                    false,
                    crate::parsing::Tipe::uint(8, vec![Some(0)]),
                    "".to_string(),
                    0,
                ),
            )))
            .unwrap();
        */
        let mut_ = match json["mut"].as_str().unwrap() {
            "i" => crate::parsing::mem_mode::imutabel,
            "m" => crate::parsing::mem_mode::mutabel,
            "s" => crate::parsing::mem_mode::statik,
            _ => {
                panic!()
            }
        };
        let bit;
        let tipe = if let serde_json::Value::Number(num) = &json["mem"]["uint"] {
            let mut v = Vec::new();
            if json["nilai"][0].as_u64().unwrap() == 1 {
                v.reserve_exact(1);
                let x = json["nilai"][1].as_u64().unwrap();
                //println!("-> {}", x);
                v.push(Some(x));
            } else {
                panic!()
            }
            bit = num.as_u64().unwrap();
            crate::parsing::Tipe::int(
                crate::parsing::mem_tipe::unint(Some(v)),
                num.as_u64().unwrap(),
            )
        } else if let serde_json::Value::Number(num) = &json["mem"]["int"] {
            let mut v = Vec::new();
            if json["nilai"][0].as_u64().unwrap() == 1 {
                v.reserve_exact(1);
                let x = json["nilai"][1].as_i64().unwrap();
                //println!("-> {}", x);
                v.push(Some(x));
            } else {
                panic!()
            }
            bit = num.as_u64().unwrap();

            crate::parsing::Tipe::int(
                crate::parsing::mem_tipe::int(Some(v)),
                num.as_u64().unwrap(),
            )
        } else if let serde_json::Value::Number(num) = &json["mem"]["float"] {
            let mut v = Vec::new();
            if json["nilai"][0].as_u64().unwrap() == 1 {
                v.reserve_exact(1);
                let x = json["nilai"][1].as_f64().unwrap();
                println!("-> {}", x);
                v.push(Some(x));
            } else {
                panic!()
            }
            bit = num.as_u64().unwrap();
            crate::parsing::Tipe::int(
                crate::parsing::mem_tipe::float(Some(v)),
                num.as_u64().unwrap(),
            )
        } else {
            panic!()
        };
        let nama_file = json["nama_file"].as_str().unwrap().to_string();
        let nomer_baris = json["nomer_baris"].as_u64().unwrap();
        //jika None maka akan langsung dikirim
        let _let: crate::parsing::Let_ =
            crate::parsing::Let_::baru(&mut self.id, mut_, false, tipe, nama_file, nomer_baris);
        let kunci = json["nama"].as_str().unwrap().to_string();
        //panic!();
        let mut v = Vec::with_capacity(1);
        v.push(_let);
        let v_len = v.len();
        var.insert(kunci.clone(), v);
        if v_len == 1 {
            self.kirim(crate::parsing::Pohon::Let(kunci, 0));
        } else {
            panic!()
        }

        /*
        if let Some(var) = var.get_mut(&kunci) {
            var.push(_let);
            if mut_ == crate::parsing::mem_mode::statik {
                let l = var.len() - 1;
                self.kirim(crate::parsing::Pohon::Let(kunci, l))
            }
        } else {
            let mut v = Vec::with_capacity(1);
            v.push(_let);
            //println!("-> {:?}",v);
            var.insert(kunci.clone(), v);
            if mut_ == crate::parsing::mem_mode::statik {
                self.kirim(crate::parsing::Pohon::Let(kunci, 0))
            }
        }
        */
        self.id += 1;
    }
    fn halaman(
        &mut self,
        json: &serde_json::Value,
        var: &mut crate::parsing::Arrmap<String, Vec<crate::parsing::Let_>>,
    ) {
    }
    fn tidur(
        &mut self,
        json: &serde_json::Value,
        var: &mut crate::parsing::Arrmap<String, Vec<crate::parsing::Let_>>,
    ) {
        if let serde_json::Value::Number(num) = json {
            self.kirim(crate::parsing::Pohon::tidur(crate::parsing::args::Str_int(
                num.as_u64().unwrap() as i128,
            )))
        } else {
            println!("{:?}", json);
            panic!()
        }
    }
    fn cetak(
        &mut self,
        json: &serde_json::Value,
        var: &mut crate::parsing::Arrmap<String, Vec<crate::parsing::Let_>>,
        var_nama_alias: &mut crate::parsing::Arrmap<String, Vec<String>>,
    ) {
        let mut i = 0;
        while let Some(nilai) = json["nilai"][i].as_str() {
            match nilai {
                "var" => {
                    i += 1;
                    let t = &json["nilai"][i].as_str().unwrap().to_string();
                    if let Some(id) = var_nama_alias.get_mut(t) {
                        self.kirim(crate::parsing::Pohon::Cetak(crate::parsing::args::internar_memory(id.last().unwrap().to_string())))
                    } else if let Some(var) = var.get_mut(t)
                    {
                        let v = var.last().unwrap();
                        let id = v.id;
                        if v.bisa_diprediksi && v.mut_ != crate::parsing::mem_mode::statik {
                        } else {
                            let nilai =
                                crate::parsing::Pohon::Cetak(crate::parsing::args::penunjuk(id));
                            self.kirim(nilai.clone())
                        }
                    } else {
                        panic!()
                        // tidak ditemukan
                    }
                }
                "langsung" => {
                    i += 1;
                    let str_ = json["nilai"][i].as_str().unwrap().to_string();
                    let str_ =
                        crate::parsing::Pohon::Cetak(crate::parsing::args::Str_Lansung(str_)); /**/
                    self.kirim(str_)
                }
                _ => {
                    println!("-> {}", nilai);
                    self.kirim(crate::parsing::Pohon::Error);
                    panic!()
                }
            }
            i += 1;
        }
    }
    fn putar(
        &mut self,
        json: &serde_json::Value,
        //len:&mut usize,
        var: &mut crate::parsing::Arrmap<String, Vec<crate::parsing::Let_>>,
        var_nama_alias: &mut crate::parsing::Arrmap<String, Vec<String>>,
    ) {
        let mut x = format!("BL{}", self.blok_id);
        self.kirim(crate::parsing::Pohon::blok(crate::parsing::eterator::Blok(
            x.clone(),
        )));
        match (
            &json["arg"][0],
            &json["arg"][1],
            &json["arg"][2],
            &json["arg"][3],
        ) {
            (serde_json::Value::Number(n), ..) => {
                x.remove(0);
                self.kirim(crate::parsing::Pohon::blok(crate::parsing::eterator::Iter(
                    x,
                    crate::parsing::args::Str_int(n.as_i64().unwrap() as i128),
                    crate::parsing::args::null,
                )));
            }
            (serde_json::Value::Null, ..) => {
                x.remove(0);
                self.kirim(crate::parsing::Pohon::blok(
                    crate::parsing::eterator::Putar(x),
                ));
            }
            /*
            a if a.0 == "iter" && let serde_json::Value::String(nama_var_iter) = a.1 =>{
                panic!()
            }
            */
            (
                serde_json::Value::String(tipe_loop),
                serde_json::Value::String(nama_var_iter),
                serde_json::Value::Number(num),
                serde_json::Value::Null,
            ) => {
                if let Some(v) = var_nama_alias.get_all_mut(&nama_var_iter.to_string()) {
                    v.1.push(x.clone());
                    self.kirim(crate::parsing::Pohon::blok(crate::parsing::eterator::Iter(
                        x,
                        crate::parsing::args::Str_int(num.as_i64().unwrap() as i128),
                        crate::parsing::args::null,
                    )));
                } else {
                    var_nama_alias.insert(nama_var_iter.to_string(),vec![ x.clone() ]);
                    self.kirim(crate::parsing::Pohon::blok(crate::parsing::eterator::Iter(
                        x,
                        crate::parsing::args::Str_int(num.as_i64().unwrap() as i128),
                        crate::parsing::args::null,
                    )));
                }
                
            }
            _ => {
                panic!()
            }
        }
        /*
        match &json["arg"][0] {
            serde_json::Value::Number(n) => {
                self.kirim(crate::parsing::Pohon::blok(crate::parsing::eterator::Iter(
                    x,
                    crate::parsing::args::Str_int(n.as_i64().unwrap() as i128),
                    crate::parsing::args::null,
                )));
            }
            serde_json::Value::Null => {
                self.kirim(crate::parsing::Pohon::blok(
                    crate::parsing::eterator::Putar(x),
                ));
            }
            serde_json::Value::String(tipe_loop) => match tipe_loop.as_str() {
                "iter" => match (&json["arg"][1], &json["arg"][2], &json["arg"][3]) {
                    (serde_json::Value::Null,..) =>{
                        panic!()
                    }
                    (
                        serde_json::Value::String(nama_var_iter),
                        serde_json::Value::Number(num),
                        serde_json::Value::Null,
                    ) => {
                        if let Some(v) = var_nama_alias.get_all_mut(&nama_var_iter.to_string()) {
                            v.1.push(x.clone())
                        } else {
                            var_nama_alias.insert(nama_var_iter.to_string(),vec![ x.clone() ]);
                        }
                        self.kirim(crate::parsing::Pohon::blok(crate::parsing::eterator::Iter(
                            x,
                            crate::parsing::args::Str_int(num.as_i64().unwrap() as i128),
                            crate::parsing::args::null,
                        )));
                    }
                    _ => {
                        panic!()
                    }
                },
                _ => {
                    panic!()
                }
            },
            _ => {
                panic!()
            }
        }
        */
        self.blok_id += 1;
        //json[*len].0 += 1;
        //self.kirim(crate::parsing::Pohon::br(format!("L{}",self.blok_id)));
        //self.kirim(crate::parsing::Pohon::blok_);
        //self.blok_id += 1
    }
    fn tulis(
        &mut self,
        nilai: &serde_json::Value,
        var: &mut crate::parsing::Arrmap<String, Vec<crate::parsing::Let_>>,
    ) {
        let mut hasil: Vec<crate::parsing::nilai> = Vec::with_capacity(3);
        let mut i = 0;
        loop {
            match &nilai["nilai"][i] {
                serde_json::Value::String(token) => match token.as_str() {
                    "+" => hasil.push(crate::parsing::nilai::tambah),
                    _ => {
                        let v = var.get_mut(token).unwrap().last().unwrap();
                        match v.mut_ {
                            crate::parsing::mem_mode::imutabel => {
                                panic!()
                            }
                            crate::parsing::mem_mode::mutabel => {
                                panic!()
                            }
                            crate::parsing::mem_mode::statik => {
                                hasil.push(crate::parsing::nilai::penunjuk(token.clone()))
                            }
                        }
                    }
                },
                serde_json::Value::Number(angka) => hasil.push(crate::parsing::nilai::angka(
                    angka.as_i64().unwrap().try_into().unwrap(),
                )),
                serde_json::Value::Null => break,
                _ => {}
            }
            i += 1
        }
        //pengaturan posisi
        //
        i = 0;
        while i < hasil.len() {
            match hasil[i] {
                crate::parsing::nilai::tambah => {
                    if i == 0 || (i + 1) == hasil.len() {
                        panic!()
                    }
                    hasil[i - 1] = crate::parsing::nilai::aritmatik(Box::new(
                        crate::parsing::aritmatik::tambah(
                            hasil[i - 1].clone(),
                            hasil[i + 1].clone(),
                        ),
                    ));
                    hasil.remove(i);
                    hasil.remove(i);
                }
                _ => i += 1,
            }
        }
        self.kirim(crate::parsing::Pohon::tulis(
            nilai["nama"].as_str().unwrap().to_string(),
            0,
            hasil[0].clone(),
        ));
        //println!("nilai -> {:?}\nhasil -> {:?}",nilai,hasil);
    }
    fn parse(
        &mut self,
        //spageti:&mut Vec<(usize,&serde_json::Value)>,
        json: serde_json::Value,
        var: &mut crate::parsing::Arrmap<String, Vec<crate::parsing::Let_>>,
        var_nama_alias: &mut crate::parsing::Arrmap<String, Vec<String>>,
    ) -> bool {
        //variabel vector pointer serde_json dan index
        //pointer menggarah ke elemen vector sebelumnya
        //variabel ini digunakan untuk iterator serde_json::Value
        //varibel ini sangat rumit dan saling terhubung antar elemen divector yang sama (makanya disebut "spageti")
        let mut spageti: Vec<(usize, &serde_json::Value, usize)> = Vec::with_capacity(3);
        spageti.push((0, &json, 0));
        while !spageti.is_empty() {
            let offset = spageti.len() - 1;
            let lapisan = &spageti[offset];
            match lapisan.1["nilai"][lapisan.0]["tipe"].as_str() {
                Some("var") => {
                    self.var(&lapisan.1["nilai"][lapisan.0], var);
                }
                Some("cetak") => {
                    self.cetak(&lapisan.1["nilai"][lapisan.0], var ,var_nama_alias);
                }
                Some("halaman") => {
                    self.halaman(&lapisan.1["nilai"][lapisan.0], var);
                }
                Some("tidur") => self.tidur(&lapisan.1["nilai"][lapisan.0]["tidur"], var),
                Some("putar") => {
                    self.putar(&lapisan.1["nilai"][lapisan.0], var, var_nama_alias);
                    let t = &lapisan.1["nilai"][lapisan.0];
                    spageti.push((0, t, self.blok_id as usize - 1));
                }
                Some("tulis") => self.tulis(&lapisan.1["nilai"][lapisan.0], var),
                Some("blok") => {
                    self.kirim(crate::parsing::Pohon::blok_);
                }
                None => {
                    if spageti.len() == 1 {
                        break;
                    }
                    if let Some("putar") = lapisan.1["tipe"].as_str() {
                        let f = format!("$L{}", lapisan.2);
                        match &lapisan.1["arg"][0] {
                            //serde_json::Value::Number(v)=>{
                            //kurangi varibel penghitung putaran
                            //}
                            _ => {}
                        }
                        self.kirim(crate::parsing::Pohon::br(f));
                        self.kirim(crate::parsing::Pohon::blok_);
                    }
                    self.kirim(crate::parsing::Pohon::blok_);
                    spageti.pop();
                    continue;
                }
                _ => {}
            }
            spageti[offset].0 += 1
        }

        /*
        let mut i = 0;
        while !lapisan.is_empty() {
            match lapisan[i].1[lapisan[i].0]["tipe"].as_str() {
                Some("var") => {
                    self.var(&lapisan[i].1[lapisan[i].0], var);
                }
                Some("cetak") => {
                    self.cetak(&lapisan[i].1[lapisan[i].0], var);
                }
                Some("halaman") => {
                    self.halaman(&lapisan[i].1[lapisan[i].0], var);
                }
                Some("tidur") => self.tidur(&lapisan[i].1[lapisan[i].0]["tidur"], var),
                Some("putar") => {
                    self.kirim(crate::parsing::Pohon::blok("test".to_string()));
                    lapisan.push((0,&lapisan[i].1[lapisan[i].0]["nilai"]));
                    //lapisan[i].0 += 1;
                    i += 1;
                },
                Some("blok") => {
                    lapisan.push((0,&json["nilai"][lapisan[i].0]));
                    lapisan[i].0 += 1;
                    i += 1;
                    continue
                }
                None => {
                    //panic!();
                    if lapisan.len() == 1 {
                        break;
                    }
                    if let Some("putar")=lapisan[0].1[lapisan[0].0 - 1]["tipe"].as_str() {
                        self.kirim(crate::parsing::Pohon::br("0".to_string()));
                    }
                    self.kirim(crate::parsing::Pohon::blok_);
                    lapisan.pop();
                    i -= 1
                }
                _ => {}
            }
            lapisan[i].0 += 1
        }
        */
        false
    }
    fn minta_json(&mut self, i: usize) -> Option<serde_json::Value> {
        let t = self.fungsi[i].clone();
        self.kirim_json.send(t).unwrap();
        self.terima.recv().unwrap()
    }
    pub fn mulai(mut self, var: &mut crate::parsing::Arrmap<String, Vec<crate::parsing::Let_>>) -> bool{
        let mut alias: crate::parsing::Arrmap<String, Vec<String>> = crate::parsing::Arrmap::new();
        for i in 0..self.fungsi.len() {
            if let Some(nilai) = self.minta_json(i) {
                if !self.parse(nilai, var, &mut alias) {
                    break
                }
            } else {
                break
            };
        }
        //mengecek jika variabel tidak digunakan
        
        for i in var.iter() {
            let xx = i.1.last().unwrap();
            match (xx.mut_, xx.sudah_dibaca) {
                (crate::parsing::mem_mode::imutabel|crate::parsing::mem_mode::mutabel,false) =>{
                    self.kirim(crate::parsing::Pohon::Error);
                    println!("varibel '{}' dibuat tapi tidak digunakan {}:{}\n",i.0,xx.nama_file,xx.nomer_baris);
                    return false
                }
                _=>{}
            }
        }
        self.kirim(crate::parsing::Pohon::Selesai);
        true
    }
}
pub fn parse(
    _var: &mut crate::parsing::Arrmap<String, Vec<crate::parsing::Let_>>,
    terima: std::sync::mpsc::Receiver<std::option::Option<serde_json::Value>>,
    kirim_pwa: std::sync::mpsc::Sender<crate::parsing::Pohon>,
    kirim_cpp: std::sync::mpsc::Sender<crate::parsing::Pohon>,
    kirim_json: std::sync::mpsc::Sender<([String; 3], bool)>,
    kom: (bool, bool,bool),
) -> bool {
    let app: App = App {
        fungsi: vec![(
            ["main".to_string(), "main".to_string(), "".to_string()],
            false,
        )],
        id: 0,
        terima,
        kirim_pwa,
        kirim_cpp,
        kirim_json,
        kom,
        blok_id: 0,
    };
    app.mulai(_var)
}
