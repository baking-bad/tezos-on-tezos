
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
              },
              {
                \"string\": \"Password\"
              },
              {
                \"string\": \"Is\"
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
              },
              {
                \"string\": \"P\"
              },
              {
                \"string\": \"I\"
              }
            ]
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_map_liststring_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
