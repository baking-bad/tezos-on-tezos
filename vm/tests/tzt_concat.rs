mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_concat_liststring_00() -> Result<()> {
    TZT::load("tzt_concat_liststring_00.json")?.run()
}

#[test]
fn tzt_concat_string_02() -> Result<()> {
    TZT::load("tzt_concat_string_02.json")?.run()
}

#[test]
fn tzt_concat_liststring_02() -> Result<()> {
    TZT::load("tzt_concat_liststring_02.json")?.run()
}

#[test]
fn tzt_concat_liststring_01() -> Result<()> {
    TZT::load("tzt_concat_liststring_01.json")?.run()
}

#[test]
fn tzt_concat_bytes_00() -> Result<()> {
    TZT::load("tzt_concat_bytes_00.json")?.run()
}

#[test]
fn tzt_concat_liststring_03() -> Result<()> {
    TZT::load("tzt_concat_liststring_03.json")?.run()
}

#[test]
fn tzt_concat_string_00() -> Result<()> {
    TZT::load("tzt_concat_string_00.json")?.run()
}

#[test]
fn tzt_concat_liststring_04() -> Result<()> {
    TZT::load("tzt_concat_liststring_04.json")?.run()
}

#[test]
fn tzt_concat_listbytes_02() -> Result<()> {
    TZT::load("tzt_concat_listbytes_02.json")?.run()
}

#[test]
fn tzt_concat_listbytes_01() -> Result<()> {
    TZT::load("tzt_concat_listbytes_01.json")?.run()
}

#[test]
fn tzt_concat_listbytes_00() -> Result<()> {
    TZT::load("tzt_concat_listbytes_00.json")?.run()
}

#[test]
fn tzt_concat_string_01() -> Result<()> {
    TZT::load("tzt_concat_string_01.json")?.run()
}

#[test]
fn tzt_concat_bytes_01() -> Result<()> {
    TZT::load("tzt_concat_bytes_01.json")?.run()
}
