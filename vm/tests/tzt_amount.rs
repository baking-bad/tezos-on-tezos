mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_amount_00() -> Result<()> {
    TZT::load("tzt_amount_00.json")?.run()
}
