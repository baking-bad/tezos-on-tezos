
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
          \"prim\": \"IF\",
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
              \"prim\": \"False\"
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
              \"int\": \"1\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_if_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
