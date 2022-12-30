
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
          \"prim\": \"FAILWITH\"
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
              \"prim\": \"int\"
            },
            {
              \"int\": \"0\"
            }
          ]
        }
      ]
    ]
  },
  {
    \"prim\": \"output\",
    \"args\": [
      {
        \"prim\": \"Failed\",
        \"args\": [
          {
            \"int\": \"0\"
          }
        ]
      }
    ]
  }
]";

#[test]
fn tzt_failwith_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
