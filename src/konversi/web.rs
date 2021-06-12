use std::io::prelude::*;
use std::fs::File;
use std::io::BufWriter;
pub fn app(pohon:&std::vec::Vec<crate::parsing::parse_3::Pohon>,path:&String){
    std::fs::create_dir_all(format!("{}\\target\\www",path)).expect("tidak dapat membuat target direktori (target)");
    std::fs::create_dir_all(format!("{}\\target\\www\\aset",path)).expect("tidak dapat membuat target direktori (target)");
    std::fs::create_dir_all(format!("{}\\target\\www\\wasm",path)).expect("tidak dapat membuat target direktori (target)");
    let mut html = std::fs::File::create(format!("{}\\target\\www\\index.html",path)).unwrap();
    html.write_all(b"\
    <!DOCTYPE html>\
    <html lang=\"en\">\
        <head>\
            <meta charset=\"UTF-8\">\
            <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\
            <title></title>\
            <script>function createWorker(fn) { return new Worker(URL.createObjectURL(new Blob(['self.onmessage = ', fn.toString()], { type: 'text/javascript' })));}</script>\
            <script src=\"index.js\" defer></script>\
        </head>\
    </html>\
    ").unwrap();
    let mut js = BufWriter::new( File::create(format!("{}\\target\\www\\index.js",path)).unwrap() );
    js.write(b"var main = function(){\n").unwrap();
    use crate::parsing::parse_3::Pohon;
    for i in pohon{
        match i {
            Pohon::cetak(o)=>{
                match o {
                    crate::parsing::parse_3::Nilai::lansung(o)=>{
                        let v = format!("console.log(\"{}\")\n",o);
                        js.write(v.as_bytes()).unwrap();
                    }
                    crate::parsing::parse_3::Nilai::minta(o)=>{}
                    crate::parsing::parse_3::Nilai::penujuk(o)=>{}
                }
            }
            Pohon::var(o)=>{}
        }
    }
    js.write(b"}()").unwrap();
}