
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
          \"prim\": \"MAP\",
          \"args\": [
            [
              {
                \"prim\": \"DROP\"
              },
              {
                \"prim\": \"PUSH\",
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
                  \"prim\": \"string\"
                }
              ]
            },
            [
              {
                \"string\": \"\"
              },
              {
                \"string\": \"\"
              },
              {
                \"string\": \"\"
              },
              {
                \"string\": \"\"
              }
            ]
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_map_listint_03() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
