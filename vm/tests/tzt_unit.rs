mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_unit_00() -> Result<()> {
    TZT::load("tzt_unit_00.json")?.run()
}
