mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_or_bool_bool_02() -> Result<()> {
    TZT::load("tzt_or_bool_bool_02.json")?.run()
}

#[test]
fn tzt_or_bool_bool_00() -> Result<()> {
    TZT::load("tzt_or_bool_bool_00.json")?.run()
}

#[test]
fn tzt_or_bool_bool_03() -> Result<()> {
    TZT::load("tzt_or_bool_bool_03.json")?.run()
}

#[test]
fn tzt_or_nat_nat_06() -> Result<()> {
    TZT::load("tzt_or_nat_nat_06.json")?.run()
}

#[test]
fn tzt_or_nat_nat_04() -> Result<()> {
    TZT::load("tzt_or_nat_nat_04.json")?.run()
}

#[test]
fn tzt_or_nat_nat_01() -> Result<()> {
    TZT::load("tzt_or_nat_nat_01.json")?.run()
}

#[test]
fn tzt_or_bool_bool_01() -> Result<()> {
    TZT::load("tzt_or_bool_bool_01.json")?.run()
}

#[test]
fn tzt_or_nat_nat_05() -> Result<()> {
    TZT::load("tzt_or_nat_nat_05.json")?.run()
}

#[test]
fn tzt_or_nat_nat_00() -> Result<()> {
    TZT::load("tzt_or_nat_nat_00.json")?.run()
}

#[test]
fn tzt_or_nat_nat_02() -> Result<()> {
    TZT::load("tzt_or_nat_nat_02.json")?.run()
}

#[test]
fn tzt_or_nat_nat_03() -> Result<()> {
    TZT::load("tzt_or_nat_nat_03.json")?.run()
}
