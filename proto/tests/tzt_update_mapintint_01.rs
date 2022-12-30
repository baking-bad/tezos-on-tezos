
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
          \"prim\": \"UPDATE\"
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
              \"int\": \"1\"
            }
          ]
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"option\",
              \"args\": [
                {
                  \"prim\": \"int\"
                }
              ]
            },
            {
              \"prim\": \"Some\",
              \"args\": [
                {
                  \"int\": \"12\"
                }
              ]
            }
          ]
        },
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
                    \"int\": \"3\"
                  }
                ]
              },
              {
                \"prim\": \"Elt\",
                \"args\": [
                  {
                    \"int\": \"5\"
                  },
                  {
                    \"int\": \"2\"
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
                    \"int\": \"12\"
                  }
                ]
              },
              {
                \"prim\": \"Elt\",
                \"args\": [
                  {
                    \"int\": \"5\"
                  },
                  {
                    \"int\": \"2\"
                  }
                ]
              }
            ]
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_update_mapintint_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
