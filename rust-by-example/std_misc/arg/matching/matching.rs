use std::env;

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!("usage:
matching <string>
    Check whether given string is the answer.
matching {{increase|decrease}} <integer>
    Increase or decrease given integer by one");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            println!("My name is 'matching'. Try passing some arguments!");
        },

        // one argument passed 
        2 => {
            match args[1].parse() {
                Ok(42) => println!("This is the answer"),
                _      => println!("This is not the answer"),
            }
        },

        // one command and one argument passed
        3 => {
            let cmd = &args[1];
            let num = &args[2];

            let number: i32 = match num.parse() {
                Ok(n)  => { n },
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                },
            };

            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _          => {
                    eprintln!("error: invalid command");
                    help();
                },
            }
        },
        _ => {
            help();
        }
    }
}
