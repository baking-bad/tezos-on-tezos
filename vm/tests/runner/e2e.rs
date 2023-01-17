use tezos_michelson::micheline::{
    primitive_application::PrimitiveApplication, sequence::Sequence, Micheline,
};
use vm::{interpreter::OperationScope, script::MichelsonScript, Error, Result};

use crate::runner::{
    micheline::{parse_literal, read_from_file},
    mock::{default_scope, MockContext},
};

pub enum Expectation {
    Storage(Micheline),
    Error(Error),
}

pub struct E2E {
    script: MichelsonScript,
    parameter: Micheline,
    storage: Micheline,
    expected: Expectation,
}

impl E2E {
    pub fn run(&mut self) -> Result<()> {
        let scope = OperationScope {
            parameters: Some(("default".into(), self.parameter.clone())),
            storage: self.storage.clone(),
            balance: "4000000000000".try_into()?,
            level: 1,
            chain_id: "NetXdQprcVkpaWU".try_into()?,
            self_type: self.script.get_type().into(),
            ..default_scope()
        };
        let mut context = MockContext::default();
        let ret = match self.script.call(&scope, &mut context) {
            Ok(res) => res,
            Err(err) => {
                err.print();
                panic!("{}", err);
            }
        };

        match self.expected {
            Expectation::Storage(ref expected) => {
                assert_eq!(*expected, ret.storage);
            }
            _ => {}
        }

        Ok(())
    }

    pub fn load(filename: &str) -> Result<Self> {
        let src = read_from_file("e2e", filename)?;
        let sections = Sequence::try_from(src)?;

        let mut parameter: Option<Micheline> = None;
        let mut storage: Option<Micheline> = None;
        let mut expected: Option<Expectation> = None;
        let mut script: Option<Micheline> = None;

        for section in sections.into_values() {
            let prim = PrimitiveApplication::try_from(section)?;
            match prim.prim() {
                "parameter" => parameter = Some(prim.into_args().unwrap().remove(0)),
                "storage" => storage = Some(prim.into_args().unwrap().remove(0)),
                "result" => {
                    expected = {
                        let result = prim.into_args().unwrap().remove(0);
                        Some(Expectation::Storage(result.normalized())) // VM output is always normalized
                    }
                }
                "script" => {
                    let filename: String = parse_literal(prim);
                    script = Some(read_from_file("scripts", filename.as_str())?);
                }
                prim => panic!("Unexpected section {}", prim),
            }
        }

        Ok(Self {
            script: script.expect("Script section is missing").try_into()?,
            parameter: parameter.expect("Parameter section is missing"),
            storage: storage.expect("Storage section is missing"),
            expected: expected.expect("Both result and error sections are missing"),
        })
    }
}
