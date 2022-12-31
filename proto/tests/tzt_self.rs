mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_self_00() -> Result<()> {
    TZT::load("tzt_self_00.json")?.run()
}
