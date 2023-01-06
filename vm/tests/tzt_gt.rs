mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_gt_03() -> Result<()> {
    TZT::load("tzt_gt_03.json")?.run()
}

#[test]
fn tzt_gt_02() -> Result<()> {
    TZT::load("tzt_gt_02.json")?.run()
}

#[test]
fn tzt_gt_00() -> Result<()> {
    TZT::load("tzt_gt_00.json")?.run()
}

#[test]
fn tzt_gt_01() -> Result<()> {
    TZT::load("tzt_gt_01.json")?.run()
}

#[test]
fn tzt_gt_04() -> Result<()> {
    TZT::load("tzt_gt_04.json")?.run()
}
