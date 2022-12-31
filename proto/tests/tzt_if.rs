mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_if_00() -> Result<()> {
    TZT::load("tzt_if_00.json")?.run()
}

#[test]
fn tzt_if_01() -> Result<()> {
    TZT::load("tzt_if_01.json")?.run()
}
