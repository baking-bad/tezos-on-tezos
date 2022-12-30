
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
              \"prim\": \"string\"
            },
            {
              \"string\": \"2\"
            }
          ]
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"option\",
              \"args\": [
                {
                  \"prim\": \"string\"
                }
              ]
            },
            {
              \"prim\": \"Some\",
              \"args\": [
                {
                  \"string\": \"three\"
                }
              ]
            }
          ]
        },
        {
          \"prim\": \"Stack_elt\",
          \"args\": [
            {
              \"prim\": \"big_map\",
              \"args\": [
                {
                  \"prim\": \"string\"
                },
                {
                  \"prim\": \"string\"
                }
              ]
            },
            {
              \"int\": \"0\"
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
              \"prim\": \"big_map\",
              \"args\": [
                {
                  \"prim\": \"string\"
                },
                {
                  \"prim\": \"string\"
                }
              ]
            },
            [
              {
                \"prim\": \"Elt\",
                \"args\": [
                  {
                    \"string\": \"1\"
                  },
                  {
                    \"string\": \"one\"
                  }
                ]
              },
              {
                \"prim\": \"Elt\",
                \"args\": [
                  {
                    \"string\": \"2\"
                  },
                  {
                    \"string\": \"three\"
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
    \"prim\": \"big_maps\",
    \"args\": [
      [
        {
          \"prim\": \"Big_map\",
          \"args\": [
            {
              \"int\": \"0\"
            },
            {
              \"prim\": \"string\"
            },
            {
              \"prim\": \"string\"
            },
            [
              {
                \"prim\": \"Elt\",
                \"args\": [
                  {
                    \"string\": \"1\"
                  },
                  {
                    \"string\": \"one\"
                  }
                ]
              },
              {
                \"prim\": \"Elt\",
                \"args\": [
                  {
                    \"string\": \"2\"
                  },
                  {
                    \"string\": \"two\"
                  }
                ]
              }
            ]
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_update_bigmapstringstring_02() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
