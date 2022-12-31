mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_le_01() -> Result<()> {
    TZT::load("tzt_le_01.json")?.run()
}

#[test]
fn tzt_le_02() -> Result<()> {
    TZT::load("tzt_le_02.json")?.run()
}

#[test]
fn tzt_le_00() -> Result<()> {
    TZT::load("tzt_le_00.json")?.run()
}

#[test]
fn tzt_le_04() -> Result<()> {
    TZT::load("tzt_le_04.json")?.run()
}

#[test]
fn tzt_le_03() -> Result<()> {
    TZT::load("tzt_le_03.json")?.run()
}
