
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
          \"prim\": \"CONTRACT\",
          \"args\": [
            {
              \"prim\": \"unit\"
            }
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
              \"prim\": \"address\"
            },
            {
              \"string\": \"tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx\"
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
                  \"prim\": \"contract\",
                  \"args\": [
                    {
                      \"prim\": \"unit\"
                    }
                  ]
                }
              ]
            },
            {
              \"prim\": \"Some\",
              \"args\": [
                {
                  \"string\": \"tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx\"
                }
              ]
            }
          ]
        }
      ]
    ]
  },
  {
    \"prim\": \"other_contracts\",
    \"args\": [
      [
        {
          \"prim\": \"Contract\",
          \"args\": [
            {
              \"string\": \"tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx\"
            },
            {
              \"prim\": \"unit\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_contract_04() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
