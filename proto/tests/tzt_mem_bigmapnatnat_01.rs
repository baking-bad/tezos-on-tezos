
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
              \"prim\": \"nat\"
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
              \"prim\": \"big_map\",
              \"args\": [
                {
                  \"prim\": \"nat\"
                },
                {
                  \"prim\": \"nat\"
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
              \"prim\": \"bool\"
            },
            {
              \"prim\": \"False\"
            }
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
              \"prim\": \"nat\"
            },
            {
              \"prim\": \"nat\"
            },
            []
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_mem_bigmapnatnat_01() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
