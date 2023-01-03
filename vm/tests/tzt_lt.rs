mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_lt_04() -> Result<()> {
    TZT::load("tzt_lt_04.json")?.run()
}

#[test]
fn tzt_lt_01() -> Result<()> {
    TZT::load("tzt_lt_01.json")?.run()
}

#[test]
fn tzt_lt_02() -> Result<()> {
    TZT::load("tzt_lt_02.json")?.run()
}

#[test]
fn tzt_lt_00() -> Result<()> {
    TZT::load("tzt_lt_00.json")?.run()
}

#[test]
fn tzt_lt_03() -> Result<()> {
    TZT::load("tzt_lt_03.json")?.run()
}
