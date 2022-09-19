use clap::Parser;
use cli::Args;
use generator::GeneratorBuilder;

mod cli;
mod errors;
mod generator;
mod manifest;
mod utils;

fn main() {
    let cli = Args::parse_from(wild::args());

    let generator = GeneratorBuilder::default()
        .with_whitelist(cli.whitelist)
        .version(cli.version)
        .depends_on(cli.depends_on)
        .with_blacklist(cli.blacklist)
        .build();

    generator.generate(cli.input, cli.output);
}
