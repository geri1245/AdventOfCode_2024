use std::io;

fn get_part_after_after_colon(text: &str) -> &str {
    let colon_index = text.find(':').unwrap();
    &text[(colon_index + 2)..]
}

fn get_value(registers: &mut [u32], value: u32) -> u32 {
    if value < 4 {
        value
    } else if value > 3 && value < 7 {
        registers[(value - 4) as usize]
    } else {
        unreachable!()
    }
}

fn perform_instruction(
    registers: &mut [u32],
    instruction: u32,
    operand: u32,
) -> (Option<u32>, Option<u32>) {
    let mut output = None;
    let mut new_instruction_pointer_index = None;
    match instruction {
        0 => registers[0] = registers[0] / 2_u32.pow(get_value(registers, operand) as u32),
        1 => registers[1] = registers[1] ^ operand,
        2 => registers[1] = get_value(registers, operand) % 8,
        3 => {
            if registers[0] > 0 {
                new_instruction_pointer_index = Some(operand);
            }
        }
        4 => registers[1] = registers[1] ^ registers[2],
        5 => output = Some(get_value(registers, operand) % 8),
        6 => registers[1] = registers[0] / 2_u32.pow(get_value(registers, operand)),
        7 => registers[2] = registers[0] / 2_u32.pow(get_value(registers, operand)),
        _ => unreachable!(),
    }

    (output, new_instruction_pointer_index)
}

fn perform_instructions(registers: &mut [u32], instrsuctions: &Vec<u32>) -> Vec<String> {
    let mut instruction_pointer = 0u32;
    let mut output_vec = Vec::new();
    while instruction_pointer < (instrsuctions.len() - 1) as u32 {
        let (instruction_output, maybe_instruction_pointer_index) = perform_instruction(
            registers,
            instrsuctions[instruction_pointer as usize],
            instrsuctions[(instruction_pointer + 1) as usize],
        );

        if let Some(item) = instruction_output {
            output_vec.push(item.to_string());
        }

        if let Some(instruction_pointer_index) = maybe_instruction_pointer_index {
            instruction_pointer = instruction_pointer_index;
        } else {
            instruction_pointer += 2;
        }
    }

    output_vec
}

pub fn solve(input_lines: &[String]) -> io::Result<usize> {
    let mut registers = [
        get_part_after_after_colon(&input_lines[0])
            .parse::<u32>()
            .unwrap(),
        get_part_after_after_colon(&input_lines[1])
            .parse::<u32>()
            .unwrap(),
        get_part_after_after_colon(&input_lines[2])
            .parse::<u32>()
            .unwrap(),
    ];

    let program = get_part_after_after_colon(&input_lines[4])
        .split(',')
        .map(|item| item.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let output = perform_instructions(&mut registers, &program);
    let output_str = output.join(",");

    println!("{output_str:?}");

    let result = 0;

    Ok(result)
}
