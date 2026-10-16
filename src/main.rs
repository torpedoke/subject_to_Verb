
fn main() {

    enum Subject {
        I, You,
        He, She, It,
        We, They,
    }
    use std::fmt::{Display, Formatter};
    impl Display for Subject {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self {
                Subject::I => "I",
                Subject::You => "You",
                Subject::He => "He",
                Subject::She => "She",
                Subject::It => "It",
                Subject::We => "We",
                Subject::They => "They",
            })
        }
    }

    enum Verb {
        Am, Are,
        Is,
    }
    impl Display for Verb {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self {
                Verb::Am => "am",
                Verb::Are => "are",
                Verb::Is => "is",
            })
        }
    }

    let mut input = String::new();

    use std::io;
    // just print a line! :n
    println!();
    loop {
        // If me don't clear input, Will make to infinite nested( Not beautiful )
        input.clear();
        println!(
            "Enter a subject (I, You, He, She, It, We, They):\n\
            You will get a verb.\n\
            Or you also can enter 'q'/'quit' to quit\n"
        );
        // read line then expect
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim().to_lowercase();
        // shadowing it
        let trimmed_input = trimmed_input.as_str();

        if trimmed_input == "q" || trimmed_input == "quit" {
            break;
        } else if trimmed_input == "no" || trimmed_input == "not" {
            // TRUST ME! TRUST ME! TRUST ME! TRUST ME! TRUST ME!
            println!("TRUST ME!\n");
            continue;
        }

        let subject = match trimmed_input {
            "i" => Some(Subject::I),
            "you" => Some(Subject::You),
            "he" => Some(Subject::He),
            "she" => Some(Subject::She),
            "it" => Some(Subject::It),
            "we" => Some(Subject::We),
            "they" => Some(Subject::They),
            _ => None,
        };


        // use if let shadowing Option<Subject> to-> Subject
        if let Some(subject) = subject {
            // now, subject is not Option in here!!!!! ;p
            let verb = match subject {
                Subject::I   => Verb::Am,
                Subject::You => Verb::Are,
                Subject::He  => Verb::Is,
                Subject::She => Verb::Is,
                Subject::It  => Verb::Is,
                Subject::We  => Verb::Are,
                Subject::They => Verb::Are,
            };
            println!("for Subject \"{}\" the Verb is \"{}\"\n", trimmed_input, verb);
        } else {
            println!("Unknown subject: \"{}\"\n", trimmed_input);
        }

    }
}
