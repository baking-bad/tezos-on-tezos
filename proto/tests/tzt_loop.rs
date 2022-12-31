mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_loop_01() -> Result<()> {
    TZT::load("tzt_loop_01.json")?.run()
}

#[test]
fn tzt_loop_00() -> Result<()> {
    TZT::load("tzt_loop_00.json")?.run()
}

#[test]
fn tzt_loop_02() -> Result<()> {
    TZT::load("tzt_loop_02.json")?.run()
}
