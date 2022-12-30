
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
          \"prim\": \"LEFT\",
          \"args\": [
            {
              \"prim\": \"nat\"
            }
          ]
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
              \"prim\": \"int\"
            },
            {
              \"int\": \"5\"
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
              \"prim\": \"or\",
              \"args\": [
                {
                  \"prim\": \"int\"
                },
                {
                  \"prim\": \"nat\"
                }
              ]
            },
            {
              \"prim\": \"Left\",
              \"args\": [
                {
                  \"int\": \"5\"
                }
              ]
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_left_int_nat_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
