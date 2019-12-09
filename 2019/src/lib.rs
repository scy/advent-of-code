use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{self, BufRead, BufReader};
use std::ops::Range;
use itertools::Itertools;

pub type Address = u64;
pub type Value = i64;

#[derive(Clone,Copy,PartialEq)]
pub enum MachineState {
    Ready,
    Running,
    Done,
    Waiting,
}

#[derive(Clone)]
pub struct IntcodeMachine {
    memory: HashMap<Address, Value>,
    ip: Address,
    relative_base: Address,
    input: Vec<Value>,
    output: Vec<Value>,
    state: MachineState,
}

impl IntcodeMachine {
    pub fn from_string(input: &str) -> Self {
        Self {
            memory: {
                let mut hashmap = HashMap::new();
                for (idx, value) in input.split(',').map(|num| num.parse::<Value>().unwrap()).enumerate() {
                    hashmap.insert(idx as Address, value);
                }
                hashmap
            },
            ip: 0,
            relative_base: 0,
            input: vec![],
            output: vec![],
            state: MachineState::Ready,
        }
    }

    pub fn from_stdin() -> Self {
        let buffered = BufReader::new(io::stdin());
        Self::from_string(&buffered.lines().map(|line| line.unwrap()).join(","))
    }

    pub fn set_input(&mut self, input: Vec<Value>) {
        self.input = input;
    }

    pub fn push_input(&mut self, input: Value) {
        self.input.push(input);
    }

    pub fn get_outputs(&self) -> Vec<Value> {
        self.output.clone()
    }

    pub fn get_output(&self) -> Value {
        let output = self.get_outputs();
        if output.len() < 1 {
            panic!("There was no output!");
        }
        if output.len() > 1 {
            panic!("There was more than one output!");
        }
        self.output[0]
    }

    pub fn compute(&mut self) -> MachineState {
        self.state = MachineState::Running;
        loop {
            let opvalue = OpValue::new(self.get_memory(self.ip));
            match opvalue.opcode {
                1 => self.add(&opvalue),
                2 => self.mul(&opvalue),
                3 => self.input(&opvalue),
                4 => self.output(&opvalue),
                5 => self.jump_if_true(&opvalue),
                6 => self.jump_if_false(&opvalue),
                7 => self.less_than(&opvalue),
                8 => self.equals(&opvalue),
                9 => self.set_relative_base(&opvalue),
                99 => break,
                _ => panic!("unknown opcode {:?} at position {}", opvalue.opcode, self.ip),
            }
            if self.state == MachineState::Waiting {
                return self.state;
            }
        }
        MachineState::Done
    }

    pub fn get_memory(&self, address: Address) -> Value {
        match self.memory.get(&address) {
            Some(value) => *value,
            None => 0,
        }
    }

    pub fn get_memory_vec(&self, range: Range<Address>) -> Vec<Value> {
        let mut result = vec![];
        for idx in range {
            result.push(self.get_memory(idx));
        }
        result
    }

    pub fn set_memory(&mut self, param_mode: ParamMode, address: Value, value: Value) {
        self.memory.insert(match param_mode {
            ParamMode::Position => address as Address,
            ParamMode::Relative => (address + (self.relative_base as i64)) as Address,
            _ => panic!("set_memory not implemented for param mode {:?}", param_mode),
        }, value);
    }

    fn fetch_params(&mut self, count: Address) -> Vec<Value> {
        self.ip += 1;
        let result = self.get_memory_vec(self.ip..(self.ip+count));
        self.ip += count;
        result
    }

    fn resolve_position_params(&self, params: &[Value], opvalue: &OpValue) -> Vec<Value> {
        let mut result = vec![];
        for (idx, param) in params.iter().enumerate() {
            result.push(match opvalue.param_mode(idx) {
                ParamMode::Immediate => *param,
                ParamMode::Position => self.get_memory(*param as Address),
                ParamMode::Relative => self.get_memory((*param + (self.relative_base as i64)) as Address),
            });
        }
        result
    }

    fn add(&mut self, opvalue: &OpValue) {
        let params = self.fetch_params(3);
        let resolved_params = self.resolve_position_params(&params, &opvalue);
        self.set_memory(opvalue.param_mode(2), params[2], resolved_params[0] + resolved_params[1]);
    }

    fn mul(&mut self, opvalue: &OpValue) {
        let params = self.fetch_params(3);
        let resolved_params = self.resolve_position_params(&params, &opvalue);
        self.set_memory(opvalue.param_mode(2), params[2], resolved_params[0] * resolved_params[1]);
    }

    fn input(&mut self, opvalue: &OpValue) {
        if self.input.len() == 0 {
            self.state = MachineState::Waiting;
            return;
        }
        let params = self.fetch_params(1);
        let input = self.input.remove(0);
        self.set_memory(opvalue.param_mode(0), params[0], input);
    }

    fn output(&mut self, opvalue: &OpValue) {
        let params = self.fetch_params(1);
        let resolved_params = self.resolve_position_params(&params, &opvalue);
        self.output.push(resolved_params[0]);
    }

    fn jump_if_true(&mut self, opvalue: &OpValue) {
        let params = self.fetch_params(2);
        let resolved_params = self.resolve_position_params(&params, &opvalue);
        if resolved_params[0] != 0 {
            self.ip = resolved_params[1] as Address;
        }
    }

    fn jump_if_false(&mut self, opvalue: &OpValue) {
        let params = self.fetch_params(2);
        let resolved_params = self.resolve_position_params(&params, &opvalue);
        if resolved_params[0] == 0 {
            self.ip = resolved_params[1] as Address;
        }
    }

    fn less_than(&mut self, opvalue: &OpValue) {
        let params = self.fetch_params(3);
        let resolved_params = self.resolve_position_params(&params, &opvalue);
        self.set_memory(opvalue.param_mode(2), params[2], if resolved_params[0] < resolved_params[1] { 1 } else { 0 });
    }

    fn equals(&mut self, opvalue: &OpValue) {
        let params = self.fetch_params(3);
        let resolved_params = self.resolve_position_params(&params, &opvalue);
        self.set_memory(opvalue.param_mode(2), params[2], if resolved_params[0] == resolved_params[1] { 1 } else { 0 });
    }

    fn set_relative_base(&mut self, opvalue: &OpValue) {
        let params = self.fetch_params(1);
        let resolved_params = self.resolve_position_params(&params, &opvalue);
        self.relative_base = ((self.relative_base as i64) + resolved_params[0]).try_into().unwrap();
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParamMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug, PartialEq)]
pub struct OpValue {
    opcode: u8,
    param_modes: Vec<ParamMode>,
}

impl OpValue {
    fn new(value: Value) -> Self {
        let opcode = (value % 100).try_into().unwrap();
        let mut param_modes: Vec<ParamMode> = (value / 100).to_string().chars().map(|d| match d {
            '0' => ParamMode::Position,
            '1' => ParamMode::Immediate,
            '2' => ParamMode::Relative,
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
    assert_eq!(machine.get_memory_vec(0..3), vec![1,2,3]);
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
    assert_eq!(machine.get_memory_vec(0..5), vec![1001,1,14,15,99]);
}

#[test]
fn test_day2() {
    let mut machine = IntcodeMachine::from_string("1,9,10,3,2,3,11,0,99,30,40,50");
    machine.compute();
    assert_eq!(machine.get_memory_vec(0..12), vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
}

#[test]
fn test_relative_mode() {
    let mut machine = IntcodeMachine::from_string("109,19,204,-2019,99");
    machine.relative_base = 2000;
    machine.compute();
    assert_eq!(machine.get_output(), 109);
}

#[test]
fn test_day9_quine() {
    let mut machine = IntcodeMachine::from_string("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    machine.compute();
    assert_eq!(machine.get_outputs(), vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
}

#[test]
fn test_day9_16digit() {
    let mut machine = IntcodeMachine::from_string("1102,34915192,34915192,7,4,7,99,0");
    machine.compute();
    assert_eq!(machine.get_output().to_string().len(), 16);
}

#[test]
fn test_day9_long_number() {
    let mut machine = IntcodeMachine::from_string("104,1125899906842624,99");
    machine.compute();
    assert_eq!(machine.get_output(), 1125899906842624);
}

#[test]
fn test_write_to_relative() {
    let mut machine = IntcodeMachine::from_string("109,5,21101,5,23,14,99");
    machine.compute();
    assert_eq!(machine.get_memory_vec(0..20), vec![109,5,21101,5,23,14,99,0,0,0,0,0,0,0,0,0,0,0,0,28]);
}
