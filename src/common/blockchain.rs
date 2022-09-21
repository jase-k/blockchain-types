use serde::{Deserialize, Serialize};
use std::fmt;


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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockChainStats {
    blockchain_name: String,
    id: String,       
    short_description: String, // (bitcoin_30_days; bitcoin_90_days, bitcoin_1_year)
    time_offset: u32, // seconds

    #[serde(default = "default_f64")]
    total_coin_issuance: f64,

    #[serde(default = "default_f64")]
    total_coin_in_circulation: f64,
    
    #[serde(default = "default_u32")]
    block_height: u32,

    #[serde(default = "default_u32")]
    block_range_start: u32,
    
    #[serde(default = "default_u32")]
    block_range_end: u32,
    
    #[serde(default = "default_u32")]
    date_range_start: u32,
    
    #[serde(default = "default_u32")]
    date_range_end:u32,
    
    #[serde(default = "default_u32")]
    active_address_total: u32,

    #[serde(default = "default_u32")]
    last_updated: u32,
    
    #[serde(default = "default_stat_type")]
    stat_type: BlockChainStatType,
}

fn default_f64() -> f64{
    return 0.0
}

fn default_u32() -> u32{
    return 0
}


fn default_stat_type() -> BlockChainStatType {
    BlockChainStatType::Default
}

impl BlockChainStats {
    pub fn new(
        blockchain_name: BlockChainNames,
        id: String,
        short_description: String,
        time_offset: u32, 
        total_active_coins: f64,
        total_coin_issuance: f64,
        block_height: u32,
        active_addresses: u32,
        last_updated: u32,
        stat_type: BlockChainStatType,
        block_range_start: u32,
        block_range_end: u32,
        date_range_start: u32,
        date_range_end:u32
    ) -> Self {
        BlockChainStats {
            blockchain_name: blockchain_name.to_string(),
            id,
            short_description,      
            time_offset,
            total_coin_issuance,
            total_coin_in_circulation: total_active_coins,
            block_height,
            block_range_start,
            block_range_end,
            date_range_start,
            date_range_end,
            active_address_total: active_addresses,
            last_updated,
            stat_type: stat_type,
        }
    }
    pub fn block_range_start(&self) -> u32 {
        self.block_range_start
    }
    pub fn block_range_end(&self) -> u32 {
        self.block_range_end
    }
    pub fn date_range_start(&self) -> u32 {
        self.date_range_start
    }
    pub fn date_range_end(&self) -> u32 {
        self.date_range_end
    }
    pub fn id(&self) -> String {
        self.id.clone()
    }
    pub fn time_offset(&self) -> u32 {
        self.time_offset
    }
    pub fn last_updated(&self) -> u32 {
        self.last_updated
    }
    pub fn active_address_total(&self) -> u32 {
        self.active_address_total
    }
    pub fn block_height(&self) -> u32 {
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

    pub fn update_last_updated(&mut self, time: u32) -> () {
        self.last_updated = time;
    }
    pub fn update_date_range(&mut self, start_time: u32, end_time: u32) -> &mut Self {
        self.date_range_start = start_time;
        self.date_range_end = end_time;
        self
    }
    pub fn update_block_range(&mut self, start_block: u32, end_block: u32) -> &mut Self {
        self.block_range_start = start_block;
        self.block_range_end = end_block;
        self
    }
    pub fn update_active_address_total(&mut self, total: u32) -> &mut Self {
        self.active_address_total = total;
        self
    }
    pub fn update_block_height(&mut self, height: u32) -> &mut Self {
        self.block_height = height;
        self
    }
    pub fn update_total_coin_issuance_by_block(&mut self, mut block_height: u32) -> () {
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