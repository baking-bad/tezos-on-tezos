
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
          \"prim\": \"LOOP_LEFT\",
          \"args\": [
            [
              {
                \"prim\": \"PUSH\",
                \"args\": [
                  {
                    \"prim\": \"int\"
                  },
                  {
                    \"int\": \"2\"
                  }
                ]
              },
              {
                \"prim\": \"ADD\"
              },
              {
                \"prim\": \"DUP\"
              },
              {
                \"prim\": \"DIP\",
                \"args\": [
                  [
                    {
                      \"prim\": \"PUSH\",
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
                \"prim\": \"COMPARE\"
              },
              {
                \"prim\": \"GE\"
              },
              {
                \"prim\": \"IF\",
                \"args\": [
                  [
                    {
                      \"prim\": \"DROP\"
                    },
                    {
                      \"prim\": \"PUSH\",
                      \"args\": [
                        {
                          \"prim\": \"string\"
                        },
                        {
                          \"string\": \"hello\"
                        }
                      ]
                    },
                    {
                      \"prim\": \"RIGHT\",
                      \"args\": [
                        {
                          \"prim\": \"int\"
                        }
                      ]
                    }
                  ],
                  [
                    {
                      \"prim\": \"LEFT\",
                      \"args\": [
                        {
                          \"prim\": \"string\"
                        }
                      ]
                    }
                  ]
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
              \"prim\": \"or\",
              \"args\": [
                {
                  \"prim\": \"int\"
                },
                {
                  \"prim\": \"string\"
                }
              ]
            },
            {
              \"prim\": \"Left\",
              \"args\": [
                {
                  \"int\": \"1\"
                }
              ]
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
              \"string\": \"hello\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_loopleft_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
