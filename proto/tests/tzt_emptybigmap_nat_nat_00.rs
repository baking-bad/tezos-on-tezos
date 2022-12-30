
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
          \"prim\": \"EMPTY_BIG_MAP\",
          \"args\": [
            {
              \"prim\": \"nat\"
            },
            {
              \"prim\": \"nat\"
            }
          ]
        }
      ]
    ]
  },
  {
    \"prim\": \"input\",
    \"args\": [
      []
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
                  \"prim\": \"nat\"
                },
                {
                  \"prim\": \"nat\"
                }
              ]
            },
            []
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_emptybigmap_nat_nat_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
