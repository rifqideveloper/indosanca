use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use sha256::digest;
use std::io::{BufReader, BufRead};
use std::io::Read;
use std::iter;
use std::thread::Thread;
use std::thread::Builder;
use serde::{Serialize, Deserialize};

//[web export]
//<!DOCTYPE html>
//<html>
//<head>
//<meta name="viewport" content="width=device-width" />
//<link rel="apple-touch-icon" href="aset/icon512.png" />
//<link rel="manifest" href="./manifest.json" />
//<script>
// if ('serviceWorker' in navigator) {
// [code]
// else {
// }
//</body>
//</html>

fn parse2(file:String) {
	let html = "index.html" ;
}


#[derive(serde::Deserialize::<(_,)>,serde::Serialize)]
#[serde(remote = "Self")]
enum _ref_str {
	
	_ref(String),
	
	direc(String)
}


#[derive(serde::Deserialize::<(_,)>,serde::Serialize)]
#[serde(remote = "Self")]
enum data_tipe {
	
	void,
	
	i64,
	
	str(_ref_str::_ref)
}


#[derive(serde::Deserialize::<(_,)>,serde::Serialize)]
#[serde(remote = "Self")]
enum parse_code {
	
    _func(String,data_tipe /*,Vec<(String,u64)>*/),
	
    _let(String,data_tipe),
	
	_print(data_tipe)
}


fn parse(token:& Vec<String>,name:& String,file:& mut File,code :& mut Vec<parse_code> ) {
	let mut count = 0; 
	let stop = token.len()  ;
	let mut data = String::from("") ;
	println!("token: {:?}", token);
	while count < stop {
		if token[count].starts_with("$") {
			
			count += 1;
			code.push(parse_code::_func(
				format!("{}_{}",name,token[count]),
				data_tipe::void
			));
			/*
			data = format!("func ${}_{} {} {}",
			name,token[count],
				if token[count - 1].ends_with("i64") {
					"i64"
				} else {
					"void"
				},
				{
					""
				}
			) ;
			data.push('\n');
			file.write_all(data.as_bytes()).unwrap();
			*/
		} else if token[count] == "let" {
			count += 1;
			
			
			
			//data = format!("let {} ",token[count]) ;
			count += 1;
			if token[count] == "=" {
				count += 1;
				if token[count].starts_with("\"") {
					
					code.push(
						parse_code::_let( token[count].clone(),data_tipe::str(_ref_str::direc(token[count].clone())) )
					);
				}
				/*
				count += 1;
				if token[count].starts_with("\"") {
					data.push_str(&format!("str[{}] {}",token[count].len() - 2,token[count].clone()));
				}
				*/
			}
			/*
			data.push('\n');
			file.write_all(data.as_bytes()).unwrap();
			*/
			
		} else if token[count] == "print" { 
			count += 1;
			code.push(
				parse_code::_print(data_tipe::str(_ref_str::_ref(token[count].clone())))
			)
			/*
			count += 1;
			data = format!("print {} ",token[count]) ;
			
			data.push('\n');
			file.write_all(data.as_bytes()).unwrap();
			*/
		}
		
		count += 1;
	}
	//println!("code: {:?}", code);
	
	
}
fn tokenize(line:String,name:String,precompile:&String) -> Vec<String> {
	let mut file = File::create(precompile).unwrap();
	let mut token : Vec<String> = Vec::new();
	let mut x = false ;
	let mut buf = String::from("") ;
	let mut code : Vec<parse_code> = Vec::new();
	for i in line.chars() {
		if i == '\"' {
			x = !x
		}
		if x {
			buf.push(i);
			
		} else if i == '\n' {
			token.push(buf.clone());
			//if token.len() != 0 {
				parse(&token,&name,&mut file,&mut code);
				buf.clear();
				token.clear();
			//}
			
		} else if i != ' '  && i != '\t' {
			buf.push(i);
		} else if !buf.is_empty(){
			token.push(buf.clone());
			buf.clear();
		}
		
	}
	parse(&token,&name,&mut file,&mut code);
	let encoded: Vec<u8> = bincode::serialize(
		&code
	).unwrap();
	file.write_all( &encoded ).unwrap();
	return token
}

fn compile(code:File,path:&String,arg:&String) -> String {
	let name : Vec<&str>= path.split(&['/','\\']).collect() ;
	let name : String = format!("{}_{}",name[name.len() - 2],name[name.len() - 1] );
	let precompile = format!("{}/precompile/{}",arg,name) ;
	let precompile_sha = format!("{}/precompile/{}.sha256",arg,name) ;
	let mut data : String  = String::new();
	{
		let mut reader = BufReader::new(code);
		reader.read_to_string(&mut data).unwrap();
	}
	
    let hash = digest(&*data);
	if let Ok(mut file ) = File::open(&precompile_sha) {
		let mut last_hash = String::new();
		file.read_to_string(&mut last_hash).unwrap();
		if hash == last_hash {
			//return precompile
		}
	} 
	{
		
		let mut file = File::create(precompile_sha).unwrap();
		file.write_all(hash.as_bytes()).unwrap();
	}
	
	
	tokenize(data,name,&precompile);
	return precompile
	
}


fn build(arg:& String) {
	let paths = fs::read_dir(format!("{}/main",arg)).unwrap();
	let mut parse_2 = Vec::new() ;
	//let mut tread = Vec::new() ;
	//let mut builder = Builder::new() ;
	fs::create_dir_all(format!("{}/precompile",arg));
    for path in paths {
		//println!("Name: {}", path.as_ref().unwrap().path().display());
		 match path { 
			Ok(entry) => {
				if let Ok(mut code ) = File::open(entry.path()) { 
					
					//let handler = builder.spawn(move || {
					//	compile(code,&format!("{}", entry.path().display()) , arg)
					//}).unwrap();
					//tread.push(handler)
					parse_2.push( compile(code,&format!("{}", entry.path().display()) , arg) )
				}
			},
            Err(e) => println!("Error: {}", e),
		 }
		 
        
    }
	//for i in tread {
	//	i.join().unwrap();
	//}
	for i in parse_2 {
		parse2(i)
	}
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() { 
			"build"=>{
				if args.len() > 2 {
					build( &args[2] )
				} else {
					build( &"test".to_string() )
				}
				
			}
			"new"=>{
				fs::create_dir_all("test/main");
				if let Ok(mut file) = File::create(r"test/main/main") { 
					file.write_all(b"$void main\n\tlet hello = \"Hello World!\"\n\tprint hello");
				} else {
					println!("error");
				}
			}
			_ =>{}
		}
    }
}
