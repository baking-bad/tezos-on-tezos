
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
              \"prim\": \"nat\"
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
fn tzt_contract_05() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
