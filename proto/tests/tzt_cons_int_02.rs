
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
          \"prim\": \"CONS\"
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
              \"prim\": \"int\"
            },
            {
              \"int\": \"1\"
            }
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
            [
              {
                \"int\": \"1\"
              },
              {
                \"int\": \"2\"
              },
              {
                \"int\": \"3\"
              },
              {
                \"int\": \"4\"
              },
              {
                \"int\": \"5\"
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
              \"prim\": \"list\",
              \"args\": [
                {
                  \"prim\": \"int\"
                }
              ]
            },
            [
              {
                \"int\": \"1\"
              },
              {
                \"int\": \"1\"
              },
              {
                \"int\": \"2\"
              },
              {
                \"int\": \"3\"
              },
              {
                \"int\": \"4\"
              },
              {
                \"int\": \"5\"
              }
            ]
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_cons_int_02() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
