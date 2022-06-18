use crate::commands;
use crate::cli::*;

pub fn parse() -> App {
    let usage = "rust-clap-cli [OPTIONS] [SUBCOMMAND]";
    App::new("rust-clap-cli")
        .allow_external_subcommands(true)
        .setting(AppSettings::DeriveDisplayOrder)
        .disable_colored_help(false)
        .override_usage(usage)
        .help_template(get_template())
        .arg(flag("version", "Print version info and exit").short('V'))
        .arg(flag("help", "List command"))
        .subcommands(commands::builtin())
}

pub fn print_help() {
    println!("{}", get_template()
        .replace("{usage}", "rust-clap-cli [OPTIONS] [SUBCOMMAND]")
        .replace("{options}", "\t--help")
    );
}

fn get_template() -> &'static str {
    "\
rust-clap-cli v0.1
USAGE:
    {usage}
OPTIONS:
{options}
Some common rust-clap-cli commands are (see all commands with --list):
    help           Show help
See 'rust-clap-cli help <command>' for more information on a specific command.\n"
}
