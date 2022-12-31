mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_sender_00() -> Result<()> {
    TZT::load("tzt_sender_00.json")?.run()
}
