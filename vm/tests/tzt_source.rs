mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_source_00() -> Result<()> {
    TZT::load("tzt_source_00.json")?.run()
}
