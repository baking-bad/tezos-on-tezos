mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_implicitaccount_00() -> Result<()> {
    TZT::load("tzt_implicitaccount_00.json")?.run()
}
