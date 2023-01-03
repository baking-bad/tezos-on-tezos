mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_cdr_01() -> Result<()> {
    TZT::load("tzt_cdr_01.json")?.run()
}

#[test]
fn tzt_cdr_00() -> Result<()> {
    TZT::load("tzt_cdr_00.json")?.run()
}
