use anyhow::{anyhow, Context, Result};
use std::io::{stdin, Read};
use std::string::ToString;

#[derive(Copy, Clone, Debug)]
enum Param {
    Position(usize),
    Immediate(isize),
}

impl Param {
    fn resolve_value(&self, prog: &[isize]) -> Result<isize> {
        match self {
            Self::Position(at) => {
                if *at >= prog.len() {
                    return Err(anyhow!("param out of range"));
                }
                Ok(prog[*at])
            }
            Self::Immediate(val) => Ok(*val),
        }
    }

    fn position(&self) -> Result<usize> {
        match self {
            Self::Position(at) => Ok(*at),
            _ => Err(anyhow!("expected position mode")),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Add { x: Param, y: Param, dest: Param },
    Mul { x: Param, y: Param, dest: Param },
    In { dest: Param },
    Out { val: Param },
    Exit,
}

impl Instruction {
    fn parse(input: &[isize], at: usize) -> Result<Self> {
        if at >= input.len() {
            return Err(anyhow!("at out of range"));
        }
        let op = Op::parse(input[at]).context("failed to parse op")?;
        let params: Vec<Param> = op
            .param_modes
            .iter()
            .enumerate()
            .map(|(index, pm)| {
                let val = input[at + index + 1];
                match pm {
                    ParamMode::Position => Param::Position(val as usize),
                    ParamMode::Immediate => Param::Immediate(val),
                }
            })
            .collect();
        Ok(match op.code {
            OP_CODE_ADD => Instruction::Add {
                x: params[0],
                y: params[1],
                dest: params[2],
            },
            OP_CODE_MUL => Instruction::Mul {
                x: params[0],
                y: params[1],
                dest: params[2],
            },
            OP_CODE_IN => Instruction::In { dest: params[0] },
            OP_CODE_OUT => Instruction::Out { val: params[0] },
            OP_CODE_EXIT => Instruction::Exit,
            _ => return Err(anyhow!("invalid opcode")),
        })
    }
}

type OpCode = isize;

enum ParamMode {
    Position,
    Immediate,
}

impl ParamMode {
    fn parse(input: isize) -> Result<Self> {
        match input {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            _ => Err(anyhow!("invalid param mode")),
        }
    }
}

const OP_CODE_ADD: OpCode = 1;
const OP_CODE_MUL: OpCode = 2;
const OP_CODE_IN: OpCode = 3;
const OP_CODE_OUT: OpCode = 4;
const OP_CODE_EXIT: OpCode = 99;

fn op_args(op_code: OpCode) -> usize {
    match op_code {
        OP_CODE_IN | OP_CODE_OUT => 1,
        OP_CODE_ADD | OP_CODE_MUL => 3,
        _ => 0,
    }
}

struct Op {
    code: OpCode,
    param_modes: Vec<ParamMode>,
}

impl Op {
    fn parse(input: isize) -> Result<Self> {
        let mut op_iter = OpIter::new(input);
        let code = op_iter.next().ok_or_else(|| anyhow!("invalid op code"))?;
        Ok(Self {
            code,
            param_modes: op_iter
                .take(op_args(code))
                .map(ParamMode::parse)
                .collect::<Result<Vec<ParamMode>>>()?,
        })
    }
}

struct OpIter {
    input: isize,
    outputted_op: bool,
}

impl OpIter {
    fn new(input: isize) -> Self {
        Self {
            input,
            outputted_op: false,
        }
    }
}

const OP_ITER_OP_DIV: isize = 100;
const OP_ITER_MODE_DIV: isize = 10;

impl Iterator for OpIter {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let div = if self.outputted_op {
            OP_ITER_MODE_DIV
        } else {
            OP_ITER_OP_DIV
        };
        self.outputted_op = true;
        match self.input {
            0 => Some(0),
            x => {
                let val = x % div;
                self.input /= div;
                Some(val)
            }
        }
    }
}

fn main() -> Result<()> {
    let mut raw_input = String::new();
    stdin().lock().read_to_string(&mut raw_input)?;
    let mut input: Vec<isize> = raw_input
        .trim()
        .split(',')
        .map(|w| w.parse::<isize>().context("failed parsing number"))
        .collect::<Result<Vec<isize>>>()?;
    let output = run(&mut input, &[1])?;
    println!(
        "{}",
        output
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n")
    );
    Ok(())
}

fn run(prog: &mut [isize], inputs: &[isize]) -> Result<Vec<isize>> {
    let mut input_iter = inputs.iter();
    let mut pos: usize = 0;
    let mut output: Vec<isize> = vec![];
    loop {
        match Instruction::parse(prog, pos)? {
            Instruction::Add { x, y, dest } => {
                prog[dest.position()?] = x.resolve_value(prog)? + y.resolve_value(prog)?;
                pos += op_args(OP_CODE_ADD) + 1;
            }
            Instruction::Mul { x, y, dest } => {
                prog[dest.position()?] = x.resolve_value(prog)? * y.resolve_value(prog)?;
                pos += op_args(OP_CODE_MUL) + 1;
            }
            Instruction::In { dest } => {
                prog[dest.position()?] =
                    *input_iter.next().ok_or_else(|| anyhow!("no inputs left"))?;
                pos += op_args(OP_CODE_IN) + 1;
            }
            Instruction::Out { val } => {
                output.push(val.resolve_value(prog)?);
                pos += op_args(OP_CODE_OUT) + 1;
            }
            Instruction::Exit => return Ok(output),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_run() -> Result<()> {
        let mut input: Vec<isize> = vec![1, 0, 0, 0, 99];
        run(&mut input, &[])?;
        assert_eq!(input, vec![2, 0, 0, 0, 99]);

        input = vec![2, 3, 0, 3, 99];
        run(&mut input, &[])?;
        assert_eq!(input, vec![2, 3, 0, 6, 99]);

        input = vec![2, 4, 4, 5, 99, 0];
        run(&mut input, &[])?;
        assert_eq!(input, vec![2, 4, 4, 5, 99, 9801]);

        input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run(&mut input, &[])?;
        assert_eq!(input, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);

        Ok(())
    }
}
