
struct Token{
    x : Vec<String>,
    //buf:String,
    _str:(bool,bool,bool),
    blok:(bool,u64,u64,u64),
    kirim : std::sync::mpsc::Sender<Vec<String>>
}
impl Token{
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
    fn token_slice(&mut self,mut data:String,extra:&std::sync::mpsc::Receiver<String>){
        // tokenizer 2.0.1
        // tidak butuh bufer 
        // belum selesai     
        let mut x = 0 ;
        'main:loop {
            match &data[x..x + 1] {
                "\n"=>{
                    self.lanjut();
                    break
                }
                ";"=>{
                    self.lanjut()
                }
                "="|"<"|">"|"!"|"+"|"-"|"/"|"*"|"%"=>{
                    if data[x + 1..x + 2].to_string() == "=" {
                        self.x.push(data[x..x + 2].to_string());
                        x += 1
                    } else {
                        self.x.push(data[x..x + 1].to_string())
                    }
                
                }
                ","|":"|"&"|"."|"?"=>{
                    self.x.push(data[x..x + 1].to_string())
                }
                "$"=>{
                    match &data[x..x + 2]{
                        "$!"|"$?"=>{
                            self.x.push(data[x..x + 2].to_string());
                            x += 1
                        }
                        _=>{
                            self.x.push("$".to_string());
                        }
                    }
                }
                "\""=>{
                    for i in x+1..{
                        match &data[i..i+1] {
                        "\""=>{
                            self.x.push(data[x..i+1].replace("\r", " ").to_string());
                            x = i + 1;
                            continue 'main
                        }
                        "\n"=>{
                            //error
                            data.pop();
                            data.push_str(
                                {format!("{}", 
                                    { 
                                        let mut v = extra.recv().unwrap();
                                        if v.ends_with("\"") {
                                            v.pop();
                                        } 
                                        v = v.trim_start().to_string();
                                        v
                                        
                                    }  )
                                }.as_str()  
                            );
                            
                            println!("lexet test {}",data.to_string());

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
                            "<"|">"|"="|":"|"!"|","|"&"|"*"|"."|"("|")"=>{
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
    
}
pub fn baca_2(terima:std::sync::mpsc::Receiver<String>,kirim:std::sync::mpsc::Sender<Vec<String>>){
    let mut token = Token{
        x : Vec::with_capacity(10),
        //buf:String::with_capacity(10),
        _str:(false,false,false),
        blok:(false,0,0,0),
        kirim:kirim
    };
    terima.iter().for_each(|o| 
        if !o.is_empty(){
            token.token_slice(o,&terima)
        } else {
            token.kirim.send([].to_vec()).unwrap();
            return
        }
    );
    
}