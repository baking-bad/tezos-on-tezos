
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
          \"prim\": \"LOOP_LEFT\",
          \"args\": [
            [
              {
                \"prim\": \"DUP\"
              },
              {
                \"prim\": \"CAR\"
              },
              {
                \"prim\": \"DIP\",
                \"args\": [
                  [
                    {
                      \"prim\": \"CDR\"
                    }
                  ]
                ]
              },
              {
                \"prim\": \"IF_CONS\",
                \"args\": [
                  [
                    {
                      \"prim\": \"SWAP\"
                    },
                    {
                      \"prim\": \"DIP\",
                      \"args\": [
                        [
                          {
                            \"prim\": \"CONS\"
                          }
                        ]
                      ]
                    },
                    {
                      \"prim\": \"PAIR\"
                    },
                    {
                      \"prim\": \"LEFT\",
                      \"args\": [
                        {
                          \"prim\": \"list\",
                          \"args\": [
                            {
                              \"prim\": \"string\"
                            }
                          ]
                        }
                      ]
                    }
                  ],
                  [
                    {
                      \"prim\": \"RIGHT\",
                      \"args\": [
                        {
                          \"prim\": \"pair\",
                          \"args\": [
                            {
                              \"prim\": \"list\",
                              \"args\": [
                                {
                                  \"prim\": \"string\"
                                }
                              ]
                            },
                            {
                              \"prim\": \"list\",
                              \"args\": [
                                {
                                  \"prim\": \"string\"
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                ]
              }
            ]
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
              \"prim\": \"or\",
              \"args\": [
                {
                  \"prim\": \"pair\",
                  \"args\": [
                    {
                      \"prim\": \"list\",
                      \"args\": [
                        {
                          \"prim\": \"string\"
                        }
                      ]
                    },
                    {
                      \"prim\": \"list\",
                      \"args\": [
                        {
                          \"prim\": \"string\"
                        }
                      ]
                    }
                  ]
                },
                {
                  \"prim\": \"list\",
                  \"args\": [
                    {
                      \"prim\": \"string\"
                    }
                  ]
                }
              ]
            },
            {
              \"prim\": \"Right\",
              \"args\": [
                [
                  {
                    \"string\": \"d\"
                  },
                  {
                    \"string\": \"c\"
                  },
                  {
                    \"string\": \"b\"
                  },
                  {
                    \"string\": \"a\"
                  }
                ]
              ]
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
              \"prim\": \"list\",
              \"args\": [
                {
                  \"prim\": \"string\"
                }
              ]
            },
            [
              {
                \"string\": \"d\"
              },
              {
                \"string\": \"c\"
              },
              {
                \"string\": \"b\"
              },
              {
                \"string\": \"a\"
              }
            ]
          ]
        }
      ]
    ]
  }
]";

#[test]
fn tzt_loopleft_03() -> Result<()> {
    TZT::try_from(CASE)?.run()
}
