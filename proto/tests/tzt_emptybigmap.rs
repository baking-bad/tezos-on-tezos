mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_emptybigmap_nat_nat_00() -> Result<()> {
    TZT::load("tzt_emptybigmap_nat_nat_00.json")?.run()
}
