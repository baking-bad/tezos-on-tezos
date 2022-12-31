mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_neg_int_02() -> Result<()> {
    TZT::load("tzt_neg_int_02.json")?.run()
}

#[test]
fn tzt_neg_int_01() -> Result<()> {
    TZT::load("tzt_neg_int_01.json")?.run()
}

#[test]
fn tzt_neg_nat_00() -> Result<()> {
    TZT::load("tzt_neg_nat_00.json")?.run()
}

#[test]
fn tzt_neg_int_00() -> Result<()> {
    TZT::load("tzt_neg_int_00.json")?.run()
}

#[test]
fn tzt_neg_nat_01() -> Result<()> {
    TZT::load("tzt_neg_nat_01.json")?.run()
}
