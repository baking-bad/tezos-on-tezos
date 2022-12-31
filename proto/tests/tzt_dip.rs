mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_dip_02() -> Result<()> {
    TZT::load("tzt_dip_02.json")?.run()
}

#[test]
fn tzt_dip_00() -> Result<()> {
    TZT::load("tzt_dip_00.json")?.run()
}

#[test]
fn tzt_dip_01() -> Result<()> {
    TZT::load("tzt_dip_01.json")?.run()
}
