mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_nil_nat_00() -> Result<()> {
    TZT::load("tzt_nil_nat_00.json")?.run()
}
