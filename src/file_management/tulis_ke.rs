pub mod wasm {
    pub fn js(proyek:&String,terima:std::sync::mpsc::Receiver<std::string::String>){
        use std::io::Write;
        let mut file = std::fs::File::create(format!("{}\\target\\www\\index.js",proyek)).expect("");
        file.write("var main=function(){\n".as_bytes()).expect("msg: &str");
        let mut buf = terima.recv().expect("msg: &str");
        loop{
            if buf == "" {break}
            if buf == "var_dek" {
                let x = format!(
                    "var {}={}\n",
                    terima.recv().expect("msg: &str"),
                    terima.recv().expect("msg: &str")
                );
                file.write(x.as_bytes()).expect("msg: &str");
            }
            buf = terima.recv().expect("msg: &str");
        }
        file.write("\n}()".as_bytes()).expect("msg: &str");
    }
    pub fn html(proyek:&String,terima:std::sync::mpsc::Receiver<std::string::String>){
        use std::io::Write;
        let mut file = std::fs::File::create(format!("{}\\target\\www\\index.html",proyek)).expect("");
        file.write(r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta http-equiv="X-UA-Compatible" content="IE=edge">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Document</title>
            <script src="index.js" defer></script>
        "#.as_bytes()).expect("msg: &str");
        file.write("<script>function createWorker(fn) { return new Worker(URL.createObjectURL(new Blob(['self.onmessage = ', fn.toString()], { type: 'text/javascript' })));}</script>".as_bytes()).expect("msg: &str");
        let mut buf =terima.recv().expect("msg: &str");
        loop{
            if buf == "" {break}
            file.write(buf.as_bytes()).expect("msg: &str");
            buf =terima.recv().expect("msg: &str");
        }
        
        file.write("</head><body></body></html>".as_bytes()).expect("msg: &str");

    }
    pub fn wasm(proyek:&String,terima:std::sync::mpsc::Receiver<[std::string::String; 2]>){
        //let mut file = std::fs::File::create(format!("{}\\target\\www\\wasm\\index.wasm",proyek)).expect("");
        use std::io::Write;
        loop {
            let buf = terima.recv().expect("msg: &str");
            if buf == ["".to_string(),"".to_string()] { break }
            std::fs::File::create(format!("{}\\target\\www\\wasm\\{}.wasm",proyek,buf[0]))
                .expect("")
                .write(&wat::parse_str(buf[1].clone()).expect("msg: &str"))
                .expect("msg: &str");
            
        }
    }
}
pub mod parsing {
    pub fn tulis(f:&str,_proyek:&String,terima:std::sync::mpsc::Receiver<std::string::String>){
        use std::io::Write;
        let mut file = std::fs::File::create(format!("{}\\parsing\\{}",_proyek,f)).expect("");
        let mut _buf = String::with_capacity(40);
        loop {
            _buf = terima.recv().expect("");
            if _buf == "" {
                file.write(_buf.as_bytes()).expect("");
                break
            }
            file.write(_buf.as_bytes()).expect("");
        }
    }
}
