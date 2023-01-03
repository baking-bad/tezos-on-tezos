mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_ifnone_optionint_00() -> Result<()> {
    TZT::load("tzt_ifnone_optionint_00.json")?.run()
}

#[test]
fn tzt_ifnone_optionnat_00() -> Result<()> {
    TZT::load("tzt_ifnone_optionnat_00.json")?.run()
}
