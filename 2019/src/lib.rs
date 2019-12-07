use std::convert::TryInto;
use std::io::{self, BufRead, BufReader};
use itertools::Itertools;

pub struct IntcodeMachine {
    program: Vec<i32>,
    ip: usize,
    input: Vec<i32>,
    output: Option<i32>,
}

impl IntcodeMachine {
    pub fn from_string(input: &str) -> Self {
        Self {
            program: input.split(',').map(|num| num.parse::<i32>().unwrap()).collect(),
            ip: 0,
            input: vec![],
            output: None,
        }
    }

    pub fn from_stdin() -> Self {
        let buffered = BufReader::new(io::stdin());
        Self::from_string(&buffered.lines().map(|line| line.unwrap()).join(","))
    }

    pub fn set_input(&mut self, input: Vec<i32>) {
        self.input = input;
    }

    pub fn get_output(&self) -> i32 {
        self.output.unwrap()
    }

    pub fn compute(&mut self) {
        loop {
            let opvalue = OpValue::new(self.program[self.ip]);
            match opvalue.opcode {
                1 => self.add(&opvalue),
                2 => self.mul(&opvalue),
                3 => self.input(),
                4 => self.output(&opvalue),
                5 => self.jump_if_true(&opvalue),
                6 => self.jump_if_false(&opvalue),
                7 => self.less_than(&opvalue),
                8 => self.equals(&opvalue),
                99 => break,
                _ => panic!("unknown opcode {:?} at position {}", opvalue.opcode, self.ip),
            }
        }
    }

    pub fn get_program(&self) -> String {
        self.program.iter().map(|x| x.to_string()).join(",")
    }

    fn fetch_params(&mut self, count: usize) -> Vec<i32> {
        self.ip += 1;
        let result = self.program[self.ip..(self.ip+count)].to_vec();
        self.ip += count;
        result
    }

    fn resolve_position_params(&self, params: &[i32], opvalue: &OpValue) -> Vec<i32> {
        let mut result = vec![];
        for (idx, param) in params.iter().enumerate() {
            result.push(match opvalue.param_mode(idx) {
                ParamMode::Immediate => *param,
                ParamMode::Position => self.program[*param as usize],
            });
        }
        result
    }

    fn add(&mut self, opvalue: &OpValue) {
        let params = self.fetch_params(3);
        let resolved_params = self.resolve_position_params(&params, &opvalue);
        self.program[params[2] as usize] = resolved_params[0] + resolved_params[1];
    }

    fn mul(&mut self, opvalue: &OpValue) {
        let params = self.fetch_params(3);
        let resolved_params = self.resolve_position_params(&params, &opvalue);
        self.program[params[2] as usize] = resolved_params[0] * resolved_params[1];
    }

    fn input(&mut self) {
        let params = self.fetch_params(1);
        self.program[params[0] as usize] = self.input.remove(0);
    }

    fn output(&mut self, opvalue: &OpValue) {
        let params = self.fetch_params(1);
        let resolved_params = self.resolve_position_params(&params, &opvalue);
        self.output = Some(resolved_params[0]);
    }

    fn jump_if_true(&mut self, opvalue: &OpValue) {
        let params = self.fetch_params(2);
        let resolved_params = self.resolve_position_params(&params, &opvalue);
        if resolved_params[0] != 0 {
            self.ip = resolved_params[1] as usize;
        }
    }

    fn jump_if_false(&mut self, opvalue: &OpValue) {
        let params = self.fetch_params(2);
        let resolved_params = self.resolve_position_params(&params, &opvalue);
        if resolved_params[0] == 0 {
            self.ip = resolved_params[1] as usize;
        }
    }

    fn less_than(&mut self, opvalue: &OpValue) {
        let params = self.fetch_params(3);
        let resolved_params = self.resolve_position_params(&params, &opvalue);
        self.program[params[2] as usize] = if resolved_params[0] < resolved_params[1] { 1 } else { 0 };
    }

    fn equals(&mut self, opvalue: &OpValue) {
        let params = self.fetch_params(3);
        let resolved_params = self.resolve_position_params(&params, &opvalue);
        self.program[params[2] as usize] = if resolved_params[0] == resolved_params[1] { 1 } else { 0 };
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParamMode {
    Position,
    Immediate,
}

#[derive(Debug, PartialEq)]
pub struct OpValue {
    opcode: u8,
    param_modes: Vec<ParamMode>,
}

impl OpValue {
    fn new(value: i32) -> Self {
        let opcode = (value % 100).try_into().unwrap();
        let mut param_modes: Vec<ParamMode> = (value / 100).to_string().chars().map(|d| match d {
            '0' => ParamMode::Position,
            '1' => ParamMode::Immediate,
            _ => unimplemented!(),
        }).collect();
        param_modes.reverse();
        OpValue { opcode, param_modes: param_modes }
    }

    fn param_mode(&self, pos: usize) -> ParamMode {
        if pos < self.param_modes.len() {
            self.param_modes[pos]
        } else {
            ParamMode::Position
        }
    }
}

#[test]
fn test_string_parse() {
    let machine = IntcodeMachine::from_string("1,2,3");
    assert_eq!(machine.program, vec![1,2,3]);
}

#[test]
fn test_opvalue_parse() {
    assert_eq!(OpValue::new(110199), OpValue {
        opcode: 99, param_modes: vec![ParamMode::Immediate, ParamMode::Position, ParamMode::Immediate, ParamMode::Immediate]
    })
}

#[test]
fn test_opvalue_default_param_mode() {
    assert_eq!(OpValue::new(110199).param_mode(5), ParamMode::Position);
}

#[test]
fn test_add() {
    let mut machine = IntcodeMachine::from_string("1001,1,14,3,99");
    machine.compute();
    assert_eq!(machine.program, vec![1001,1,14,15,99]);
}

#[test]
fn test_day2() {
    let mut machine = IntcodeMachine::from_string("1,9,10,3,2,3,11,0,99,30,40,50");
    machine.compute();
    assert_eq!(machine.program, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
}
