struct Token{
    x : Vec<String>,
    
    buf:String,
    _str:(bool,bool,bool),
              //()//{}/[]
    blok:(bool,u64,u64,u64),
    kirim : std::sync::mpsc::Sender<Vec<String>>
}
impl Token{
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
                    _=>{self.buf.push(w);}
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
    fn _blok_(&mut self){
        if self.blok.1 == 0 && self.blok.2 == 0 && self.blok.3 == 0{
            if self.x.last() == Some(&",".to_string()) {
                self.x.pop().expect("");
            }
            self.blok.0 = false
        }

    }
    fn lanjut(&mut self){
        if !self.blok.0{
            if !self.x.is_empty(){
                self.kirim.send(self.x.clone()).expect("");
                #[cfg(debug_assertions)]
                println!("[lexer]\n{:#?}",self.x.clone());
                self.x.clear()
            }
        }
    }
    fn token(&mut self,data:String){
        if data == "mod>\n"{
            
            self.kirim.send(["mod".to_string(),">".to_string()].to_vec()).expect("");
            return
        }
        let mut y = data.chars();
        loop{
            match y.next(){
                i if !self._str.0 && i == Some('\n') ||i == None =>{
                    self.lanjut();
                    break
                }
                Some(i)=>{
                    match i {
                        '('|')'=>{ 
                            if i == '('{
                                self.blok.1 += 1;
                                self.blok.0 = true
                            } else {
                                self.blok.1 -= 1;
                                self._blok_();
                            }
                            self.x.push(i.to_string());
                        }
                        '{'|'}'=>{
                            if i == '{'{
                                self.blok.2 += 1;
                                self.blok.0 = true
                            } else {
                                self.blok.2 -= 1;
                                self._blok_();
                            }
                            self.x.push(i.to_string());
                        }
                        '['|']'=>{
                            if i == '['{
                                self.blok.3 += 1;
                                self.blok.0 = true
                            } else {
                                self.blok.3 -= 1;
                                self._blok_();
                            }
                            self.x.push(i.to_string());
                        }
                        '<'|'>'|'='|':'|';'|'!'|','=>{ self.x.push(i.to_string()) }
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
                        _=>{}
                    }
                }
                None=>{break}
            }
        }
    }
    fn selesai(self){
        self.kirim.send([].to_vec()).expect("")
    }
}
pub fn baca_2(terima:std::sync::mpsc::Receiver<String>,kirim:std::sync::mpsc::Sender<Vec<String>>){
    let mut token = Token{
        x : Vec::with_capacity(10),
        buf:String::with_capacity(10),
        _str:(false,false,false),
        blok:(false,0,0,0),
        kirim:kirim
    };
    loop{
        let data = terima.recv().expect("msg: &str");
        if data == "" {break}
        token.token(data)
    }
    token.selesai()
}