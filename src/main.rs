use std::io;
use std::io::BufRead;
use std::collections::HashMap;

use simple_interpreter::{Token, Token::*};
use simple_interpreter::Interpreter;
use cell_automata_1d::CellOperator;

fn main() {
    let literal_token_map = HashMap::from([
        ("&", AndOp),
        ("|", OrOp),
        ("^", XorOp),
        ("~", NotOp),
        ("=", EqOp),
        ("(", LeftBrace),
        (")", RightBrace) 
    ]);

    let oper_prioriry_map = HashMap::from([
        (EqOp, 1),
        (OrOp, 2),
        (XorOp, 2),
        (AndOp, 3),
        (NotOp, 4)
    ]);

    let oper_args_count_map = HashMap::from([
        (OrOp, 2),
        (XorOp, 2),
        (AndOp, 2),
        (EqOp, 2),
        (NotOp, 1)
    ]);

    let mut or_op = CellOperator::new(0xfc);
    let mut xor_op = CellOperator::new(0x3c);
    let mut and_op = CellOperator::new(0xc0);
    let mut not_op = CellOperator::new(0x33);
    let mut eq_op = CellOperator::new(0xc3);

    let eval_oper_with_automata = |stack: &mut Vec<u8>, op: &Token| -> Result<u8, String> {
        match op {
            NotOp => {
                let x = stack.pop().unwrap();
                Ok(not_op.eval(0x00, x))
            }
            _ => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                match op {
                    OrOp => Ok(or_op.eval(lhs, rhs)),
                    XorOp => Ok(xor_op.eval(lhs, rhs)),
                    AndOp => Ok(and_op.eval(lhs, rhs)),
                    EqOp => Ok(eq_op.eval(lhs, rhs)),
                    _ => Err(format!("Logic error: unhandled operation {op}!"))
                }
            }
        }
    };

    let mut interpreter = Interpreter::new(
        literal_token_map,
        oper_prioriry_map,
        oper_args_count_map,
        eval_oper_with_automata
    );

    let mut lines = io::stdin().lock().lines();

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        if last_input.len() == 0 {
            break;
        }

        match interpreter.evaluate(&(last_input.trim())) {
            Ok(val) => println!("> {val:08b} ({val:x})"),
            Err(str) => println!("! {str}")
        }
    }
}
