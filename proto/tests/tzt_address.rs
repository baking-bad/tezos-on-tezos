mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_address_02() -> Result<()> {
    TZT::load("tzt_address_02.json")?.run()
}

#[test]
fn tzt_address_01() -> Result<()> {
    TZT::load("tzt_address_01.json")?.run()
}

#[test]
fn tzt_address_00() -> Result<()> {
    TZT::load("tzt_address_00.json")?.run()
}
