extern crate clap;
#[macro_use]
extern crate log;

fn main() {
    use clap::{App, Arg};
    use env_logger;
    use vhdl_parser::VHDLParser;
    env_logger::init();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("files")
                .help("The list of files to parse. The files are only parsed without any semantic analysis")
                .index(1)
                .required(true)
                .multiple(true)
        ).get_matches();

    let parser = VHDLParser::new();
    if let Some(files) = matches.values_of("files") {
        for (_file_name, _, design_file) in
            parser.parse_design_files(files.map(|x| x.to_owned()).collect(), 1)
        {
            let result = design_file.unwrap();
            info!("{:#?}", result.clone());
            info!("{}", result.clone());
        }
    }
}
