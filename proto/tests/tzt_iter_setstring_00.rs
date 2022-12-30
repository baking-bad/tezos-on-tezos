
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
          \"prim\": \"ITER\",
          \"args\": [
            [
              {
                \"prim\": \"SWAP\"
              },
              {
                \"prim\": \"CONCAT\"
              }
            ]
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
              \"prim\": \"set\",
              \"args\": [
                {
                  \"prim\": \"string\"
                }
              ]
            },
            [
              {
                \"string\": \"a\"
              },
              {
                \"string\": \"b\"
              },
              {
                \"string\": \"c\"
              },
              {
                \"string\": \"d\"
              }
            ]
          ]
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"string\"
            },
            {
              \"string\": \"\"
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
              \"prim\": \"string\"
            },
            {
              \"string\": \"abcd\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_iter_setstring_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
