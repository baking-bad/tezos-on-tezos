mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_int_nat_00() -> Result<()> {
    TZT::load("tzt_int_nat_00.json")?.run()
}

#[test]
fn tzt_int_nat_01() -> Result<()> {
    TZT::load("tzt_int_nat_01.json")?.run()
}
