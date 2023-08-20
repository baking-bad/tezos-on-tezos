// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use ibig::{IBig, UBig};
use layered_store::{LayeredStore, StoreBackend};
use tezos_core::types::{
    encoded::{Address, ContractAddress, Encoded, ScriptExprHash},
    number::Nat,
};
use tezos_michelson::{micheline::Micheline, michelson::types::Type};

use crate::{error::err_into, types::ticket::TicketBalanceDiff, Error, InterpreterContext, Result};

fn get_ticket_key_hash(
    tickiter: &Address,
    identifier: &Micheline,
    identifier_ty: &Type,
    owner: &Address,
) -> ScriptExprHash {
    todo!()
}

impl<Backend: StoreBackend> InterpreterContext for LayeredStore<Backend> {
    fn set_contract_type(&mut self, address: ContractAddress, value: Micheline) -> Result<()> {
        self.set(
            format!("/context/contracts/{}/entrypoints", address.value()),
            Some(value),
        )
        .map_err(err_into)
    }

    fn get_contract_type(&mut self, address: &ContractAddress) -> Result<Option<Micheline>> {
        self.get(format!(
            "/context/contracts/{}/entrypoints",
            address.value()
        ))
        .map_err(err_into)
    }

    fn allocate_big_map(&mut self, owner: ContractAddress) -> Result<i64> {
        let ptr = match self.get::<i64>("/context/ptr".into()) {
            Ok(Some(val)) => val + 1,
            Ok(None) => 0i64,
            Err(err) => return Err(err_into(err)),
        };
        self.set("/context/ptr".into(), Some(ptr))
            .map_err(err_into)?;
        self.set(format!("/context/bigmaps/{}/owner", ptr), Some(owner))
            .map_err(err_into)?;
        Ok(ptr)
    }

    fn get_big_map_owner(&mut self, ptr: i64) -> Result<Option<ContractAddress>> {
        self.get(format!("/context/bigmaps/{}/owner", ptr))
            .map_err(err_into)
    }

    fn has_big_map_value(&mut self, ptr: i64, key_hash: &ScriptExprHash) -> Result<bool> {
        self.has(format!(
            "/context/bigmaps/{}/values/{}",
            ptr,
            key_hash.value()
        ))
        .map_err(err_into)
    }

    fn get_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: &ScriptExprHash,
    ) -> Result<Option<Micheline>> {
        self.get(format!(
            "/context/bigmaps/{}/values/{}",
            ptr,
            key_hash.value()
        ))
        .map_err(err_into)
    }

    fn set_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: ScriptExprHash,
        value: Option<Micheline>,
    ) -> Result<()> {
        self.set(
            format!("/context/bigmaps/{}/values/{}", ptr, key_hash.value()),
            value,
        )
        .map_err(err_into)
    }

    fn get_ticket_balance(
        &mut self,
        tickiter: &Address,
        identifier: &Micheline,
        identifier_ty: &Type,
        owner: &Address,
    ) -> Result<UBig> {
        let key_hash = get_ticket_key_hash(tickiter, identifier, identifier_ty, owner);

        let balance = match self
            .get::<Nat>(format!("/context/tickets/{}", key_hash.value()))
            .map_err(err_into)?
        {
            Some(nat) => UBig::from(nat),
            None => UBig::from(0u32),
        };
        Ok(balance)
    }

    fn update_ticket_balance(
        &mut self,
        tickiter: &Address,
        identifier: &Micheline,
        identifier_ty: &Type,
        owner: &Address,
        value: IBig,
    ) -> Result<()> {
        let current_balance: UBig =
            self.get_ticket_balance(tickiter, identifier, identifier_ty, owner)?;
        let updated_balance: IBig = IBig::from(current_balance) + value.clone();
        if updated_balance < IBig::from(0i32) {
            return Err(Error::NegativeTicketBalance);
        }
        let value_nat: Option<Nat> =
            Option::Some(From::<UBig>::from(UBig::try_from(updated_balance)?));
        let key_hash = get_ticket_key_hash(tickiter, identifier, identifier_ty, owner);
        self.set(format!("/context/tickets/{}", key_hash.value()), value_nat)
            .map_err(err_into)?;

        let ticket_balance_diff = TicketBalanceDiff::new(
            tickiter.clone(),
            identifier.clone(),
            identifier_ty.clone(),
            owner.clone(),
            value,
        );

        self.set_tmp(
            format!("/tmp/{}", key_hash.value()),
            Option::Some(ticket_balance_diff.into_micheline()),
        )
        .map_err(err_into)?;

        Ok(())
    }

    fn aggregate_ticket_updates(&self) -> Vec<TicketBalanceDiff> {
        todo!()
    }
}
