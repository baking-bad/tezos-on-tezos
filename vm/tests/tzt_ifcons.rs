mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_ifcons_listint_01() -> Result<()> {
    TZT::load("tzt_ifcons_listint_01.json")?.run()
}

#[test]
fn tzt_ifcons_listnat_00() -> Result<()> {
    TZT::load("tzt_ifcons_listnat_00.json")?.run()
}

#[test]
fn tzt_ifcons_listnat_01() -> Result<()> {
    TZT::load("tzt_ifcons_listnat_01.json")?.run()
}

#[test]
fn tzt_ifcons_listint_00() -> Result<()> {
    TZT::load("tzt_ifcons_listint_00.json")?.run()
}
