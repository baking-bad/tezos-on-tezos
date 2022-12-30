
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
          \"prim\": \"CONCAT\"
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
              \"prim\": \"bytes\"
            },
            {
              \"bytes\": \"FF\"
            }
          ]
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"bytes\"
            },
            {
              \"bytes\": \"aa\"
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
              \"prim\": \"bytes\"
            },
            {
              \"bytes\": \"ffaa\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_concat_bytes_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
