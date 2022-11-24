// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in the form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// No hints this time!

// I AM NOT DONE

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;
    use std::borrow::Cow;

    // TODO: Complete the function signature!
    pub fn transformer<'a>(input: Vec<(&'a String, &Command)>) -> Vec<Cow<'a, str>> {
        // TODO: Complete the output declaration!
        let mut output: Vec<Cow<'a, str>> = vec![];
        for (string, command) in input.into_iter() {
            // TODO: Complete the function body. You can do it!
            match *command {
                Command::Uppercase => {
                    // to_uppercase method returns a new string. Could we avoid this allocation ?
                    // Maybe with [make_ascii_uppercase](https://doc.rust-lang.org/std/primitive.str.html#method.make_ascii_uppercase)
                    // but we should modify transformer signature to borrow mutable the string
                    output.push(string.to_uppercase().into());
                }
                Command::Trim => {
                    output.push(string.trim().into());
                }
                Command::Append(count) => {
                    //Could we avoid this allocation ?
                    // Maybe, but we should modify transformer signature to borrow mutable the string
                    let mut new_string = String::from(string);
                    for _ in 1..=count {
                        new_string.push_str("bar");
                    }
                    output.push(new_string.into());
                }
            };
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we have to import to have `transformer` in scope?
    use my_module::transformer;

    use super::Command;

    #[test]
    fn it_works() {
        let in_owned = vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ];
        let mut in_borrowed = Vec::<(&String, &Command)>::new();
        for tup in in_owned.iter() {
            in_borrowed.push((&tup.0, &tup.1));
        }
        let output = transformer(in_borrowed);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}

/*
fn main() {

    let in_owned = vec![
        ("hello".into(), Command::Uppercase),
        (" all roads lead to rome! ".into(), Command::Trim),
        ("foo".into(), Command::Append(1)),
        ("bar".into(), Command::Append(5)),
    ];
    let mut in_borrowed = Vec::<(&String, &Command)>::new();
    for tup in in_owned.iter() {
        in_borrowed.push((&tup.0, &tup.1));
    }

    let output = transformer(in_borrowed);
    for i in 0..4 {
        match &output[i] {
            Cow::Borrowed(s) => println!("borrowed {}",s),
            Cow::Owned(s) => println!("owned {}", s),
        }
    }

}
*/
