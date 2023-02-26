fn main() -> std::io::Result<()> {
    /*
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        println!("Too little arguments.\nThe correct way to run this is:\n\"bfp2bf <from> <to>\"\n");
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Too little arguments"));
    }
    
    let from = std::fs::read_to_string(&args[1])?;
    let mut to = std::fs::File::create(&args[2])?;
    */
    let from = std::fs::read_to_string("bf.bfp")?;
    let mut to = std::fs::File::create("bf.bf")?;
    
    use std::io::prelude::*;
    to.write_all(compile_to_bf(from).as_bytes())?;
    
    Ok(())
}

fn compile_to_bf(input: String) -> String {
    let mut out = String::new();
    
    let mut string = String::new(); /*  */
    let mut keyword = String::new();
    
    let mut might_comment = false;
    let mut might_end_comment = false;
    let mut is_sl_comment = false;
    let mut is_ml_comment = false;
    let mut is_string = false;
    
    for c in input.chars() {
        if !might_comment && !might_end_comment && !is_sl_comment && !is_ml_comment && !is_string && string == String::new() {
            match c {
                '>' | '<' | '+' |
                '-' | '.' | ',' |
                '[' | ']'
                => out.push(c),
                
                '/' => might_comment = true,
                '\"' => is_string = true,
                
                'p'
                => keyword = c.to_string(),
                
                
                _ => (),
            }
        } else if might_comment {
            match c {
                '/' => is_sl_comment = true,
                '*' => is_ml_comment = true,
                _ => might_comment = false
            }
        } else if might_end_comment {
            match c {
                '/' => {
                    might_end_comment = false;
                    is_ml_comment = false;
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
                '*' => might_end_comment = true,
                _ => ()
            }
        } else if is_string {
            match format!("{}", c).chars().nth(0).unwrap() {
                '\"' => {
                    is_string = false;
                    keyword = String::new();
                },
                _ => string.push(c),
            }
            if string != String::new() {
                if keyword == String::new() {
                    out.push('>');
                }
                for bs in string.as_bytes() {
                    for _ in 0..*bs {
                        out.push('+');
                    }
                    if keyword == "p" {
                        out.push_str(".[-]");
                    }
                    out.push('\n');
                }
                string = String::new();
            }
        }
    }
    if is_ml_comment {
        return
        "++++++++++[>+>+++>+++++++>++++++++++<<<<-] Error: Multi line comment did not end
        >>>-.>++++++++++++++..---.+++.<-----------.
        <++.>+++++++++++++++++++.>+++.---------.+++
        +++++.-----------.<<.>>+++.---.+++++.------
        ---.<<.>>--.++++++++++++.--..--------.+++++
        ++++.++++++.<<.>>----------------.+++++.---
        --.<<.>>++++++++++.+.+++++.<<.>>-----------
        ----.+++++++++.----------.".to_string();
    }
    if is_string {
        return 
        "++++++++++[>+>+++>+++++++>++++++++++<<<<-] Error: Unterminated String
        >>>-.>++++++++++++++..---.+++.<-----------.
        <++.>+++++++++++++++++++++++++++.>----.++++
        ++.---------------.+++++++++++++.-----.----
        .+++++.<++++++++++++.>++++++.<++++.-.<.>---
        --------------.>.--.---------.+++++.-------
        .".to_string();
    }
    out
}
