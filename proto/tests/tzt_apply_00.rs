
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
          \"prim\": \"APPLY\"
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
              \"string\": \"Hi\"
            }
          ]
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"lambda\",
              \"args\": [
                {
                  \"prim\": \"pair\",
                  \"args\": [
                    {
                      \"prim\": \"string\"
                    },
                    {
                      \"prim\": \"int\"
                    }
                  ]
                },
                {
                  \"prim\": \"nat\"
                }
              ]
            },
            [
              {
                \"prim\": \"DROP\"
              },
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
              \"prim\": \"lambda\",
              \"args\": [
                {
                  \"prim\": \"int\"
                },
                {
                  \"prim\": \"nat\"
                }
              ]
            },
            [
              {
                \"prim\": \"PUSH\",
                \"args\": [
                  {
                    \"prim\": \"string\"
                  },
                  {
                    \"string\": \"Hi\"
                  }
                ]
              },
              {
                \"prim\": \"PAIR\"
              },
              [
                {
                  \"prim\": \"DROP\"
                },
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
              ]
            ]
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_apply_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
