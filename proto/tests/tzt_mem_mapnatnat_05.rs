
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
          \"prim\": \"MEM\"
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
              \"prim\": \"nat\"
            },
            {
              \"int\": \"3\"
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
                  \"prim\": \"nat\"
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
                    \"int\": \"1\"
                  },
                  {
                    \"int\": \"4\"
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
                    \"int\": \"11\"
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
]";

#[test]
fn tzt_mem_mapnatnat_05() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
