use tezos_core::{
    internal::{
        coder::{ConsumingDecoder, Decoder, Encoder},
        consumable_list::{ConsumableBytes, ConsumableList},
    },
    types::{
        encoded::{Encoded, PublicKey, ImplicitAddress},
        mutez::Mutez,
        number::Nat,
    },
    Error,
    Result
};

#[derive(Debug, Clone)]
pub struct Account {
    pub address: ImplicitAddress,
    pub balance: Mutez,
    pub counter: Nat,
    pub public_key: Option<PublicKey>
}

impl Account {
    pub fn new(address: ImplicitAddress, balance: Mutez, counter: Nat, public_key: Option<PublicKey>) -> Self {
        Self { address, balance, counter, public_key }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        return AccountBytesCoder::encode(self);
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        return AccountBytesCoder::decode(bytes);
    }
}

pub struct AccountBytesCoder;

impl Encoder<Account, Vec<u8>, Error> for AccountBytesCoder {
    fn encode(account: &Account) -> std::result::Result<Vec<u8>, Error> {
        let address_bytes = account.address.to_bytes()?;
        let balance_bytes = account.balance.to_bytes()?;
        let counter_bytes = account.counter.to_bytes()?;
        let public_key_bytes = match &account.public_key {
            Some(value) => value.to_bytes()?,  // public key starts with \x00, \x01, or \x02 depending on the curve
            None => [b'\xff'].to_vec()
        };
        Ok([address_bytes, balance_bytes, counter_bytes, public_key_bytes].concat())
    }
}

impl Decoder<Account, [u8], Error> for AccountBytesCoder {
    fn decode(value: &[u8]) -> Result<Account> {
        Self::decode_consuming(&mut ConsumableBytes::new(value))
    }
}

impl ConsumingDecoder<Account, u8, Error> for AccountBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Account> {
        let address = ImplicitAddress::from_consumable_bytes(value)?;
        let balance = Mutez::from_consumable_bytes(value)?;
        let counter = Nat::from_consumable_bytes(value)?;
        let public_key = match value.inner_value().first() {
            Some(b'\xff') => None,
            Some(_) => Some(PublicKey::from_consumable_bytes(value)?),
            None => panic!("Could not decode account (public key)")
        };
        Ok(Account::new(address, balance, counter, public_key))
    }
}