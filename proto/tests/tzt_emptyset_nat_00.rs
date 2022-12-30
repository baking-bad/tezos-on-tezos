
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
          \"prim\": \"EMPTY_SET\",
          \"args\": [
            {
              \"prim\": \"nat\"
            }
          ]
        }
      ]
    ]
  },
  {
    \"prim\": \"input\",
    \"args\": [
      []
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
              \"prim\": \"set\",
              \"args\": [
                {
                  \"prim\": \"nat\"
                }
              ]
            },
            []
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_emptyset_nat_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
