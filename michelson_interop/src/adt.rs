use tezos_michelson::michelson::{
    types::{Type, self},
    data::{Data, self},
};
use crate::{Result, Error};

pub fn flatten_pair(ty: Type, data: Data) -> Result<data::Pair> {
    let pair_ty: types::Pair = ty.try_into()?;
    let pair: data::Pair = data.try_into()?;

    let mut args = pair.values;

    for (i, arg_type) in pair_ty.types.iter().enumerate() {
        if i >= args.len() {
            return Err(Error::TypeMismatch {
                message: format!("Expected pair with {} fields, got only {}", pair_ty.types.len(), args.len())
            })
        }

        while let Data::Pair(_) = &args[i] {
            if let Type::Pair(_) = arg_type {
                // Named inner pair does not collapse
                break
            } else {
                let inner_pair: data::Pair = args.remove(i).try_into()?;
                for (j, inner_arg) in inner_pair.values.into_iter().enumerate() {
                    args.insert(i + j, inner_arg)
                }
            }
        }
    }

    Ok(data::pair(args))
}