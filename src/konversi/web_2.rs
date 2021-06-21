use crate::parsing::parse_3::{Pohon,Nilai};
use std::io::Write;
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
        let t= if self.import_lib[2] {
            format!("(data (i{}.const {}) {})\n",p,self.offset,d)
        } else {
            format!("(data (i{}.const {}) {})\n",p,self.offset,d)
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
    let mut v_num = 0;
    wat.write(b"(module\n").unwrap();
    for i in pohon{
        match i {
            Pohon::cetak(o)=>{
                match o {
                    Nilai::lansung(o)=>{
                        let t = format!("i32.const {}\ni32.const {}\ncall $log\n",_web.offset,o.len());
                        _main.write(
                            t.as_bytes()
                        ).unwrap();
                        _data.write(_web.data("32",o.len(),format!("\"{}\"" , o.as_str() ) ).as_bytes()).unwrap();
                    }
                    Nilai::minta(o)=>{}
                    Nilai::penujuk(o)=>{}
                }
                if !_web.import_lib[1]{
                    _data.write(b"(import \"import\"\"mem\" (memory 1))\n").unwrap();
                    imp.write(b"\"mem\": new WebAssembly.Memory({initial: 1}),").unwrap();
                    _web.import_lib[1] = true;
                }
                if !_web.import_lib[0]{
                    let t = _web.import_log();
                    wat.write(t.0.as_bytes()).unwrap();
                    imp.write(t.1.as_bytes()).unwrap();
                }
            }
            Pohon::var(o)=>{
                let v = format!("(local $v_{0} i32)\n",o.id);

                _main.write(v.as_bytes()).unwrap();
            }
        }
    }
    drop((_main,_data,imp));
    {wat}.write({
        let mut t = std::fs::read_to_string(format!("{}\\parsing\\www\\data",path)).unwrap();
        t.push_str("(func (export \"main\")\n");
        t.push_str(&std::fs::read_to_string(format!("{}\\parsing\\www\\main",path)).unwrap() );
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
        html.write(b"<script>var import_object={\"import\" :{").unwrap();
        html.write(&import).unwrap();
        html.write(b"}};fetch('main.wasm').then(response=>response.arrayBuffer()).then(bytes => WebAssembly.instantiate(bytes,import_object)).then(results => {var main=function(){results.instance.exports.main();}();}).catch(console.error);</script></head>").unwrap();
    } else {
        html.write(b"<script>fetch('main.wasm').then(response=>response.arrayBuffer()).then(bytes => WebAssembly.instantiate(bytes)).then(results => {var main=function(){results.instance.exports.main();}();}).catch(console.error);</script></head>").unwrap();
    }
}