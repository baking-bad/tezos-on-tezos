
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
          \"prim\": \"SUB\"
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
              \"int\": \"5\"
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
              \"int\": \"13\"
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
        \"prim\": \"MutezUnderflow\",
        \"args\": [
          {
            \"int\": \"5\"
          },
          {
            \"int\": \"13\"
          }
        ]
      }
    ]
  }
]";

#[test]
fn tzt_sub_mutez_mutez_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
