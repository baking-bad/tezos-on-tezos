
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
          \"prim\": \"SUB\"
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
              \"prim\": \"timestamp\"
            },
            {
              \"string\": \"1970-01-01T00:03:20Z\"
            }
          ]
        },
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
              \"int\": \"200\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_sub_timestamp_timestamp_03() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
