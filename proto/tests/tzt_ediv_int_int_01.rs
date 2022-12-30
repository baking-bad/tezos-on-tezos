
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
          \"prim\": \"EDIV\"
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
              \"prim\": \"option\",
              \"args\": [
                {
                  \"prim\": \"pair\",
                  \"args\": [
                    {
                      \"prim\": \"int\"
                    },
                    {
                      \"prim\": \"nat\"
                    }
                  ]
                }
              ]
            },
            {
              \"prim\": \"None\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_ediv_int_int_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
