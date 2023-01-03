mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_left_int_nat_00() -> Result<()> {
    TZT::load("tzt_left_int_nat_00.json")?.run()
}
