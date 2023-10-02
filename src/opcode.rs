/// References <https://evm.codes>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Opcode {
    /// Halts execution.
    Stop,
    /// Addition operation
    Add,
    /// Multiplication Operation
    Mul,
    /// Subtraction Operation
    Sub,
    /// Integer Division Operation
    Div,
    /// Signed Integer Division Operation
    Sdiv,
    /// Modulo Remainder Operation
    Mod,
    /// Signed Modulo Remainder Operation
    Smod,
    /// Modulo Addition Operation
    Addmod,
    /// Modulo Multiplication Operation
    Mulmod,
    /// Exponential Operation
    Exp,
    /// Extend Length of Two's Complement Signed Integer
    Signextend,
    /// Less-than Comparison
    Lt,
    /// Greater-than Comparison
    Gt,
    /// Signed Less-than Comparison
    Slt,
    /// Signed Greater-than Comparison
    Sgt,
    /// Equality Comparison
    Eq,
    /// Not Operation
    Iszero,
    /// Bitwise AND Operation
    And,
    /// Bitwise OR Operation
    Or,
    /// Bitwise XOR Operation
    Xor,
    /// Bitwise NOT Operation
    Not,
    /// Retrieve Single Byte from Word
    Byte,
    /// Left Shift Operation
    Shl,
    /// Right Shift Operation
    Shr,
    /// Arithmetic Shift Right Operation
    Sar,
    /// Compute the Keccak-256 hash of a 32-byte word
    Sha3,
    /// Address of currently executing account
    Address,
    /// Balance of a given account
    Balance,
    /// Address of execution origination
    Origin,
    /// Address of the caller
    Caller,
    /// Value of the call
    Callvalue,
    /// Loads Calldata
    Calldataload,
    /// Size of the Calldata
    Calldatasize,
    /// Copies the Calldata to Memory
    Calldatacopy,
    /// Size of the Executing Code
    Codesize,
    /// Copies Executing Code to Memory
    Codecopy,
    /// Current Price of Gas
    Gasprice,
    /// Size of an Account's Code
    Extcodesize,
    /// Copies an Account's Code to Memory
    Extcodecopy,
    /// Size of Output Data from Previous Call
    Returndatasize,
    /// Copies Output Data from Previous Call to Memory
    Returndatacopy,
    /// Hash of a Block from the most recent 256 blocks
    Blockhash,
    /// The Current Blocks Beneficiary Address
    Coinbase,
    /// The Current Blocks Timestamp
    Timestamp,
    /// The Current Blocks Number
    Number,
    /// The Current Blocks Difficulty
    Difficulty,
    /// Pseudorandomness from the Beacon Chain
    Prevrandao,
    /// The Current Blocks Gas Limit
    Gaslimit,
    /// The Chain ID
    Chainid,
    /// Balance of the Currently Executing Account
    Selfbalance,
    /// Base Fee
    Basefee,
    /// Removes an Item from the Stack
    Pop,
    /// Loads a word from Memory
    Mload,
    /// Stores a word in Memory
    Mstore,
    /// Stores a byte in Memory
    Mstore8,
    /// Load a word from Storage
    Sload,
    /// Store a word in Storage
    Sstore,
    /// Alter the Program Counter
    Jump,
    /// Conditionally Alter the Program Counter
    Jumpi,
    /// Value of the Program Counter Before the Current Instruction
    Pc,
    /// Size of Active Memory in Bytes
    Msize,
    /// Amount of available gas including the cost of the current instruction
    Gas,
    /// Marks a valid destination for jumps
    Jumpdest,
    /// Places a zero on top of the stack
    Push0,
    /// Places 1 byte item on top of the stack
    Push1,
    /// Places 2 byte item on top of the stack
    Push2,
    /// Places 3 byte item on top of the stack
    Push3,
    /// Places 4 byte item on top of the stack
    Push4,
    /// Places 5 byte item on top of the stack
    Push5,
    /// Places 6 byte item on top of the stack
    Push6,
    /// Places 7 byte item on top of the stack
    Push7,
    /// Places 8 byte item on top of the stack
    Push8,
    /// Places 9 byte item on top of the stack
    Push9,
    /// Places 10 byte item on top of the stack
    Push10,
    /// Places 11 byte item on top of the stack
    Push11,
    /// Places 12 byte item on top of the stack
    Push12,
    /// Places 13 byte item on top of the stack
    Push13,
    /// Places 14 byte item on top of the stack
    Push14,
    /// Places 15 byte item on top of the stack
    Push15,
    /// Places 16 byte item on top of the stack
    Push16,
    /// Places 17 byte item on top of the stack
    Push17,
    /// Places 18 byte item on top of the stack
    Push18,
    /// Places 19 byte item on top of the stack
    Push19,
    /// Places 20 byte item on top of the stack
    Push20,
    /// Places 21 byte item on top of the stack
    Push21,
    /// Places 22 byte item on top of the stack
    Push22,
    /// Places 23 byte item on top of the stack
    Push23,
    /// Places 24 byte item on top of the stack
    Push24,
    /// Places 25 byte item on top of the stack
    Push25,
    /// Places 26 byte item on top of the stack
    Push26,
    /// Places 27 byte item on top of the stack
    Push27,
    /// Places 28 byte item on top of the stack
    Push28,
    /// Places 29 byte item on top of the stack
    Push29,
    /// Places 30 byte item on top of the stack
    Push30,
    /// Places 31 byte item on top of the stack
    Push31,
    /// Places 32 byte item on top of the stack
    Push32,
    /// Duplicates the first stack item
    Dup1,
    /// Duplicates the 2nd stack item
    Dup2,
    /// Duplicates the 3rd stack item
    Dup3,
    /// Duplicates the 4th stack item
    Dup4,
    /// Duplicates the 5th stack item
    Dup5,
    /// Duplicates the 6th stack item
    Dup6,
    /// Duplicates the 7th stack item
    Dup7,
    /// Duplicates the 8th stack item
    Dup8,
    /// Duplicates the 9th stack item
    Dup9,
    /// Duplicates the 10th stack item
    Dup10,
    /// Duplicates the 11th stack item
    Dup11,
    /// Duplicates the 12th stack item
    Dup12,
    /// Duplicates the 13th stack item
    Dup13,
    /// Duplicates the 14th stack item
    Dup14,
    /// Duplicates the 15th stack item
    Dup15,
    /// Duplicates the 16th stack item
    Dup16,
    /// Exchange the top two stack items
    Swap1,
    /// Exchange the first and third stack items
    Swap2,
    /// Exchange the first and fourth stack items
    Swap3,
    /// Exchange the first and fifth stack items
    Swap4,
    /// Exchange the first and sixth stack items
    Swap5,
    /// Exchange the first and seventh stack items
    Swap6,
    /// Exchange the first and eighth stack items
    Swap7,
    /// Exchange the first and ninth stack items
    Swap8,
    /// Exchange the first and tenth stack items
    Swap9,
    /// Exchange the first and eleventh stack items
    Swap10,
    /// Exchange the first and twelfth stack items
    Swap11,
    /// Exchange the first and thirteenth stack items
    Swap12,
    /// Exchange the first and fourteenth stack items
    Swap13,
    /// Exchange the first and fifteenth stack items
    Swap14,
    /// Exchange the first and sixteenth stack items
    Swap15,
    /// Exchange the first and seventeenth stack items
    Swap16,
    /// Append Log Record with no Topics
    Log0,
    /// Append Log Record with 1 Topic
    Log1,
    /// Append Log Record with 2 Topics
    Log2,
    /// Append Log Record with 3 Topics
    Log3,
    /// Append Log Record with 4 Topics
    Log4,
    /// Transaction-persistent, but storage-ephemeral variable load
    TLoad,
    /// Transaction-persistent, but storage-ephemeral variable store
    TStore,
    /// Create a new account with associated code
    Create,
    /// Message-call into an account
    Call,
    /// Message-call into this account with an alternative accounts code
    Callcode,
    /// Halt execution, returning output data
    Return,
    /// Message-call into this account with an alternative accounts code, persisting the sender and
    /// value
    Delegatecall,
    /// Create a new account with associated code
    Create2,
    /// Static Message-call into an account
    Staticcall,
    /// Halt execution, reverting state changes, but returning data and remaining gas
    Revert,
    /// Invalid Instruction
    Invalid,
    /// Halt Execution and Register Account for later deletion
    Selfdestruct,
    /// Get hash of an account’s code
    Extcodehash,
}

impl Opcode {
    pub fn as_static_str(&self) -> &'static str {
        match self {
            Opcode::Lt => "lt",
            Opcode::Gt => "gt",
            Opcode::Slt => "slt",
            Opcode::Sgt => "sgt",
            Opcode::Eq => "eq",
            Opcode::Iszero => "iszero",
            Opcode::And => "and",
            Opcode::Or => "or",
            Opcode::Xor => "xor",
            Opcode::Not => "not",
            Opcode::Sha3 => "sha3",
            Opcode::Address => "address",
            Opcode::Balance => "balance",
            Opcode::Origin => "origin",
            Opcode::Caller => "caller",
            Opcode::Callvalue => "callvalue",
            Opcode::Calldataload => "calldataload",
            Opcode::Calldatasize => "calldatasize",
            Opcode::Calldatacopy => "calldatacopy",
            Opcode::Codesize => "codesize",
            Opcode::Codecopy => "codecopy",
            Opcode::Basefee => "basefee",
            Opcode::Blockhash => "blockhash",
            Opcode::Coinbase => "coinbase",
            Opcode::Timestamp => "timestamp",
            Opcode::Number => "number",
            Opcode::Difficulty => "difficulty",
            Opcode::Prevrandao => "prevrandao",
            Opcode::Gaslimit => "gaslimit",
            Opcode::Chainid => "chainid",
            Opcode::Selfbalance => "selfbalance",
            Opcode::Pop => "pop",
            Opcode::Mload => "mload",
            Opcode::Mstore => "mstore",
            Opcode::Mstore8 => "mstore8",
            Opcode::Sload => "sload",
            Opcode::Sstore => "sstore",
            Opcode::Jump => "jump",
            Opcode::Jumpi => "jumpi",
            Opcode::Pc => "pc",
            Opcode::Msize => "msize",
            Opcode::Push0 => "push0",
            Opcode::Push1 => "push1",
            Opcode::Push2 => "push2",
            Opcode::Push3 => "push3",
            Opcode::Push4 => "push4",
            Opcode::Push5 => "push5",
            Opcode::Push6 => "push6",
            Opcode::Push7 => "push7",
            Opcode::Push8 => "push8",
            Opcode::Push9 => "push9",
            Opcode::Push10 => "push10",
            Opcode::Push17 => "push17",
            Opcode::Push18 => "push18",
            Opcode::Push19 => "push19",
            Opcode::Push20 => "push20",
            Opcode::Push21 => "push21",
            Opcode::Push22 => "push22",
            Opcode::Push23 => "push23",
            Opcode::Push24 => "push24",
            Opcode::Push25 => "push25",
            Opcode::Push26 => "push26",
            Opcode::Dup1 => "dup1",
            Opcode::Dup2 => "dup2",
            Opcode::Dup3 => "dup3",
            Opcode::Dup4 => "dup4",
            Opcode::Dup5 => "dup5",
            Opcode::Dup6 => "dup6",
            Opcode::Dup7 => "dup7",
            Opcode::Dup8 => "dup8",
            Opcode::Dup9 => "dup9",
            Opcode::Dup10 => "dup10",
            Opcode::Swap1 => "swap1",
            Opcode::Swap2 => "swap2",
            Opcode::Swap3 => "swap3",
            Opcode::Swap4 => "swap4",
            Opcode::Swap5 => "swap5",
            Opcode::Swap6 => "swap6",
            Opcode::Swap7 => "swap7",
            Opcode::Swap8 => "swap8",
            Opcode::Swap9 => "swap9",
            Opcode::Swap10 => "swap10",
            Opcode::Stop => "stop",
            Opcode::Add => "add",
            Opcode::Mul => "mul",
            Opcode::Sub => "sub",
            Opcode::Div => "div",
            Opcode::Sdiv => "sdiv",
            Opcode::Mod => "mod",
            Opcode::Smod => "smod",
            Opcode::Addmod => "addmod",
            Opcode::Mulmod => "mulmod",
            Opcode::Exp => "exp",
            Opcode::Signextend => "signextend",
            Opcode::Byte => "byte",
            Opcode::Shl => "shl",
            Opcode::Shr => "shr",
            Opcode::Sar => "sar",
            Opcode::Gasprice => "gasprice",
            Opcode::Extcodesize => "extcodesize",
            Opcode::Extcodecopy => "extcodecopy",
            Opcode::Returndatasize => "returndatasize",
            Opcode::Returndatacopy => "returndatacopy",
            Opcode::Extcodehash => "extcodehash",
            Opcode::Gas => "gas",
            Opcode::Jumpdest => "jumpdest",
            Opcode::Push11 => "push11",
            Opcode::Push12 => "push12",
            Opcode::Push13 => "push13",
            Opcode::Push14 => "push14",
            Opcode::Push15 => "push15",
            Opcode::Push16 => "push16",
            Opcode::Push27 => "push27",
            Opcode::Push28 => "push28",
            Opcode::Push29 => "push29",
            Opcode::Push30 => "push30",
            Opcode::Push31 => "push31",
            Opcode::Push32 => "push32",
            Opcode::Dup11 => "dup11",
            Opcode::Dup12 => "dup12",
            Opcode::Dup13 => "dup13",
            Opcode::Dup14 => "dup14",
            Opcode::Dup15 => "dup15",
            Opcode::Dup16 => "dup16",
            Opcode::Swap11 => "swap11",
            Opcode::Swap12 => "swap12",
            Opcode::Swap13 => "swap13",
            Opcode::Swap14 => "swap14",
            Opcode::Swap15 => "swap15",
            Opcode::Swap16 => "swap16",
            Opcode::Log0 => "log0",
            Opcode::Log1 => "log1",
            Opcode::Log2 => "log2",
            Opcode::Log3 => "log3",
            Opcode::Log4 => "log4",
            Opcode::TLoad => "tLoad",
            Opcode::TStore => "tStore",
            Opcode::Create => "create",
            Opcode::Call => "call",
            Opcode::Callcode => "callcode",
            Opcode::Return => "return",
            Opcode::Delegatecall => "delegatecall",
            Opcode::Staticcall => "staticcall",
            Opcode::Create2 => "create2",
            Opcode::Revert => "revert",
            Opcode::Invalid => "invalid",
            Opcode::Selfdestruct => "selfdestruct",
        }
    }
}
