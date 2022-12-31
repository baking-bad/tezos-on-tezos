mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_ifleft_orintstring_00() -> Result<()> {
    TZT::load("tzt_ifleft_orintstring_00.json")?.run()
}

#[test]
fn tzt_ifleft_orstringint_00() -> Result<()> {
    TZT::load("tzt_ifleft_orstringint_00.json")?.run()
}
