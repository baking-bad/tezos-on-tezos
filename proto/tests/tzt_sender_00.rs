
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
          \"prim\": \"SENDER\"
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
    \"prim\": \"sender\",
    \"args\": [
      {
        \"string\": \"tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx\"
      }
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
              \"prim\": \"address\"
            },
            {
              \"string\": \"tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx\"
            }
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_sender_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
