mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_failwith_00() -> Result<()> {
    TZT::load("tzt_failwith_00.json")?.run()
}
