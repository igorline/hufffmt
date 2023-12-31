use lalrpop_util::lalrpop_mod;

mod instruction;
mod opcode;

lalrpop_mod!(pub opcodes); // synthesized by LALRPOP
lalrpop_mod!(pub instructions); // synthesized by LALRPOP

fn main() {}

// #[test]
// fn opcodes() {
//     println!("{:?}", opcodes::OpcodeParser::new().parse("shr"));
// }
//
#[test]
fn instructions() {
    println!(
        "{}",
        instructions::SourceUnitParser::new()
            .parse("0x10     0xab shr")
            .unwrap()
    );
    println!(
        "{}",
        instructions::SourceUnitParser::new()
            .parse("add add")
            .unwrap()
    );
    println!(
        "{}",
        instructions::SourceUnitParser::new()
            .parse("push1 push1")
            .unwrap()
    );
    println!(
        "{}",
        instructions::SourceUnitParser::new()
            .parse(
                r#"
    sload
    dup3
    swap6


    dup2
    dup4
    gt
    or
    0x10 jumpi
    sub
    dup4
    sstore
    sload
    dup2
    add
    dup4
    sstore
    "#
            )
            .unwrap()
    );
}

// #[test]
// fn whitespaces() {
//     println!(
//         "{:?}",
//         instructions::WhitespaceKindsParser::new().parse(
//             r#"
//
//     "#
//         )
//     );
// }
