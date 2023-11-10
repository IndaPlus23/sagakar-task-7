use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Type {
    A,
    I,
    J,
    M,
    N
}

#[derive(Debug, Clone)]
enum Operation {
    AND,
    NOT,
    OR,
    XOR,
    SHR,
    SHL,
    ADD,
    SUB,
    LOAD,
    SET,
    LDIM,
    JEQ,
    JLT,
    JGT,
    JAR,
    MOV,
}

#[derive(Debug, Clone)]
struct Instruction {
    instruction_type: Type,
    operation: Operation,
    register1: Option<usize>,
    register2: Option<usize>,
    register3: Option<usize>,
    immediate: Option<u32>
}

struct Program {
    instructions: Vec<Instruction>,
    ram: [u32; 4096],
    registers: [u32; 8],
    program_counter: usize,
    input: Vec<char>,
    char_index: usize
}

impl Program {
    // Execute the whole program
    fn execute(&mut self) {
        while self.program_counter < self.instructions.len() + 1 {
            let instruction = self.instructions[self.program_counter - 1].clone();
            match instruction.instruction_type {
                Type::A => {self.execute_type_a(&instruction)},
                Type::I => {self.execute_type_i(&instruction)},
                Type::J => {self.execute_type_j(&instruction)},
                Type::M => {self.execute_type_m(&instruction)},
                Type::N => {self.execute_type_n(&instruction)}
            }
            self.program_counter += 1;
        }
    }

    fn execute_type_a(&mut self, instruction: &Instruction) {
        let target_index = *instruction.register1.as_ref().unwrap();
        let operand1 = self.read_from_register(*instruction.register2.as_ref().unwrap());
        let operand2 = self.read_from_register(*instruction.register3.as_ref().unwrap());
        match instruction.operation {
            Operation::AND => {self.write_to_register(target_index, operand1 & operand2)},
            Operation::OR => {self.write_to_register(target_index, operand1 | operand2)},
            Operation::XOR => {self.write_to_register(target_index, operand1 ^ operand2)},
            Operation::SHR => {self.write_to_register(target_index, operand1 >> operand2)},
            Operation::SHL => {self.write_to_register(target_index, operand1 << operand2)},
            Operation::ADD => {self.write_to_register(target_index, operand1 + operand2)},
            Operation::SUB => {self.write_to_register(target_index, operand1 - operand2)},
            Operation::JEQ => {if operand1 == operand2 {self.program_counter = self.read_from_register(target_index) as usize}},
            Operation::JLT => {if operand1 < operand2 {self.program_counter = self.read_from_register(target_index) as usize}},
            Operation::JGT => {if operand1 > operand2 {self.program_counter = self.read_from_register(target_index) as usize}}
            _ => {}
        }
    }

    fn execute_type_i(&mut self, instruction: &Instruction) {
        let immediate = *instruction.immediate.as_ref().unwrap();
        self.write_to_register( 5, immediate);
    }

    fn execute_type_j(&mut self, instruction: &Instruction) {
        let target = self.read_from_register(*instruction.register1.as_ref().unwrap());
        self.write_to_register( 7, (self.program_counter) as u32);
        self.program_counter = target as usize;
    }

    fn execute_type_m(&mut self, instruction: &Instruction) {
        let target_index = *instruction.register1.as_ref().unwrap();
        let address = self.read_from_register(*instruction.register2.as_ref().unwrap()) as usize;
        let offset = *instruction.immediate.as_ref().unwrap() as usize;
        match instruction.operation {
            Operation::LOAD => {
                let data = self.ram[address + offset];
                self.write_to_register(target_index, data)
            },
            Operation::SET => {
                let data = self.read_from_register(target_index);
                self.ram[address + offset] = data;
            }
            _ => {}
        }
    }

    fn execute_type_n(&mut self, instruction: &Instruction) {
        let target_index = *instruction.register1.as_ref().unwrap();
        let operand = self.read_from_register(*instruction.register2.as_ref().unwrap());
        match instruction.operation {
            Operation::NOT => {self.write_to_register(target_index, !operand)},
            Operation::MOV => {self.write_to_register(target_index, operand)},
            _ => {},
        }
    }

    fn read_from_register(&mut self, index: usize) -> u32 {
        if index == 6 {
            return self.read_from_io()
        }
        return self.registers[index]
    }

    fn write_to_register(&mut self, index: usize, value: u32) {
        if index == 6 {
            self.write_to_io(value);
        }
        else {
            self.registers[index] = value;
        }
    }

    fn read_from_io(&mut self) -> u32 {
        let character = self.input[self.char_index];
        self.char_index += 1;
        return character as u32;
    }

    fn write_to_io(&mut self, character: u32) {
        print!("{}", char::from_u32(character).expect("invalid ouput char"))
    }
}

fn lines_to_instructions(lines: &mut Vec<Vec<String>>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    for line in &mut *lines {
        let operation = str_to_operation(&line[0]).expect("Invalid operation name");
        let instruction_type = operation_to_type(&operation);
        let mut register1: Option<usize> = None;
        let mut register2: Option<usize> = None;
        let mut register3: Option<usize> = None;
        let mut immediate: Option<u32> = None;
        match instruction_type {
            Type::A => {
                register1 = Some(str_to_register(&line[1]).expect("Invalid register"));
                register2 = Some(str_to_register(&line[2]).expect("Invalid register"));
                register3 = Some(str_to_register(&line[3]).expect("Invalid register"));
            },
            Type::I => {
                immediate = Some(str_to_immediate12(&line[1]))
            },
            Type::J => {
                register1 = Some(str_to_register(&line[1]).expect("Invalid register"))
            },
            Type::M => {
                register1 = Some(str_to_register(&line[1]).expect("Invalid register"));
                register2 = Some(str_to_register(&line[2]).expect("Invalid register"));
                immediate = Some(str_to_immediate6(&line[3]));
            },
            Type::N => {
                register1 = Some(str_to_register(&line[1]).expect("Invalid register"));
                register2 = Some(str_to_register(&line[2]).expect("Invalid register"));
            }
        }
        instructions.push(
            Instruction{
                instruction_type,
                operation,
                register1,
                register2,
                register3,
                immediate
            }
        )
    }
    return instructions
}

fn str_to_register(word: &str) -> Option<usize> {
    match word {
        "R0" => Some(0),
        "R1" => Some(1),
        "R2" => Some(2),
        "R3" => Some(3),
        "R4" => Some(4),
        "IM" => Some(5),
        "IO" => Some(6),
        "RA" => Some(7),
        _ => {None},
    }
}

fn str_to_immediate12(word: &str) -> u32 {
    let immediate: u32 = word.parse().unwrap();
    if immediate > 4095 {
        panic!("Immediate over 12 bits at I-type instruction");
    }
    return immediate
}

fn str_to_immediate6(word: &str) -> u32 {
    let immediate: u32 = word.parse().unwrap();
    if immediate > 63 {
        panic!("Immediate over 63 bits at M-type instruction");
    }
    return immediate
}

fn str_to_operation(word: &str) -> Option<Operation> {
    match word {
        "AND" => Some(Operation::AND),
        "NOT" => Some(Operation::NOT),
        "OR" => Some(Operation::OR),
        "XOR" => Some(Operation::XOR),
        "SHR" => Some(Operation::SHR),
        "SHL" => Some(Operation::SHL),
        "ADD" => Some(Operation::ADD),
        "SUB" => Some(Operation::SUB),
        "LOAD" => Some(Operation::LOAD),
        "SET" => Some(Operation::SET),
        "LDIM" => Some(Operation::LDIM),
        "JEQ" => Some(Operation::JEQ),
        "JLT" => Some(Operation::JLT),
        "JGT" => Some(Operation::JGT),
        "JAR" => Some(Operation::JAR),
        "MOV" => Some(Operation::MOV),
        _ => {None},
    }
}

fn operation_to_type(operation: &Operation) -> Type{
    match operation {
        Operation::AND => Type::A,
        Operation::NOT => Type::N,
        Operation::OR => Type::A,
        Operation::XOR => Type::A,
        Operation::SHR => Type::A,
        Operation::SHL => Type::A,
        Operation::ADD => Type::A,
        Operation::SUB => Type::A,
        Operation::LOAD => Type::M,
        Operation::SET => Type::M,
        Operation::LDIM => Type::I,
        Operation::JEQ => Type::A,
        Operation::JLT => Type::A,
        Operation::JGT => Type::A,
        Operation::JAR => Type::J,
        Operation::MOV => Type::N,
    }
}

// Strip all comments from the input data
fn remove_comments(lines: &mut Vec<Vec<String>>) {
    for line in &mut *lines {
        let mut is_comment = false;
        for word in &mut *line {      
            if word.starts_with("//") {
                is_comment = true;
            }
            if is_comment {
                *word = String::new();
            }
        }
        line.retain(|word| !word.is_empty());
    }
    lines.retain(|line| line.len() > 0);
}

// Read all constant declarations
// Replace all constant references with the corresponding immediate
// Delete the declarations
fn process_constants(lines: &mut Vec<Vec<String>>) {
    let mut constants: HashMap<String, u32> = HashMap::new();
    for line in &mut *lines {
        // Ignore any lines that are not a constant declaration 
        if !line[0].starts_with("$") {
            continue;
        }
        // Panic if incorrect number of arguments
        if line.len() != 2 {
            panic!("Incorrect number of arguments in constant declaration")
        }
        // Forbid multiple declaration of constant
        if constants.contains_key(&line[0]) {
            panic!("Double declaration of constant!")
        }
        let value: u32;
        // If value is a single char within single quotes, convert that char to a u32
        if line[1].len() == 3 && line[1].starts_with("\'") && line[1].ends_with("\'") {
            value = line[1].chars().nth(1).unwrap() as u32;
        }
        else {
            value = line[1].parse().expect("Constant not parsable as u32") // Else, parse it as an integer
        }
        constants.insert(line[0].clone(), value);
        *line = vec![String::new()]; // Empty line for removal   
    }
    lines.retain(|line| *line != vec![String::new()]); // Remove empty lines
    symbol_to_immediate(lines, &constants);
}

// Read all label declarations
// Replace all label references with the corresponding instruction address (just the line number, in this case)
// Replace all declarations with NOPs
// Must be run AFTER all code that removes lines, or the addresses will be incorrect
fn process_labels (lines: &mut Vec<Vec<String>>){
    let mut line_number: u32 = 1; 
    let mut deleted_lines: u32 = 0;
    let mut labels: HashMap<String, u32> = HashMap::new();
    for line in &mut *lines {
        // Ignore all non-labels
        if !line[0].starts_with("(") {
            line_number += 1;
            continue;
        }
        // If label does not have a closing parenthesis or is multiple words, panic
        if !line[0].ends_with(")") || line.len() != 1 {
            panic!("Incorrectly formatted label!")
        }
        line[0].remove(0); // Remove opening parenthesis
        line[0].pop(); // And closing parenthesis
        // Forbid multiple declarations of label
        if labels.contains_key(&line[0]) {
            panic!("Double declaration of label")
        }
        labels.insert(line[0].clone(), line_number - deleted_lines - 1);
        line[0] = String::new();
        line_number += 1;
        deleted_lines += 1;
    }
    lines.retain(|line| *line != vec![String::new()]);
    symbol_to_immediate(lines, &labels);
}

// Replaces all references to a symbol with the corresponding immediate
fn symbol_to_immediate (lines: &mut Vec<Vec<String>>, symbol_table: &HashMap<String, u32>) {
    for line in &mut *lines {
        for word in &mut *line {
            if symbol_table.contains_key(word) {
                let immediate = symbol_table.get(word).unwrap();
                *word = immediate.to_string();
            }
        }
    }
}

fn main() {
    // Get filename from command line arg, open file and collect lines
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("Invalid number of arguments. Usage: {} <filename> <program input>", args[0]);
    }
    // Read file and turn into vector of lines
    // Each line is a vector of words
    let filepath = args[1].clone();
    let mut lines = String::new();
    // Read from the infile and close
    {
        let mut infile = File::open(filepath).expect("Invalid file path");
        infile.read_to_string(&mut lines).expect("Read from file failed");
    }
    let mut lines = lines.split("\n")
        .map(|line| line.trim())
        .map(|line| 
            line.split_whitespace()
            .map(|word| word.to_owned())
            .collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    // Remove comments and replace all symbols with immediates
    remove_comments(&mut lines);
    process_constants(&mut lines);
    process_labels(&mut lines);
    // Replace text with instruction structs
    let instructions = lines_to_instructions(&mut lines);
    let mut program_input = String::new();
    if args.len() > 2 {
        program_input = args[2].clone();
    }
    program_input += "\0";
    let mut program = Program {
        instructions: instructions,
        ram: [0; 4096],
        registers: [0; 8],
        program_counter: 1,
        input: program_input.chars().collect(),
        char_index: 0
    };
    program.execute();
}
