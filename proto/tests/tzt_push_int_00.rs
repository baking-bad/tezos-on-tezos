
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
          \"prim\": \"PUSH\",
          \"args\": [
            {
              \"prim\": \"int\"
            },
            {
              \"int\": \"6\"
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
              \"prim\": \"int\"
            },
            {
              \"int\": \"6\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_push_int_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
