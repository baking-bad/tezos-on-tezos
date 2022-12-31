mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_size_listint_00() -> Result<()> {
    TZT::load("tzt_size_listint_00.json")?.run()
}

#[test]
fn tzt_size_setint_01() -> Result<()> {
    TZT::load("tzt_size_setint_01.json")?.run()
}

#[test]
fn tzt_size_setint_03() -> Result<()> {
    TZT::load("tzt_size_setint_03.json")?.run()
}

#[test]
fn tzt_size_mapintint_00() -> Result<()> {
    TZT::load("tzt_size_mapintint_00.json")?.run()
}

#[test]
fn tzt_size_setint_02() -> Result<()> {
    TZT::load("tzt_size_setint_02.json")?.run()
}

#[test]
fn tzt_size_listint_01() -> Result<()> {
    TZT::load("tzt_size_listint_01.json")?.run()
}

#[test]
fn tzt_size_setstring_00() -> Result<()> {
    TZT::load("tzt_size_setstring_00.json")?.run()
}

#[test]
fn tzt_size_mapstringnat_01() -> Result<()> {
    TZT::load("tzt_size_mapstringnat_01.json")?.run()
}

#[test]
fn tzt_size_mapstringnat_00() -> Result<()> {
    TZT::load("tzt_size_mapstringnat_00.json")?.run()
}

#[test]
fn tzt_size_bytes_00() -> Result<()> {
    TZT::load("tzt_size_bytes_00.json")?.run()
}

#[test]
fn tzt_size_string_00() -> Result<()> {
    TZT::load("tzt_size_string_00.json")?.run()
}

#[test]
fn tzt_size_mapstringnat_02() -> Result<()> {
    TZT::load("tzt_size_mapstringnat_02.json")?.run()
}

#[test]
fn tzt_size_mapstringnat_03() -> Result<()> {
    TZT::load("tzt_size_mapstringnat_03.json")?.run()
}

#[test]
fn tzt_size_listint_02() -> Result<()> {
    TZT::load("tzt_size_listint_02.json")?.run()
}

#[test]
fn tzt_size_setint_00() -> Result<()> {
    TZT::load("tzt_size_setint_00.json")?.run()
}

#[test]
fn tzt_size_listint_03() -> Result<()> {
    TZT::load("tzt_size_listint_03.json")?.run()
}
