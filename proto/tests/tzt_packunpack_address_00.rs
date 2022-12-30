
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
          \"prim\": \"PACK\"
        },
        {
          \"prim\": \"UNPACK\",
          \"args\": [
            {
              \"prim\": \"address\"
            }
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
              \"prim\": \"address\"
            },
            {
              \"string\": \"tz1cxcwwnzENRdhe2Kb8ZdTrdNy4bFNyScx5\"
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
              \"prim\": \"option\",
              \"args\": [
                {
                  \"prim\": \"address\"
                }
              ]
            },
            {
              \"prim\": \"Some\",
              \"args\": [
                {
                  \"string\": \"tz1cxcwwnzENRdhe2Kb8ZdTrdNy4bFNyScx5\"
                }
              ]
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_packunpack_address_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
