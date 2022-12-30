
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
                    \"prim\": \"int\"
                  },
                  {
                    \"prim\": \"int\"
                  },
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
                    },
                    {
                      \"prim\": \"ADD\"
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
              \"prim\": \"int\"
            },
            {
              \"int\": \"5\"
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
              \"int\": \"10\"
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
              \"int\": \"6\"
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
              \"int\": \"10\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_exec_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
