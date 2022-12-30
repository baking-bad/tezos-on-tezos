
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
                  \"prim\": \"nat\"
                }
              ]
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
fn tzt_address_02() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
