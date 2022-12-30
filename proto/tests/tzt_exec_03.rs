
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
                \"prim\": \"NIL\",
                \"args\": [
                  {
                    \"prim\": \"string\"
                  }
                ]
              },
              {
                \"prim\": \"SWAP\"
              },
              {
                \"prim\": \"CONS\"
              },
              {
                \"prim\": \"SWAP\"
              },
              {
                \"prim\": \"CONS\"
              },
              {
                \"prim\": \"CONCAT\"
              }
            ]
          ]
        },
        {
          \"prim\": \"SWAP\"
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
              \"string\": \"test\"
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
              \"string\": \"test_abc\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_exec_03() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
