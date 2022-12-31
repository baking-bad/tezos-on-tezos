mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_pair_int_int_00() -> Result<()> {
    TZT::load("tzt_pair_int_int_00.json")?.run()
}

#[test]
fn tzt_pair_pair_nat_string_pair_string_nat_00() -> Result<()> {
    TZT::load("tzt_pair_pair_nat_string_pair_string_nat_00.json")?.run()
}

#[test]
fn tzt_pair_nat_string_00() -> Result<()> {
    TZT::load("tzt_pair_nat_string_00.json")?.run()
}
