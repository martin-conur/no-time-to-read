use clap::{Parser, Subcommand, ValueEnum};

#[derive(Clone, Copy, ValueEnum, Debug)]
enum Language {
    Spanish,
    English,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// summarize from a file input
    #[command(arg_required_else_help = true)]
    File {
        /// the path to read
        path: String,
    },
    /// summarize from a text input
    #[command(arg_required_else_help = true)]
    Text {
        /// text string
        input_text: String,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// text to summarize
    #[command(subcommand)]
    command: Commands,

    /// which language the text argument refers
    #[arg(long, short, value_enum, default_value_t = Language::English, global=true)]
    language: Language,
}

fn main() {
    let args = Args::parse();
    let text = match args.command {
        Commands::File { path } => {
            format!("input from file: {path}")
        }
        Commands::Text { input_text } => {
            format!("input from text:{input_text}")
        }
    };

    println!("{:?}, {:?}", text, args.language)
}
