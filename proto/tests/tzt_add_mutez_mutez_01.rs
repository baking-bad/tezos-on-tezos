
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
          \"prim\": \"ADD\"
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
              \"prim\": \"mutez\"
            },
            {
              \"int\": \"9223372036854775807\"
            }
          ]
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"mutez\"
            },
            {
              \"int\": \"1\"
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
        \"prim\": \"MutezOverflow\",
        \"args\": [
          {
            \"int\": \"9223372036854775807\"
          },
          {
            \"int\": \"1\"
          }
        ]
      }
    ]
  }
]";

#[test]
fn tzt_add_mutez_mutez_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
