use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "myapp", author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Say hello world. Name is optional
    HelloWorld {
        /// Set file path to open image
        #[arg(long, default_value = "")]
        name: String,
    },
    /// testing wrong command
    WrongCommand,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let mut res = Ok(()) ;

    match args.command {
        Some(Commands::HelloWorld { name }) => {
            res = say_hello_world(name);
        },
        Some(Commands::WrongCommand) => {
            res = Err(String::from("It's brokem"));
            panic!("It's broken");
        }
        None => {
            res = Err(String::from("no command given"));
        }
    }

    res
}



fn say_hello_world(name: String) -> Result<(), String> {
    if !name.is_empty() && (name.len() == 1 || name.len() > 10) {
        Err("Name cannot be less than 2 characters or more than 10 characters".to_string())
    } else if name.to_lowercase() == "fred" {
        Err("Fred cannot be greeted".to_string())
    } else {
        let final_name = if name.is_empty() { "friend" } else { &name };
        println!("Hello, {}!", final_name);

        Ok(())
    }
}


