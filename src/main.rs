extern crate byteorder;
#[macro_use] 
extern crate enum_primitive_derive;
extern crate num_traits;

use std::fs::File;
use std::io::{Cursor, Error, Read};
use std::env;

use byteorder::{BigEndian, ReadBytesExt};
use num_traits::FromPrimitive;

mod bitrange;
use bitrange::BitRange;

mod opcodes;
use opcodes::Instruction;

fn read_bin(filename: &String) -> Result<Vec<u8>, Error> {
    let mut file = File::open(filename)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

fn display_n_instructions(num: usize, cursor: &mut Cursor<Vec<u8>>) {
    for n in 0..num {
        let raw = cursor.read_u32::<BigEndian>().unwrap();
        if let Some(instruction) = Instruction::from_u32(raw) {
            println!("{}", instruction);
        } else {
            let opcode = raw.range_u8(26..31);
            println!("{:06b} {:#x}", opcode, opcode);
        }
    }
}

fn main() {
    let filename = env::args().nth(1).expect("filename?");
    let data = read_bin(&filename).unwrap();

    let mut cursor = Cursor::new(data);
    display_n_instructions(100, &mut cursor);
}
