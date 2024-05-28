use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about, author)]
struct Args {
    #[arg(index = 1)]
    pub input: Vec<String>,
}

// TODO Create a way to pipe recho into a file:
// recho hello world >> test.txt

fn main() {
    let args = Args::parse();
    let mut output = String::new();

    for (index, word) in args.input.iter().enumerate() {
        if index > 0 && index < args.input.len() {
            output.push(' ');
        }
        output.push_str(word);
    }

    println!("{}", output);
}
