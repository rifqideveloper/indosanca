use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "asset/"]
#[prefix = "prefix/"]
pub struct Asset;
pub fn bantuan(){
    let beranda : std::borrow::Cow<[u8]> = Asset::get("prefix/bantuan.txt").unwrap().data;
    println!("{}", std::str::from_utf8(beranda.as_ref()).unwrap());
    let (input,mut line) = (std::io::stdin(),String::with_capacity(10));
    loop{
        println!("{}[2J{}", 27 as char,std::str::from_utf8(beranda.as_ref()).unwrap());
        input.read_line(&mut line).unwrap();
        match line.trim_end() {
            "keluar"=>{break}
            "dokument"=>{

            }
            "siapa anda"=>{
                println!("sebelum saya menjawab sebutkan nama mu dulu");
                let mut nama = String::new();
                loop{
                    input.read_line(&mut nama).unwrap();
                    nama = nama.trim_end().to_string();
                    if !nama.is_empty() {break} else {
                        println!("jawab dulu pertanyaan saya");
                    }
                }
                if nama.len() > 3 {println!("{} nama pendek sekali tapi bagus",nama)}
                else if nama.len() < 20 {println!("{} nama panjang sekali tapi bagus",nama)}
                else {println!("{} nama yang bagus",nama)}
                println!("ok nama saya indosanca dan saya adalah kompiler👩‍💻👨‍💻");
                println!("saya berasal dari indonesia 🇮🇩");
                println!("apa anda tahu kompiler itu apa ? (ketik 'y' untuk iya dan 't' untuk tidak)");
                loop{
                    line.clear();
                    input.read_line(&mut line).unwrap();
                    match line.trim_end(){
                        "y"|"Y"=>{println!("bagus 💯👍 pintar sekali");break}
                        "t"|"T"=>{
                            println!("kompiler/Kompilator (Inggris: compiler) adalah sebuah program komputer yang berguna untuk menerjemahkan program komputer yang ditulis dalam bahasa pemrograman tertentu menjadi program yang ditulis dalam bahasa pemrograman lain.Terlepas dari pengertiannya yang demikian relatif luas, istilah kompilator biasa digunakan untuk program komputer yang menerjemahkan program yang ditulis dalam bahasa pemrograman tingkat tinggi (semacam bahasa Pascal, C++, BASIC, FORTRAN, Visual Basic, Visual C#, Java, xBase, atau COBOL) menjadi bahasa mesin, biasanya dengan bahasa Assembly sebagai perantara. ");
                            println!("sumber Dari Wikipedia bahasa Indonesia, ensiklopedia bebas https://id.wikipedia.org/wiki/Kompilator");
                            println!("\nuntuk saya menerjemahkan ke web(js/html/wasm) dan assembli");
                            break
                        }
                        _=>{println!("jawab yang benar");}
                    }
                }
                input.read_line(&mut line).unwrap();
            }
            _=>{}
        }
        line.clear()
    }
    
}