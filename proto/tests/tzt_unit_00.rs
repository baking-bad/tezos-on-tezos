
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
          \"prim\": \"UNIT\"
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
              \"prim\": \"unit\"
            },
            {
              \"prim\": \"Unit\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_unit_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
