mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_some_string_00() -> Result<()> {
    TZT::load("tzt_some_string_00.json")?.run()
}

#[test]
fn tzt_some_int_00() -> Result<()> {
    TZT::load("tzt_some_int_00.json")?.run()
}

#[test]
fn tzt_some_pairintint_00() -> Result<()> {
    TZT::load("tzt_some_pairintint_00.json")?.run()
}
