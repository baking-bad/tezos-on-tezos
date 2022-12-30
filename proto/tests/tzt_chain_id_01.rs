
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
          \"prim\": \"CHAIN_ID\"
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
              \"prim\": \"chain_id\"
            },
            {
              \"bytes\": \"7a06a770\"
            }
          ]
        }
      ]
    ]
  },
  {
    \"prim\": \"chain_id\",
    \"args\": [
      {
        \"bytes\": \"7a06a770\"
      }
    ]
  }
]";

#[test]
fn tzt_chain_id_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
