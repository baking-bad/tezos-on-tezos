mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_not_nat_05() -> Result<()> {
    TZT::load("tzt_not_nat_05.json")?.run()
}

#[test]
fn tzt_not_nat_01() -> Result<()> {
    TZT::load("tzt_not_nat_01.json")?.run()
}

#[test]
fn tzt_not_nat_02() -> Result<()> {
    TZT::load("tzt_not_nat_02.json")?.run()
}

#[test]
fn tzt_not_bool_01() -> Result<()> {
    TZT::load("tzt_not_bool_01.json")?.run()
}

#[test]
fn tzt_not_nat_03() -> Result<()> {
    TZT::load("tzt_not_nat_03.json")?.run()
}

#[test]
fn tzt_not_int_00() -> Result<()> {
    TZT::load("tzt_not_int_00.json")?.run()
}

#[test]
fn tzt_not_nat_07() -> Result<()> {
    TZT::load("tzt_not_nat_07.json")?.run()
}

#[test]
fn tzt_not_bool_00() -> Result<()> {
    TZT::load("tzt_not_bool_00.json")?.run()
}

#[test]
fn tzt_not_nat_06() -> Result<()> {
    TZT::load("tzt_not_nat_06.json")?.run()
}

#[test]
fn tzt_not_nat_04() -> Result<()> {
    TZT::load("tzt_not_nat_04.json")?.run()
}

#[test]
fn tzt_not_nat_00() -> Result<()> {
    TZT::load("tzt_not_nat_00.json")?.run()
}
