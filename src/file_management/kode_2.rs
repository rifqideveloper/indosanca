use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
//use rust_embed::RustEmbed;
//#[derive(RustEmbed)]
//#[folder = "asset/"]
//#[prefix = "prefix/"]
//struct Asset;
struct Data<'a> {
    kirim:
        &'a std::sync::mpsc::Sender<(/*nomer baris*/ u64, /*nama file*/ String, String)>,
}
impl Data<'_> {
    fn file(&self, nama_file: &str, buf: &mut String) {
        let mut nomer_baris = 1;
        self.kirim
            .send((
                nomer_baris,
                nama_file
                    .to_string()
                    .replace("\\", "\\\\")
                    .replace("/", "\\\\"),
                format!(
                    "modul {}\n",
                    nama_file
                        .clone()
                        .split(&['\\', '/'][..])
                        .last()
                        .expect("")
                        .replace(".is", "")
                ),
            ))
            .expect("msg: &str");
        let mut read = BufReader::with_capacity(1000, File::open(&nama_file).expect(""));
        while read.read_line(buf).unwrap() != 0 {
            *buf = buf.trim_start().to_string(); //buf.trim_start();
            if !buf.is_empty() {
                self.kirim
                    .send(if buf.ends_with("\n") {
                        (
                            nomer_baris,
                            nama_file
                                .to_string()
                                .replace("\\", "\\\\")
                                .replace("/", "\\\\"),
                            buf.to_string(),
                        )
                    } else {
                        (
                            nomer_baris,
                            nama_file
                                .to_string()
                                .replace("\\", "\\\\")
                                .replace("/", "\\\\"),
                            buf.to_string() + "\n",
                        )
                    })
                    .expect("msg: &str");
            }
            buf.clear();
            nomer_baris += 1
        }
        self.kirim
            .send((
                nomer_baris,
                nama_file
                    .to_string()
                    .replace("\\", "\\\\")
                    .replace("/", "\\\\"),
                "modul_\n".to_string(),
            ))
            .expect("msg: &str");
    }
    fn selesai(self) {
        self.kirim
            .send((0, "".to_string(), "".to_string()))
            .unwrap()
    }
}
pub fn baca(
    buf: &mut String,
    path: &String,
    kirim: std::sync::mpsc::Sender<(/*nomer baris*/ u64, /*nama file*/ String, String)>,
) {
    let data = Data { kirim: &kirim };
    for path in fs::read_dir(format!("{}\\kode", path)).unwrap() {
        data.file(&path.unwrap().path().display().to_string(), buf);
    }
    data.selesai()
}
