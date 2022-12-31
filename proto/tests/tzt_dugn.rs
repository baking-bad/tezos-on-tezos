mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_dugn_00() -> Result<()> {
    TZT::load("tzt_dugn_00.json")?.run()
}
