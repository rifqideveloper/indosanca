
struct Token{
    x : Vec<String>,
    //buf:String,
    _str:(bool,bool,bool),
    blok:(bool,u64,u64,u64),
    kirim : std::sync::mpsc::Sender<Vec<String>>
}

impl Token {
    fn _blok_(&mut self){
        if self.blok.1 == 0 && self.blok.2 == 0 && self.blok.3 == 0{
            if self.x.last() == Some(&",".to_string()) {
                self.x.pop().expect("");
            }
            self.blok.0 = false
        }

    }
    fn lanjut(&mut self) {
        if !self.blok.0 && !self.x.is_empty(){
            self.kirim.send(self.x.clone()).expect("");
            #[cfg(debug_assertions)] 
            println!("[lexer]\n{:#?}",self.x.clone());
            self.x.clear()
        }
    }
    fn token_slice(&mut self,mut data:String,extra:&std::sync::mpsc::Receiver<String>){
        // tokenizer 2.0.2
        // tidak butuh bufer 
        // belum selesai     
        let mut x = 0 ;
        let mut y = 0 ;
        'main:loop {
            match &data[x..x + 1] {
                "="|"<"|">"|"!"|"+"|"-"|"/"|"*"|"%"=>{
                    x += if data[x + 1..x + 2].to_string() == "=" {
                        self.x.push(data[x..x + 2].to_string());
                        2
                    } else {
                        self.x.push(data[x..x + 1].to_string());
                        1
                    }
                }
                "." =>{
                    self.x.push(data[x..x + 1].to_string());
                    x += 1
                }
                ","|":"|"&"=>{
                    self.x.push(data[x..x + 1].to_string());
                    x += 1
                }
                "$"=>{
                    x += match &data[x..x + 2]{
                        "$!"|"$?"=>{
                            self.x.push(data[x..x + 2].to_string());
                            1
                        }
                        _=>{
                            self.x.push("$".to_string());
                            1
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
                "[" =>{
                    //panic!();
                    self.x.push(data[x..x + 1].to_string());
                    y += 1;
                    x += 1

                }
                "]" =>{
                    //panic!();
                    self.x.push(data[x..x + 1].to_string());
                    y -= 1;
                    x += 1
                }
                "?" => {
                    //panic!();
                    /*
                    x += if &data[x + 1 .. x + 2] == "?" {
                        self.x.push("??".to_string());
                        if &data[x + 2 .. x + 3] == "\r" {
                            break
                        }
                        2
                    } else {
                        self.x.push("?".to_string());
                        1
                    } ;
                    */
                    self.x.push("?".to_string());
                    x += 1
                    //println!("{:?}",self.x);

                    //panic!()
                }
                ";"=> {
                    if y == 0 {
                        self.lanjut()
                    } else {
                        self.x.push(data[x..x + 1].to_string());
                        /*
                        x += 1 ;
                        if &data[x..x + 1] == "]" { panic!() }
                        if data[x..x + 1].parse::<usize>().is_ok() {
                            let mut _len = (x.clone(),x.clone() + 1);
                            loop{
                                x += 1 ;
                                if !data[x..x + 1].parse::<usize>().is_ok() {
                                    break
                                }
                                _len.1 += 1;
                            }
                            self.x.push(data[_len.0.._len.1].to_string());
                            if &data[x..x + 1] == "]" {
                                self.x.push("]".to_string());
                                x += 1 ;
                                continue
                                //panic!()
                            }
                            //panic!()
                            */
                        
                    }
                    x += 1

                }
                "\n"|"\r"=>{
                    self.lanjut();
                    break 'main
                }
                i if i != " " && i != "\t" && i != "\r" =>{
                    for mut i in x + 1..data.len(){
                        match &data[i..i+1] {
                            " "|";"|"\""|"\r" /*|"<"|">"|"="|":"|"!"|","|"&"|"*"|"\n"*/=>{
                                self.x.push(data[x..i].to_string());
                                x = i ;
                                continue 'main
                            }
                            "." =>{
                                if data[x..i+1].parse::<f64>().is_ok(){
                                    //println!("float");
                                    loop {
                                        i += 1 ;
                                        if !data[x..i+1].parse::<f64>().is_ok() {
                                            //println!("float {}",&data[x..i+1]);
                                            break
                                        }
                                    }
                                    self.x.push(data[x..i].to_string());
                                    x = i + 1;
                                    continue 'main
                                }
                                self.x.push(data[x..i].to_string());
                                self.x.push(data[i..i + 1].to_string());
                                x = i + 1;
                                continue 'main
                            }
                            "<"|">"|"="|":"|"!"|","|"&"|"*"|"("|")"=>{
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
                            "]"=>{
                                self.x.push(data[x..i].to_string());
                                self.x.push(data[i..i + 1].to_string());
                                y -= 1 ;
                                x = i + 1;
                                continue 'main
                            }
                            _=>{}
                        }
                    }
                }
                _=>{ x += 1 /*panic!()*/}
            }

        }
    }
}

/* error tida diketahui
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
        let mut y = 0 ;
        'main:loop {
            if data[x..x + 1].ends_with(']') {
                panic!()
            }
            match &data[x..x + 1] {
                "\n"=>{
                    self.lanjut();
                    break
                }
                ";"=>{
                    if y == 0 {
                        self.lanjut()
                    } else {
                        self.x.push(data[x..x + 1].to_string());
                        /*
                        x += 1 ;
                        if &data[x..x + 1] == "]" { panic!() }
                        if data[x..x + 1].parse::<usize>().is_ok() {
                            let mut _len = (x.clone(),x.clone() + 1);
                            loop{
                                x += 1 ;
                                if !data[x..x + 1].parse::<usize>().is_ok() {
                                    break
                                }
                                _len.1 += 1;
                            }
                            self.x.push(data[_len.0.._len.1].to_string());
                            if &data[x..x + 1] == "]" {
                                self.x.push("]".to_string());
                                x += 1 ;
                                continue
                                //panic!()
                            }
                            //panic!()
                            */
                        }
                    }
                }
                "[" =>{
                    //panic!();
                    self.x.push(data[x..x + 1].to_string());
                    y += 1;
                }
                "]" =>{
                    //panic!();
                    self.x.push(data[x..x + 1].to_string());
                    y -= 1;
                }
                /*
                n if n.ends_with(']') =>{
                    //self.x.push( data[x..x + ( n.len() - 1 ) ].to_string() );
                    panic!();
                    let z = n.split_at(n.len());
                    self.x.push(z.0.to_string());
                    self.x.push(z.1.to_string());
                    x += n.len() ;
                    y -= 1;
                    continue
                }
                */
                "?" => {
                    if &data[x + 1 .. x + 2] == "?" {
                        self.x.push("??".to_string());
                        x += 2 ;
                        continue
                    } else {
                        self.x.push("?".to_string());
                    }
                    //println!("{:?}",self.x);

                    //panic!()
                }
                "="|"<"|">"|"!"|"+"|"-"|"/"|"*"|"%"=>{
                    if data[x + 1..x + 2].to_string() == "=" {
                        self.x.push(data[x..x + 2].to_string());
                        x += 1
                    } else {
                        self.x.push(data[x..x + 1].to_string())
                    }
                
                }
                ","|":"|"&"|"."=>{
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
                    for i in x + 1..data.len(){
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
                            "]"=>{
                                self.x.push(data[x..i].to_string());
                                self.x.push(data[i..i + 1].to_string());
                                y -= 1 ;
                                x = i + 1;
                                continue 'main
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
*/
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