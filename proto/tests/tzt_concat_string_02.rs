
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
          \"prim\": \"ITER\",
          \"args\": [
            [
              {
                \"prim\": \"SWAP\"
              },
              {
                \"prim\": \"DIP\",
                \"args\": [
                  [
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
                    }
                  ]
                ]
              },
              {
                \"prim\": \"CONS\"
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
    \"prim\": \"input\",
    \"args\": [
      [
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"list\",
              \"args\": [
                {
                  \"prim\": \"string\"
                }
              ]
            },
            [
              {
                \"string\": \"Hello\"
              },
              {
                \"string\": \" \"
              },
              {
                \"string\": \"World\"
              },
              {
                \"string\": \"!\"
              }
            ]
          ]
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"string\"
            },
            {
              \"string\": \"\"
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
              \"string\": \"Hello World!\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_concat_string_02() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
