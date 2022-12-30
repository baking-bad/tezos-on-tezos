
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
                \"prim\": \"CDR\"
              },
              {
                \"prim\": \"CONS\"
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
              \"prim\": \"map\",
              \"args\": [
                {
                  \"prim\": \"int\"
                },
                {
                  \"prim\": \"int\"
                }
              ]
            },
            [
              {
                \"prim\": \"Elt\",
                \"args\": [
                  {
                    \"int\": \"5\"
                  },
                  {
                    \"int\": \"2\"
                  }
                ]
              }
            ]
          ]
        },
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
            []
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
              \"prim\": \"list\",
              \"args\": [
                {
                  \"prim\": \"int\"
                }
              ]
            },
            [
              {
                \"int\": \"2\"
              }
            ]
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_iter_mapintint_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
