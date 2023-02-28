 Brainfuck+ Compiler
This also has a vscode extension named [Brainfuck+ Syntax](https://github.com/tadaHrd/bfp-syntax)

It's in alpha right now

If you want an executable go [here](https://github.com/tadaHrd/bfp-compiler/releases/tag/1.0.0.0)

---
# How to code in this

## Installation instructions
1. Create a new file named "`bfp_file_name.bfp`"
2. Download the bfp executable from [this](https://github.com/tadaHrd/bfp-compiler/releases/tag/1.0.0.0)
3. Copy the bfp executable to your folder with the file
4. If there isn't a binary for your system you must compile the project with [Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html#building-and-running-a-cargo-project)
5. Open a command prompt in the folder
6. Run: "`bfp bfp_file_name.bfp bf_file_name.bf`" or "`./bfp bfp_file_name.bfp bf_file_name.bf`"
7. If you have windows you can download the [interpreter](https://github.com/tadaHrd/bfp-compiler/releases/tag/1.0.0.0)
8. Run: "`interpreter.exe bf_file_name.exe`" or "`./interpreter.exe bf_file_name.exe`"
9. Enjoy writing in Brainfuck+

## Documentation
1. Comments
    1. Single-line: "`//`"
    2. Multi-line: "`/*`" and "`*/`"
2. Keywords
    1. p
       - Print statement
       - Usage: "`p "Hello, world!`"
3. Data types
   1. Chars
      - Declared with `"` and ended with `"`
   2. Strings
      - Declared with `"` and ended with `"`
4. Escape sequences
   1. '`\"`' - '`"`' character
   2. '`\\`' - '`\`' character
5. Commands
   1. '`+`' - Adds 1 to the current cell
   2. '`-`' - Removes 1 from the current cell
   3. '`>`' - Moves the pointer right
   4. '`<`' - Moves the pointer left
   5. '`[`' - Starts a while loop
      - Explanation:
      - When it ends it will find if the current cell is 0:
        - If it is, it will end the while loop
        - If it isn't, it will continue looping
    6. '`]`' - Ends a while loop
    7. '`.`' - Prints the current cell value as an ASCII character
    8. '`,`' - Stores user input inside the current cell
    - Including ones stated earlier (Non-Brainfuck, only Brainfuck+)

If you have vscode, you can install the extension (it should ask automatically or download it from the top link)