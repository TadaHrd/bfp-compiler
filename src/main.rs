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
    
    let mut is_comment = false;
    let mut is_string = false;
    
    for c in input.chars() {
        if !is_comment && !is_string && string == String::new() {
            match c {
                '>' | '<' | '+' |
                '-' | '.' | ',' |
                '[' | ']'
                => out.push(c),
                
                ';' => is_comment = true,
                '\"' => is_string = true,
                
                'p'
                => keyword = c.to_string(),
                
                
                _ => (),
            }
        } else if is_comment {
            match c {
                '\n' => is_comment = false,
                _ => (),
            }
        } else if is_string {
            match format!("{}", c).chars().nth(0).unwrap() {
                '\"' => is_string = false,
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
                keyword = String::new();
            }
        }
    }
    out
}
