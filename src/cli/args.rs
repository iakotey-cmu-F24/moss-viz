use clap::{Parser};

use crate::engines::GraphvizEngine;
use crate::formats::OutputFormat;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct CliArgs {

    /// engine to use to generate visualizations
    #[clap(short = 'e', long)]
    #[arg(value_enum, default_value = "dot")]
    pub engine: GraphvizEngine,

    /// Output format to generate visualizations
    #[clap(short = 'f', long)]
    #[arg(value_enum, default_value = "svg")]
    pub format: OutputFormat,

}