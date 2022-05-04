use std::io::Cursor;
use x86asm::{InstructionWriter, Mnemonic, Mode, Operand, Reg};
use std::io::Read;
use std::io::BufRead;
pub fn x86_64(
    terima: std::sync::mpsc::Receiver<crate::parsing::Pohon>,
    path: &String
){
    //let buffer = Cursor::new(Vec::new());
    //let mut writer = InstructionWriter::new(buffer, Mode::Protected);
    //let stack : Vec<[u64;4]> = Vec::with_capacity( 1 );
    for terima in terima {
        match terima {
            crate::parsing::Pohon::Let(nama,index) =>{

                //writer.write2(Mnemonic::, Operand::Direct(Reg::), Operand::Literal8(0)).unwrap();
                //writer.write2(Mnemonic::PUSH, Operand::Direct(Reg::), Operand::Literal8(0)).unwrap();
                //writer.write1(Mnemonic::PUSH, Operand::Literal8(0)).unwrap();
            }
            crate::parsing::Pohon::Cetak(args) =>{

            }
            crate::parsing::Pohon::tidur(args) =>{

            }
            crate::parsing::Pohon::tulis(a,b,c)=>{
                
            }
            crate::parsing::Pohon::panic(pesan) =>{

            }
            crate::parsing::Pohon::blok(_)=>{}
            crate::parsing::Pohon::blok_=>{}
            crate::parsing::Pohon::br(_)=>{}
            crate::parsing::Pohon::Selesai => {
                //writer.write0(Mnemonic::RET).unwrap();
                break
            }
            crate::parsing::Pohon::Error => {
                return
            }
        }
    }
    //let bit : &std::io::Cursor<Vec<u8>> = writer.get_inner_writer_ref() ;
    if let Ok(_) = std::fs::create_dir_all(format!("{}\\target\\debug\\x86_64",path)) {

    }
    /*std::fs::write(
        format!("{}\\target\\debug\\x86_64\\app.o",path), 
        bit.get_ref()
    ).unwrap();*/
}