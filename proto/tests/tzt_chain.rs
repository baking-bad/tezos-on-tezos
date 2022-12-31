mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_chain_id_01() -> Result<()> {
    TZT::load("tzt_chain_id_01.json")?.run()
}

#[test]
fn tzt_chain_id_00() -> Result<()> {
    TZT::load("tzt_chain_id_00.json")?.run()
}
