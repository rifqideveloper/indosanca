use std::io::Write;
fn print(offset:u32,len:u32,func:&str) -> String{
    format!("i32.const {0}\ni32.const {1}\ncall {2}\n",offset,len,func)
}
fn data(offset:u32,da:&str) -> String {
    format!("(data (i32.const {}) {})",offset,da)
}
pub fn wat(path:&String){
    const ADD :&[u8; 124]= b"(func $add (param $lhs i32) (param $rhs i32) (result i32)\n\
                local.get $lhs\n\
                local.get $rhs\n\
                i32.add)\n\
                (export \"add\" (func $add))\n\
    ";
    
    {
        let mut f = std::io::BufWriter::with_capacity(
            1000, 
            std::fs::File::create(format!("{}\\parsing\\main.wat",path)).unwrap()
        );
    
        f.write(b"(module\n").unwrap();
        f.write(b"(import \"import\" \"log\" (func $log (param i32 i32)))\n(import \"import\"\"mem\" (memory 1))\n").unwrap();
        f.write(ADD).unwrap();
        f.write(b"\
        (data (i32.const 0) \"Hi\")\n\
        (func (export \"main\")\n\
        i32.const 0\n\
        i32.const 2\n\
        call $log\n\
        )").unwrap();
        f.write(b"\n)").unwrap();
    }
    std::io::BufWriter::with_capacity(
        1000,
        std::fs::File::create(format!("{}\\target\\www\\main.wasm",path)).unwrap()
    ).write_all(
        &wat::parse_file(format!("{}\\parsing\\main.wat",path)).unwrap()
    ).unwrap();
}