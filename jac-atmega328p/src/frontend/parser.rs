#[derive(Debug, Clone)]
pub struct Program(pub Vec<Item>);

peg::parser! {
    grammar assembler_parser() for str {
        rule _() =  quiet!{___+}
        rule __() = quiet!{ ____()+ }
        rule ___() = [' ' | '\n' | '\t']
        rule ____() = [' ' | '\t']

        pub rule program() -> Program = _? items:item()* { Program(items) }

        rule item() -> Item = ldi()
        rule register() -> Register
            = r16()
            / r17()
            / r18()
            / r19()
            / r20()
            / r21()
            / r22()
            / r23()
            / r24()
            / r25()
            / r26()
            / r27()
            / r28()
            / r29()
            / r30()
            / r31()

        rule number() -> u16
            = n:$(['0'..='9']+) { n.parse::<u16>().unwrap() }

        rule ldi() -> Item =
            "ldi"
            _
            reg: register()
            _?
            ","
            _?
            imm: number()
            _?
        {
            Item::Ldi(reg, imm)
        }


    

        rule r16() -> Register = "r16" {
            Register::R16
        }

        rule r17() -> Register = "r17" {
            Register::R17
        }

        rule r18() -> Register = "r18" {
            Register::R18
        }

        rule r19() -> Register = "r19" {
            Register::R19
        }

        rule r20() -> Register = "r20" {
            Register::R20
        }

        rule r21() -> Register = "r21" {
            Register::R21
        }

        rule r22() -> Register = "r22" {
            Register::R22
        }

        rule r23() -> Register = "r23" {
            Register::R23
        }

        rule r24() -> Register = "r24" {
            Register::R24
        }

        rule r25() -> Register = "r25" {
            Register::R25
        }

        rule r26() -> Register = "r26" {
            Register::R26
        }

        rule r27() -> Register = "r27" {
            Register::R27
        }

        rule r28() -> Register = "r28" {
            Register::R28
        }

        rule r29() -> Register = "r29" {
            Register::R29
        }

        rule r30() -> Register = "r30" {
            Register::R30
        }

        rule r31() -> Register = "r31" {
            Register::R31
        }
    }



}

pub use assembler_parser::program;

#[derive(Debug, Clone)]
pub enum Item {
    Ldi(Register, u16),
}

// R16-R31
#[repr(u8)]
#[derive(Clone, Debug)]
pub enum Register {
    R16,
    R17,
    R18,
    R19,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R30,
    R31,
}
