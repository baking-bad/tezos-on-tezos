
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
          \"prim\": \"BALANCE\"
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
              \"int\": \"22\"
            }
          ]
        }
      ]
    ]
  },
  {
    \"prim\": \"balance\",
    \"args\": [
      {
        \"int\": \"22\"
      }
    ]
  }
]";

#[test]
fn tzt_balance_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
