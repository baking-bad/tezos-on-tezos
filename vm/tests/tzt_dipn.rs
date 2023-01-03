mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_dipn_00() -> Result<()> {
    TZT::load("tzt_dipn_00.json")?.run()
}
