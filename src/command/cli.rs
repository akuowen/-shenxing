use clap::{command, Parser};

use super::ConnectOpt;

#[derive(Debug, Parser)]
pub enum SxCommand {
    #[command(
        name = "connect",
        about = "Connect to a dataset and register it to Shenxing"
    )]
    Connect(ConnectOpt),
}
