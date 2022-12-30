
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
              \"string\": \"KT1BEqzn5Wx8uJrZNvuS9DVHmLvG9td3fDLi\"
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
              \"prim\": \"None\"
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
              \"string\": \"KT1BEqzn5Wx8uJrZNvuS9DVHmLvG9td3fDLi\"
            },
            {
              \"prim\": \"nat\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_contract_02() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
