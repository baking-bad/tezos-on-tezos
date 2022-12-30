
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
          \"prim\": \"IF_CONS\",
          \"args\": [
            [
              {
                \"prim\": \"DIP\",
                \"args\": [
                  [
                    {
                      \"prim\": \"DROP\"
                    }
                  ]
                ]
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
              \"prim\": \"list\",
              \"args\": [
                {
                  \"prim\": \"int\"
                }
              ]
            },
            [
              {
                \"int\": \"1\"
              },
              {
                \"int\": \"2\"
              },
              {
                \"int\": \"3\"
              },
              {
                \"int\": \"4\"
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
              \"prim\": \"int\"
            },
            {
              \"int\": \"1\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_ifcons_listnat_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
