mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_right_nat_int_00() -> Result<()> {
    TZT::load("tzt_right_nat_int_00.json")?.run()
}
