
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
                \"prim\": \"UNIT\"
              },
              {
                \"prim\": \"FAILWITH\"
              }
            ],
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
                  \"prim\": \"string\"
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
                  \"int\": \"2\"
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
              \"int\": \"3\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_ifleft_orstringint_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
