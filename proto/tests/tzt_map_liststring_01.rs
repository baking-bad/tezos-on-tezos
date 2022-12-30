
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
                    \"prim\": \"nat\"
                  },
                  {
                    \"int\": \"1\"
                  }
                ]
              },
              {
                \"prim\": \"PUSH\",
                \"args\": [
                  {
                    \"prim\": \"nat\"
                  },
                  {
                    \"int\": \"0\"
                  }
                ]
              },
              {
                \"prim\": \"SLICE\"
              },
              {
                \"prim\": \"IF_NONE\",
                \"args\": [
                  [
                    {
                      \"prim\": \"UNIT\"
                    },
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
            [
              {
                \"string\": \"The\"
              }
            ]
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
            [
              {
                \"string\": \"T\"
              }
            ]
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_map_liststring_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
