const ERRORS: [&str; 7] = [
"Error: Multi line comment did not end",
"Error: Unterminated String",
"Error: Empty string",
"Error: Invalid escape sequence (for '\\' you need \"\\\\\")",
"Error: More '[' chars than ']'",
"Error: Fewer '[' chars than ']'",
"Error: Code is empty",
];

fn main() -> std::io::Result<()> {
    // // let args: Vec<String> = std::env::args().collect();
    
    // // if args.len() < 3 {
    // //     println!("Too little arguments.\nThe correct way to run this is:\n\"bfp <from> <to>\"\n");
    // //     return Err(std::io::Error::new(std::io::ErrorKind::Other, "Too little arguments"));
    // // }
    
    // // let from = std::fs::read_to_string(&args[1])?;
    // // let mut to = std::fs::File::create(&args[2])?;
    
    let from = std::fs::read_to_string("bf.bfp")?;
    let mut to = std::fs::File::create("bf.bf")?;
    
    use std::io::prelude::*;
    to.write_all(compile_to_bf(from).as_bytes())?;
    
    Ok(())
}

fn compile_to_bf(input: String) -> String {
    let mut out = String::new();
    
    let mut string = String::new();
    let mut keyword = String::new();
    
    let mut opening_square_bracket_count = 0;
    let mut closing_square_bracket_count = 0;
    
    let mut might_comment = false;
    let mut might_end_ml_comment = false;
    let mut is_sl_comment = false;
    let mut is_ml_comment = false;
    let mut is_string = false;
    let mut is_string_empty = true;
    let mut is_escape_seq = false;
    
    for c in input.chars() {
        if !might_comment && !might_end_ml_comment && !is_sl_comment && !is_ml_comment && !is_string && string == String::new() {
            match c {
                '>' | '<' | '+' |
                '-' | '.' | ','
                => out.push(c),
                
                '[' => {
                    out.push(c);
                    opening_square_bracket_count += 1;
                },
                ']' => {
                    out.push(c);
                    closing_square_bracket_count += 1;
                }
                
                '/' => might_comment = true,
                '\"' => is_string = true,
                
                'p'
                => keyword = c.to_string(),
                
                
                _ => (),
            }
        } else if might_comment {
            match c {
                '/' => {
                    is_sl_comment = true;
                    might_comment = false;
                },
                '*' => {
                    is_ml_comment = true;
                    might_comment = false;
                },
                _ => might_comment = false
            }
        } else if might_end_ml_comment {
            match c {
                '/' => {
                    is_ml_comment = false;
                    might_end_ml_comment = false;
                },
                _ => ()
            }
        } else if is_sl_comment {
            match c {
                '\n' => is_sl_comment = false,
                _ => (),
            }
        } else if is_ml_comment {
            match c {
                '*' => might_end_ml_comment = true,
                _ => ()
            }
        }
        else if is_string {
            if is_escape_seq {
                is_string_empty = true;
                match c {
                    '\\' | '\"'
                    => {
                        string.push(c);
                        is_escape_seq = false;
                        is_string_empty = false;
                    }
                    
                    'n' => {
                        string.push('\n');
                        is_escape_seq = false;
                        is_string_empty = false;
                    }
                    
                    _ => {
                        println!("{}", ERRORS[3]);
                        return ERRORS[3].to_string();
                    }
                }
            } else {
                match c {
                    '\"' => {
                        is_string = false;
                        keyword = String::new();
                        if is_string_empty {
                            println!("{}", ERRORS[2]);
                            return ERRORS[2].to_string();
                        }
                    },
                    '\\' => is_escape_seq = true,
                    
                    _ => {
                        string.push(c);
                        is_string_empty = false;
                    }
                }
            }
            if string != String::new() {
                if keyword == String::new() {
                    out.push('>');
                }
                for b in string.as_bytes() {
                    for _ in 0..*b {
                        out.push('+');
                    }
                }
                if keyword == "p" {
                    out.push_str(".");
                    out.push_str("[-]");
                }
                
                string = String::new();
            }
        }
    }
    if is_ml_comment {
        println!("{}", ERRORS[0]);
        return ERRORS[0].to_string();
    }
    if is_string {
        println!("{}", ERRORS[1]);
        return ERRORS[1].to_string();
    }
    if opening_square_bracket_count > closing_square_bracket_count {
        println!("{}", ERRORS[4].to_string());
        return ERRORS[4].to_string();
    } else if closing_square_bracket_count > opening_square_bracket_count {
        println!("{}", ERRORS[5].to_string());
        return ERRORS[5].to_string();
    }
    if out.len() == 0 {
        println!("{}", ERRORS[6].to_string());
        return ERRORS[6].to_string();
    }
    out
}