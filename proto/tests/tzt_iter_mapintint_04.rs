
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
          \"prim\": \"ITER\",
          \"args\": [
            [
              {
                \"prim\": \"DIP\",
                \"args\": [
                  [
                    {
                      \"prim\": \"UNPAIR\"
                    }
                  ]
                ]
              },
              {
                \"prim\": \"UNPAIR\"
              },
              {
                \"prim\": \"DIG\",
                \"args\": [
                  {
                    \"int\": \"2\"
                  }
                ]
              },
              {
                \"prim\": \"ADD\"
              },
              {
                \"prim\": \"DIP\",
                \"args\": [
                  [
                    {
                      \"prim\": \"ADD\"
                    }
                  ]
                ]
              },
              {
                \"prim\": \"PAIR\"
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
              \"prim\": \"map\",
              \"args\": [
                {
                  \"prim\": \"int\"
                },
                {
                  \"prim\": \"int\"
                }
              ]
            },
            [
              {
                \"prim\": \"Elt\",
                \"args\": [
                  {
                    \"int\": \"1\"
                  },
                  {
                    \"int\": \"1\"
                  }
                ]
              },
              {
                \"prim\": \"Elt\",
                \"args\": [
                  {
                    \"int\": \"2\"
                  },
                  {
                    \"int\": \"100\"
                  }
                ]
              }
            ]
          ]
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"pair\",
              \"args\": [
                {
                  \"prim\": \"int\"
                },
                {
                  \"prim\": \"int\"
                }
              ]
            },
            {
              \"prim\": \"Pair\",
              \"args\": [
                {
                  \"int\": \"0\"
                },
                {
                  \"int\": \"0\"
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
              \"prim\": \"pair\",
              \"args\": [
                {
                  \"prim\": \"int\"
                },
                {
                  \"prim\": \"int\"
                }
              ]
            },
            {
              \"prim\": \"Pair\",
              \"args\": [
                {
                  \"int\": \"3\"
                },
                {
                  \"int\": \"101\"
                }
              ]
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_iter_mapintint_04() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
