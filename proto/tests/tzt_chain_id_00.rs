
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
              \"bytes\": \"00000000\"
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
        \"bytes\": \"00000000\"
      }
    ]
  }
]";

#[test]
fn tzt_chain_id_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
