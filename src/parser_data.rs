use clap::Parser;

#[derive(Parser)]
pub(crate) struct ParserData {
    pub(crate) path: String,

    /// If set, the program will include any directory within the path.
    #[arg(short, long, default_value = "false")]
    pub(crate) recursive: bool,

    /// Use it to limit the depth of the directory traversal.
    #[arg(short, long, default_value = "999")]
    pub(crate) depth: Option<usize>,

    /// If set, the program will print only summary information.
    #[arg(short, long, default_value = "false")]
    pub(crate) quiet: bool,

    /// If set, the program will ignore the file extension.
    #[arg(short, long)]
    pub(crate) ignore_extension: Option<Vec<String>>,

    /// If set, the program will include only this file extension. It will ignore any other file extension.
    #[arg(short, long)]
    pub(crate) only_extension: Option<Vec<String>>,

    /// If set, the program will print the size in human-readable format.
    #[arg(long, default_value = "false")]
    pub(crate) human_unit: bool,
}
