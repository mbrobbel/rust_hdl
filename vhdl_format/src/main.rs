fn main() {
    use clap::{App, Arg};
    use env_logger::{Builder, Env};
    use log::info;
    use std::fs::File;
    use std::io::Write;
    use vhdl_parser::VHDLParser;
    Builder::from_env(Env::default().default_filter_or("debug")).init();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("files")
                .help("The list of files to format.")
                .index(1)
                .required(true)
                .multiple(true),
        )
        .get_matches();

    let parser = VHDLParser::new();
    if let Some(files) = matches.values_of("files") {
        for (_file_name, _, design_file) in
            parser.parse_design_files(files.map(|x| x.to_owned()).collect(), 1)
        {
            let result = design_file.unwrap();

            let mut buffer = File::create("output.vhd").expect("file create failed");
            write!(buffer, "{}", result.clone()).expect("vhd write failed");

            let mut buffer = File::create("output.vhd.ast").expect("file create failed");
            write!(buffer, "{:#?}", result.clone()).expect("ast write failed");

            for (_, messages, _) in parser.parse_design_files(vec!["output.vhd".to_owned()], 1) {
                info!("{:?}", messages);
            }

            info!("{}", result.clone());
        }
    }
}
