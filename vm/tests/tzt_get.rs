mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_get_mapintint_00() -> Result<()> {
    TZT::load("tzt_get_mapintint_00.json")?.run()
}

#[test]
fn tzt_get_bigmapstringstring_01() -> Result<()> {
    TZT::load("tzt_get_bigmapstringstring_01.json")?.run()
}

#[test]
fn tzt_get_mapintint_01() -> Result<()> {
    TZT::load("tzt_get_mapintint_01.json")?.run()
}

#[test]
fn tzt_get_mapstringstring_02() -> Result<()> {
    TZT::load("tzt_get_mapstringstring_02.json")?.run()
}

#[test]
fn tzt_get_mapstringstring_01() -> Result<()> {
    TZT::load("tzt_get_mapstringstring_01.json")?.run()
}

#[test]
fn tzt_get_mapstringstring_00() -> Result<()> {
    TZT::load("tzt_get_mapstringstring_00.json")?.run()
}

#[test]
fn tzt_get_bigmapstringstring_00() -> Result<()> {
    TZT::load("tzt_get_bigmapstringstring_00.json")?.run()
}

#[test]
fn tzt_get_bigmapstringstring_02() -> Result<()> {
    TZT::load("tzt_get_bigmapstringstring_02.json")?.run()
}
