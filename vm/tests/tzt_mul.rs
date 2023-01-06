mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_mul_nat_int_00() -> Result<()> {
    TZT::load("tzt_mul_nat_int_00.json")?.run()
}

#[test]
fn tzt_mul_int_nat_00() -> Result<()> {
    TZT::load("tzt_mul_int_nat_00.json")?.run()
}

#[test]
fn tzt_mul_mutez_nat_00() -> Result<()> {
    TZT::load("tzt_mul_mutez_nat_00.json")?.run()
}

#[test]
fn tzt_mul_int_int_00() -> Result<()> {
    TZT::load("tzt_mul_int_int_00.json")?.run()
}

#[test]
fn tzt_mul_nat_mutez_00() -> Result<()> {
    TZT::load("tzt_mul_nat_mutez_00.json")?.run()
}

#[test]
fn tzt_mul_nat_mutez_01() -> Result<()> {
    TZT::load("tzt_mul_nat_mutez_01.json")?.run()
}

#[test]
fn tzt_mul_nat_nat_00() -> Result<()> {
    TZT::load("tzt_mul_nat_nat_00.json")?.run()
}

#[test]
fn tzt_mul_mutez_nat_01() -> Result<()> {
    TZT::load("tzt_mul_mutez_nat_01.json")?.run()
}
