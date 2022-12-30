
mod runner;

use proto::Result;
use runner::tzt::TZT;

const CASE: &str = "
[
  {
    \"prim\": \"code\",
    \"args\": [
      []
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
              \"prim\": \"timestamp\"
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
              \"prim\": \"timestamp\"
            },
            {
              \"string\": \"1970-01-01T00:00:00Z\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_add_timestamp_int_03() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
