mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_none_int_00() -> Result<()> {
    TZT::load("tzt_none_int_00.json")?.run()
}

#[test]
fn tzt_none_pair_nat_string() -> Result<()> {
    TZT::load("tzt_none_pair_nat_string.json")?.run()
}
