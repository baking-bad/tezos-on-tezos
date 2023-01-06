mod runner;

use vm::Result;
use runner::tzt::TZT;

#[test]
fn tzt_packunpack_nat_00() -> Result<()> {
    TZT::load("tzt_packunpack_nat_00.json")?.run()
}

#[test]
fn tzt_packunpack_int_00() -> Result<()> {
    TZT::load("tzt_packunpack_int_00.json")?.run()
}

#[test]
fn tzt_packunpack_timestamp_00() -> Result<()> {
    TZT::load("tzt_packunpack_timestamp_00.json")?.run()
}

#[test]
fn tzt_packunpack_keyhash_00() -> Result<()> {
    TZT::load("tzt_packunpack_keyhash_00.json")?.run()
}

#[test]
fn tzt_packunpack_address_00() -> Result<()> {
    TZT::load("tzt_packunpack_address_00.json")?.run()
}

#[test]
fn tzt_packunpack_string_00() -> Result<()> {
    TZT::load("tzt_packunpack_string_00.json")?.run()
}

#[test]
fn tzt_packunpack_bool_00() -> Result<()> {
    TZT::load("tzt_packunpack_bool_00.json")?.run()
}

#[test]
fn tzt_packunpack_bytes_00() -> Result<()> {
    TZT::load("tzt_packunpack_bytes_00.json")?.run()
}

#[test]
fn tzt_packunpack_mutez_00() -> Result<()> {
    TZT::load("tzt_packunpack_mutez_00.json")?.run()
}
