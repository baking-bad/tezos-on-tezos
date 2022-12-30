
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
          \"prim\": \"SOURCE\"
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
              \"prim\": \"address\"
            },
            {
              \"string\": \"tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx\"
            }
          ]
        }
      ]
    ]
  },
  {
    \"prim\": \"source\",
    \"args\": [
      {
        \"string\": \"tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx\"
      }
    ]
  }
]";

#[test]
fn tzt_source_00() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
