mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_ediv_mutez_nat_06() -> Result<()> {
    TZT::load("tzt_ediv_mutez_nat_06.json")?.run()
}

#[test]
fn tzt_ediv_int_int_00() -> Result<()> {
    TZT::load("tzt_ediv_int_int_00.json")?.run()
}

#[test]
fn tzt_ediv_mutez_nat_01() -> Result<()> {
    TZT::load("tzt_ediv_mutez_nat_01.json")?.run()
}

#[test]
fn tzt_ediv_mutez_mutez_01() -> Result<()> {
    TZT::load("tzt_ediv_mutez_mutez_01.json")?.run()
}

#[test]
fn tzt_ediv_mutez_nat_04() -> Result<()> {
    TZT::load("tzt_ediv_mutez_nat_04.json")?.run()
}

#[test]
fn tzt_ediv_int_int_01() -> Result<()> {
    TZT::load("tzt_ediv_int_int_01.json")?.run()
}

#[test]
fn tzt_ediv_int_int_03() -> Result<()> {
    TZT::load("tzt_ediv_int_int_03.json")?.run()
}

#[test]
fn tzt_ediv_mutez_nat_03() -> Result<()> {
    TZT::load("tzt_ediv_mutez_nat_03.json")?.run()
}

#[test]
fn tzt_ediv_mutez_nat_00() -> Result<()> {
    TZT::load("tzt_ediv_mutez_nat_00.json")?.run()
}

#[test]
fn tzt_ediv_mutez_nat_02() -> Result<()> {
    TZT::load("tzt_ediv_mutez_nat_02.json")?.run()
}

#[test]
fn tzt_ediv_mutez_mutez_03() -> Result<()> {
    TZT::load("tzt_ediv_mutez_mutez_03.json")?.run()
}

#[test]
fn tzt_ediv_mutez_mutez_02() -> Result<()> {
    TZT::load("tzt_ediv_mutez_mutez_02.json")?.run()
}

#[test]
fn tzt_ediv_mutez_mutez_00() -> Result<()> {
    TZT::load("tzt_ediv_mutez_mutez_00.json")?.run()
}

#[test]
fn tzt_ediv_mutez_nat_05() -> Result<()> {
    TZT::load("tzt_ediv_mutez_nat_05.json")?.run()
}

#[test]
fn tzt_ediv_int_int_02() -> Result<()> {
    TZT::load("tzt_ediv_int_int_02.json")?.run()
}
