mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_iter_listint_02() -> Result<()> {
    TZT::load("tzt_iter_listint_02.json")?.run()
}

#[test]
fn tzt_iter_setint_01() -> Result<()> {
    TZT::load("tzt_iter_setint_01.json")?.run()
}

#[test]
fn tzt_iter_setint_00() -> Result<()> {
    TZT::load("tzt_iter_setint_00.json")?.run()
}

#[test]
fn tzt_iter_setstring_01() -> Result<()> {
    TZT::load("tzt_iter_setstring_01.json")?.run()
}

#[test]
fn tzt_iter_mapstringstring_00() -> Result<()> {
    TZT::load("tzt_iter_mapstringstring_00.json")?.run()
}

#[test]
fn tzt_iter_liststring_01() -> Result<()> {
    TZT::load("tzt_iter_liststring_01.json")?.run()
}

#[test]
fn tzt_iter_listint_01() -> Result<()> {
    TZT::load("tzt_iter_listint_01.json")?.run()
}

#[test]
fn tzt_iter_mapintint_01() -> Result<()> {
    TZT::load("tzt_iter_mapintint_01.json")?.run()
}

#[test]
fn tzt_iter_listint_00() -> Result<()> {
    TZT::load("tzt_iter_listint_00.json")?.run()
}

#[test]
fn tzt_iter_setstring_00() -> Result<()> {
    TZT::load("tzt_iter_setstring_00.json")?.run()
}

#[test]
fn tzt_iter_setint_02() -> Result<()> {
    TZT::load("tzt_iter_setint_02.json")?.run()
}

#[test]
fn tzt_iter_mapintint_04() -> Result<()> {
    TZT::load("tzt_iter_mapintint_04.json")?.run()
}

#[test]
fn tzt_iter_mapintint_00() -> Result<()> {
    TZT::load("tzt_iter_mapintint_00.json")?.run()
}

#[test]
fn tzt_iter_listint_03() -> Result<()> {
    TZT::load("tzt_iter_listint_03.json")?.run()
}

#[test]
fn tzt_iter_mapintint_03() -> Result<()> {
    TZT::load("tzt_iter_mapintint_03.json")?.run()
}

#[test]
fn tzt_iter_mapintint_02() -> Result<()> {
    TZT::load("tzt_iter_mapintint_02.json")?.run()
}

#[test]
fn tzt_iter_setstring_02() -> Result<()> {
    TZT::load("tzt_iter_setstring_02.json")?.run()
}

#[test]
fn tzt_iter_liststring_00() -> Result<()> {
    TZT::load("tzt_iter_liststring_00.json")?.run()
}
