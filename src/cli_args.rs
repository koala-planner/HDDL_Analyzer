use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct CLIArgs {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Verify(HDDLInfo),
    Metadata(HDDLInfo)
}

#[derive(Parser)]
pub struct HDDLInfo {
    #[arg(index = 1)]
    pub domain_path: String,
    #[arg(short, long)]
    pub problem_path: Option<String>,
}