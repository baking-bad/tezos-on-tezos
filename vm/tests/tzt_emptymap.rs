mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_emptymap_nat_nat_00() -> Result<()> {
    TZT::load("tzt_emptymap_nat_nat_00.json")?.run()
}

#[test]
fn tzt_emptymap_string_string_00() -> Result<()> {
    TZT::load("tzt_emptymap_string_string_00.json")?.run()
}
