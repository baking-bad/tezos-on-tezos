mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_eq_01() -> Result<()> {
    TZT::load("tzt_eq_01.json")?.run()
}

#[test]
fn tzt_eq_02() -> Result<()> {
    TZT::load("tzt_eq_02.json")?.run()
}

#[test]
fn tzt_eq_03() -> Result<()> {
    TZT::load("tzt_eq_03.json")?.run()
}

#[test]
fn tzt_eq_00() -> Result<()> {
    TZT::load("tzt_eq_00.json")?.run()
}

#[test]
fn tzt_eq_04() -> Result<()> {
    TZT::load("tzt_eq_04.json")?.run()
}
