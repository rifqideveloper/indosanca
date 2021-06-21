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
    js.write(b"\
    var import_object = {\n\
    \"import\" :{
        \"mem\": new WebAssembly.Memory({initial: 1}),\n\
        \"log\":(offset, length) => {\
                var  bytes = new  Uint8Array(import_object.import.mem.buffer, offset, length);\
                var  string = new  TextDecoder('utf8').decode(bytes);\
                console.log(string)\
                },\n\
                \"decode_print\": (start, end) => {\n\
                    document.write(new TextDecoder(\"utf-8\").decode(\
                        new Uint8Array(import_object.js.mem.buffer, start, end)\
                    ));\n\
                }}\
    }\n").unwrap();
    js.write(b"\
    fetch('main.wasm').then(response =>\n\
        response.arrayBuffer()\n\
    ).then(bytes => WebAssembly.instantiate(bytes,import_object)).then(results => {\n\
        var instance = results.instance.exports\n\
        var main = function(){\n").unwrap();
    use crate::parsing::parse_3::Pohon;
    //js.write(b"console.log(instance.add(1, 2))\n").unwrap();
    /*
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
    */
    js.write(b"instance.main()\n").unwrap();
    //js.write(b"var f = instance.exports.main()\n").unwrap();
    js.write(b"}()\n\
    }).catch(console.error);").unwrap();
}