mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_slice_bytes_00() -> Result<()> {
    TZT::load("tzt_slice_bytes_00.json")?.run()
}

#[test]
fn tzt_slice_bytes_03() -> Result<()> {
    TZT::load("tzt_slice_bytes_03.json")?.run()
}

#[test]
fn tzt_slice_string_02() -> Result<()> {
    TZT::load("tzt_slice_string_02.json")?.run()
}

#[test]
fn tzt_slice_string_00() -> Result<()> {
    TZT::load("tzt_slice_string_00.json")?.run()
}

#[test]
fn tzt_slice_string_01() -> Result<()> {
    TZT::load("tzt_slice_string_01.json")?.run()
}

#[test]
fn tzt_slice_string_03() -> Result<()> {
    TZT::load("tzt_slice_string_03.json")?.run()
}

#[test]
fn tzt_slice_bytes_02() -> Result<()> {
    TZT::load("tzt_slice_bytes_02.json")?.run()
}

#[test]
fn tzt_slice_string_05() -> Result<()> {
    TZT::load("tzt_slice_string_05.json")?.run()
}

#[test]
fn tzt_slice_string_04() -> Result<()> {
    TZT::load("tzt_slice_string_04.json")?.run()
}

#[test]
fn tzt_slice_bytes_04() -> Result<()> {
    TZT::load("tzt_slice_bytes_04.json")?.run()
}

#[test]
fn tzt_slice_bytes_01() -> Result<()> {
    TZT::load("tzt_slice_bytes_01.json")?.run()
}
