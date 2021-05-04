macro_rules! token_pust {
    ($buf:expr,$token:expr,$($x:expr),+) => {
        $( if $x != ""{ $token.push($x) } )*
        $buf.clear();
    };
    ($token:expr,$($x:expr),+) => {
        $( if $x != ""{ $token.push($x) } )*
    };
}
macro_rules! ke_lex_f_send {
    ($ke_lex_f:expr,$buf:expr,$token:expr) => {
        if $token.len() != 0 {
            $ke_lex_f.send($token.clone()).expect("") ;
            $buf.clear();
            $token.clear();
        }
    };
    (debug? $ke_lex_f:expr,$buf:expr,$token:expr) => {
        debug!($token);
        ke_lex_f_send!($ke_lex_f,$buf,$token);
    };
    
}
macro_rules! debug {
    ($x:expr)=>{
        print!("[");
        for i in $x {
            print!("\"{}\",",i);
        }
        print!("]\n");
    }
}

pub fn lexer(terima:std::sync::mpsc::Receiver<std::string::String>,ke_lex_f:std::sync::mpsc::Sender<Vec<String>>){
    let mut buf = String::with_capacity(40);
    let mut sintak = String::with_capacity(40);
    let mut token :Vec<String> = [].to_vec();
    let mut nama = false;
    loop {
        buf = terima.recv().expect("msg: &str");
        if buf == "" {break}
        if buf == "\n"{continue}
        for i in buf.clone().split(""){
            match (sintak.as_str(),i) {
                ("cpu"," ")=>{
                    token.push(sintak.clone());
                    buf = buf.trim_end().to_string();
                    buf.remove(0);
                    buf.remove(0);
                    buf.remove(0);
                    buf.remove(0);
                    token.push(buf.trim_end().to_string());
                    sintak.clear();
                    ke_lex_f.send(token.clone()).expect("");
                    token.clear();
                    break
                }
                _=>{}
            }
            if i != " " {
                sintak.push_str(i);
            }
            
        }

    }
    ke_lex_f.send([].to_vec()).expect("")
}
