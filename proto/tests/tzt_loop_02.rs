
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
          \"prim\": \"LOOP\",
          \"args\": [
            [
              {
                \"prim\": \"IF_CONS\",
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
                            \"prim\": \"CONS\"
                          }
                        ]
                      ]
                    },
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
                    }
                  ],
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
                      \"prim\": \"PUSH\",
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
              \"prim\": \"True\"
            }
          ]
        },
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
        },
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
        },
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
fn tzt_loop_02() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
