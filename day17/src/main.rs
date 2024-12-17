fn combo_operand(registers: &[i64; 3], operand: u8) -> i64 {
    match operand {
        0..4 => operand as i64,
        4..7 => registers[operand as usize - 4],
        _ => panic!("invalid combo operand")
    }
}

fn run_vm(registers: &[i64; 3], program: &Vec<u8>) -> Vec<u8> {
    let mut registers = registers.clone();
    let mut instruction_pointer = 0;
    let mut output = Vec::<u8>::new();

    while instruction_pointer < program.len() - 1 {
        let mut opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];
        instruction_pointer += 2;

        let mut target_register = 0;
        if opcode == 6 || opcode == 7 {
            target_register = opcode as usize - 5;
            opcode = 0;
        }

        match opcode {
            0 => {
                let numerator = registers[0] as f64;
                let denominator = 2_i64.pow(combo_operand(&registers, operand) as u32) as f64;
                registers[target_register] = (numerator / denominator) as i64;
            },
            1 => registers[1] = registers[1] ^ (operand as i64),
            2 => registers[1] = combo_operand(&registers, operand) % 8,
            3 => if registers[0] != 0 {
                instruction_pointer = operand as usize;
            },
            4 => registers[1] = registers[1] ^ registers[2],
            5 => output.push((combo_operand(&registers, operand) % 8) as u8),
            _ => panic!("invalid opcode")
        };
    }

    output
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let content = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let mut content = content.split("\n\n");
    
    let mut registers = [0_i64; 3];
    for (i, v) in content.next().expect("invalid puzzle input (registers)").split("\n").enumerate() {
        let v = v.split(": ").skip(1).next().expect("parsing of initial register values failed");
        registers[i] = v.parse::<i64>().expect("parsing into i64 failed");
    }

    let program = content
        .next()
        .expect("invalid puzzle input (program [1/3])")
        .split("\n")
        .next()
        .expect("invalid puzzle input (program [2/3])")
        .split(": ")
        .skip(1)
        .next()
        .expect("invalid puzzle input (program [3/3])")
        .split(",")
        .map(|s| s.parse::<u8>().expect("parsing into u8 failed"))
        .collect::<Vec<_>>();

    let output = run_vm(&registers, &program).into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
    println!("[part one] output: {}", output);

    let mut search_space = vec![0_i64; program.len()];
    search_space[0] = 1;
    let mut i = 0;
    loop {
        let mut a = 0;
        for &x in &search_space {
            a = (a << 3) + x;
        }

        registers[0] = a;
        let output = run_vm(&registers, &program);
        
        if output[output.len() - (i+1)] != program[program.len() - (i+1)] {
            search_space[i] += 1;
            while search_space[i] == 8 {
                search_space[i] = 0;
                if i == 0 {
                    panic!("search failed");
                }
                i -= 1;
                search_space[i] += 1;
            }
        } else {
            i += 1;
            if i == program.len() {
                println!("[part two] A = {}", a);
                break;
            }
        }
    }
}
