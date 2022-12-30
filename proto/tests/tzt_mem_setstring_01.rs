
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
          \"prim\": \"MEM\"
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
              \"prim\": \"string\"
            },
            {
              \"string\": \"Hi\"
            }
          ]
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"set\",
              \"args\": [
                {
                  \"prim\": \"string\"
                }
              ]
            },
            [
              {
                \"string\": \"Hi\"
              }
            ]
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
              \"prim\": \"bool\"
            },
            {
              \"prim\": \"True\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_mem_setstring_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
