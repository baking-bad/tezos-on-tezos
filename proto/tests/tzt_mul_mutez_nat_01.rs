
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
              \"prim\": \"mutez\"
            },
            {
              \"int\": \"10\"
            }
          ]
        },
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
            \"int\": \"10\"
          },
          {
            \"int\": \"2000000000000000000000000000000000000000000000000000000000000000000000\"
          }
        ]
      }
    ]
  }
]";

#[test]
fn tzt_mul_mutez_nat_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
