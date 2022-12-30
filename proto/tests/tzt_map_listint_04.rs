
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
                \"prim\": \"DIP\",
                \"args\": [
                  [
                    {
                      \"prim\": \"DUP\"
                    }
                  ]
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
                \"int\": \"1\"
              },
              {
                \"int\": \"1\"
              },
              {
                \"int\": \"1\"
              }
            ]
          ]
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"int\"
            },
            {
              \"int\": \"0\"
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
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"int\"
            },
            {
              \"int\": \"4\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_map_listint_04() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
