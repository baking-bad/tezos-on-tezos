
mod runner;

use proto::Result;
use runner::tzt::TZT;

const CASE: &str = "
[
  {
    \"prim\": \"code\",
    \"args\": [
      [
        {
          \"prim\": \"CAR\"
        }
      ]
    ]
  },
  {
    \"prim\": \"input\",
    \"args\": [
      [
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"pair\",
              \"args\": [
                {
                  \"prim\": \"int\"
                },
                {
                  \"prim\": \"int\"
                }
              ]
            },
            {
              \"prim\": \"Pair\",
              \"args\": [
                {
                  \"int\": \"0\"
                },
                {
                  \"int\": \"1\"
                }
              ]
            }
          ]
        }
      ]
    ]
  },
  {
    \"prim\": \"output\",
    \"args\": [
      [
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"int\"
            },
            {
              \"int\": \"0\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_car_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
