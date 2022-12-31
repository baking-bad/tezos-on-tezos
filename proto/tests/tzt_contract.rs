mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_contract_05() -> Result<()> {
    TZT::load("tzt_contract_05.json")?.run()
}

#[test]
fn tzt_contract_02() -> Result<()> {
    TZT::load("tzt_contract_02.json")?.run()
}

#[test]
fn tzt_contract_03() -> Result<()> {
    TZT::load("tzt_contract_03.json")?.run()
}

#[test]
fn tzt_contract_00() -> Result<()> {
    TZT::load("tzt_contract_00.json")?.run()
}

#[test]
fn tzt_contract_01() -> Result<()> {
    TZT::load("tzt_contract_01.json")?.run()
}

#[test]
fn tzt_contract_04() -> Result<()> {
    TZT::load("tzt_contract_04.json")?.run()
}
