pub use i_default_interest_rate_strategy::*;
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
pub mod i_default_interest_rate_strategy {
    pub use super::super::shared_types::*;
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
                    ::std::borrow::ToOwned::to_owned(
                        "MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("MAX_EXCESS_USAGE_RATIO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_EXCESS_USAGE_RATIO",
                            ),
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
                    ::std::borrow::ToOwned::to_owned(
                        "OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("OPTIMAL_USAGE_RATIO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OPTIMAL_USAGE_RATIO",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("calculateInterestRates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateInterestRates",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DataTypes.CalculateInterestRatesParams",
                                        ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("getBaseStableBorrowRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getBaseStableBorrowRate",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getBaseVariableBorrowRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getBaseVariableBorrowRate",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getMaxVariableBorrowRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getMaxVariableBorrowRate",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getStableRateExcessOffset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStableRateExcessOffset",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getStableRateSlope1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStableRateSlope1",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getStableRateSlope2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStableRateSlope2",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getVariableRateSlope1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getVariableRateSlope1",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getVariableRateSlope2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getVariableRateSlope2",
                            ),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IDEFAULTINTERESTRATESTRATEGY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IDefaultInterestRateStrategy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IDefaultInterestRateStrategy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IDefaultInterestRateStrategy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IDefaultInterestRateStrategy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IDefaultInterestRateStrategy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IDefaultInterestRateStrategy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IDefaultInterestRateStrategy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IDEFAULTINTERESTRATESTRATEGY_ABI.clone(),
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
        ///Calls the contract's `MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO` (0xfe5fd698) function
        pub fn max_excess_stable_to_total_debt_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([254, 95, 214, 152], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_EXCESS_USAGE_RATIO` (0xa9c622f8) function
        pub fn max_excess_usage_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([169, 198, 34, 248], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO` (0x6fb92589) function
        pub fn optimal_stable_to_total_debt_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([111, 185, 37, 137], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OPTIMAL_USAGE_RATIO` (0x54c365c6) function
        pub fn optimal_usage_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 195, 101, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateInterestRates` (0xa5898709) function
        pub fn calculate_interest_rates(
            &self,
            params: CalculateInterestRatesParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([165, 137, 135, 9], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBaseStableBorrowRate` (0xacd78686) function
        pub fn get_base_stable_borrow_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([172, 215, 134, 134], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBaseVariableBorrowRate` (0x34762ca5) function
        pub fn get_base_variable_borrow_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 118, 44, 165], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxVariableBorrowRate` (0x80031e37) function
        pub fn get_max_variable_borrow_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([128, 3, 30, 55], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStableRateExcessOffset` (0xbc626908) function
        pub fn get_stable_rate_excess_offset(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([188, 98, 105, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStableRateSlope1` (0xd5cd7391) function
        pub fn get_stable_rate_slope_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([213, 205, 115, 145], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStableRateSlope2` (0x14e32da4) function
        pub fn get_stable_rate_slope_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([20, 227, 45, 164], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVariableRateSlope1` (0x0b3429a2) function
        pub fn get_variable_rate_slope_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([11, 52, 41, 162], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVariableRateSlope2` (0xf4202409) function
        pub fn get_variable_rate_slope_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([244, 32, 36, 9], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IDefaultInterestRateStrategy<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    ///Container type for all input parameters for the `MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO` function with signature `MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO()` and selector `0xfe5fd698`
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
    #[ethcall(
        name = "MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO",
        abi = "MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO()"
    )]
    pub struct MaxExcessStableToTotalDebtRatioCall;
    ///Container type for all input parameters for the `MAX_EXCESS_USAGE_RATIO` function with signature `MAX_EXCESS_USAGE_RATIO()` and selector `0xa9c622f8`
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
    #[ethcall(name = "MAX_EXCESS_USAGE_RATIO", abi = "MAX_EXCESS_USAGE_RATIO()")]
    pub struct MaxExcessUsageRatioCall;
    ///Container type for all input parameters for the `OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO` function with signature `OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO()` and selector `0x6fb92589`
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
    #[ethcall(
        name = "OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO",
        abi = "OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO()"
    )]
    pub struct OptimalStableToTotalDebtRatioCall;
    ///Container type for all input parameters for the `OPTIMAL_USAGE_RATIO` function with signature `OPTIMAL_USAGE_RATIO()` and selector `0x54c365c6`
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
    #[ethcall(name = "OPTIMAL_USAGE_RATIO", abi = "OPTIMAL_USAGE_RATIO()")]
    pub struct OptimalUsageRatioCall;
    ///Container type for all input parameters for the `calculateInterestRates` function with signature `calculateInterestRates((uint256,uint256,uint256,uint256,uint256,uint256,uint256,address,address))` and selector `0xa5898709`
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
    #[ethcall(
        name = "calculateInterestRates",
        abi = "calculateInterestRates((uint256,uint256,uint256,uint256,uint256,uint256,uint256,address,address))"
    )]
    pub struct CalculateInterestRatesCall {
        pub params: CalculateInterestRatesParams,
    }
    ///Container type for all input parameters for the `getBaseStableBorrowRate` function with signature `getBaseStableBorrowRate()` and selector `0xacd78686`
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
    #[ethcall(name = "getBaseStableBorrowRate", abi = "getBaseStableBorrowRate()")]
    pub struct GetBaseStableBorrowRateCall;
    ///Container type for all input parameters for the `getBaseVariableBorrowRate` function with signature `getBaseVariableBorrowRate()` and selector `0x34762ca5`
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
    #[ethcall(name = "getBaseVariableBorrowRate", abi = "getBaseVariableBorrowRate()")]
    pub struct GetBaseVariableBorrowRateCall;
    ///Container type for all input parameters for the `getMaxVariableBorrowRate` function with signature `getMaxVariableBorrowRate()` and selector `0x80031e37`
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
    #[ethcall(name = "getMaxVariableBorrowRate", abi = "getMaxVariableBorrowRate()")]
    pub struct GetMaxVariableBorrowRateCall;
    ///Container type for all input parameters for the `getStableRateExcessOffset` function with signature `getStableRateExcessOffset()` and selector `0xbc626908`
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
    #[ethcall(name = "getStableRateExcessOffset", abi = "getStableRateExcessOffset()")]
    pub struct GetStableRateExcessOffsetCall;
    ///Container type for all input parameters for the `getStableRateSlope1` function with signature `getStableRateSlope1()` and selector `0xd5cd7391`
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
    #[ethcall(name = "getStableRateSlope1", abi = "getStableRateSlope1()")]
    pub struct GetStableRateSlope1Call;
    ///Container type for all input parameters for the `getStableRateSlope2` function with signature `getStableRateSlope2()` and selector `0x14e32da4`
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
    #[ethcall(name = "getStableRateSlope2", abi = "getStableRateSlope2()")]
    pub struct GetStableRateSlope2Call;
    ///Container type for all input parameters for the `getVariableRateSlope1` function with signature `getVariableRateSlope1()` and selector `0x0b3429a2`
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
    #[ethcall(name = "getVariableRateSlope1", abi = "getVariableRateSlope1()")]
    pub struct GetVariableRateSlope1Call;
    ///Container type for all input parameters for the `getVariableRateSlope2` function with signature `getVariableRateSlope2()` and selector `0xf4202409`
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
    #[ethcall(name = "getVariableRateSlope2", abi = "getVariableRateSlope2()")]
    pub struct GetVariableRateSlope2Call;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IDefaultInterestRateStrategyCalls {
        AddressesProvider(AddressesProviderCall),
        MaxExcessStableToTotalDebtRatio(MaxExcessStableToTotalDebtRatioCall),
        MaxExcessUsageRatio(MaxExcessUsageRatioCall),
        OptimalStableToTotalDebtRatio(OptimalStableToTotalDebtRatioCall),
        OptimalUsageRatio(OptimalUsageRatioCall),
        CalculateInterestRates(CalculateInterestRatesCall),
        GetBaseStableBorrowRate(GetBaseStableBorrowRateCall),
        GetBaseVariableBorrowRate(GetBaseVariableBorrowRateCall),
        GetMaxVariableBorrowRate(GetMaxVariableBorrowRateCall),
        GetStableRateExcessOffset(GetStableRateExcessOffsetCall),
        GetStableRateSlope1(GetStableRateSlope1Call),
        GetStableRateSlope2(GetStableRateSlope2Call),
        GetVariableRateSlope1(GetVariableRateSlope1Call),
        GetVariableRateSlope2(GetVariableRateSlope2Call),
    }
    impl ::ethers::core::abi::AbiDecode for IDefaultInterestRateStrategyCalls {
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
                = <MaxExcessStableToTotalDebtRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxExcessStableToTotalDebtRatio(decoded));
            }
            if let Ok(decoded)
                = <MaxExcessUsageRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxExcessUsageRatio(decoded));
            }
            if let Ok(decoded)
                = <OptimalStableToTotalDebtRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OptimalStableToTotalDebtRatio(decoded));
            }
            if let Ok(decoded)
                = <OptimalUsageRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OptimalUsageRatio(decoded));
            }
            if let Ok(decoded)
                = <CalculateInterestRatesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CalculateInterestRates(decoded));
            }
            if let Ok(decoded)
                = <GetBaseStableBorrowRateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetBaseStableBorrowRate(decoded));
            }
            if let Ok(decoded)
                = <GetBaseVariableBorrowRateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetBaseVariableBorrowRate(decoded));
            }
            if let Ok(decoded)
                = <GetMaxVariableBorrowRateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetMaxVariableBorrowRate(decoded));
            }
            if let Ok(decoded)
                = <GetStableRateExcessOffsetCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetStableRateExcessOffset(decoded));
            }
            if let Ok(decoded)
                = <GetStableRateSlope1Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetStableRateSlope1(decoded));
            }
            if let Ok(decoded)
                = <GetStableRateSlope2Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetStableRateSlope2(decoded));
            }
            if let Ok(decoded)
                = <GetVariableRateSlope1Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetVariableRateSlope1(decoded));
            }
            if let Ok(decoded)
                = <GetVariableRateSlope2Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetVariableRateSlope2(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IDefaultInterestRateStrategyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressesProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxExcessStableToTotalDebtRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxExcessUsageRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptimalStableToTotalDebtRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptimalUsageRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateInterestRates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBaseStableBorrowRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBaseVariableBorrowRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxVariableBorrowRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStableRateExcessOffset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStableRateSlope1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStableRateSlope2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVariableRateSlope1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVariableRateSlope2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IDefaultInterestRateStrategyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressesProvider(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxExcessStableToTotalDebtRatio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxExcessUsageRatio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptimalStableToTotalDebtRatio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptimalUsageRatio(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateInterestRates(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetBaseStableBorrowRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetBaseVariableBorrowRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMaxVariableBorrowRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStableRateExcessOffset(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStableRateSlope1(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStableRateSlope2(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetVariableRateSlope1(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetVariableRateSlope2(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddressesProviderCall>
    for IDefaultInterestRateStrategyCalls {
        fn from(value: AddressesProviderCall) -> Self {
            Self::AddressesProvider(value)
        }
    }
    impl ::core::convert::From<MaxExcessStableToTotalDebtRatioCall>
    for IDefaultInterestRateStrategyCalls {
        fn from(value: MaxExcessStableToTotalDebtRatioCall) -> Self {
            Self::MaxExcessStableToTotalDebtRatio(value)
        }
    }
    impl ::core::convert::From<MaxExcessUsageRatioCall>
    for IDefaultInterestRateStrategyCalls {
        fn from(value: MaxExcessUsageRatioCall) -> Self {
            Self::MaxExcessUsageRatio(value)
        }
    }
    impl ::core::convert::From<OptimalStableToTotalDebtRatioCall>
    for IDefaultInterestRateStrategyCalls {
        fn from(value: OptimalStableToTotalDebtRatioCall) -> Self {
            Self::OptimalStableToTotalDebtRatio(value)
        }
    }
    impl ::core::convert::From<OptimalUsageRatioCall>
    for IDefaultInterestRateStrategyCalls {
        fn from(value: OptimalUsageRatioCall) -> Self {
            Self::OptimalUsageRatio(value)
        }
    }
    impl ::core::convert::From<CalculateInterestRatesCall>
    for IDefaultInterestRateStrategyCalls {
        fn from(value: CalculateInterestRatesCall) -> Self {
            Self::CalculateInterestRates(value)
        }
    }
    impl ::core::convert::From<GetBaseStableBorrowRateCall>
    for IDefaultInterestRateStrategyCalls {
        fn from(value: GetBaseStableBorrowRateCall) -> Self {
            Self::GetBaseStableBorrowRate(value)
        }
    }
    impl ::core::convert::From<GetBaseVariableBorrowRateCall>
    for IDefaultInterestRateStrategyCalls {
        fn from(value: GetBaseVariableBorrowRateCall) -> Self {
            Self::GetBaseVariableBorrowRate(value)
        }
    }
    impl ::core::convert::From<GetMaxVariableBorrowRateCall>
    for IDefaultInterestRateStrategyCalls {
        fn from(value: GetMaxVariableBorrowRateCall) -> Self {
            Self::GetMaxVariableBorrowRate(value)
        }
    }
    impl ::core::convert::From<GetStableRateExcessOffsetCall>
    for IDefaultInterestRateStrategyCalls {
        fn from(value: GetStableRateExcessOffsetCall) -> Self {
            Self::GetStableRateExcessOffset(value)
        }
    }
    impl ::core::convert::From<GetStableRateSlope1Call>
    for IDefaultInterestRateStrategyCalls {
        fn from(value: GetStableRateSlope1Call) -> Self {
            Self::GetStableRateSlope1(value)
        }
    }
    impl ::core::convert::From<GetStableRateSlope2Call>
    for IDefaultInterestRateStrategyCalls {
        fn from(value: GetStableRateSlope2Call) -> Self {
            Self::GetStableRateSlope2(value)
        }
    }
    impl ::core::convert::From<GetVariableRateSlope1Call>
    for IDefaultInterestRateStrategyCalls {
        fn from(value: GetVariableRateSlope1Call) -> Self {
            Self::GetVariableRateSlope1(value)
        }
    }
    impl ::core::convert::From<GetVariableRateSlope2Call>
    for IDefaultInterestRateStrategyCalls {
        fn from(value: GetVariableRateSlope2Call) -> Self {
            Self::GetVariableRateSlope2(value)
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
    ///Container type for all return fields from the `MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO` function with signature `MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO()` and selector `0xfe5fd698`
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
    pub struct MaxExcessStableToTotalDebtRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_EXCESS_USAGE_RATIO` function with signature `MAX_EXCESS_USAGE_RATIO()` and selector `0xa9c622f8`
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
    pub struct MaxExcessUsageRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO` function with signature `OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO()` and selector `0x6fb92589`
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
    pub struct OptimalStableToTotalDebtRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `OPTIMAL_USAGE_RATIO` function with signature `OPTIMAL_USAGE_RATIO()` and selector `0x54c365c6`
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
    pub struct OptimalUsageRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateInterestRates` function with signature `calculateInterestRates((uint256,uint256,uint256,uint256,uint256,uint256,uint256,address,address))` and selector `0xa5898709`
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
    pub struct CalculateInterestRatesReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getBaseStableBorrowRate` function with signature `getBaseStableBorrowRate()` and selector `0xacd78686`
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
    pub struct GetBaseStableBorrowRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getBaseVariableBorrowRate` function with signature `getBaseVariableBorrowRate()` and selector `0x34762ca5`
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
    pub struct GetBaseVariableBorrowRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getMaxVariableBorrowRate` function with signature `getMaxVariableBorrowRate()` and selector `0x80031e37`
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
    pub struct GetMaxVariableBorrowRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getStableRateExcessOffset` function with signature `getStableRateExcessOffset()` and selector `0xbc626908`
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
    pub struct GetStableRateExcessOffsetReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getStableRateSlope1` function with signature `getStableRateSlope1()` and selector `0xd5cd7391`
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
    pub struct GetStableRateSlope1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getStableRateSlope2` function with signature `getStableRateSlope2()` and selector `0x14e32da4`
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
    pub struct GetStableRateSlope2Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVariableRateSlope1` function with signature `getVariableRateSlope1()` and selector `0x0b3429a2`
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
    pub struct GetVariableRateSlope1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVariableRateSlope2` function with signature `getVariableRateSlope2()` and selector `0xf4202409`
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
    pub struct GetVariableRateSlope2Return(pub ::ethers::core::types::U256);
}
