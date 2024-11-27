use crate::generate::c;

pub enum Error {}

type Result<T> = core::result::Result<T, Error>;

pub(crate) fn write(node: &c::Node) -> Result<String> {
    let writer = Writer {};
    writer.write(node)
}

pub(crate) struct Writer {}

impl Writer {
    pub(crate) fn write(&self, node: &c::Node) -> Result<String> {
        Ok(r#"
#include <stdio.h>
int main(void){
    printf("Hello World\n");
    return 0;
}
        "#.to_string())
    }
}