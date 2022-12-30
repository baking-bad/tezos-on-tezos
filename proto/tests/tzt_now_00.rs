
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
          \"prim\": \"NOW\"
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
              \"prim\": \"timestamp\"
            },
            {
              \"int\": \"45\"
            }
          ]
        }
      ]
    ]
  },
  {
    \"prim\": \"now\",
    \"args\": [
      {
        \"int\": \"45\"
      }
    ]
  }
]";

#[test]
fn tzt_now_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
