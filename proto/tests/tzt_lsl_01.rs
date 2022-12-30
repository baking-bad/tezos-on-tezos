
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
          \"prim\": \"LSL\"
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
              \"int\": \"1\"
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
              \"int\": \"257\"
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
        \"prim\": \"GeneralOverflow\",
        \"args\": [
          {
            \"int\": \"1\"
          },
          {
            \"int\": \"257\"
          }
        ]
      }
    ]
  }
]";

#[test]
fn tzt_lsl_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
