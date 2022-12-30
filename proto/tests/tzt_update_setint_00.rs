
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
          \"prim\": \"UPDATE\"
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
              \"int\": \"2\"
            }
          ]
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"bool\"
            },
            {
              \"prim\": \"False\"
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
                  \"prim\": \"int\"
                }
              ]
            },
            [
              {
                \"int\": \"1\"
              },
              {
                \"int\": \"3\"
              },
              {
                \"int\": \"5\"
              },
              {
                \"int\": \"7\"
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
              \"prim\": \"set\",
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
                \"int\": \"3\"
              },
              {
                \"int\": \"5\"
              },
              {
                \"int\": \"7\"
              }
            ]
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_update_setint_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
