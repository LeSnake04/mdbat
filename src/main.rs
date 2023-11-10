use std::{fs::read_to_string, path::PathBuf};

use anyhow::{Context, Result};
use clap::{ArgAction, Parser, Subcommand};
use termimad::{Area, MadSkin};

#[derive(Parser)]
struct Cli {
    file: PathBuf,
    #[command(subcommand)]
    /// render inside the give bo
    command: Option<CliCommand>,
    #[arg(long, action = ArgAction::SetTrue)]
    /// Optimise colors for black on white terminals
    light: bool,
}

#[derive(Subcommand)]
enum CliCommand {
    Area {
        left: u16,
        top: u16,
        width: u16,
        height: u16,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let content = read_to_string(&cli.file).context("Failed to read file")?;
    let is_tty = atty::is(atty::Stream::Stdout);
    let skin = match (cli.light, is_tty) {
        (false, true) => MadSkin::default_dark(),
        (true, true) => MadSkin::default_light(),
        (_, false) => MadSkin::no_style(),
    };
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
    Ok(())
}
