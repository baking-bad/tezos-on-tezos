mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_balance_00() -> Result<()> {
    TZT::load("tzt_balance_00.json")?.run()
}
