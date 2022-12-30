
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
          \"prim\": \"SIZE\"
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
                  \"prim\": \"string\"
                },
                {
                  \"prim\": \"nat\"
                }
              ]
            },
            [
              {
                \"prim\": \"Elt\",
                \"args\": [
                  {
                    \"string\": \"a\"
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
                    \"string\": \"b\"
                  },
                  {
                    \"int\": \"2\"
                  }
                ]
              },
              {
                \"prim\": \"Elt\",
                \"args\": [
                  {
                    \"string\": \"c\"
                  },
                  {
                    \"int\": \"3\"
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
              \"prim\": \"nat\"
            },
            {
              \"int\": \"3\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_size_mapstringnat_02() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
