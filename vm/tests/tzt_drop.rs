mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_drop_00() -> Result<()> {
    TZT::load("tzt_drop_00.json")?.run()
}
