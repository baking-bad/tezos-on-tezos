
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
          \"prim\": \"IMPLICIT_ACCOUNT\"
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
              \"prim\": \"key_hash\"
            },
            {
              \"string\": \"tz1NwQ6hkenkn6aYYio8VnJvjtb4K1pfeU1Z\"
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
              \"prim\": \"contract\",
              \"args\": [
                {
                  \"prim\": \"unit\"
                }
              ]
            },
            {
              \"string\": \"tz1NwQ6hkenkn6aYYio8VnJvjtb4K1pfeU1Z\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_implicitaccount_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
