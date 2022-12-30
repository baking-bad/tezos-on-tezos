
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
          \"prim\": \"AMOUNT\"
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
              \"prim\": \"mutez\"
            },
            {
              \"int\": \"10\"
            }
          ]
        }
      ]
    ]
  },
  {
    \"prim\": \"amount\",
    \"args\": [
      {
        \"int\": \"10\"
      }
    ]
  }
]";

#[test]
fn tzt_amount_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
