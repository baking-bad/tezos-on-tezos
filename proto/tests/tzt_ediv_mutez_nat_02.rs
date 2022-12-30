
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
              \"prim\": \"mutez\"
            },
            {
              \"int\": \"2\"
            }
          ]
        },
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
                      \"prim\": \"mutez\"
                    },
                    {
                      \"prim\": \"mutez\"
                    }
                  ]
                }
              ]
            },
            {
              \"prim\": \"Some\",
              \"args\": [
                {
                  \"prim\": \"Pair\",
                  \"args\": [
                    {
                      \"int\": \"0\"
                    },
                    {
                      \"int\": \"2\"
                    }
                  ]
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
fn tzt_ediv_mutez_nat_02() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
