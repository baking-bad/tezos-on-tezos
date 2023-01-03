mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_unpair_pairstringstring_00() -> Result<()> {
    TZT::load("tzt_unpair_pairstringstring_00.json")?.run()
}
