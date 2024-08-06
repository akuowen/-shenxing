mod command;
pub use command::{ConnectOpt, SxCommand};
use reedline_repl_rs::CallBackMap;
pub struct ShenXingContext {}

impl ShenXingContext {
    pub fn new() -> ShenXingContext {
        ShenXingContext {}
    }
}

pub type ReplCallBacks = CallBackMap<ShenXingContext, reedline_repl_rs::Error>;

pub fn func() -> ReplCallBacks {
    ReplCallBacks::new()
}
