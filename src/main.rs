use std::process::Command;
use std::io::Read;
use std::fs::File;
use std::io::Write;
use std::env;
use std::fs;
use sha256::digest;
use serde::{Serialize, Deserialize};
use std::thread::Builder;
use std::process::ExitStatus;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;
use std::thread::JoinHandle;
#[derive(Serialize, Deserialize, PartialEq, Debug,Clone)]
enum _RefeStr {
	_Ref(String),
	_Direc(String)
}
#[derive(Serialize, Deserialize, PartialEq, Debug,Clone)]
enum DataTipe {
	Void,
	I64(i64),
	Str(_RefeStr),
	Bool,
	Pointer(String)
}
/*-----Switch_case-----
	if _ {
		//code
		//break 
		goto there{index};
	}
	//Switch_
	there:
*/
#[derive(Serialize, Deserialize, PartialEq, Debug,Clone)]
enum Switch_case {
	eq,
	not,
	_i_,
	more,
	less,
	or,
	and
	
}
#[derive(Serialize, Deserialize, PartialEq, Debug,Clone)]
enum Entity {
    Func(DataTipe,String,Vec<Entity>),
	Func_call(String,Vec<(String,DataTipe)>),
	ret_value(Vec<String>),
    Let(String,DataTipe,bool),
	Let_mod(String,Vec<String>),
	Print(_RefeStr),
	/*
	File_write_all(_RefeStr),
	File_read_all(_RefeStr,DataTipe),
	File_(_RefeStr),
	*/
	Switch (Vec<DataTipe>),
	Case(Vec<Switch_case>),
	r#break,
	Switch_,
	r#loop(Vec<String>)
}
fn token_line(coder:&mut Vec<Entity>,token :Vec<String>)  {
	
	if token[0] == "let" || token[0] == "mut" {
		if let Entity::Func(_,_,coder) = coder.last_mut().unwrap() {
			if token[2] == "=" {
				if token[3].starts_with("\"") {
					coder.push(Entity::Let(
						token[1].clone(),
						DataTipe::Str(
							_RefeStr::_Direc(
								token[3].clone()
							)
						),
						token[0] == "mut"
					));
					return
				}
				if false {
					return
				}
			}
			if token[2] == "int" {
				
				return
			}
			if token[2] == "float" {
				
				return
			}
			if token[2] == "string" {
				if token[4].starts_with("\"") {
					coder.push(Entity::Let(
						token[1].clone(),
						DataTipe::Str(
							_RefeStr::_Direc(
								token[4].clone()
							)
						),
						token[0] == "mut"
					));
				}
				return
			}
		}
		return
	}
	if token[0] == "print" {
		if let Entity::Func(_,_,coder) = coder.last_mut().unwrap() {
			coder.push(Entity::Print(
				_RefeStr::_Ref(token[1].clone())
			))
		}
		return
	}
	if token[0] == "Switch" {
		if let Entity::Func(_,_,coder) = coder.last_mut().unwrap() {
			coder.push(Entity::Switch(Vec::new()));
		}
		return
	}
	if token[0] == "case" {
		if let Entity::Func(_,_,coder) = coder.last_mut().unwrap() {
			coder.push(Entity::Case(Vec::new()));
		}
		return
	}
	if token[0] == "break" {
		if let Entity::Func(_,_,coder) = coder.last_mut().unwrap() {
			coder.push(Entity::r#break);
		}
		return
	}
	if token[0] == "loop" {
		if let Entity::Func(_,_,coder) = coder.last_mut().unwrap() {
			
			let mut v = Vec::new();
			for n in 1..token.len() {
				v.push(token[n].clone());
			}
			coder.push(Entity::r#loop(v));
		}
		return
	}
	if token[0] == "@" {
		if let Entity::Func(_,_,coder) = coder.last_mut().unwrap() {
					
		let mut v = Vec::new();
		
		coder.push(Entity::Func_call(token[1].clone(),v));
		}
		return
	}
	if token[0] == "Switch_" {
		if let Entity::Func(_,_,coder) = coder.last_mut().unwrap() {
			coder.push(Entity::Switch_);
		}
		return
	}
	if token[0].starts_with("$") {
		if token[0].ends_with("void") {
			coder.push(Entity::Func(DataTipe::Void,token[1].clone(),Vec::new()));
		} else if token[0].ends_with("bool") {
			coder.push(Entity::Func(DataTipe::Bool,token[1].clone(),Vec::new()));
		}
		return
	}
	if token[0].starts_with("&") {
		if let Entity::Func(_,_,coder) = coder.last_mut().unwrap() {
			let mut v = Vec::new();
			for i in 3..token.len() {
				v.push(token[i].clone());
			}
			coder.push(Entity::Let_mod(token[1].clone(),v));
		}
		return
	}
	
	if token[0].starts_with("return") {
		if let Entity::Func(_,_,coder) = coder.last_mut().unwrap() {
			let mut v : Vec<String> = Vec::new() ; 
			for i in 1..token.len() {
				v.push(token[i].clone());
			}
			coder.push(Entity::ret_value(v));
		}
		return
	}
	
}
fn tokenizer(line:String,file : String) -> Vec<Entity> {
	let mut tread : Vec<JoinHandle<_>>= Vec::new() ;
	let mut coder : Arc<Mutex< Vec<Entity> >> = Arc::new(Mutex::new( Vec::new() ));
	let mut token : Vec<String> = Vec::new();
	let mut buf :String = String::from("") ;
	let mut collom : bool= false ;
	
	for i in line.chars() {
		if i == '\"' {
			collom = !collom
		}
		if i == '\n' {
			let mut token1 = token.clone() ;
			token1.push(buf.clone());
			buf.clear();
			token.clear();
			
			let coder = Arc::clone(&coder);
			let builder = Builder::new() ;
			if let Some(t) = tread.pop() {
				t.join().unwrap();
			}
			let handler : JoinHandle<_> = builder.spawn(move || {
				let mut coder = coder.lock().unwrap();
				token_line(&mut coder,token1);
			}).unwrap();
			tread.push(handler);
			
			
		} else if i == '@'{
			if !buf.is_empty() {
				token.push(buf.clone());
				buf.clear()
			}
			token.push("@".to_string());
		} else if collom || i != ' '  && i != '\t' {
			buf.push(i);
		} else if !buf.is_empty(){
			token.push(buf.clone());
			buf.clear()
		} 
		
		
	}
	for i in tread {
		i.join().unwrap();
	}
	let mut coder = coder.lock().unwrap();
	println!("{:?}",coder);
	let encoded: Vec<u8> = bincode::serialize(&coder.to_vec()).unwrap();
	let mut f = File::create(file).unwrap() ;
	println!("store precompile");
	f.write_all(&encoded).unwrap() ;
	return coder.to_vec()
}
fn token_optimize(full_token:Vec<Entity>) -> Vec<Entity> {
	let mut i = 0 ;
	let mut token = full_token;
	while i != token.len() {
		if let Entity::Func(_,_,coder) = &mut token[i] {
			
			for i in 0..coder.len() {
				let mut v : Option<DataTipe> = None;
				if i != 0 && let Entity::Let(_name,_value,_mut) = &mut coder[i] {
					
					let mut x = i ;
					let eq = _value.clone();
					loop {
						x -= 1;
						if let Entity::Let(_name,_value2,_mut) = & coder[x] {
							
							if let DataTipe::Str(value1) = &eq && let DataTipe::Str(value2) = _value2 {
								
								if let _RefeStr::_Direc(var1) = value1 && let _RefeStr::_Direc(var2) = value2 {
									if var1 == var2 {
										v = Some(DataTipe::Str( _RefeStr::_Ref(_name.to_string()) ));
										break;
									}
								}
							}
							
						}
						if x == 0 { break }
					}
					
				}
				if let Some(v) = v && let Entity::Let(_,_value,_) = &mut coder[i] {
					*_value = v
				}
			}
		}
		i += 1 ;
	}
	return token
}
fn build(args: &String,load_precompile:bool,run:bool) {
	let mut coder : Arc<Mutex< Vec<(String,Vec<Entity>)> >> = Arc::new(Mutex::new( Vec::new() ));
	let paths = fs::read_dir(format!("{}/main",args)).unwrap();
	let mut tread = Vec::new() ;
	//static mut LEVELS: Vec<String> = Vec::new();
	for path in paths { 
		match path { 
			Ok(ref entry) => {
				let entry = entry.path() ;
				let name : String = entry.display().to_string() ;
				let name : Vec<&str>= name.split(&['/','\\']).collect() ;
				let name : String = format!("{}_{}",name[name.len() - 2],name[name.len() - 1] );
				let precompile = format!("{}/precompile/{}",args,name) ;
				let precompile_sha = format!("{}/precompile/{}.sha256",args,name) ;
				let builder = Builder::new() ;
				let coder = Arc::clone(&coder) ;
				let handler = builder.spawn(move || {
					if let Ok(mut file ) = File::open(entry) {
						let mut data = "".to_string() ;
						file.read_to_string(&mut data).unwrap();
						let hash = digest(&*data);
						loop {
							if load_precompile {
								if let Ok(mut file ) = File::open(&precompile_sha) {
									let mut last_hash = String::new();
									file.read_to_string(&mut last_hash).unwrap();
									if hash == last_hash {
										if let Ok(mut file ) = File::open(&precompile) {
											data.clear();
											file.read_to_string(&mut data).unwrap();
											let decoded: Vec<Entity> = bincode::deserialize(&data.as_bytes()).unwrap();
											let mut coder = coder.lock().unwrap();
											
											coder.push((name.clone(),decoded));
											
											println!("load precompile from {}",precompile);
											break
										}
									}
								}
							}
							if !data.ends_with("\n") {
								data.push('\n')
							}
							println!("store precompile");
							let mut file = File::create(precompile_sha).unwrap();
							file.write_all(hash.as_bytes()).unwrap();
							let test = (name,token_optimize ( tokenizer(data,precompile) ));
							let mut coder = coder.lock().unwrap();
							
							coder.push(test);
							
							break
						}
						
					}
				});
				tread.push(handler);
				
			},
			Err(e) => println!("Error: {}", e)
		 }
	}
	for i in tread {
		i.expect("REASON").join().unwrap();
	}
	let mut f = false ;
	let file_prec = format!("{}/precompile/{}",args,"main.c") ;
	let mut file = File::create(file_prec).unwrap();
	file.write_all(b"#include <stdio.h>\n#include <stdbool.h>\n#include <string.h>\n").unwrap();
	let coder = coder.lock().unwrap();
	unsafe{ 
	for i in coder.iter() {
		let main_f = i.0 == "main_main"; 
		for mut x in &i.1 {
			if let Entity::Func(ret_type,name,code) = &mut x {
				let mut case : Vec<bool>= Vec::new();
				file.write_all( format!("{}{} {}(void){{\n",
					if f {"}"}else{f = true ;""},
					if main_f && name == "main" {"int"} 
						else if let DataTipe::Void = ret_type  {"void"} 
						else if let DataTipe::Bool = ret_type {"bool"}
						else { "" }
				,name,).as_bytes() ).unwrap();
				
				for i in 0..code.len() {
					match &code[i] {
						Entity::Let(name,code,mutation) => {
							if let DataTipe::Str(value) = code {
								if let _RefeStr::_Ref(var) = value {
									file.write_all( format!("char *{} = &{};\n",name,var).as_bytes() ).unwrap();
								} else if let _RefeStr::_Direc(var) = value {
									file.write_all( format!("char {}[{}] = {};\n",name,var.len() - 2,var).as_bytes() ).unwrap();
								}
							} else if let DataTipe::Pointer(name) = code {
								file.write_all(format!("{}=",name).as_bytes()).unwrap();
								
								file.write_all(b";\n").unwrap();
							}
							
						}
						Entity::Let_mod(name,V) => {
							
							for i in V{
								if i.starts_with("\"") {
									file.write_all( format!("strcat({},{});\n",name,i).as_bytes() ).unwrap();
									break
								}
							}
							
							
						}
						Entity::Print(code) =>{
							if let _RefeStr::_Ref(var) = code {
								file.write_all( format!("printf(\"%s\",{});\n",var).as_bytes() ).unwrap();
							} else if let _RefeStr::_Direc(var) = code {
								file.write_all( format!("printf(\"%s\",{});\n",var).as_bytes() ).unwrap();
							}
						}
						Entity::Case(v) => {
							if let Some(c) = case.last_mut() {
								if *c {
									file.write_all( "}".as_bytes()).unwrap();
								} else {
									*c = true
								}
							}			
							file.write_all( "if (true) {\n".as_bytes() ).unwrap();
						}
						Entity::Switch(v) => {
							case.push(false);
						}
						Entity::Switch_ => {
							if let Some(c) = case.last_mut() {
								if *c {
									file.write_all( "}".as_bytes()).unwrap();
								} 
								case.pop();
							}
							file.write_all( "there:\n".as_bytes() ).unwrap();
						}
						Entity::r#break => {
							file.write_all( "goto there;\n".as_bytes() ).unwrap();
						}
						Entity::r#loop(c) => {
							if c.len() == 0 {
								
							} else if c.len() == 1 {
								file.write_all( format!("while ({})\n",c[0]).as_bytes() ).unwrap();
							
							} else if c.len() <= 3 {
								if c[1].starts_with( "<" ) {
									file.write_all( format!("for (int {0} = 0; {0} < {1} ; ++{0})\n",c[0].clone(),c[2].clone()).as_bytes() ).unwrap();
								} else if c[1].starts_with( ">" ) {
									file.write_all( format!("for (int {0} = {1}; {0} == 0 ; --{0})\n",c[0].clone(),c[2].clone()).as_bytes() ).unwrap();
								} 
							} 
							
						}
						Entity::Func_call(name,v)=>{
							if code.len() != 1 {
								if let Entity::r#loop(c) = &code[i - 1] {
									if c.len() == 0 {
										if true {
											println!("========================loop {:?}",c);
											file.write_all( format!("while( {}() );\n",name).as_bytes() ).unwrap();
											continue
										}
									} else if c.len() <= 3 {
										if c[1].starts_with( ":" ) {
											//iter
											//for (unsigt int i = 0,  i < {iter len} ; ++{0} ) 
											//	{func name}({iter name}[i])
											continue
										}
									}
								}
							}
							file.write_all( format!("{}();\n",name).as_bytes() ).unwrap();
						}
						Entity::ret_value(v) => {
							file.write_all(b"return ").unwrap();
							for i in v {
								if true {
									file.write_all(i.as_bytes()).unwrap();
								}
							}
							file.write_all(b";\n").unwrap();
						}
						_ =>{}
					}
				}
			}
		}
		
		
		}
		
	}
	{file}.write_all(b"}\n").unwrap();
	let (windowsx86_64) : (bool)= target() ;
	
	if cfg!(target_os = "windows") {
		
		
		//windows & x86_64
		
		if windowsx86_64 || (run && cfg!(arget_arch = "x86_64")) {
			print!("\ncompile to windows x86_64 :");
			fs::create_dir_all(format!("{}/target/x86_64/windows",args)).unwrap();
			let window_64 = Command::new("cmd")
				.args(["/C",&format!("zig cc -o {0}/target/x86_64/windows/app.exe {0}/precompile/main.c -target x86_64-windows",args )])
				.output()
				.expect("failed to execute process")
				;
			if window_64.status.success() {
				print!(" Ok\n");
			} else {
				println!("Error \n{:?}\n",window_64);
			}
			
			
		}
		
		//linux & x86_64
		if false {
			print!("\ncompile to linux x86_64 :");
			fs::create_dir_all(format!("{}/target/x86_64/linux",args)).unwrap();
			let window_64 = Command::new("cmd")
				.args(["/C",&format!("zig cc -o {0}/target/x86_64/linux/app.x86_64 {0}/precompile/main.c -target x86_64-linux-gnu",args )])
				.output()
				.expect("failed to execute process")
				;
			
			if window_64.status.success() {
				print!(" Ok\n");
			} else {
				println!("Error \n{:?}\n",window_64);
			}
		}
		//website
		if false {
			print!("\ncompile to wasm32-wasi :");
			fs::create_dir_all(format!("{}/target/website",args)).unwrap();
			let window_64 = Command::new("cmd")
				.args(["/C",&format!("zig cc -o {0}/target/website/app.wasm {0}/precompile/main.c -target wasm32-wasi",args )])
				.output()
				.expect("failed to execute process")
				;
			if window_64.status.success() {
				print!(" Ok\n");
			} else {
				println!("Error \n{:?}\n",window_64);
			}
		}
		//
		if run {
			if cfg!(arget_arch = "x86_64") {
				let mut test = Command::new(&format!("{0}/target/x86_64/windows/app.exe",args)) ;
				let window_64 = &test
					.spawn()
					.expect("failed to execute process");
			}
		}
	}
	if cfg!(target_os = "linux") {
		//linux & x86_64
		if true || (run && cfg!(arget_arch = "x86_64")) {
			fs::create_dir_all(format!("{}/target/x86_64/linux",args)).unwrap();
		}
		//windows & x86_64
		if false {
			fs::create_dir_all(format!("{}/target/x86_64/windows",args)).unwrap();
		}
		//website
		if false {
			fs::create_dir_all(format!("{}/target/website",args)).unwrap();
		}
		if run {
			if cfg!(arget_arch = "x86_64") {
				
			}
		}
	}
	
}
fn target() -> (bool) {
	let mut windowsx86_64 = false ;
	for i in ["x86_64"] {
		if i == "x86_64" {
			windowsx86_64 = true
		}
	}
	return (windowsx86_64)
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let test = "./test".to_string();
	
    if args.len() > 1 {
		if args[1].starts_with("build") {
			build( if args.len() > 2 { &args[2] } else { &test } ,!args[1].ends_with("full") ,false);
			return;
		}
		if args[1].starts_with("run") {
			build( &test ,!args[1].ends_with("full"),true);
			return;
		}
		if args[1].starts_with("new") {
			match fs::create_dir_all("test/main") { 
				Ok(_entry) => {}
				Err(e) => println!("Error: {}", e),
			}
			match File::create(r"test/main/main") {
				Ok(mut file) => {
					match file.write_all(b"$void main\n\tlet hello = \"Hello World!\"\n\tprint hello") { 
						Ok(_entry) => {}
						Err(e) => println!("Error: {}", e),
					}
				}
				Err(e) => println!("Error: {}", e),
			}
			return;
		}
		
		/*
        match args[1].as_str() { 
			"build"=>{
				println!("build");
				build( if args.len() > 2 { &args[2] } else { &test } ,true,false)
				
			}
			"build_full" => {
				println!("build");
				
				build( if args.len() > 2 { &args[2] } else { &test } ,false,false)
			}
			"run" => {
				build( &test ,true,true)
			}
			"new"=>{
				match fs::create_dir_all("test/main") { 
					Ok(_entry) => {}
					Err(e) => println!("Error: {}", e),
				}
				if let Ok(mut file) = File::create(r"test/main/main") { 
					match file.write_all(b"$void main\n\tlet hello = \"Hello World!\"\n\tprint hello") { 
						Ok(_entry) => {}
						Err(e) => println!("Error: {}", e),
					}
					
				} else {
					println!("error");
				}
			}
			_ =>{}
		}
		*/
    }
	println!("<=========================================================>");
	println!("USAGE:\n\tindosanca [+toolchain] [OPTIONS] [SUBCOMMAND]");
	println!("\ttoolchain:");
	println!("\t\tbuild       Compile the current package if need");
	println!("\t\tbuildfull   Compile the current full package");
	println!("\t\trun         Compile the current package if need and run app");
	println!("\t\trunfull     Compile the current full package and run app");
	println!("\t\tnew         Create a new package");
	println!("<=========================================================>");
	
}