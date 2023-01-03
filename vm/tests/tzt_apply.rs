mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_apply_00() -> Result<()> {
    TZT::load("tzt_apply_00.json")?.run()
}
