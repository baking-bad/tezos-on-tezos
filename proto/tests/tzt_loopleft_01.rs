
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
          \"prim\": \"LOOP_LEFT\",
          \"args\": [
            [
              {
                \"prim\": \"FAILWITH\"
              }
            ]
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
              \"prim\": \"or\",
              \"args\": [
                {
                  \"prim\": \"int\"
                },
                {
                  \"prim\": \"string\"
                }
              ]
            },
            {
              \"prim\": \"Right\",
              \"args\": [
                {
                  \"string\": \"AAAA\"
                }
              ]
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
              \"prim\": \"string\"
            },
            {
              \"string\": \"AAAA\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_loopleft_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
