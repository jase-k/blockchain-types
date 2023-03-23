use serde::{Deserialize, Serialize};
use serde::Deserializer;
use std::fmt;
use named_type_derive::*;
use named_type::NamedType;
use devii::devii::DeviiTrait;
use std::error::Error;
use easy_error::bail;
use getset::{CopyGetters, Getters};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BlockChainStatType {
    Default, // Always have a time range that ends with SystemTime::now()
    Custom,  // Has a time range in the past, and isn't updated
}

impl fmt::Display for BlockChainStatType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        match self {
            BlockChainStatType::Default => write!(f, "Default"),
            BlockChainStatType::Custom => write!(f, "Custom")
        }
    }
}

impl std::default::Default for BlockChainStatType {
    fn default() -> Self {
        BlockChainStatType::Default
    }
}

#[derive (Serialize, Deserialize, Debug, Clone, NamedType, Default)]
pub struct ChainStats {
    #[serde(deserialize_with = "deserialize_i64_or_string")]
    #[serde(skip_serializing)]
    id: Option<i64>,       
    blockchain_name: String,
    short_description: String, // (bitcoin_30_days; bitcoin_90_days, bitcoin_1_year)
    time_offset: i64, // seconds

    #[serde(default = "default_f64")]
    total_coin_issuance: f64,

    #[serde(default = "default_f64")]
    total_active_coins: f64,

    #[serde(default = "default_f64")]
    total_unknown_supply: Option<f64>,
    
    #[serde(default = "default_i64")]
    block_height: i64,

    #[serde(default = "default_i64")]
    block_range_start: i64,
    
    #[serde(default = "default_i64")]
    block_range_end: i64,
    
    #[serde(default = "default_i64")]
    date_range_start: i64,
    
    #[serde(default = "default_i64")]
    date_range_end:i64,
    
    #[serde(default = "default_i64")]
    active_addresses: i64,

    #[serde(default = "default_i64")]
    last_updated: i64,
    
    #[serde(default = "default_stat_type")]
    stat_type: BlockChainStatType,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrU64 { U64(i64), Str(String) }
pub fn deserialize_i64_or_string<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where D: Deserializer<'de>
{
    match StringOrU64::deserialize(deserializer)? {
        StringOrU64::U64(v) => { Ok(Some(v)) }
        StringOrU64::Str(v) => {
            let res = v.parse::<i64>();
            if let Ok(r) = res {
                Ok(Some(r))
            } else {
                Err(serde::de::Error::custom("Can't parse id!"))
            }
        }
    }
}

fn default_f64() -> f64{
    return 0.0
}

fn default_i64() -> i64{
    return 0
}


fn default_stat_type() -> BlockChainStatType {
    BlockChainStatType::Default
}

impl ChainStats {
    pub fn new(
        blockchain_name: BlockChainNames,
        short_description: String,
        time_offset: i64, 
    ) -> Self {
        ChainStats {
            id: None,
            blockchain_name: blockchain_name.to_string(),
            short_description,     
            time_offset,
            total_coin_issuance : 0.0,
            total_active_coins: 0.0,
            total_unknown_supply: Some(0.0),
            block_height: 0,
            block_range_start: 0,
            block_range_end: 0,
            date_range_start: 0,
            date_range_end: 0,
            active_addresses: 0,
            last_updated: 0,
            stat_type: BlockChainStatType::default(),
        }
    }
    pub fn block_range_start(&self) -> i64 {
        self.block_range_start
    }
    pub fn block_range_end(&self) -> i64 {
        self.block_range_end
    }
    pub fn date_range_start(&self) -> i64 {
        self.date_range_start
    }
    pub fn date_range_end(&self) -> i64 {
        self.date_range_end
    }
    pub fn id(&self) -> Option<i64> {
        self.id
    }
    pub fn time_offset(&self) -> i64 {
        self.time_offset
    }
    pub fn last_updated(&self) -> i64 {
        self.last_updated
    }
    pub fn active_addresses(&self) -> i64 {
        self.active_addresses
    }
    pub fn block_height(&self) -> i64 {
        self.block_height
    }
    pub fn total_coin_issuance(&self) -> f64 {
        self.total_coin_issuance
    }
    pub fn blockchain_name(&self) -> String {
        self.blockchain_name.clone()
    }
    pub fn total_active_coins(&self) -> f64 {
        self.total_active_coins
    }
    pub fn total_unknown_supply(&self) -> Option<f64> {
        self.total_unknown_supply
    }
    pub fn short_description(&self) -> String {
        self.short_description.clone()
    }
    pub fn stat_type(&self) -> BlockChainStatType {
        self.stat_type.clone()
    }

    pub fn update_last_updated(&mut self, time: i64) -> () {
        self.last_updated = time;
    }
    pub fn update_date_range(&mut self, start_time: i64, end_time: i64) -> &mut Self {
        self.date_range_start = start_time;
        self.date_range_end = end_time;
        self
    }
    pub fn update_block_range(&mut self, start_block: i64, end_block: i64) -> &mut Self {
        self.block_range_start = start_block;
        self.block_range_end = end_block;
        self
    }
    pub fn update_active_addresses(&mut self, total: i64) -> &mut Self {
        self.active_addresses = total;
        self
    }
    pub fn update_block_height(&mut self, height: i64) -> &mut Self {
        self.block_height = height;
        self
    }
    pub fn update_total_coin_issuance_by_block(&mut self, mut block_height: i64) -> () {
        let mut bitcoin_reward: f64 = 50.0;
        let mut total_mined: f64 = 0.0;

        while block_height > 210000 {
            total_mined += bitcoin_reward * 210000.0;
            bitcoin_reward = bitcoin_reward / 2.0;
            block_height -= 210000;
        }

        total_mined += bitcoin_reward * block_height as f64;

        self.total_coin_issuance = total_mined;
    }
    pub fn update_total_coin_issuance(&mut self, amount: f64) -> () {
        self.total_coin_issuance = amount;
    }
    pub fn update_total_active_coins(&mut self, amount: f64) -> () {
        self.total_active_coins = amount;
    }
    pub fn update_total_unknown_supply(&mut self, amount: Option<f64>) -> () {
        self.total_unknown_supply = amount;
    }
}

impl DeviiTrait for ChainStats {
    fn fetch_fields() -> String {
        format!("{{ id, blockchain_name, short_description, time_offset, total_coin_issuance, total_active_coins, total_unknown_supply, block_height, block_range_start, block_range_end, date_range_start, date_range_end, active_addresses, last_updated, stat_type}}")
    }
    fn insert_query(&self, param: String) -> String{
        format!("create_chain_stats (input: ${} ){{ id }}", param)
    }
    fn input_type(&self) -> String {
        "chain_statsInput".to_string()
    }
    fn graphql_inputs(&self) -> serde_json::Value {
        serde_json::to_value(&self).unwrap()
    }
    fn delete_input(&self) -> String {
        format!("id: \"{}\"", self.id().unwrap())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BlockChainNames {
    Bitcoin,
    BitcoinCash,
    Dogecoin,
    Litecoin,
    Dash,
    Ethereum,
    EthereumClassic
}

impl fmt::Display for BlockChainNames {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        match self {
            BlockChainNames::Bitcoin => write!(f, "Bitcoin"),
            BlockChainNames::BitcoinCash => write!(f, "Bitcoin_Cash"),
            BlockChainNames::Dogecoin => write!(f, "Dogecoin"),
            BlockChainNames::Litecoin => write!(f, "Litecoin"),
            BlockChainNames::Dash => write!(f, "Dash"),
            BlockChainNames::Ethereum => write!(f, "Ethereum"),
            BlockChainNames::EthereumClassic => write!(f, "Ethereum_Classic"),
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Getters, CopyGetters)]
pub struct BlockChain { 
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    short_description: String,
    #[getset(get = "pub")]
    key: String,
    #[getset(get_copy = "pub")]
    decimal_places: u8
}

impl Eq for BlockChain {}

impl BlockChain {
    pub fn new(name: BlockChainNames) -> Self {
        match name {
            BlockChainNames::Bitcoin => BlockChain {
                name: "Bitcoin".to_string(),
                short_description: "BTC".to_string(),
                key: "bitcoin".to_string(),
                decimal_places: 8
            },
            BlockChainNames::BitcoinCash => BlockChain {
                name: "Bitcoin Cash".to_string(),
                short_description: "BCH".to_string(),
                key: "bitcoin-cash".to_string(),
                decimal_places: 8
            }, 
            BlockChainNames::Dogecoin => BlockChain {
                name: "Dogecoin".to_string(),
                short_description: "DOGE".to_string(),
                key: "dogecoin".to_string(),
                decimal_places: 8
            },
            BlockChainNames::Litecoin => BlockChain {
                name: "Litecoin".to_string(),
                short_description: "LTC".to_string(),
                key: "litecoin".to_string(),
                decimal_places: 8
            },
            BlockChainNames::Dash => BlockChain {
                name: "Dash".to_string(),
                short_description: "DASH".to_string(),
                key: "dash".to_string(),
                decimal_places: 8
            },
            BlockChainNames::Ethereum => BlockChain {
                name: "Ethereum".to_string(),
                short_description: "ETH".to_string(),
                key: "ethereum".to_string(),
                decimal_places: 18
            },
            BlockChainNames::EthereumClassic => BlockChain {
                name: "Ethereum Classic".to_string(),
                short_description: "ETC".to_string(),
                key: "ethereum-classic".to_string(),
                decimal_places: 18
            }
        }
    }
    pub fn new_from_string(name: String) -> Result<Self, Box<dyn Error>> {
        let name = name.to_lowercase();
        match name.as_str() {
            "bitcoin" => Ok(BlockChain::new(BlockChainNames::Bitcoin)),
            "bitcoin-cash" => Ok(BlockChain::new(BlockChainNames::BitcoinCash)),
            "dogecoin" => Ok(BlockChain::new(BlockChainNames::Dogecoin)),
            "litecoin" => Ok(BlockChain::new(BlockChainNames::Litecoin)),
            "dash" => Ok(BlockChain::new(BlockChainNames::Dash)),
            "ethereum" => Ok(BlockChain::new(BlockChainNames::Ethereum)),
            "ethereum-classic" => Ok(BlockChain::new(BlockChainNames::EthereumClassic)),
            _ => bail!("Invalid blockchain name")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blockchain_new() {
        let bitcoin = BlockChain::new(BlockChainNames::Bitcoin);
        assert_eq!(bitcoin.name, "Bitcoin");
        assert_eq!(bitcoin.short_description, "BTC");
        assert_eq!(bitcoin.key, "bitcoin");
        assert_eq!(bitcoin.decimal_places, 8);
    }
    #[test]
    fn test_blockchain_new_from_string() {
        let bitcoin = BlockChain::new_from_string("Bitcoin".to_string()).unwrap();
        assert_eq!(bitcoin.name, "Bitcoin");
        assert_eq!(bitcoin.short_description, "BTC");
        assert_eq!(bitcoin.key, "bitcoin");
        assert_eq!(bitcoin.decimal_places, 8);
    }
    #[test]
    fn test_blockchain_new_from_string_invalid() {
        let bitcoin = BlockChain::new_from_string("BitcoinCash".to_string());
        assert!(bitcoin.is_err());
    }
}