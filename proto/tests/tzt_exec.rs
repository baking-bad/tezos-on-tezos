mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_exec_00() -> Result<()> {
    TZT::load("tzt_exec_00.json")?.run()
}

#[test]
fn tzt_exec_02() -> Result<()> {
    TZT::load("tzt_exec_02.json")?.run()
}

#[test]
fn tzt_exec_03() -> Result<()> {
    TZT::load("tzt_exec_03.json")?.run()
}

#[test]
fn tzt_exec_01() -> Result<()> {
    TZT::load("tzt_exec_01.json")?.run()
}
