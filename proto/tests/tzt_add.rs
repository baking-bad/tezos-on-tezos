mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_add_int_nat_00() -> Result<()> {
    TZT::load("tzt_add_int_nat_00.json")?.run()
}

#[test]
fn tzt_add_timestamp_int_02() -> Result<()> {
    TZT::load("tzt_add_timestamp_int_02.json")?.run()
}

#[test]
fn tzt_add_nat_int_00() -> Result<()> {
    TZT::load("tzt_add_nat_int_00.json")?.run()
}

#[test]
fn tzt_add_timestamp_int_00() -> Result<()> {
    TZT::load("tzt_add_timestamp_int_00.json")?.run()
}

#[test]
fn tzt_add_timestamp_int_01() -> Result<()> {
    TZT::load("tzt_add_timestamp_int_01.json")?.run()
}

#[test]
fn tzt_add_int_nat_01() -> Result<()> {
    TZT::load("tzt_add_int_nat_01.json")?.run()
}

#[test]
fn tzt_add_mutez_mutez_01() -> Result<()> {
    TZT::load("tzt_add_mutez_mutez_01.json")?.run()
}

#[test]
fn tzt_add_int_timestamp_00() -> Result<()> {
    TZT::load("tzt_add_int_timestamp_00.json")?.run()
}

#[test]
fn tzt_add_timestamp_int_03() -> Result<()> {
    TZT::load("tzt_add_timestamp_int_03.json")?.run()
}

#[test]
fn tzt_add_mutez_mutez_00() -> Result<()> {
    TZT::load("tzt_add_mutez_mutez_00.json")?.run()
}

#[test]
fn tzt_add_int_int_00() -> Result<()> {
    TZT::load("tzt_add_int_int_00.json")?.run()
}

#[test]
fn tzt_add_nat_nat_00() -> Result<()> {
    TZT::load("tzt_add_nat_nat_00.json")?.run()
}
