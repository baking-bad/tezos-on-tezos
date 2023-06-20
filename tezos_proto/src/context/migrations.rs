use tezos_core::types::mutez::Mutez;
use tezos_rpc::models::balance_update::{BalanceUpdate, Contract, Kind, Origin};

use crate::{
    context::{head::Head, TezosContext},
    Result,
};

const SEED_ACCOUNTS: [&str; 8] = [
    "tz1grSQDByRpnVs7sPtaprNZRp531ZKz6Jmm", // Pytezos built-in key
    "tz1VSUr8wwNhLAzempoch5d6hLRiTh8Cjcjb", // Alice from Flextesa
    "tz1TGu6TN5GSez2ndXXeDX6LgUDvLzPLqgYV", // Activator from Tezos sandbox
    "tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx", // Bootstrap 1 from Tezos sandbox
    "tz1gjaF81ZRRvdzjobyfVNsAeSC6PScjfQwN", // Bootstrap 2 from Tezos sandbox
    "tz1faswCTDciRzE4oJ9jn2Vm2dvjeyA9fUzU", // Bootstrap 3 from Tezos sandbox
    "tz1b7tUupMgCNw2cCLpKTkSD1NZzB5TkP2sv", // Bootstrap 4 from Tezos sandbox
    "tz1ddb9NMYHZi5UzPdzTZMYQQZoMub195zgv", // Bootstrap 5 from Tezos sandbox
];
const SEED_BALANCE: u64 = 40_000_000_000_000u64;

pub fn genesis_migration(context: &mut impl TezosContext) -> Result<Vec<BalanceUpdate>> {
    let mut updates: Vec<BalanceUpdate> = Vec::with_capacity(SEED_ACCOUNTS.len());
    let balance = Mutez::try_from(SEED_BALANCE).unwrap();

    for account in SEED_ACCOUNTS.into_iter() {
        context.set_balance(&account, balance)?;
        updates.push(BalanceUpdate::Contract(Contract {
            kind: Kind::Contract,
            change: SEED_BALANCE.to_string(),
            contract: account.to_string(),
            origin: Some(Origin::Migration),
        }));
    }

    context.commit()?;
    Ok(updates)
}

pub fn run_migrations(
    context: &mut impl TezosContext,
    head: &Head,
) -> Result<Option<Vec<BalanceUpdate>>> {
    context.check_no_pending_changes()?;
    match head.level {
        -1 => Ok(Some(genesis_migration(context)?)),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{context::TezosEphemeralContext, Result};

    #[test]
    fn test_seed_acconuts() -> Result<()> {
        let mut context = TezosEphemeralContext::new();

        let head = context.get_head()?;
        assert_eq!(-1, head.level);

        let updates = run_migrations(&mut context, &head)?.expect("Seed balance updates");
        assert_eq!(8, updates.len());

        let balance = context
            .get_balance(&"tz1grSQDByRpnVs7sPtaprNZRp531ZKz6Jmm")?
            .expect("Seed balance");
        assert_eq!(Mutez::try_from(SEED_BALANCE).unwrap(), balance);

        Ok(())
    }
}
