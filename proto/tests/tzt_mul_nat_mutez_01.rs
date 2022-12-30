
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
          \"prim\": \"MUL\"
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
              \"prim\": \"nat\"
            },
            {
              \"int\": \"2000000000000000000000000000000000000000000000000000000000000000000000\"
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
              \"int\": \"10\"
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
            \"int\": \"2000000000000000000000000000000000000000000000000000000000000000000000\"
          },
          {
            \"int\": \"10\"
          }
        ]
      }
    ]
  }
]";

#[test]
fn tzt_mul_nat_mutez_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
