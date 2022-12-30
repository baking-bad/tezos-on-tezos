
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
              \"prim\": \"list\",
              \"args\": [
                {
                  \"prim\": \"bytes\"
                }
              ]
            },
            [
              {
                \"bytes\": \"00ab\"
              },
              {
                \"bytes\": \"cd\"
              },
              {
                \"bytes\": \"ef\"
              },
              {
                \"bytes\": \"00\"
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
              \"prim\": \"bytes\"
            },
            {
              \"bytes\": \"00abcdef00\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_concat_listbytes_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
