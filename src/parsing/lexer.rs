pub fn baca(path:&String,kirim:std::sync::mpsc::Sender<Vec<String>>){
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let mut baris = String::with_capacity(15);
    let mut file = BufReader::with_capacity(10, File::open(format!("{}\\parsing\\main",path)).expect("membuka main file"));
    let mut _str = String::new();
    while file.read_line(&mut baris).expect("membaca main file") != 0 {
        let mut x : Vec<String> = Vec::with_capacity(20);
        let mut y = baris.chars();
        loop {
            let i = y.next().clone();
            if i == Some('\n') {break}
            match i {
                Some(r)=>{
                    match r {
                        '<'|'>'|'='|':'|';'=>{ x.push(r.to_string()) }
                        '\"'|'\''=>{
                            loop {
                                let w = y.next().expect("token_1");
                                if w == r{
                                    x.push( 
                                        format!("\"{}\"",_str)
                                    );
                                    _str.clear();
                                    break
                                }
                                _str.push(w)
                            }
                        }
                        n if n != ' '=>{
                            _str.push(r);
                            loop{
                                let w = y.next().expect("token_2");
                                match w {
                                    ' '|'\n'=>{
                                        let w = _str.trim().to_string();
                                        if w != ""{x.push(w)}
                                        _str.clear();
                                        break
                                    }
                                    '<'|'>'|'='|':'|';'=>{
                                        let q = _str.trim().to_string();
                                        if q != ""{x.push(q)}
                                        x.push(w.to_string());
                                        _str.clear();
                                        break
                                    }
                                    _=>{_str.push(w)}
                                }
                            }
                        }    
                        _=>{}
                    }
                    
                }
                None=>{break}
            }
            
        }
        //println!("{:?}",x);
        baris.clear();
        if !x.is_empty(){kirim.send(x.clone()).expect("kirim token ke parse")}
        x.clear();
    }
    kirim.send([].to_vec()).expect("");
}