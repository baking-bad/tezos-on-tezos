mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_lsl_04() -> Result<()> {
    TZT::load("tzt_lsl_04.json")?.run()
}

#[test]
fn tzt_lsl_01() -> Result<()> {
    TZT::load("tzt_lsl_01.json")?.run()
}

#[test]
fn tzt_lsl_06() -> Result<()> {
    TZT::load("tzt_lsl_06.json")?.run()
}

#[test]
fn tzt_lsl_02() -> Result<()> {
    TZT::load("tzt_lsl_02.json")?.run()
}

#[test]
fn tzt_lsl_05() -> Result<()> {
    TZT::load("tzt_lsl_05.json")?.run()
}

#[test]
fn tzt_lsl_00() -> Result<()> {
    TZT::load("tzt_lsl_00.json")?.run()
}

#[test]
fn tzt_lsl_03() -> Result<()> {
    TZT::load("tzt_lsl_03.json")?.run()
}
