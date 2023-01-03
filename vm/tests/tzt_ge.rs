mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_ge_03() -> Result<()> {
    TZT::load("tzt_ge_03.json")?.run()
}

#[test]
fn tzt_ge_01() -> Result<()> {
    TZT::load("tzt_ge_01.json")?.run()
}

#[test]
fn tzt_ge_00() -> Result<()> {
    TZT::load("tzt_ge_00.json")?.run()
}

#[test]
fn tzt_ge_04() -> Result<()> {
    TZT::load("tzt_ge_04.json")?.run()
}

#[test]
fn tzt_ge_02() -> Result<()> {
    TZT::load("tzt_ge_02.json")?.run()
}
