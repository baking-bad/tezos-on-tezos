mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_cons_int_01() -> Result<()> {
    TZT::load("tzt_cons_int_01.json")?.run()
}

#[test]
fn tzt_cons_int_00() -> Result<()> {
    TZT::load("tzt_cons_int_00.json")?.run()
}

#[test]
fn tzt_cons_string_00() -> Result<()> {
    TZT::load("tzt_cons_string_00.json")?.run()
}

#[test]
fn tzt_cons_int_02() -> Result<()> {
    TZT::load("tzt_cons_int_02.json")?.run()
}
