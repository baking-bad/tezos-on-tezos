mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_sub_timestamp_int_00() -> Result<()> {
    TZT::load("tzt_sub_timestamp_int_00.json")?.run()
}

#[test]
fn tzt_sub_int_int_00() -> Result<()> {
    TZT::load("tzt_sub_int_int_00.json")?.run()
}

#[test]
fn tzt_sub_timestamp_timestamp_01() -> Result<()> {
    TZT::load("tzt_sub_timestamp_timestamp_01.json")?.run()
}

#[test]
fn tzt_sub_mutez_mutez_01() -> Result<()> {
    TZT::load("tzt_sub_mutez_mutez_01.json")?.run()
}

#[test]
fn tzt_sub_mutez_mutez_00() -> Result<()> {
    TZT::load("tzt_sub_mutez_mutez_00.json")?.run()
}

#[test]
fn tzt_sub_timestamp_int_04() -> Result<()> {
    TZT::load("tzt_sub_timestamp_int_04.json")?.run()
}

#[test]
fn tzt_sub_timestamp_timestamp_00() -> Result<()> {
    TZT::load("tzt_sub_timestamp_timestamp_00.json")?.run()
}

#[test]
fn tzt_sub_timestamp_int_03() -> Result<()> {
    TZT::load("tzt_sub_timestamp_int_03.json")?.run()
}

#[test]
fn tzt_sub_timestamp_int_01() -> Result<()> {
    TZT::load("tzt_sub_timestamp_int_01.json")?.run()
}

#[test]
fn tzt_sub_timestamp_timestamp_02() -> Result<()> {
    TZT::load("tzt_sub_timestamp_timestamp_02.json")?.run()
}

#[test]
fn tzt_sub_timestamp_timestamp_03() -> Result<()> {
    TZT::load("tzt_sub_timestamp_timestamp_03.json")?.run()
}

#[test]
fn tzt_sub_timestamp_int_02() -> Result<()> {
    TZT::load("tzt_sub_timestamp_int_02.json")?.run()
}

#[test]
fn tzt_sub_int_int_01() -> Result<()> {
    TZT::load("tzt_sub_int_int_01.json")?.run()
}
