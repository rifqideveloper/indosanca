struct Data {
    dup :(usize, usize,String),
    baris: String,
    dir:String,
    buf:String,
    _str:(bool,bool),
    file:std::fs::File,
}
impl Data {
    pub fn duplikat(&mut self) ->bool {
        if self.baris.starts_with("duplikat>"){
            use std::io::Write;
            if self.dup.0 == 2 {
                //prototype
                //self.buf.replace("?d", "test");
               
            } else {
                self.file.write(
                    std::iter::repeat(self.buf.as_str())
                    .take(self.dup.1).collect::<String>().as_bytes()
                ).expect("");
            }
            self.dup.0 = 0;
            self.buf.clear();
            self.baris.clear();
            return true
        } else if self.baris.starts_with("<duplikat") {
            let dd = self.baris.split(":").collect::<Vec<&str>>();
            match dd[1].trim().parse::<usize>(){
                Ok(e)=>{
                    self.dup.1 = e;
                    self.baris.clear();
                    self.dup.0 = 1;
                }
                Err(_)=>{}
            }
            return true
        }
        false
    }
    pub fn format(&mut self){
        self.baris = self.baris.trim_start().to_string();
        if !self.baris.ends_with("\n") { self.baris.push('\n') }
    }
    pub fn format_str(&mut self,i:&str) ->bool{
        if self._str.0 {
            if i == "\n" {
                self.buf.replace_range(self.buf.len()-1.., " ");
                return true
            }
            else if i == "\"" {
                self._str = (false,false);
            }
            self.buf.push_str(i);
            return true
        }
        else if i == "\""{
            self._str = (true,false);
            self.buf.push_str(i);
            return true
        }
        else {
            self.buf.push_str(i);
        }
        return false
    }
}
#[allow(non_snake_case)]
pub fn baca(path:&String){
    use std::fs;    
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::io::Write;
    //use serde_json::json;use serde_json::{Result, Value};
    let mut buffer = Data{
        file:std::fs::File::create(format!("{}\\parsing\\main",path)).expect(""),
        dup:(0,0,String::with_capacity(40)),
        baris:String::with_capacity(40),
        dir:String::with_capacity(40),
        buf:String::with_capacity(40),
        _str:(false,false),
    };
    //std::iter::repeat("ha").take(5).collect::<String>()
    for path in fs::read_dir(format!("{}\\kode",path)).unwrap() 
    {
        let x = path.unwrap().path().display().to_string();
        buffer.file.write_fmt(format_args!("<mod {}\n",
            x.clone().split(&['\\','/'][..]).last().expect("").replace(".","_")
        )).expect("");
        buffer.dir = x;
        //unsafe
        while BufReader::new(File::open(&buffer.dir).expect("")).read_line(&mut buffer.baris).expect("") != 0 {
            buffer.format();
            if buffer.duplikat() {continue}
            for i in buffer.baris.clone().split(""){
                if buffer.format_str(&i) { continue }
                if i == "\n" && buffer.dup.0 == 0 {
                    buffer.file.write(buffer.buf.as_bytes()).expect("");
                    
                    buffer.buf.clear();
                }
            }
            buffer.baris.clear()
            //print!("{}",baris);
            //terima.recv().expect("");
        }
        buffer.baris.clear();
        //unsafe
        buffer.file.write_fmt(format_args!("mod>\n")).expect("");  
    }
    
}