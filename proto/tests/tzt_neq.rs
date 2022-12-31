mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_neq_04() -> Result<()> {
    TZT::load("tzt_neq_04.json")?.run()
}

#[test]
fn tzt_neq_02() -> Result<()> {
    TZT::load("tzt_neq_02.json")?.run()
}

#[test]
fn tzt_neq_03() -> Result<()> {
    TZT::load("tzt_neq_03.json")?.run()
}

#[test]
fn tzt_neq_01() -> Result<()> {
    TZT::load("tzt_neq_01.json")?.run()
}

#[test]
fn tzt_neq_00() -> Result<()> {
    TZT::load("tzt_neq_00.json")?.run()
}
