const ERRORS = [
    "Error: Multi line comment did not end",
    "Error: Unterminated String",
    "Error: Empty string",
    "Error: Invalid escape sequence (for '\\' you need \"\\\\\")",
    "Error: More '[' chars than ']'",
    "Error: Fewer '[' chars than ']'",
    "Error: Code is empty",
];

function submit() {
    const codeInput = document.querySelector("#code-input");

    let output = compile_to_bf(codeInput.value);
    document.querySelector("#code-output").value = output;
}

function compile_to_bf(input) {
    let out = ""; //* This is returned

    let string = ""; //* The string that is added to when is_string
    let keyword = ""; //* Keywords like p, etc.

    let opening_square_bracket_count = 0; //^ For compile-time errors (Better than runtime ones)
    let closing_square_bracket_count = 0; //^ For compile-time errors (Better than runtime ones)

    let might_comment = false; //^ true when there is '/' char
    let might_end_ml_comment = false; //^ true when there is '*' char and is multi line comment
    let is_sl_comment = false; //^ true when might_comment and '/' char
    let is_ml_comment = false; //^ true when might_comment and '*' char
    let is_string = false; //^ true when '"' char
    let is_string_empty = true; //^ For an error to prevent empty strings
    let is_escape_seq = false; //^ true when is_string and char is '\\' (backslash)

    for (let i = 0; i < input.length; i++) {
      let c = input.charAt(i);;
      if (
        !might_comment &&
        !might_end_ml_comment &&
        !is_sl_comment &&
        !is_ml_comment &&
        !is_string &&
        string == ""
      ) {
        switch (c) {
          case ">":
          case "<":
          case "+":
          case "-":
          case ".":
          case ",":
            out += c;
            keyword = "";
            break;

          case "[":
            out += c;
            opening_square_bracket_count++;
            keyword = "";
            break;

          case "]":
            out += c;
            closing_square_bracket_count++;
            keyword = "";
            break;

          case "/":
            might_comment = true;
            keyword = "";
            break;

          case '"':
            is_string = true;
            break;

          case "p":
            keyword = c;
            break;
          case "\n":
          case " ":
            break;

          default:
            keyword = "";
            break;
        }
      } else if (might_comment) {
        switch (c) {
            case "/":
                is_sl_comment = true;
                might_comment = false;
                break;
            case "*":
                is_ml_comment = true;
                might_comment = false;
                break;
            default:
                might_comment = false;
                break;
        }
      } else if (might_end_ml_comment) {
        switch (c) {
            case "/":
                is_sl_comment = false;
                might_end_ml_comment = false;
                break;
        }
      } else if (is_sl_comment) {
        switch (c) {
            case "\n":
                is_sl_comment = false;
            break;
        }

      } else if (is_ml_comment) {
        switch (c) {
            case "*":
                might_end_ml_comment = true;
                break;
        }
      } else if (is_string) {
        if (is_escape_seq) {
            is_string_empty = true;
            switch (c) {
                case "\\":
                    string += c;
                    is_escape_seq = false;
                    break;

                case 'n':
                    string += "\n";
                    is_escape_seq = false;
                    is_string_empty = false;
                    break;

                default:
                    console.error(ERRORS[3]);
                    return ERRORS[3];
            }
        } else {
            switch (c) {
                case "\\":
                  is_escape_seq = true;
                  break;

                case '"':
                    is_string = false;
                    keyword = "";
                    if (is_string_empty) {
                        console.error(ERRORS[2]);
                        return ERRORS[2];
                    }
                    break;

                default:
                    string += c;
                    is_string_empty = false;
            }
        }
        if (string != "") {
            if (keyword == "") {
                out += ">";
            }
            for (let i = 0; i < string.length; i++) {
                for(let j = 0; j < string.charCodeAt(i); j++) {
                    out += "+";
                }
            }
            if (keyword == "p") {
                out += ".";
                out += "[-]";
            }

            string = "";
        }
      }
    }
    if (is_ml_comment) {
        console.error(ERRORS[0]);
        return ERRORS[0];
    }
    if (is_string) {
        console.error(ERRORS[1]);
        return ERRORS[1];
    }
    if (opening_square_bracket_count > closing_square_bracket_count) {
        console.error(ERRORS[4]);
        return ERRORS[4];
    }
    if (opening_square_bracket_count < closing_square_bracket_count) {
        console.error(ERRORS[5]);
        return ERRORS[5];
    }
    if (out.length == 0) {
        console.error(ERRORS[6]);
        return ERRORS[6];
    }
    return out;
}
