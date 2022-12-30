
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
          \"prim\": \"IF_NONE\",
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
              \"prim\": \"option\",
              \"args\": [
                {
                  \"prim\": \"nat\"
                }
              ]
            },
            {
              \"prim\": \"None\"
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
              \"prim\": \"nat\"
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
fn tzt_ifnone_optionnat_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
