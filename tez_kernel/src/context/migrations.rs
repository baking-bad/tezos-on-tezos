use tez_proto::{
    context::Context,
    context::types::Mutez,
    executor::block::balance_update::{Contract, Kind, Origin, BalanceUpdate}
};

use crate::{error::Result, migration_error};

const SEED_ACCOUNTS: [&str; 8] = [
    "tz1grSQDByRpnVs7sPtaprNZRp531ZKz6Jmm",  // Pytezos built-in key
    "tz1VSUr8wwNhLAzempoch5d6hLRiTh8Cjcjb",  // Alice from Flextesa
    "tz1TGu6TN5GSez2ndXXeDX6LgUDvLzPLqgYV",  // Activator from Tezos sandbox
    "tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx",  // Bootstrap 1 from Tezos sandbox
    "tz1gjaF81ZRRvdzjobyfVNsAeSC6PScjfQwN",  // Bootstrap 2 from Tezos sandbox
    "tz1faswCTDciRzE4oJ9jn2Vm2dvjeyA9fUzU",  // Bootstrap 3 from Tezos sandbox
    "tz1b7tUupMgCNw2cCLpKTkSD1NZzB5TkP2sv",  // Bootstrap 4 from Tezos sandbox
    "tz1ddb9NMYHZi5UzPdzTZMYQQZoMub195zgv",  // Bootstrap 5 from Tezos sandbox
];
const SEED_BALANCE: u64 = 40_000_000_000_000u64;

pub fn genesis_migration(context: &mut impl Context) -> Result<Vec<BalanceUpdate>> {
    if context.has_pending_changes() {
        return migration_error!("Cannot proceed with uncommitted changes")
    }

    let mut updates: Vec<BalanceUpdate> = Vec::with_capacity(SEED_ACCOUNTS.len());
    let balance = Mutez::try_from(SEED_BALANCE).unwrap();

    for account in SEED_ACCOUNTS.into_iter() {
        context.set_balance(&account, &balance)?;
        updates.push(BalanceUpdate::Contract(Contract {
            kind: Kind::Contract,
            change: SEED_BALANCE.to_string(),
            contract: account.to_string(),
            origin: Some(Origin::Migration)
        }));
    }

    context.commit()?;
    Ok(updates)
}

pub fn run_migrations(context: &mut impl Context, _inbox_level: i32) -> Result<Option<Vec<BalanceUpdate>>> {
    let head = context.get_head()?;
    // TODO: check that `level + 1 == inbox_level - origination_level`
    match head.level {
        -1 => Ok(Some(genesis_migration(context)?)),
        _ => Ok(None)
    }
}