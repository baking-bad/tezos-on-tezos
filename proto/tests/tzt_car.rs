mod runner;

use proto::Result;
use runner::tzt::TZT;

#[test]
fn tzt_car_01() -> Result<()> {
    TZT::load("tzt_car_01.json")?.run()
}

#[test]
fn tzt_car_00() -> Result<()> {
    TZT::load("tzt_car_00.json")?.run()
}
