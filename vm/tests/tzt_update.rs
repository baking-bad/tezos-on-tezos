mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_update_bigmapstringstring_01() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_01.json")?.run()
}

#[test]
fn tzt_update_bigmapstringstring_04() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_04.json")?.run()
}

#[test]
fn tzt_update_mapintint_00() -> Result<()> {
    TZT::load("tzt_update_mapintint_00.json")?.run()
}

#[test]
fn tzt_update_bigmapstringstring_03() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_03.json")?.run()
}

#[test]
fn tzt_update_bigmapstringstring_02() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_02.json")?.run()
}

#[test]
fn tzt_update_bigmapstringstring_06() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_06.json")?.run()
}

#[test]
fn tzt_update_bigmapstringstring_00() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_00.json")?.run()
}

#[test]
fn tzt_update_bigmapstringstring_05() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_05.json")?.run()
}

#[test]
fn tzt_update_bigmapstringstring_07() -> Result<()> {
    TZT::load("tzt_update_bigmapstringstring_07.json")?.run()
}

#[test]
fn tzt_update_setint_02() -> Result<()> {
    TZT::load("tzt_update_setint_02.json")?.run()
}

#[test]
fn tzt_update_setint_00() -> Result<()> {
    TZT::load("tzt_update_setint_00.json")?.run()
}

#[test]
fn tzt_update_setint_01() -> Result<()> {
    TZT::load("tzt_update_setint_01.json")?.run()
}

#[test]
fn tzt_update_mapintint_01() -> Result<()> {
    TZT::load("tzt_update_mapintint_01.json")?.run()
}
