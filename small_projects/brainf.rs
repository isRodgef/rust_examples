use std::env;

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    let mut tape = [0u8; 1024];

    if args.len() < 2 {
        println!("Usage: {} <brainf_program>", args[0]);
        return;
    }

    let program = &args[1];
    let mut pc = 0;
    let mut tp = 0;
    // fix loop 
    // fix overflows
    while pc < program.len() {
        match program.chars().nth(pc).unwrap() {
            '>' => tp += 1,
            '<' => tp -= 1,
            '+' => tape[tp] += 1,
            '-' => tape[tp] -= 1,
            '.' => print!("{}", tape[tp] as char),
            '[' => {
                if tape[tp] == 0 {
                    let mut loop_count = 1;
                    while loop_count > 0 {
                        pc += 1;
                        match program.chars().nth(pc).unwrap() {
                            '[' => loop_count += 1,
                            ']' => loop_count -= 1,
                            _ => ()
                        }
                    }
                }
            },
            ']' => {
                if tape[tp] != 0 {
                    let mut loop_count = 1;
                    while loop_count > 0 {
                        pc -= 1;
                        match program.chars().nth(pc).unwrap() {
                            '[' => loop_count -= 1,
                            ']' => loop_count += 1,
                            _ => ()
                        }
                    }
                }
            },
            _ => ()
        }
        pc += 1;
    }


}