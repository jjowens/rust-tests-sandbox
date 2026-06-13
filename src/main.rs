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
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::HelloWorld { name }) => {
            let _ = say_hello_world(name).unwrap();
        },
        None => {
            println!("Not implemented!");
            std::process::exit(1);
        }
    }
}

fn say_hello_world(name: String) -> std::io::Result<()> {
    let final_name = if name.is_empty() { "friend" } else { &name };
    println!("Hello, {}!", final_name);

    Ok(())
}


