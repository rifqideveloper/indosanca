pub fn baris(path:&String,nama:&str,baris:usize,/*selesai:std::sync::mpsc::Sender<String>*/){
    {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        let mut file_ganti = std::fs::File::create(format!("{}\\parsing\\{}_pengganti",path,nama)).expect("");
        let mut file = BufReader::new(File::open(format!("{}\\parsing\\{}",path,nama)).expect("membuka file"));
        let mut buf = String::with_capacity(100);
        let mut i = 0;
        while file.read_line(&mut buf).expect("membuka file ") != 0 {
            if i != baris {
                use std::io::Write;
                    file_ganti.write(buf.as_bytes()).expect("");
            }
            buf.clear();
            i = i + 1
        }
    }
    std::fs::rename(format!("{}\\parsing\\{}_pengganti",path,nama), format!("{}\\parsing\\{}",path,nama)).expect("msg: &str");
    //selesai.send("".to_string()).expect("");
}