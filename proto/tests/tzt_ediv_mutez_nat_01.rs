
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
              \"int\": \"102\"
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
                      \"int\": \"34\"
                    },
                    {
                      \"int\": \"0\"
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
fn tzt_ediv_mutez_nat_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
