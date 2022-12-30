
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
          \"prim\": \"LOOP\",
          \"args\": [
            [
              {
                \"prim\": \"PUSH\",
                \"args\": [
                  {
                    \"prim\": \"int\"
                  },
                  {
                    \"int\": \"2\"
                  }
                ]
              },
              {
                \"prim\": \"ADD\"
              },
              {
                \"prim\": \"DUP\"
              },
              {
                \"prim\": \"DIP\",
                \"args\": [
                  [
                    {
                      \"prim\": \"PUSH\",
                      \"args\": [
                        {
                          \"prim\": \"int\"
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
                \"prim\": \"COMPARE\"
              },
              {
                \"prim\": \"LE\"
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
              \"prim\": \"bool\"
            },
            {
              \"prim\": \"True\"
            }
          ]
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"int\"
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
      [
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"int\"
            },
            {
              \"int\": \"11\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_loop_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
