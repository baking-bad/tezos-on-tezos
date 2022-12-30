
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
          \"prim\": \"ADDRESS\"
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
              \"prim\": \"contract\",
              \"args\": [
                {
                  \"prim\": \"unit\"
                }
              ]
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
              \"prim\": \"address\"
            },
            {
              \"string\": \"tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_address_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
