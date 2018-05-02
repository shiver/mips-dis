use std::fmt;

use num_traits::FromPrimitive;

use bitrange::BitRange;

// General Purpose Register
type GPRType = u8;

// Coprocessor Register
type CPRType = u8;

// const OP_LUI_COMMENT: &str = "load upper immediate";
// const OP_MTC0_COMMENT: &str = "move from gpr rt to cp0 rd";
// const OP_ORI_COMMENT: &str = "bitwise logical OR with constant";
// const OP_LW_COMMENT: &str = "load word from memory as signed value";
// const OP_ANDI_COMMENT: &str = "bitwise logical AND with constant";

#[derive(Primitive)]
enum OpCode {
    ANDI = 0xc,
    BEQL = 0x14,
    LUI = 0xf,
    LW = 0x23,
    MTC0 = 0x10,
    ORI = 0xd,
}

#[derive(Debug)]
pub struct ANDIVars {
    rs: GPRType,
    rt: GPRType,
    immediate: u16,
}

#[derive(Debug)]
struct BEQLVars {
    rs: GPRType,
    rt: GPRType,
    offset: i32,
}

#[derive(Debug)]
struct LUIVars {
    rt: GPRType,
    immediate: u16,
}

#[derive(Debug)]
struct LWVars {
    base: GPRType,
    rt: GPRType,
    offset: u16,
}

#[derive(Debug)]
struct MTC0Vars {
    rt: GPRType,
    rd: CPRType,
    sel: u8,
}

#[derive(Debug)]
struct ORIVars {
    rs: GPRType,
    rt: GPRType,
    immediate: u16,
}

#[derive(Debug)]
pub enum Instruction {
    ANDI(ANDIVars),
    BEQL(BEQLVars),
    LUI(LUIVars),
    LW(LWVars),
    MTC0(MTC0Vars),
    ORI(ORIVars),
}

impl Instruction {
    pub fn from_u32(value: u32) -> Option<Instruction> {
        match OpCode::from_u8(value.range_u8(26..31)) {
            Some(OpCode::ANDI) => {
                let vars = ANDIVars {
                    rs: value.range_u8(21..25),
                    rt: value.range_u8(16..20),
                    immediate: value.range_u16(0..15),
                };
                Some(Instruction::ANDI(vars))
            }

            // TODO: Not correctly handling the offset value 
            Some(OpCode::BEQL) => {
                let vars = BEQLVars {
                    rs: value.range_u8(21..25),
                    rt: value.range_u8(16..20),
                    offset: (value.range_u16(0..15) as i32) << 2,
                };
                Some(Instruction::BEQL(vars))
            }

            Some(OpCode::LUI) => {
                let vars = LUIVars {
                    rt: value.range_u8(21..25),
                    immediate: value.range_u16(0..15),
                };
                Some(Instruction::LUI(vars))
            }

            Some(OpCode::LW) => {
                let vars = LWVars {
                    base: value.range_u8(21..25),
                    rt: value.range_u8(16..20),
                    offset: value.range_u16(0..15),
                };
                Some(Instruction::LW(vars))
            }

            Some(OpCode::MTC0) => {
                let vars = MTC0Vars {
                    rt: value.range_u8(16..20),
                    rd: value.range_u8(11..15),
                    sel: value.range_u8(0..3),
                };
                Some(Instruction::MTC0(vars))
            }

            Some(OpCode::ORI) => {
                let vars = ORIVars {
                    rs: value.range_u8(21..25),
                    rt: value.range_u8(16..20),
                    immediate: value.range_u16(0..15),
                };
                Some(Instruction::ORI(vars))
            }

            _ => None,
        }
    }
}

fn instruction_format(instruction: &Instruction) -> String {
    match instruction {
        Instruction::ANDI(ANDIVars { rt, rs, immediate }) => format!(
            "andi r{rt}, r{rs}, {immediate:#x}",
            rt = rt,
            rs = rs,
            immediate = immediate
        ),

        Instruction::BEQL(BEQLVars { rt, rs, offset }) => format!(
            "beql r{rs}, r{rt}, {offset:#x}",
            rs = rs,
            rt = rt,
            offset = offset
        ),

        Instruction::LUI(LUIVars { rt, immediate }) => {
            format!("lui r{rt}, {immediate:#x}", rt = rt, immediate = immediate)
        }

        Instruction::LW(LWVars { rt, offset, base }) => format!(
            "lw r{rt}, {offset}({base})",
            rt = rt,
            offset = offset,
            base = base
        ),

        Instruction::MTC0(MTC0Vars { rt, rd, sel }) => {
            format!("mtc0 r{rt}, c{rd}, {sel:#x}", rt = rt, rd = rd, sel = sel)
        }

        Instruction::ORI(ORIVars { rt, rs, immediate }) => {
            format!("ori r{rt}, r{rs}, {immediate:#x}", rt = rt, rs = rs, immediate = immediate)
        }

        _ => format!("Nope"),
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", instruction_format(&self))
    }
}
