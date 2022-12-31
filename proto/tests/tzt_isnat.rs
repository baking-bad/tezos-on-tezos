mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_isnat_00() -> Result<()> {
    TZT::load("tzt_isnat_00.json")?.run()
}

#[test]
fn tzt_isnat_01() -> Result<()> {
    TZT::load("tzt_isnat_01.json")?.run()
}
