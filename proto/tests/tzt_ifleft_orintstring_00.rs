
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
          \"prim\": \"IF_LEFT\",
          \"args\": [
            [
              {
                \"prim\": \"PUSH\",
                \"args\": [
                  {
                    \"prim\": \"int\"
                  },
                  {
                    \"int\": \"1\"
                  }
                ]
              },
              {
                \"prim\": \"ADD\"
              }
            ],
            [
              {
                \"prim\": \"UNIT\"
              },
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
              \"prim\": \"Left\",
              \"args\": [
                {
                  \"int\": \"1\"
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
              \"prim\": \"int\"
            },
            {
              \"int\": \"2\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_ifleft_orintstring_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
