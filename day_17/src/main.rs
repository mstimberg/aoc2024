use std::fs;

#[derive(Debug)]
struct Computer {
    // registers
    a: i64,
    b: i64,
    c: i64,
    ip: usize,
    output: Vec<i64>,
    program: Vec<i64>,
}

impl Computer {
    fn new(a: i64, b: i64, c: i64, program: &Vec<i64>) -> Computer {
        Computer {
            a,
            b,
            c,
            program: program.clone(),
            ip: 0,
            output: Vec::new(),
        }
    }

    fn combo(&self, operand: i64) -> i64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid operand"),
        }
    }

    fn out(&mut self, value: i64) {
        self.output.push(value);
    }

    fn div(&self, a: i64, b: i64) -> i64 {
        let mut b = b;
        let mut result = a;
        while b > 0 && result > 0 {
            result /= 2;
            b -= 1;
        }
        result
    }

    fn print_program(&self) {
        println!(
            "Program:\n",     
        );
        for c in self.program.chunks(2) {
            let (opcode, operand) = (c[0], c[1]);
            let combo = match operand {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "a",
                5 => "b",
                6 => "c",
                _ => panic!("Invalid operand"),
            };
            match opcode {
                0 => println!("a = a/2**{}", combo),
                1 => println!("b = b ^ {}", operand),
                2 => println!("b = {} % 8", combo),
                3 => println!("jump if a!=0: {}", operand),
                4 => println!("b = b ^ c ({} is ignored)", operand),
                5 => println!("out: {} % 8", combo),
                6 => println!("b = a/2**{}", combo),
                7 => println!("c = a/2**{}", combo),
                _ => panic!("Invalid opcode"),
            }
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.output.clear();
    }

    fn step(&mut self) -> bool {
        if self.ip >= self.program.len() {
            return false;
        }
        let opcode = self.program[self.ip as usize];
        let operand = self.program[self.ip as usize + 1];
        // println!("State before: {:?}", self);
        match opcode {
            0 => {
                let result = self.a >> self.combo(operand);
                self.a = result;
                self.ip += 2;
            }
            1 => {
                let result = self.b ^ operand;
                self.b = result;
                self.ip += 2;
            }
            2 => {
                let result = self.combo(operand) % 8;
                self.b = result;
                self.ip += 2;
            }
            3 => {
                if self.a == 0 {
                    self.ip += 2;
                } else {
                    self.ip = operand as usize;
                }
            }
            4 => {
                let result = self.b ^ self.c;
                self.b = result;
                self.ip += 2;
            }
            5 => {
                let result = self.combo(operand) % 8;
                self.out(result);
                self.ip += 2;
            }
            6 => {
                let result = self.a >> self.combo(operand);
                self.b = result;
                self.ip += 2;
            }
            7 => {
                let result = self.a >> self.combo(operand);
                self.c = result;
                self.ip += 2;
            }
            _ => panic!("Invalid opcode"),
        }
        true
    }

    fn run(&mut self) {
        while self.step() {
        }
    }
}

fn find_program(prefix: i64, computer: &mut Computer) -> i64 {
    println!("prefix: {}", prefix);
    for new_digit in 0..8 {
        let new_guess = (prefix * 8) | new_digit;
        computer.reset();
        computer.a = new_guess;
        computer.run();
        if computer.output.len() == computer.program.len() && computer.output[0] == computer.program[0] {
            println!("Full match!");
            return new_guess;
        }
        if computer.output[0] != computer.program[computer.program.len()-computer.output.len()] {
            continue;
        } else {
            println!("Continuing with {:o} (returned {:?})", new_guess, computer.output);
            let result= find_program(new_guess, computer);
            if result != -1 {
                return result; // a solution!
            }
        }

    }
    -1  // no solution
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines: Vec<_> = contents.lines().collect();
    assert!(lines.len() == 5);
    let a = &lines[0][11..].trim().parse::<i64>().unwrap();
    let b = &lines[1][11..].trim().parse::<i64>().unwrap();
    let c = &lines[2][11..].trim().parse::<i64>().unwrap();
    let program = lines[4][9..]
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut computer = Computer::new(*a, *b, *c, &program);
    let solution = find_program(0, &mut computer);
    println!("Value in decimal notation: {}", solution);
}
