mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_loopleft_02() -> Result<()> {
    TZT::load("tzt_loopleft_02.json")?.run()
}

#[test]
fn tzt_loopleft_03() -> Result<()> {
    TZT::load("tzt_loopleft_03.json")?.run()
}

#[test]
fn tzt_loopleft_00() -> Result<()> {
    TZT::load("tzt_loopleft_00.json")?.run()
}

#[test]
fn tzt_loopleft_04() -> Result<()> {
    TZT::load("tzt_loopleft_04.json")?.run()
}

#[test]
fn tzt_loopleft_01() -> Result<()> {
    TZT::load("tzt_loopleft_01.json")?.run()
}
