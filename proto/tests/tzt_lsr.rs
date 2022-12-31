mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_lsr_00() -> Result<()> {
    TZT::load("tzt_lsr_00.json")?.run()
}

#[test]
fn tzt_lsr_05() -> Result<()> {
    TZT::load("tzt_lsr_05.json")?.run()
}

#[test]
fn tzt_lsr_01() -> Result<()> {
    TZT::load("tzt_lsr_01.json")?.run()
}

#[test]
fn tzt_lsr_04() -> Result<()> {
    TZT::load("tzt_lsr_04.json")?.run()
}

#[test]
fn tzt_lsr_02() -> Result<()> {
    TZT::load("tzt_lsr_02.json")?.run()
}

#[test]
fn tzt_lsr_03() -> Result<()> {
    TZT::load("tzt_lsr_03.json")?.run()
}
