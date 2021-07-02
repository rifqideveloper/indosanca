
struct Token{
    x : Vec<String>,
    //buf:String,
    _str:(bool,bool,bool),
    blok:(bool,u64,u64,u64),
    kirim : std::sync::mpsc::Sender<Vec<String>>
}
impl Token{
    /*
    fn _kata(&mut self,i:char, y:&mut std::str::Chars){
        self.buf.push(i);
        loop{
            let w = y.next().expect("token_2");
                match w {
                    ' '|'\n'=>{
                        let w = self.buf.trim().to_string();
                        if w != ""{self.x.push(w)}
                        self.buf.clear();
                        break
                    }
                    '<'|'>'|'='|':'|';'=>{
                        let q = self.buf.trim().to_string();
                        if q != ""{self.x.push(q)}
                        self.x.push(w.to_string());
                        self.buf.clear();
                        break
                    }
                    _=>{self.buf.push(w)}
                }
        }
    }
    fn _str_(&mut self,i:char, y:&mut std::str::Chars){
        if self._str.2 {
            self._str.2 = false
        }else if i == '\\'{
            match y.next() {
                Some(_t)=>{
                    match _t {
                        '\\'=>{
                            self.buf.push(_t);
                        }
                        'n'|'t'=>{
                            self.buf.push('\\');
                            self.buf.push(_t);
                        }
                        _=>{}
                    }
                }
                None=>{}
            }
            self._str.2 = true
        }else if i == '\n'{
            if self._str.1 {
                self.buf.push(' ')
            } else {
                self._str.1 = true;
            }
        }else if i != '\r' {
            self.buf.push(i)
        }
    }
    */
    fn _blok_(&mut self){
        if self.blok.1 == 0 && self.blok.2 == 0 && self.blok.3 == 0{
            if self.x.last() == Some(&",".to_string()) {
                self.x.pop().expect("");
            }
            self.blok.0 = false
        }

    }
    fn lanjut(&mut self){
        if !self.blok.0 && !self.x.is_empty(){
            self.kirim.send(self.x.clone()).expect("");
            #[cfg(debug_assertions)] println!("[lexer]\n{:#?}",self.x.clone());
            self.x.clear()
        }
    }
    fn token_slice(&mut self,data:&mut String,extra:&std::sync::mpsc::Receiver<String>){
        // tokenizer 2.0.0
        // tidak butuh bufer 
        // belum selesai     
        let mut x = 0 ;
        'main:loop {
            match &data[x..x + 1] {
                "\n"=>{self.lanjut();break}
                ";"=>{self.lanjut()}
                "<"|">"|"="|":"|"!"|","|"&"|"*"|"+"|"-"|"/"=>{
                    self.x.push(data[x..x + 1].to_string())
                }
                "\""=>{
                    for i in x+1..data.len(){
                        match &data[i..i+1] {
                        "\""=>{
                            self.x.push(data[x..i+1].to_string());
                            x = i + 1;
                            continue 'main
                        }
                        "\n"=>{
                            data.replace_range(i..i+1," ");
                            data.push_str(&extra.recv().unwrap().trim_start());
                        }
                        _=>{}
                        }
                    }
                }
            
                i if i != " " && i != "\t" && i != "\r" =>{
                    for i in x+1..data.len(){
                        match &data[i..i+1] {
                            " "|";"|"\""|"\r" /*|"<"|">"|"="|":"|"!"|","|"&"|"*"|"\n"*/=>{
                                self.x.push(data[x..i].to_string());
                                x = i ;
                                continue 'main
                            }
                            "<"|">"|"="|":"|"!"|","|"&"|"*"=>{
                                self.x.push(data[x..i].to_string());
                                self.x.push(data[i..i + 1].to_string());
                                x = i + 1;
                                continue 'main

                            }
                            "\n"=>{
                                self.x.push(data[x..i].to_string());
                                self.lanjut();
                                break 'main
                            }
                            
                            _=>{}
                        }
                    }
                }
                _=>{}
            }
            x += 1 ;
        }
        //println!("{:#?}",self.x);

        
    }
    /*
    fn token(&mut self,data:String){
        // tokenizer 1.0.0
        if data == "mod>\n"{
            self.kirim.send(["mod".to_string(),">".to_string()].to_vec()).expect("");
            return
        } 
        let mut y = data.chars();
        loop{
            match y.next(){
                i if !self._str.0 && i == Some('\n') || i == None => {
                    self.lanjut();
                    break
                }
                Some(i)=>{
                    match i {
                        '('=>{ 
                            self.blok.1 += 1;
                            self.blok.0 = true;
                            self.x.push(i.to_string());
                        }
                        ')'=>{
                            self.blok.1 -= 1;
                            self._blok_();
                            self.x.push(i.to_string());
                        }        
                        '['=>{
                            self.blok.3 += 1;
                            self.blok.0 = true;
                            self.x.push(i.to_string());
                        }
                        ']'=>{
                            self.blok.3 -= 1;
                            self._blok_();
                            self.x.push(i.to_string());
                        }
                        '{'|'}'=>{
                            self.lanjut();
                            self.kirim.send(
                                [i.to_string()].to_vec()
                            ).unwrap()
                        }
                        '<'|'>'|'='|':'|'!'|','|'&'|'*'=>{ self.x.push(i.to_string()) }
                        '\"'=>{
                            self.buf.push(i);
                            if !self._str.0{
                                match y.next() {
                                    Some(e)=>{
                                        if e == '"'{
                                            self.buf.push(e);
                                            self.x.push(self.buf.clone());
                                            self.buf.clear();
                                            if self.blok.3 != 0 && y.next() != Some(']') || y.next() != Some(','){
                                                self.x.push(",".to_string());
                                            }
                                        } else if e != '\r' {
                                            self.buf.push(e);
                                        }
                                    }
                                    None=>{}
                                }
                            } else {
                                //if self.buf.chars().last() == Some('\r'){self.buf = self.buf.pop().unwrap().to_string();}
                                self.x.push(self.buf.clone());
                                self.buf.clear();
                            }
                            self._str.0 = !self._str.0;
                            self._str.1 = false;
                        }
                        _ if self._str.0 =>{
                            self._str_(i, &mut y);
                        }
                        n if n != ' ' =>{
                            self._kata(i,&mut y)
                        }
                        ';' => { self.lanjut() }
                        _=>{}
                    }
                }
                None=>{break}
            }
        }
    }
    */
    fn selesai(self){
        match self.kirim.send([].to_vec()){
            Ok(_)=>{}
            Err(_)=>{panic!()}
        }
    }
}
pub fn baca_2(terima:std::sync::mpsc::Receiver<String>,kirim:std::sync::mpsc::Sender<Vec<String>>){
    let mut token = Token{
        x : Vec::with_capacity(10),
        //buf:String::with_capacity(10),
        _str:(false,false,false),
        blok:(false,0,0,0),
        kirim:kirim
    };
    loop{
        match terima.recv(){
            Ok(mut o)=>{
                if o.is_empty(){
                    token.selesai();
                    return
                }
                token.token_slice(&mut o,&terima);
            }
            Err(_)=>{
                println!("gagal menerima");
                std::process::exit(1);
            }
        };

    }
    
}