mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_push_string_00() -> Result<()> {
    TZT::load("tzt_push_string_00.json")?.run()
}

#[test]
fn tzt_push_int_00() -> Result<()> {
    TZT::load("tzt_push_int_00.json")?.run()
}
