use anyhow::{anyhow, Context, Result};
use permutohedron::Heap;

use std::io::{stdin, Read};

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
    Input { dest: Param },
    Output { val: Param },
    JumpIfTrue { test: Param, dest: Param },
    JumpIfFalse { test: Param, dest: Param },
    LessThan { x: Param, y: Param, dest: Param },
    Equals { x: Param, y: Param, dest: Param },
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
            OpCode::Add => Instruction::Add {
                x: params[0],
                y: params[1],
                dest: params[2],
            },
            OpCode::Mul => Instruction::Mul {
                x: params[0],
                y: params[1],
                dest: params[2],
            },
            OpCode::Input => Instruction::Input { dest: params[0] },
            OpCode::Output => Instruction::Output { val: params[0] },
            OpCode::JumpIfTrue => Instruction::JumpIfTrue {
                test: params[0],
                dest: params[1],
            },
            OpCode::JumpIfFalse => Instruction::JumpIfFalse {
                test: params[0],
                dest: params[1],
            },
            OpCode::LessThan => Instruction::LessThan {
                x: params[0],
                y: params[1],
                dest: params[2],
            },
            OpCode::Equals => Instruction::Equals {
                x: params[0],
                y: params[1],
                dest: params[2],
            },
            OpCode::Exit => Instruction::Exit,
        })
    }
}

enum OpCode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Exit,
}

impl OpCode {
    fn parse(input: isize) -> Result<Self> {
        Ok(match input {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            99 => OpCode::Exit,
            _ => return Err(anyhow!("invalid opcode")),
        })
    }

    fn args(&self) -> usize {
        match *self {
            OpCode::Exit => 0,
            OpCode::Input | OpCode::Output => 1,
            OpCode::JumpIfTrue | OpCode::JumpIfFalse => 2,
            OpCode::Add | OpCode::Mul | OpCode::LessThan | OpCode::Equals => 3,
        }
    }
}

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

struct Op {
    code: OpCode,
    param_modes: Vec<ParamMode>,
}

impl Op {
    fn parse(input: isize) -> Result<Self> {
        let mut op_iter = OpIter::new(input);
        let code = OpCode::parse(op_iter.next().ok_or_else(|| anyhow!("invalid op code"))?)?;
        let num_args = code.args();
        Ok(Self {
            code,
            param_modes: op_iter
                .take(num_args)
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

#[derive(Debug, PartialEq, Eq)]
enum HaltCause {
    Input { pos: usize },
    Exit,
}

fn main() -> Result<()> {
    let mut raw_input = String::new();
    stdin().lock().read_to_string(&mut raw_input)?;
    let mut input: Vec<isize> = raw_input
        .trim()
        .split(',')
        .map(|w| w.parse::<isize>().context("failed parsing number"))
        .collect::<Result<Vec<isize>>>()?;
    let (output, _) = amp_loop_max(&mut input)?;
    println!("{}", output);
    Ok(())
}

struct Halt {
    cause: HaltCause,
    output: Vec<isize>,
}

fn run(prog: &mut [isize], inputs: &[isize]) -> Result<Halt> {
    resume(prog, 0, inputs)
}

fn resume(prog: &mut [isize], pos: usize, inputs: &[isize]) -> Result<Halt> {
    let mut input_iter = inputs.iter();
    let mut output: Vec<isize> = vec![];
    let mut pos = pos;
    loop {
        match Instruction::parse(prog, pos)? {
            Instruction::Add { x, y, dest } => {
                prog[dest.position()?] = x.resolve_value(prog)? + y.resolve_value(prog)?;
                pos += OpCode::Add.args() + 1;
            }
            Instruction::Mul { x, y, dest } => {
                prog[dest.position()?] = x.resolve_value(prog)? * y.resolve_value(prog)?;
                pos += OpCode::Mul.args() + 1;
            }
            Instruction::Input { dest } => {
                match input_iter.next() {
                    Some(i) => prog[dest.position()?] = *i,
                    None => {
                        return Ok(Halt {
                            cause: HaltCause::Input { pos },
                            output,
                        })
                    }
                }
                pos += OpCode::Input.args() + 1;
            }
            Instruction::Output { val } => {
                output.push(val.resolve_value(prog)?);
                pos += OpCode::Output.args() + 1;
            }
            Instruction::JumpIfTrue { test, dest } => {
                if test.resolve_value(prog)? > 0 {
                    pos = dest.resolve_value(prog)? as usize;
                } else {
                    pos += OpCode::JumpIfTrue.args() + 1;
                }
            }
            Instruction::JumpIfFalse { test, dest } => {
                if test.resolve_value(prog)? == 0 {
                    pos = dest.resolve_value(prog)? as usize;
                } else {
                    pos += OpCode::JumpIfFalse.args() + 1;
                }
            }
            Instruction::LessThan { x, y, dest } => {
                prog[dest.position()?] = if x.resolve_value(prog)? < y.resolve_value(prog)? {
                    1
                } else {
                    0
                };
                pos += OpCode::LessThan.args() + 1;
            }
            Instruction::Equals { x, y, dest } => {
                prog[dest.position()?] =
                    (x.resolve_value(prog)? == y.resolve_value(prog)?) as isize;
                pos += OpCode::Equals.args() + 1;
            }
            Instruction::Exit => {
                return Ok(Halt {
                    cause: HaltCause::Exit,
                    output,
                })
            }
        }
    }
}

struct WaitingProg {
    prog: Vec<isize>,
    pos: usize,
}

fn amp_loop(prog: &mut [isize], phases: &[isize]) -> Result<isize> {
    let mut progs: Vec<WaitingProg> = phases
        .iter()
        .map(|p| {
            let mut prog = prog.to_owned();
            let halt = run(&mut prog, &[*p])?;
            Ok(WaitingProg {
                prog,
                pos: match halt.cause {
                    HaltCause::Input { pos } => pos,
                    HaltCause::Exit => 0,
                },
            })
        })
        .collect::<Result<Vec<WaitingProg>>>()?;
    let mut signal: isize = 0;
    let mut amp_ptr: usize = 0;
    loop {
        let pos = progs[amp_ptr].pos;
        let halt = resume(&mut progs[amp_ptr].prog, pos, &[signal])?;
        signal = halt.output[0];
        match halt.cause {
            HaltCause::Input { pos: new_pos } => progs[amp_ptr].pos = new_pos,
            HaltCause::Exit => {
                if amp_ptr == progs.len() - 1 {
                    return Ok(signal);
                }
            }
        }
        amp_ptr = (amp_ptr + 1) % progs.len();
    }
}

fn amp_loop_max(prog: &mut [isize]) -> Result<(isize, Vec<isize>)> {
    let mut highest: Option<isize> = None;
    let mut highest_phases: Option<Vec<isize>> = None;
    let mut initial_phases: Vec<isize> = vec![5, 6, 7, 8, 9];
    let heap = Heap::new(&mut initial_phases);
    for phases in heap {
        let mut prog = prog.to_owned();
        let output = amp_loop(&mut prog, &phases)?;
        if highest.is_none() || output > highest.unwrap() {
            highest = Some(output);
            highest_phases = Some(phases.to_owned());
        }
    }
    Ok((
        highest.ok_or_else(|| anyhow!("could not find highest phases"))?,
        highest_phases.unwrap(),
    ))
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

    #[test]
    fn test_equals_with_position_mode() -> Result<()> {
        // Program tests whether input is equal to 8
        let prog: Vec<isize> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run(&mut prog.clone(), &[8])?.output, vec![1]);
        assert_eq!(run(&mut prog.clone(), &[9])?.output, vec![0]);
        Ok(())
    }

    #[test]
    fn test_equals_with_immediate_mode() -> Result<()> {
        // Program tests whether input is equal to 8
        let prog: Vec<isize> = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(run(&mut prog.clone(), &[8])?.output, vec![1]);
        assert_eq!(run(&mut prog.clone(), &[9])?.output, vec![0]);
        Ok(())
    }

    #[test]
    fn test_less_than_with_position_mode() -> Result<()> {
        // Program tests whether input is less than 8
        let prog: Vec<isize> = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run(&mut prog.clone(), &[7])?.output, vec![1]);
        assert_eq!(run(&mut prog.clone(), &[8])?.output, vec![0]);
        Ok(())
    }

    #[test]
    fn test_less_than_with_immediate_mode() -> Result<()> {
        // Program tests whether input is less than 8
        let prog: Vec<isize> = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(run(&mut prog.clone(), &[7])?.output, vec![1]);
        assert_eq!(run(&mut prog.clone(), &[8])?.output, vec![0]);
        Ok(())
    }

    #[test]
    fn test_jump_with_position_mode() -> Result<()> {
        // Program tests whether input is non-zero
        let prog: Vec<isize> = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(run(&mut prog.clone(), &[0])?.output, vec![0]);
        assert_eq!(run(&mut prog.clone(), &[1])?.output, vec![1]);
        assert_eq!(run(&mut prog.clone(), &[2])?.output, vec![1]);
        Ok(())
    }

    #[test]
    fn test_jump_with_immediate_mode() -> Result<()> {
        // Program tests whether input is non-zero
        let prog: Vec<isize> = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(run(&mut prog.clone(), &[0])?.output, vec![0]);
        assert_eq!(run(&mut prog.clone(), &[1])?.output, vec![1]);
        assert_eq!(run(&mut prog.clone(), &[2])?.output, vec![1]);
        Ok(())
    }

    #[test]
    fn test_amp_loop() -> Result<()> {
        assert_eq!(
            amp_loop(
                &mut [
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5
                ],
                &[9, 8, 7, 6, 5]
            )?,
            139629729
        );
        assert_eq!(
            amp_loop(
                &mut [
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
                ],
                &[9, 7, 8, 5, 6]
            )?,
            18216
        );
        Ok(())
    }
}
