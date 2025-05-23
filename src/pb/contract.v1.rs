#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag = "1")]
    pub factory_fee_amount_enableds: ::prost::alloc::vec::Vec<FactoryFeeAmountEnabled>,
    #[prost(message, repeated, tag = "2")]
    pub factory_owner_changeds: ::prost::alloc::vec::Vec<FactoryOwnerChanged>,
    #[prost(message, repeated, tag = "3")]
    pub factory_pool_createds: ::prost::alloc::vec::Vec<FactoryPoolCreated>,
    #[prost(message, repeated, tag = "4")]
    pub pool_burns: ::prost::alloc::vec::Vec<PoolBurn>,
    #[prost(message, repeated, tag = "5")]
    pub pool_collects: ::prost::alloc::vec::Vec<PoolCollect>,
    #[prost(message, repeated, tag = "6")]
    pub pool_collect_protocols: ::prost::alloc::vec::Vec<PoolCollectProtocol>,
    #[prost(message, repeated, tag = "7")]
    pub pool_flashes: ::prost::alloc::vec::Vec<PoolFlash>,
    #[prost(message, repeated, tag = "8")]
    pub pool_increase_observation_cardinality_nexts: ::prost::alloc::vec::Vec<
        PoolIncreaseObservationCardinalityNext,
    >,
    #[prost(message, repeated, tag = "9")]
    pub pool_initializes: ::prost::alloc::vec::Vec<PoolInitialize>,
    #[prost(message, repeated, tag = "10")]
    pub pool_mints: ::prost::alloc::vec::Vec<PoolMint>,
    #[prost(message, repeated, tag = "11")]
    pub pool_set_fee_protocols: ::prost::alloc::vec::Vec<PoolSetFeeProtocol>,
    #[prost(message, repeated, tag = "12")]
    pub pool_swaps: ::prost::alloc::vec::Vec<PoolSwap>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FactoryFeeAmountEnabled {
    #[prost(string, tag = "1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub evt_index: u32,
    #[prost(message, optional, tag = "3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "4")]
    pub evt_block_number: u64,
    #[prost(uint64, tag = "5")]
    pub fee: u64,
    #[prost(int64, tag = "6")]
    pub tick_spacing: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FactoryOwnerChanged {
    #[prost(string, tag = "1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub evt_index: u32,
    #[prost(message, optional, tag = "3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "4")]
    pub evt_block_number: u64,
    #[prost(bytes = "vec", tag = "5")]
    pub old_owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FactoryPoolCreated {
    #[prost(string, tag = "1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub evt_index: u32,
    #[prost(message, optional, tag = "3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "4")]
    pub evt_block_number: u64,
    #[prost(bytes = "vec", tag = "5")]
    pub token0: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub token1: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "7")]
    pub fee: u64,
    #[prost(int64, tag = "8")]
    pub tick_spacing: i64,
    #[prost(bytes = "vec", tag = "9")]
    pub pool: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolBurn {
    #[prost(string, tag = "1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub evt_index: u32,
    #[prost(message, optional, tag = "3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "4")]
    pub evt_block_number: u64,
    #[prost(string, tag = "5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "7")]
    pub tick_lower: i64,
    #[prost(int64, tag = "8")]
    pub tick_upper: i64,
    #[prost(string, tag = "9")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolCollect {
    #[prost(string, tag = "1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub evt_index: u32,
    #[prost(message, optional, tag = "3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "4")]
    pub evt_block_number: u64,
    #[prost(string, tag = "5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "8")]
    pub tick_lower: i64,
    #[prost(int64, tag = "9")]
    pub tick_upper: i64,
    #[prost(string, tag = "10")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolCollectProtocol {
    #[prost(string, tag = "1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub evt_index: u32,
    #[prost(message, optional, tag = "3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "4")]
    pub evt_block_number: u64,
    #[prost(string, tag = "5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "8")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolFlash {
    #[prost(string, tag = "1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub evt_index: u32,
    #[prost(message, optional, tag = "3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "4")]
    pub evt_block_number: u64,
    #[prost(string, tag = "5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "8")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub paid0: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub paid1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolIncreaseObservationCardinalityNext {
    #[prost(string, tag = "1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub evt_index: u32,
    #[prost(message, optional, tag = "3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "4")]
    pub evt_block_number: u64,
    #[prost(string, tag = "5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "6")]
    pub observation_cardinality_next_old: u64,
    #[prost(uint64, tag = "7")]
    pub observation_cardinality_next_new: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolInitialize {
    #[prost(string, tag = "1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub evt_index: u32,
    #[prost(message, optional, tag = "3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "4")]
    pub evt_block_number: u64,
    #[prost(string, tag = "5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub sqrt_price_x96: ::prost::alloc::string::String,
    #[prost(int64, tag = "7")]
    pub tick: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolMint {
    #[prost(string, tag = "1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub evt_index: u32,
    #[prost(message, optional, tag = "3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "4")]
    pub evt_block_number: u64,
    #[prost(string, tag = "5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "8")]
    pub tick_lower: i64,
    #[prost(int64, tag = "9")]
    pub tick_upper: i64,
    #[prost(string, tag = "10")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolSetFeeProtocol {
    #[prost(string, tag = "1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub evt_index: u32,
    #[prost(message, optional, tag = "3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "4")]
    pub evt_block_number: u64,
    #[prost(string, tag = "5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "6")]
    pub fee_protocol0_old: u64,
    #[prost(uint64, tag = "7")]
    pub fee_protocol1_old: u64,
    #[prost(uint64, tag = "8")]
    pub fee_protocol0_new: u64,
    #[prost(uint64, tag = "9")]
    pub fee_protocol1_new: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolSwap {
    #[prost(string, tag = "1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub evt_index: u32,
    #[prost(message, optional, tag = "3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "4")]
    pub evt_block_number: u64,
    #[prost(string, tag = "5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "8")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub sqrt_price_x96: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub liquidity: ::prost::alloc::string::String,
    #[prost(int64, tag = "12")]
    pub tick: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolBurnCall {
    #[prost(string, tag = "1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "3")]
    pub call_block_number: u64,
    #[prost(uint64, tag = "4")]
    pub call_ordinal: u64,
    #[prost(bool, tag = "5")]
    pub call_success: bool,
    #[prost(string, tag = "6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(int64, tag = "7")]
    pub tick_lower: i64,
    #[prost(int64, tag = "8")]
    pub tick_upper: i64,
    #[prost(string, tag = "9")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub output_amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub output_amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolCollectCall {
    #[prost(string, tag = "1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "3")]
    pub call_block_number: u64,
    #[prost(uint64, tag = "4")]
    pub call_ordinal: u64,
    #[prost(bool, tag = "5")]
    pub call_success: bool,
    #[prost(string, tag = "6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "8")]
    pub tick_lower: i64,
    #[prost(int64, tag = "9")]
    pub tick_upper: i64,
    #[prost(string, tag = "10")]
    pub amount0_requested: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub amount1_requested: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub output_amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub output_amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolCollectProtocolCall {
    #[prost(string, tag = "1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "3")]
    pub call_block_number: u64,
    #[prost(uint64, tag = "4")]
    pub call_ordinal: u64,
    #[prost(bool, tag = "5")]
    pub call_success: bool,
    #[prost(string, tag = "6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "8")]
    pub amount0_requested: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub amount1_requested: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub output_amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub output_amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolFlashCall {
    #[prost(string, tag = "1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "3")]
    pub call_block_number: u64,
    #[prost(uint64, tag = "4")]
    pub call_ordinal: u64,
    #[prost(bool, tag = "5")]
    pub call_success: bool,
    #[prost(string, tag = "6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "8")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "10")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolIncreaseObservationCardinalityNextCall {
    #[prost(string, tag = "1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "3")]
    pub call_block_number: u64,
    #[prost(uint64, tag = "4")]
    pub call_ordinal: u64,
    #[prost(bool, tag = "5")]
    pub call_success: bool,
    #[prost(string, tag = "6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub observation_cardinality_next: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolInitializeCall {
    #[prost(string, tag = "1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "3")]
    pub call_block_number: u64,
    #[prost(uint64, tag = "4")]
    pub call_ordinal: u64,
    #[prost(bool, tag = "5")]
    pub call_success: bool,
    #[prost(string, tag = "6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub sqrt_price_x96: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolMintCall {
    #[prost(string, tag = "1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "3")]
    pub call_block_number: u64,
    #[prost(uint64, tag = "4")]
    pub call_ordinal: u64,
    #[prost(bool, tag = "5")]
    pub call_success: bool,
    #[prost(string, tag = "6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "8")]
    pub tick_lower: i64,
    #[prost(int64, tag = "9")]
    pub tick_upper: i64,
    #[prost(string, tag = "10")]
    pub amount: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "11")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "12")]
    pub output_amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub output_amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolSetFeeProtocolCall {
    #[prost(string, tag = "1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "3")]
    pub call_block_number: u64,
    #[prost(uint64, tag = "4")]
    pub call_ordinal: u64,
    #[prost(bool, tag = "5")]
    pub call_success: bool,
    #[prost(string, tag = "6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub fee_protocol0: u64,
    #[prost(uint64, tag = "8")]
    pub fee_protocol1: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolSwapCall {
    #[prost(string, tag = "1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "3")]
    pub call_block_number: u64,
    #[prost(uint64, tag = "4")]
    pub call_ordinal: u64,
    #[prost(bool, tag = "5")]
    pub call_success: bool,
    #[prost(string, tag = "6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "8")]
    pub zero_for_one: bool,
    #[prost(string, tag = "9")]
    pub amount_specified: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub sqrt_price_limit_x96: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "11")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "12")]
    pub output_amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub output_amount1: ::prost::alloc::string::String,
}
