pub struct AddCommand {}

impl AddCommand {
    pub fn new() -> Self {
        AddCommand {}
    }
}

impl<'a, 'b> crate::commands::Command<'a, 'b> for AddCommand {
    fn clap_command() -> clap::App<'a, 'b> {
        clap::SubCommand::with_name("add")
            .version(crate::get_version_str())
            .author(crate::get_author())
            .alias("install")
            .arg(clap::Arg::with_name("package")
                .help("Name of the package that should be installed. Optionally in the form of package@branch.")
                .index(1)
            )
            .about("Install a Nebula package. Default location is ~/Packages")
            .after_help(
                include_str!("add.txt")
            )
            .into()
    }
}
