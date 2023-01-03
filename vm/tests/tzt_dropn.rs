mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_dropn_00() -> Result<()> {
    TZT::load("tzt_dropn_00.json")?.run()
}

#[test]
fn tzt_dropn_01() -> Result<()> {
    TZT::load("tzt_dropn_01.json")?.run()
}
