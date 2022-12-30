
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
              \"prim\": \"mutez\"
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
              \"prim\": \"mutez\"
            },
            {
              \"int\": \"1000\"
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
                  \"prim\": \"mutez\"
                }
              ]
            },
            {
              \"prim\": \"Some\",
              \"args\": [
                {
                  \"int\": \"1000\"
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
fn tzt_packunpack_mutez_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
