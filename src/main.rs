use climb::{App, create_app, Command, CommandOption};
use owo_colors::OwoColorize;

pub mod commands;
pub mod built;
pub mod utils;

fn main() -> anyhow::Result<()> {
    let result = create_app!()
        .name(built::PKG_NAME)
        .desc(built::PKG_DESCRIPTION)
        .version(built::PKG_VERSION)
        .command(
            Command::new(
                "list",
                "List all crate workspace members",
                commands::list::run,
            )
            .option(
                CommandOption::new(
                    "path",
                    "Optional path to workspace",
                )
                .alias("p")
                .arg("PATH")
            )
        )
        .run();

    match result {
        Ok(_) => (),
        Err(e) => println!("{} {}", "Error:".red().bold(), e),
    };

    Ok(())
}
