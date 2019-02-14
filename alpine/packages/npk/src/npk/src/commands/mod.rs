pub mod add;

pub trait Command<'a, 'b> {
    fn clap_command() -> clap::App<'a, 'b>;
}