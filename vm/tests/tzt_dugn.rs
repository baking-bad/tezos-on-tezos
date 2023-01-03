mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_dugn_00() -> Result<()> {
    TZT::load("tzt_dugn_00.json")?.run()
}
