mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_abs_00() -> Result<()> {
    TZT::load("tzt_abs_00.json")?.run()
}

#[test]
fn tzt_abs_01() -> Result<()> {
    TZT::load("tzt_abs_01.json")?.run()
}

#[test]
fn tzt_abs_02() -> Result<()> {
    TZT::load("tzt_abs_02.json")?.run()
}
