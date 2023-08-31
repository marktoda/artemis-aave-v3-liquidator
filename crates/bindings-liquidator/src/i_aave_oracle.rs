pub use i_aave_oracle::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod i_aave_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BASE_CURRENCY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BASE_CURRENCY"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BASE_CURRENCY_UNIT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BASE_CURRENCY_UNIT"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAssetPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAssetPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAssetsPrices"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAssetsPrices"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getFallbackOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFallbackOracle"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSourceOfAsset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSourceOfAsset"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAssetSources"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAssetSources"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sources"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFallbackOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFallbackOracle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fallbackOracle"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AssetSourceUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssetSourceUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("source"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BaseCurrencySet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BaseCurrencySet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("baseCurrency"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("baseCurrencyUnit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FallbackOracleUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FallbackOracleUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fallbackOracle"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IAAVEORACLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IAaveOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IAaveOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IAaveOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IAaveOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IAaveOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IAaveOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IAaveOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IAAVEORACLE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `BASE_CURRENCY` (0xe19f4700) function
        pub fn base_currency(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([225, 159, 71, 0], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BASE_CURRENCY_UNIT` (0x8c89b64f) function
        pub fn base_currency_unit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([140, 137, 182, 79], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAssetPrice` (0xb3596f07) function
        pub fn get_asset_price(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([179, 89, 111, 7], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAssetsPrices` (0x9d23d9f2) function
        pub fn get_assets_prices(
            &self,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([157, 35, 217, 242], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFallbackOracle` (0x6210308c) function
        pub fn get_fallback_oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([98, 16, 48, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSourceOfAsset` (0x92bf2be0) function
        pub fn get_source_of_asset(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([146, 191, 43, 224], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAssetSources` (0xabfd5310) function
        pub fn set_asset_sources(
            &self,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
            sources: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 253, 83, 16], (assets, sources))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFallbackOracle` (0x170aee73) function
        pub fn set_fallback_oracle(
            &self,
            fallback_oracle: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 10, 238, 115], fallback_oracle)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AssetSourceUpdated` event
        pub fn asset_source_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssetSourceUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BaseCurrencySet` event
        pub fn base_currency_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BaseCurrencySetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FallbackOracleUpdated` event
        pub fn fallback_oracle_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FallbackOracleUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IAaveOracleEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IAaveOracle<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "AssetSourceUpdated", abi = "AssetSourceUpdated(address,address)")]
    pub struct AssetSourceUpdatedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub source: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "BaseCurrencySet", abi = "BaseCurrencySet(address,uint256)")]
    pub struct BaseCurrencySetFilter {
        #[ethevent(indexed)]
        pub base_currency: ::ethers::core::types::Address,
        pub base_currency_unit: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "FallbackOracleUpdated", abi = "FallbackOracleUpdated(address)")]
    pub struct FallbackOracleUpdatedFilter {
        #[ethevent(indexed)]
        pub fallback_oracle: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IAaveOracleEvents {
        AssetSourceUpdatedFilter(AssetSourceUpdatedFilter),
        BaseCurrencySetFilter(BaseCurrencySetFilter),
        FallbackOracleUpdatedFilter(FallbackOracleUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IAaveOracleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssetSourceUpdatedFilter::decode_log(log) {
                return Ok(IAaveOracleEvents::AssetSourceUpdatedFilter(decoded));
            }
            if let Ok(decoded) = BaseCurrencySetFilter::decode_log(log) {
                return Ok(IAaveOracleEvents::BaseCurrencySetFilter(decoded));
            }
            if let Ok(decoded) = FallbackOracleUpdatedFilter::decode_log(log) {
                return Ok(IAaveOracleEvents::FallbackOracleUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IAaveOracleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetSourceUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BaseCurrencySetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FallbackOracleUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AssetSourceUpdatedFilter> for IAaveOracleEvents {
        fn from(value: AssetSourceUpdatedFilter) -> Self {
            Self::AssetSourceUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<BaseCurrencySetFilter> for IAaveOracleEvents {
        fn from(value: BaseCurrencySetFilter) -> Self {
            Self::BaseCurrencySetFilter(value)
        }
    }
    impl ::core::convert::From<FallbackOracleUpdatedFilter> for IAaveOracleEvents {
        fn from(value: FallbackOracleUpdatedFilter) -> Self {
            Self::FallbackOracleUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `BASE_CURRENCY` function with signature `BASE_CURRENCY()` and selector `0xe19f4700`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "BASE_CURRENCY", abi = "BASE_CURRENCY()")]
    pub struct BaseCurrencyCall;
    ///Container type for all input parameters for the `BASE_CURRENCY_UNIT` function with signature `BASE_CURRENCY_UNIT()` and selector `0x8c89b64f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "BASE_CURRENCY_UNIT", abi = "BASE_CURRENCY_UNIT()")]
    pub struct BaseCurrencyUnitCall;
    ///Container type for all input parameters for the `getAssetPrice` function with signature `getAssetPrice(address)` and selector `0xb3596f07`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getAssetPrice", abi = "getAssetPrice(address)")]
    pub struct GetAssetPriceCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getAssetsPrices` function with signature `getAssetsPrices(address[])` and selector `0x9d23d9f2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getAssetsPrices", abi = "getAssetsPrices(address[])")]
    pub struct GetAssetsPricesCall {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getFallbackOracle` function with signature `getFallbackOracle()` and selector `0x6210308c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getFallbackOracle", abi = "getFallbackOracle()")]
    pub struct GetFallbackOracleCall;
    ///Container type for all input parameters for the `getSourceOfAsset` function with signature `getSourceOfAsset(address)` and selector `0x92bf2be0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getSourceOfAsset", abi = "getSourceOfAsset(address)")]
    pub struct GetSourceOfAssetCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setAssetSources` function with signature `setAssetSources(address[],address[])` and selector `0xabfd5310`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setAssetSources", abi = "setAssetSources(address[],address[])")]
    pub struct SetAssetSourcesCall {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub sources: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `setFallbackOracle` function with signature `setFallbackOracle(address)` and selector `0x170aee73`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setFallbackOracle", abi = "setFallbackOracle(address)")]
    pub struct SetFallbackOracleCall {
        pub fallback_oracle: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IAaveOracleCalls {
        BaseCurrency(BaseCurrencyCall),
        BaseCurrencyUnit(BaseCurrencyUnitCall),
        GetAssetPrice(GetAssetPriceCall),
        GetAssetsPrices(GetAssetsPricesCall),
        GetFallbackOracle(GetFallbackOracleCall),
        GetSourceOfAsset(GetSourceOfAssetCall),
        SetAssetSources(SetAssetSourcesCall),
        SetFallbackOracle(SetFallbackOracleCall),
    }
    impl ::ethers::core::abi::AbiDecode for IAaveOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BaseCurrencyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BaseCurrency(decoded));
            }
            if let Ok(decoded)
                = <BaseCurrencyUnitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BaseCurrencyUnit(decoded));
            }
            if let Ok(decoded)
                = <GetAssetPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAssetPrice(decoded));
            }
            if let Ok(decoded)
                = <GetAssetsPricesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAssetsPrices(decoded));
            }
            if let Ok(decoded)
                = <GetFallbackOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetFallbackOracle(decoded));
            }
            if let Ok(decoded)
                = <GetSourceOfAssetCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetSourceOfAsset(decoded));
            }
            if let Ok(decoded)
                = <SetAssetSourcesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetAssetSources(decoded));
            }
            if let Ok(decoded)
                = <SetFallbackOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetFallbackOracle(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IAaveOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BaseCurrency(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseCurrencyUnit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAssetPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAssetsPrices(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFallbackOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSourceOfAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAssetSources(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFallbackOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IAaveOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BaseCurrency(element) => ::core::fmt::Display::fmt(element, f),
                Self::BaseCurrencyUnit(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAssetPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAssetsPrices(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFallbackOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSourceOfAsset(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAssetSources(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFallbackOracle(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BaseCurrencyCall> for IAaveOracleCalls {
        fn from(value: BaseCurrencyCall) -> Self {
            Self::BaseCurrency(value)
        }
    }
    impl ::core::convert::From<BaseCurrencyUnitCall> for IAaveOracleCalls {
        fn from(value: BaseCurrencyUnitCall) -> Self {
            Self::BaseCurrencyUnit(value)
        }
    }
    impl ::core::convert::From<GetAssetPriceCall> for IAaveOracleCalls {
        fn from(value: GetAssetPriceCall) -> Self {
            Self::GetAssetPrice(value)
        }
    }
    impl ::core::convert::From<GetAssetsPricesCall> for IAaveOracleCalls {
        fn from(value: GetAssetsPricesCall) -> Self {
            Self::GetAssetsPrices(value)
        }
    }
    impl ::core::convert::From<GetFallbackOracleCall> for IAaveOracleCalls {
        fn from(value: GetFallbackOracleCall) -> Self {
            Self::GetFallbackOracle(value)
        }
    }
    impl ::core::convert::From<GetSourceOfAssetCall> for IAaveOracleCalls {
        fn from(value: GetSourceOfAssetCall) -> Self {
            Self::GetSourceOfAsset(value)
        }
    }
    impl ::core::convert::From<SetAssetSourcesCall> for IAaveOracleCalls {
        fn from(value: SetAssetSourcesCall) -> Self {
            Self::SetAssetSources(value)
        }
    }
    impl ::core::convert::From<SetFallbackOracleCall> for IAaveOracleCalls {
        fn from(value: SetFallbackOracleCall) -> Self {
            Self::SetFallbackOracle(value)
        }
    }
    ///Container type for all return fields from the `BASE_CURRENCY` function with signature `BASE_CURRENCY()` and selector `0xe19f4700`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BaseCurrencyReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `BASE_CURRENCY_UNIT` function with signature `BASE_CURRENCY_UNIT()` and selector `0x8c89b64f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BaseCurrencyUnitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getAssetPrice` function with signature `getAssetPrice(address)` and selector `0xb3596f07`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetAssetPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getAssetsPrices` function with signature `getAssetsPrices(address[])` and selector `0x9d23d9f2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetAssetsPricesReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `getFallbackOracle` function with signature `getFallbackOracle()` and selector `0x6210308c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetFallbackOracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSourceOfAsset` function with signature `getSourceOfAsset(address)` and selector `0x92bf2be0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetSourceOfAssetReturn(pub ::ethers::core::types::Address);
}
