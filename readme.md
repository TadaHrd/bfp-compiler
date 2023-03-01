 Brainfuck+ Compiler
This also has a vscode extension named [Brainfuck+ Syntax](https://github.com/tadaHrd/bfp-syntax)

It's in alpha right now

If you want an executable go [here](https://github.com/tadaHrd/bfp-compiler/releases/tag/4.0.0.0)

---
# How to code in this

## Installation instructions
1. Create a new file named `bfp_file_name.bfp`
2. Download the bfp executable from [this](https://github.com/tadaHrd/bfp-compiler/releases/tag/4.0.0.0)
3. Copy the bfp executable to your folder with the file
4. If there isn't a binary for your system, you must compile the project with [Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html#building-and-running-a-cargo-project)
5. Open a command prompt in the folder

### How to run this:
1. Run: `bfp bfp_file_name.bfp bf_file_name.bf` or `./bfp.exe bfp_file_name.bfp bf_file_name.bf` if it doesn't work (bfp being the compiler executable)
2. If you have Windows, you can download the [interpreter](https://github.com/tadaHrd/bfp-compiler/releases/tag/4.0.0.0)
3. On Windows, run:
   - `interpreter bf_file_name.exe` or `./interpreter.exe bf_file_name.bf` if it doesn't work
4. On other systems, you have to compile it ([Instructions](#installation-instructions)) and run it with an interpreter (I recommend [tio.run](https://tio.run/#brainfuck))
5. Enjoy writing in Brainfuck+ ([tutorial](#making-your-first-program))
---
## Documentation

If you want to know how to make your first program, go to [this](#making-your-first-program)

1. Comments
    1. Single-line: `//`
    2. Multi-line: `/*` and `*/`
2. Keywords
    1. p
       - Print statement
       - Example usage: `p "Hello, world!"`
3. Data types
   1. Chars
      - Declared with `"` and ended with `"`
   2. Strings
      - Declared with `"` and ended with `"`
4. Escape sequences
   1. `\"` - `"` character
   2. `\\` - `\` character
   3. `\n` - Newline character
5. Commands
   1. `+` - Adds 1 to the current cell
   2. `-` - Removes 1 from the current cell
   3. `>` - Moves the pointer right
   4. `<` - Moves the pointer left
   5. `[` - Starts a while loop
      - Explanation:
      - When it ends it will find if the current cell is 0:
        - If it is; it will end the while loop
        - If it isn't; it will continue looping
    6. `]` - Ends a while loop
    7. `.` - Prints the current cell value as an ASCII character
    8. `,` - Stores user input inside the current cell
    - Including ones stated earlier (Non-Brainfuck; only Brainfuck+)

### Making your first program

- Follow the [installation instructions](#installation-instructions)

  ### Hello, Brainfuck+:

   1. In the `bfp_file_name.bfp` file add this code:
         - `p "Hello, Brainfuck+!"`
   2. Now run this code ([Instructions](#how-to-run-this))
   3. It should print `"Hello, Brainfuck+!"`

If you have vscode; you can install the extension (it should ask automatically or download it from the top link)