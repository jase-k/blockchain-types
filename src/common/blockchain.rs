use serde::{Deserialize, Serialize};
use serde::Deserializer;
use std::fmt;
use named_type_derive::*;
use named_type::NamedType;
use devii::devii::DeviiTrait;

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

#[derive(Serialize, Deserialize, Debug, Clone, NamedType, Default)]
pub struct BlockChainStats {
    #[serde(deserialize_with = "deserialize_u64_or_string")]
    #[serde(skip_serializing)]
    id: Option<u64>,       
    blockchain_name: String,
    short_description: String, // (bitcoin_30_days; bitcoin_90_days, bitcoin_1_year)
    time_offset: u64, // seconds

    #[serde(default = "default_f64")]
    total_coin_issuance: f64,

    #[serde(default = "default_f64")]
    total_coin_in_circulation: f64,
    
    #[serde(default = "default_u64")]
    block_height: u64,

    #[serde(default = "default_u64")]
    block_range_start: u64,
    
    #[serde(default = "default_u64")]
    block_range_end: u64,
    
    #[serde(default = "default_u64")]
    date_range_start: u64,
    
    #[serde(default = "default_u64")]
    date_range_end:u64,
    
    #[serde(default = "default_u64")]
    active_address_total: u64,

    #[serde(default = "default_u64")]
    last_updated: u64,
    
    #[serde(default = "default_stat_type")]
    stat_type: BlockChainStatType,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrU64 { U64(u64), Str(String) }
pub fn deserialize_u64_or_string<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where D: Deserializer<'de>
{
    match StringOrU64::deserialize(deserializer)? {
        StringOrU64::U64(v) => { Ok(Some(v)) }
        StringOrU64::Str(v) => {
            let res = v.parse::<u64>();
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

fn default_u64() -> u64{
    return 0
}


fn default_stat_type() -> BlockChainStatType {
    BlockChainStatType::Default
}

impl BlockChainStats {
    pub fn new(
        blockchain_name: BlockChainNames,
        short_description: String,
        time_offset: u64, 
    ) -> Self {
        BlockChainStats {
            id: None,
            blockchain_name: blockchain_name.to_string(),
            short_description,     
            time_offset,
            total_coin_issuance : 0.0,
            total_coin_in_circulation: 0.0,
            block_height: 0,
            block_range_start: 0,
            block_range_end: 0,
            date_range_start: 0,
            date_range_end: 0,
            active_address_total: 0,
            last_updated: 0,
            stat_type: BlockChainStatType::default(),
        }
    }
    pub fn block_range_start(&self) -> u64 {
        self.block_range_start
    }
    pub fn block_range_end(&self) -> u64 {
        self.block_range_end
    }
    pub fn date_range_start(&self) -> u64 {
        self.date_range_start
    }
    pub fn date_range_end(&self) -> u64 {
        self.date_range_end
    }
    pub fn id(&self) -> Option<u64> {
        self.id
    }
    pub fn time_offset(&self) -> u64 {
        self.time_offset
    }
    pub fn last_updated(&self) -> u64 {
        self.last_updated
    }
    pub fn active_address_total(&self) -> u64 {
        self.active_address_total
    }
    pub fn block_height(&self) -> u64 {
        self.block_height
    }
    pub fn total_coin_issuance(&self) -> f64 {
        self.total_coin_issuance
    }
    pub fn blockchain_name(&self) -> String {
        self.blockchain_name.clone()
    }
    pub fn total_coin_in_circulation(&self) -> f64 {
        self.total_coin_in_circulation
    }
    pub fn short_description(&self) -> String {
        self.short_description.clone()
    }
    pub fn stat_type(&self) -> BlockChainStatType {
        self.stat_type.clone()
    }

    pub fn update_last_updated(&mut self, time: u64) -> () {
        self.last_updated = time;
    }
    pub fn update_date_range(&mut self, start_time: u64, end_time: u64) -> &mut Self {
        self.date_range_start = start_time;
        self.date_range_end = end_time;
        self
    }
    pub fn update_block_range(&mut self, start_block: u64, end_block: u64) -> &mut Self {
        self.block_range_start = start_block;
        self.block_range_end = end_block;
        self
    }
    pub fn update_active_address_total(&mut self, total: u64) -> &mut Self {
        self.active_address_total = total;
        self
    }
    pub fn update_block_height(&mut self, height: u64) -> &mut Self {
        self.block_height = height;
        self
    }
    pub fn update_total_coin_issuance_by_block(&mut self, mut block_height: u64) -> () {
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
    pub fn update_total_coin_in_circulation(&mut self, amount: f64) -> () {
        self.total_coin_in_circulation = amount;
    }
}

impl DeviiTrait for BlockChainStats {
    fn fetch_fields() -> String {
        format!("{{ id, blockchain_name, short_description,  time_offset, total_coin_issuance, total_coin_in_circulation, block_height, block_range_start, block_range_end, date_range_start, date_range_end, active_address_total, last_updated, stat_type}}")
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
            BlockChainNames::BitcoinCash => write!(f, "Bitcoin Cash"),
            BlockChainNames::Dogecoin => write!(f, "Dogecoin"),
            BlockChainNames::Litecoin => write!(f, "Litecoin"),
            BlockChainNames::Dash => write!(f, "Dash"),
            BlockChainNames::Ethereum => write!(f, "Ethereum"),
            BlockChainNames::EthereumClassic => write!(f, "Ethereum Classic"),
        }
    }
}