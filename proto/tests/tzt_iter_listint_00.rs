
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
                \"prim\": \"ADD\"
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
              \"prim\": \"list\",
              \"args\": [
                {
                  \"prim\": \"int\"
                }
              ]
            },
            [
              {
                \"int\": \"7\"
              },
              {
                \"int\": \"11\"
              },
              {
                \"int\": \"45\"
              }
            ]
          ]
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"int\"
            },
            {
              \"int\": \"1\"
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
              \"prim\": \"int\"
            },
            {
              \"int\": \"64\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_iter_listint_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
