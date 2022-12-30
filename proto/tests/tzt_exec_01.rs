
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
          \"prim\": \"DIP\",
          \"args\": [
            [
              {
                \"prim\": \"LAMBDA\",
                \"args\": [
                  {
                    \"prim\": \"string\"
                  },
                  {
                    \"prim\": \"string\"
                  },
                  [
                    {
                      \"prim\": \"PUSH\",
                      \"args\": [
                        {
                          \"prim\": \"string\"
                        },
                        {
                          \"string\": \"_abc\"
                        }
                      ]
                    },
                    {
                      \"prim\": \"SWAP\"
                    },
                    {
                      \"prim\": \"CONCAT\"
                    }
                  ]
                ]
              }
            ]
          ]
        },
        {
          \"prim\": \"EXEC\"
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
              \"prim\": \"string\"
            },
            {
              \"string\": \"def\"
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
              \"string\": \"def_abc\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_exec_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
