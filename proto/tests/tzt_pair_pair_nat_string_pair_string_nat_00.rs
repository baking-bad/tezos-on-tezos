
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
          \"prim\": \"PAIR\"
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
              \"prim\": \"pair\",
              \"args\": [
                {
                  \"prim\": \"nat\"
                },
                {
                  \"prim\": \"string\"
                }
              ]
            },
            {
              \"prim\": \"Pair\",
              \"args\": [
                {
                  \"int\": \"5\"
                },
                {
                  \"string\": \"Hello\"
                }
              ]
            }
          ]
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"pair\",
              \"args\": [
                {
                  \"prim\": \"string\"
                },
                {
                  \"prim\": \"nat\"
                }
              ]
            },
            {
              \"prim\": \"Pair\",
              \"args\": [
                {
                  \"string\": \"World\"
                },
                {
                  \"int\": \"6\"
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
                  \"prim\": \"pair\",
                  \"args\": [
                    {
                      \"prim\": \"nat\"
                    },
                    {
                      \"prim\": \"string\"
                    }
                  ]
                },
                {
                  \"prim\": \"pair\",
                  \"args\": [
                    {
                      \"prim\": \"string\"
                    },
                    {
                      \"prim\": \"nat\"
                    }
                  ]
                }
              ]
            },
            {
              \"prim\": \"Pair\",
              \"args\": [
                {
                  \"prim\": \"Pair\",
                  \"args\": [
                    {
                      \"int\": \"5\"
                    },
                    {
                      \"string\": \"Hello\"
                    }
                  ]
                },
                {
                  \"prim\": \"Pair\",
                  \"args\": [
                    {
                      \"string\": \"World\"
                    },
                    {
                      \"int\": \"6\"
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
fn tzt_pair_pair_nat_string_pair_string_nat_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
