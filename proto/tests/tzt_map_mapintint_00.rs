
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
                \"prim\": \"CDR\"
              },
              {
                \"prim\": \"DUP\"
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
                    \"int\": \"6\"
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
                    \"int\": \"4\"
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
fn tzt_map_mapintint_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
