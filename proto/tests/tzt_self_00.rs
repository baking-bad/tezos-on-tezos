
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
          \"prim\": \"SELF\"
        }
      ]
    ]
  },
  {
    \"prim\": \"input\",
    \"args\": [
      []
    ]
  },
  {
    \"prim\": \"parameter\",
    \"args\": [
      {
        \"prim\": \"int\"
      }
    ]
  },
  {
    \"prim\": \"self\",
    \"args\": [
      {
        \"string\": \"KT1BEqzn5Wx8uJrZNvuS9DVHmLvG9td3fDLi\"
      }
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
              \"prim\": \"contract\",
              \"args\": [
                {
                  \"prim\": \"int\"
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
  }
]";

#[test]
fn tzt_self_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
