
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
          \"prim\": \"RIGHT\",
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
              \"int\": \"-5\"
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
                  \"prim\": \"nat\"
                },
                {
                  \"prim\": \"int\"
                }
              ]
            },
            {
              \"prim\": \"Right\",
              \"args\": [
                {
                  \"int\": \"-5\"
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
fn tzt_right_nat_int_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
