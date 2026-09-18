//! Bytecode opcodes for the AI-Lang VM.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    Halt = 0,
    Push = 1,
    PushF64 = 2,
    PushTensor = 3,
    Pop = 4,
    Add = 5,
    Sub = 6,
    Mul = 7,
    Div = 8,
    Neg = 9,
    Eq = 10,
    NotEq = 11,
    Lt = 12,
    Gt = 13,
    Print = 14,
    Store = 15,
    Load = 16,
    Jump = 17,
    JumpIfFalse = 18,
    Call = 19,
    Ret = 20,
    ListNew = 21,
    ListPush = 22,
    ListLen = 23,
    ListGet = 24,
    // ... additional ops in full source
}

impl OpCode {
    pub fn decode(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(OpCode::Halt),
            1 => Some(OpCode::Push),
            2 => Some(OpCode::PushF64),
            3 => Some(OpCode::PushTensor),
            4 => Some(OpCode::Pop),
            5 => Some(OpCode::Add),
            6 => Some(OpCode::Sub),
            7 => Some(OpCode::Mul),
            8 => Some(OpCode::Div),
            9 => Some(OpCode::Neg),
            10 => Some(OpCode::Eq),
            11 => Some(OpCode::NotEq),
            12 => Some(OpCode::Lt),
            13 => Some(OpCode::Gt),
            14 => Some(OpCode::Print),
            15 => Some(OpCode::Store),
            16 => Some(OpCode::Load),
            17 => Some(OpCode::Jump),
            18 => Some(OpCode::JumpIfFalse),
            19 => Some(OpCode::Call),
            20 => Some(OpCode::Ret),
            21 => Some(OpCode::ListNew),
            22 => Some(OpCode::ListPush),
            23 => Some(OpCode::ListLen),
            24 => Some(OpCode::ListGet),
            _ => None,
        }
    }

    pub fn operand_size(self) -> usize {
        match self {
            OpCode::Push | OpCode::PushF64 | OpCode::PushTensor
            | OpCode::Store | OpCode::Load
            | OpCode::Jump | OpCode::JumpIfFalse | OpCode::Call => 8,
            _ => 0,
        }
    }
}
