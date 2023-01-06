mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_dig_04() -> Result<()> {
    TZT::load("tzt_dig_04.json")?.run()
}

#[test]
fn tzt_dig_02() -> Result<()> {
    TZT::load("tzt_dig_02.json")?.run()
}

#[test]
fn tzt_dig_00() -> Result<()> {
    TZT::load("tzt_dig_00.json")?.run()
}

#[test]
fn tzt_dig_03() -> Result<()> {
    TZT::load("tzt_dig_03.json")?.run()
}

#[test]
fn tzt_dig_01() -> Result<()> {
    TZT::load("tzt_dig_01.json")?.run()
}
