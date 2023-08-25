pub use i_price_oracle_sentinel::*;
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
pub mod i_price_oracle_sentinel {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ADDRESSES_PROVIDER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ADDRESSES_PROVIDER"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IPoolAddressesProvider",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getGracePeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getGracePeriod"),
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
                    ::std::borrow::ToOwned::to_owned("getSequencerOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSequencerOracle"),
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
                    ::std::borrow::ToOwned::to_owned("isBorrowAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isBorrowAllowed"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isLiquidationAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isLiquidationAllowed",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setGracePeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setGracePeriod"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newGracePeriod"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("setSequencerOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setSequencerOracle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newSequencerOracle",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("GracePeriodUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("GracePeriodUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newGracePeriod"),
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
                    ::std::borrow::ToOwned::to_owned("SequencerOracleUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SequencerOracleUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newSequencerOracle",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
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
    pub static IPRICEORACLESENTINEL_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IPriceOracleSentinel<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IPriceOracleSentinel<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IPriceOracleSentinel<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IPriceOracleSentinel<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IPriceOracleSentinel<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IPriceOracleSentinel))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IPriceOracleSentinel<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IPRICEORACLESENTINEL_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `ADDRESSES_PROVIDER` (0x0542975c) function
        pub fn addresses_provider(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([5, 66, 151, 92], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGracePeriod` (0xdbd18388) function
        pub fn get_grace_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([219, 209, 131, 136], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSequencerOracle` (0x12168dc2) function
        pub fn get_sequencer_oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([18, 22, 141, 194], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isBorrowAllowed` (0x49aa2e81) function
        pub fn is_borrow_allowed(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([73, 170, 46, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isLiquidationAllowed` (0x7a5d20ea) function
        pub fn is_liquidation_allowed(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([122, 93, 32, 234], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGracePeriod` (0xf2f65960) function
        pub fn set_grace_period(
            &self,
            new_grace_period: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 246, 89, 96], new_grace_period)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSequencerOracle` (0xf0aef31c) function
        pub fn set_sequencer_oracle(
            &self,
            new_sequencer_oracle: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 174, 243, 28], new_sequencer_oracle)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `GracePeriodUpdated` event
        pub fn grace_period_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GracePeriodUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SequencerOracleUpdated` event
        pub fn sequencer_oracle_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SequencerOracleUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IPriceOracleSentinelEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IPriceOracleSentinel<M> {
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
    #[ethevent(name = "GracePeriodUpdated", abi = "GracePeriodUpdated(uint256)")]
    pub struct GracePeriodUpdatedFilter {
        pub new_grace_period: ::ethers::core::types::U256,
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
    #[ethevent(name = "SequencerOracleUpdated", abi = "SequencerOracleUpdated(address)")]
    pub struct SequencerOracleUpdatedFilter {
        pub new_sequencer_oracle: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IPriceOracleSentinelEvents {
        GracePeriodUpdatedFilter(GracePeriodUpdatedFilter),
        SequencerOracleUpdatedFilter(SequencerOracleUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IPriceOracleSentinelEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = GracePeriodUpdatedFilter::decode_log(log) {
                return Ok(IPriceOracleSentinelEvents::GracePeriodUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SequencerOracleUpdatedFilter::decode_log(log) {
                return Ok(
                    IPriceOracleSentinelEvents::SequencerOracleUpdatedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IPriceOracleSentinelEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GracePeriodUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SequencerOracleUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GracePeriodUpdatedFilter> for IPriceOracleSentinelEvents {
        fn from(value: GracePeriodUpdatedFilter) -> Self {
            Self::GracePeriodUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<SequencerOracleUpdatedFilter>
    for IPriceOracleSentinelEvents {
        fn from(value: SequencerOracleUpdatedFilter) -> Self {
            Self::SequencerOracleUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `0x0542975c`
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
    #[ethcall(name = "ADDRESSES_PROVIDER", abi = "ADDRESSES_PROVIDER()")]
    pub struct AddressesProviderCall;
    ///Container type for all input parameters for the `getGracePeriod` function with signature `getGracePeriod()` and selector `0xdbd18388`
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
    #[ethcall(name = "getGracePeriod", abi = "getGracePeriod()")]
    pub struct GetGracePeriodCall;
    ///Container type for all input parameters for the `getSequencerOracle` function with signature `getSequencerOracle()` and selector `0x12168dc2`
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
    #[ethcall(name = "getSequencerOracle", abi = "getSequencerOracle()")]
    pub struct GetSequencerOracleCall;
    ///Container type for all input parameters for the `isBorrowAllowed` function with signature `isBorrowAllowed()` and selector `0x49aa2e81`
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
    #[ethcall(name = "isBorrowAllowed", abi = "isBorrowAllowed()")]
    pub struct IsBorrowAllowedCall;
    ///Container type for all input parameters for the `isLiquidationAllowed` function with signature `isLiquidationAllowed()` and selector `0x7a5d20ea`
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
    #[ethcall(name = "isLiquidationAllowed", abi = "isLiquidationAllowed()")]
    pub struct IsLiquidationAllowedCall;
    ///Container type for all input parameters for the `setGracePeriod` function with signature `setGracePeriod(uint256)` and selector `0xf2f65960`
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
    #[ethcall(name = "setGracePeriod", abi = "setGracePeriod(uint256)")]
    pub struct SetGracePeriodCall {
        pub new_grace_period: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setSequencerOracle` function with signature `setSequencerOracle(address)` and selector `0xf0aef31c`
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
    #[ethcall(name = "setSequencerOracle", abi = "setSequencerOracle(address)")]
    pub struct SetSequencerOracleCall {
        pub new_sequencer_oracle: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IPriceOracleSentinelCalls {
        AddressesProvider(AddressesProviderCall),
        GetGracePeriod(GetGracePeriodCall),
        GetSequencerOracle(GetSequencerOracleCall),
        IsBorrowAllowed(IsBorrowAllowedCall),
        IsLiquidationAllowed(IsLiquidationAllowedCall),
        SetGracePeriod(SetGracePeriodCall),
        SetSequencerOracle(SetSequencerOracleCall),
    }
    impl ::ethers::core::abi::AbiDecode for IPriceOracleSentinelCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddressesProviderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddressesProvider(decoded));
            }
            if let Ok(decoded)
                = <GetGracePeriodCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetGracePeriod(decoded));
            }
            if let Ok(decoded)
                = <GetSequencerOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetSequencerOracle(decoded));
            }
            if let Ok(decoded)
                = <IsBorrowAllowedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsBorrowAllowed(decoded));
            }
            if let Ok(decoded)
                = <IsLiquidationAllowedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsLiquidationAllowed(decoded));
            }
            if let Ok(decoded)
                = <SetGracePeriodCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetGracePeriod(decoded));
            }
            if let Ok(decoded)
                = <SetSequencerOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetSequencerOracle(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IPriceOracleSentinelCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressesProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGracePeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSequencerOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsBorrowAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsLiquidationAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGracePeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSequencerOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IPriceOracleSentinelCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressesProvider(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetGracePeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSequencerOracle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsBorrowAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsLiquidationAllowed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetGracePeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSequencerOracle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddressesProviderCall> for IPriceOracleSentinelCalls {
        fn from(value: AddressesProviderCall) -> Self {
            Self::AddressesProvider(value)
        }
    }
    impl ::core::convert::From<GetGracePeriodCall> for IPriceOracleSentinelCalls {
        fn from(value: GetGracePeriodCall) -> Self {
            Self::GetGracePeriod(value)
        }
    }
    impl ::core::convert::From<GetSequencerOracleCall> for IPriceOracleSentinelCalls {
        fn from(value: GetSequencerOracleCall) -> Self {
            Self::GetSequencerOracle(value)
        }
    }
    impl ::core::convert::From<IsBorrowAllowedCall> for IPriceOracleSentinelCalls {
        fn from(value: IsBorrowAllowedCall) -> Self {
            Self::IsBorrowAllowed(value)
        }
    }
    impl ::core::convert::From<IsLiquidationAllowedCall> for IPriceOracleSentinelCalls {
        fn from(value: IsLiquidationAllowedCall) -> Self {
            Self::IsLiquidationAllowed(value)
        }
    }
    impl ::core::convert::From<SetGracePeriodCall> for IPriceOracleSentinelCalls {
        fn from(value: SetGracePeriodCall) -> Self {
            Self::SetGracePeriod(value)
        }
    }
    impl ::core::convert::From<SetSequencerOracleCall> for IPriceOracleSentinelCalls {
        fn from(value: SetSequencerOracleCall) -> Self {
            Self::SetSequencerOracle(value)
        }
    }
    ///Container type for all return fields from the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `0x0542975c`
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
    pub struct AddressesProviderReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getGracePeriod` function with signature `getGracePeriod()` and selector `0xdbd18388`
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
    pub struct GetGracePeriodReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getSequencerOracle` function with signature `getSequencerOracle()` and selector `0x12168dc2`
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
    pub struct GetSequencerOracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isBorrowAllowed` function with signature `isBorrowAllowed()` and selector `0x49aa2e81`
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
    pub struct IsBorrowAllowedReturn(pub bool);
    ///Container type for all return fields from the `isLiquidationAllowed` function with signature `isLiquidationAllowed()` and selector `0x7a5d20ea`
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
    pub struct IsLiquidationAllowedReturn(pub bool);
}
