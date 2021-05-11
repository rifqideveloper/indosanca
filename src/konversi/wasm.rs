

pub fn tulis(proyek:&String,ke_js:std::sync::mpsc::Sender<std::string::String>,ke_html:std::sync::mpsc::Sender<std::string::String>,ke_wasm:std::sync::mpsc::Sender<[std::string::String; 2]>){
    /*
    let test = r#"
    (module
        (func $foo)
        (func (export "bar")
            call $foo
        )
    )
    "#;
    ke_wasm.send(["test".to_string(),test.to_string()]).expect("msg: &str");
    */
    use std::fs;
    use toml::Value;
    let v =fs::read_to_string(
        format!("{}\\parsing\\pohon.toml",proyek)
    ).expect("toml gagal terparsing").parse::<Value>().expect("msg: &str");
    
    
    println!("{}",v["program"]["prog"][3]);

    ke_js.send("var_dek".to_string()).expect("msg: &str");
    ke_js.send("test".to_string()).expect("msg: &str");
    ke_js.send("\"test\"".to_string()).expect("msg: &str");
    ke_js.send("".to_string()).expect("msg: &str");
    ke_html.send("<script src=\"index.js\" defer></script>".to_string()).expect("msg: &str");
    ke_html.send("".to_string()).expect("msg: &str");
    ke_wasm.send(["".to_string(),"".to_string()]).expect("msg: &str");
}