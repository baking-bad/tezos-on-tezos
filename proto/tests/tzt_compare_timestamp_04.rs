
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
          \"prim\": \"COMPARE\"
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
              \"string\": \"2019-09-16T08:38:05Z\"
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
              \"string\": \"2017-09-16T08:38:05Z\"
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
              \"int\": \"1\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_compare_timestamp_04() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
