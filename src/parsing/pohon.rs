use serde::{Deserialize/*, Serialize*/};
use serde_json::json;
use serde_json::{/*Result,*/ Value};
use std::fs;
#[derive(Deserialize, Debug)]
struct User {}
pub fn baca(_proyek:&String,kirim:std::sync::mpsc::Sender<std::string::String>){
    let json: Value =
        serde_json::from_str(
            &fs::read_to_string(
                format!("{}\\parsing\\parser",_proyek)
            ).expect("msg: &str")
        ).expect("JSON tidak terformat dengan baik");
    let mut i = 0;
    kirim.send(
        format!("[program]\ntipe ={}\nnama ={}\n[fn]\n",json[i]["tipe"],json[i]["nama"])
    ).expect("msg: &str");
    loop{
        i = i + 1;
        if json[i] == json!(null){break}
        else if json[i]["tipe"] == "fn/cpu"{
            kirim.send(
                format!("[fn.{}]\nkembali = {}\n",{
                    let mut x = json[i]["nama"].to_string();
                    x.pop();
                    x.remove(0);
                    x
                },json[i]["kembali"])
            ).expect("msg: &str");
            let mut x = 0;
            loop {
                if json[i]["nilai"][x] == json!(null){break}
                if json[i]["nilai"][x]["tipe"] == "var" {
                    kirim.send(format!(
                        "variable = [{},{},{}]\n",
                        json[i]["nilai"][x]["tipedata"],
                        json[i]["nilai"][x]["nama"],
                        json[i]["nilai"][x]["nilai"][0]["nilai"],
                    )).expect("msg: &str");
                }
                else if json[i]["nilai"][x]["tipe"] == "cetak"{
                    kirim.send(format!(
                        "cetak = [{}]\n",
                        if json[i]["nilai"][x]["nilai"]["var"] != json!(null){
                            format!("\"var\",{}",json[i]["nilai"][x]["nilai"]["var"])
                        }else{format!("\"str\",{}",json[i]["nilai"][x]["nilai"]["str"])},
                    )).expect("msg: &str");
                } 
                x = x + 1
            }

        }
    }
    kirim.send("".to_string()).expect("msg: &str");
}