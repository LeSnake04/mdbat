use std::{fs::read_to_string, path::PathBuf};

use anyhow::{Context, Result};
use clap::{ArgAction, Parser, Subcommand};
use termimad::{Area, MadSkin};

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    files: Vec<PathBuf>,
    #[command(subcommand)]
    /// render inside the give bo
    command: Option<CliCommand>,
    #[arg(long, action = ArgAction::SetTrue)]
    /// Optimise colors for black on white terminals
    light: bool,
    #[arg(long, action = ArgAction::SetTrue)]
    /// Dont display colors
    no_color: bool,
}

#[derive(Subcommand)]
enum CliCommand {
    /// Fit content in spezified bounds
    Area {
        left: u16,
        top: u16,
        width: u16,
        height: u16,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let is_tty = atty::is(atty::Stream::Stdout);
    let skin = match (cli.light, !cli.no_color || is_tty) {
        (false, true) => MadSkin::default_dark(),
        (true, true) => MadSkin::default_light(),
        (_, false) => MadSkin::no_style(),
    };
    for file in &cli.files {
        let content = read_to_string(file).context("Failed to read file")?;
        match cli.command {
            None => println!("{}", skin.term_text(&content)),
            Some(CliCommand::Area {
                left,
                top,
                width,
                height,
            }) => {
                let area = Area::new(left, top, width, height);
                println!("{}", skin.area_text(&content, &area));
            }
        }
    }
    Ok(())
}
