pub fn baca(
    terima:std::sync::mpsc::Receiver<(String,usize)>,
    kirim:std::sync::mpsc::Sender<std::string::String>
)
{ 
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let mut buffer = String::with_capacity(300);
    'outer: loop{
        let f_index = terima.recv().expect("menerima ke index");
        let mut i = 0;
        if f_index.0 == "" {break}
        let mut file = BufReader::new(File::open(&f_index.0).expect("membuka file diindex"));
        while file.read_line(&mut buffer).expect("membaca file diindex") != 0 {
            if i == f_index.1 {
                kirim.send(buffer.clone()).expect("data file index");
                continue 'outer;
            }
            buffer.clear();
            i = i + 1
        }
    }
}