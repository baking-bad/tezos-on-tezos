mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_now_00() -> Result<()> {
    TZT::load("tzt_now_00.json")?.run()
}
