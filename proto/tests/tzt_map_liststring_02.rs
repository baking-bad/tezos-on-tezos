
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
          \"prim\": \"MAP\",
          \"args\": [
            [
              {
                \"prim\": \"PUSH\",
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
                \"prim\": \"IF\",
                \"args\": [
                  [
                    {
                      \"prim\": \"FAILWITH\"
                    }
                  ],
                  []
                ]
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
            []
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
              \"prim\": \"list\",
              \"args\": [
                {
                  \"prim\": \"string\"
                }
              ]
            },
            []
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_map_liststring_02() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
