use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct CustomArgs {
    pub file: Option<std::path::PathBuf>,
}
