use crate::cli::*;

pub fn cli() -> App {
    subcommand("new")
        .about("Create a new rust-clap-cli  project at <path>")
        .arg_quiet()
        .arg(Arg::new("path").required(true))
        .arg(opt("registry", "Registry to use").value_name("REGISTRY"))
        .arg_new_opts()
        .after_help("Run `rust-clap-cli help new` for more detailed information.\n")
}

pub fn exec(_: &mut Config, _: &ArgMatches) -> CliResult {
    println!("Hei thanks to create new project");
    Ok(())
}
