use clap::Parser;
use diagnostic_quick::error_3rd::Url;
use diagnostic_quick::QResult;
use crate::CmdShared;

#[derive(Parser, Debug)]
pub struct CmdNew {
    pub name: Url,
    #[clap(flatten)]
    pub other: CmdShared,
}

impl CmdNew {
    pub fn run(&self) -> QResult {
        println!("{:?}", self);
        println!("cloning {}", self.name);
        Ok(())
    }
}