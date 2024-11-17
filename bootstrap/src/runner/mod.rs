use crate::ast::SourceFile;

#[derive(Debug)]
pub enum Error {}

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub struct Runner {}

impl Runner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self, source_file: SourceFile) -> Result<()> {
        todo!()
    }
}