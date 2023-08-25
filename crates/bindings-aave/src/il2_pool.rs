pub use il2_pool::*;
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
pub mod il2_pool {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("borrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("borrow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("liquidationCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidationCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("rebalanceStableBorrowRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "rebalanceStableBorrowRate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("repay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("repayWithATokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repayWithATokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("repayWithPermit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repayWithPermit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setUserUseReserveAsCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setUserUseReserveAsCollateral",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("supply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supply"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("supplyWithPermit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supplyWithPermit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("swapBorrowRateMode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapBorrowRateMode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
    pub static IL2POOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IL2Pool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IL2Pool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IL2Pool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IL2Pool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IL2Pool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IL2Pool)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IL2Pool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IL2POOL_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `borrow` (0xd5eed868) function
        pub fn borrow(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 238, 216, 104], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidationCall` (0xfd21ecff) function
        pub fn liquidation_call(
            &self,
            args_1: [u8; 32],
            args_2: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 33, 236, 255], (args_1, args_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebalanceStableBorrowRate` (0x427da177) function
        pub fn rebalance_stable_borrow_rate(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 125, 161, 119], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repay` (0x563dd613) function
        pub fn repay(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([86, 61, 214, 19], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repayWithATokens` (0xdc7c0bff) function
        pub fn repay_with_a_tokens(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([220, 124, 11, 255], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repayWithPermit` (0x94b576de) function
        pub fn repay_with_permit(
            &self,
            args: [u8; 32],
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([148, 181, 118, 222], (args, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUserUseReserveAsCollateral` (0x4d013f03) function
        pub fn set_user_use_reserve_as_collateral(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 1, 63, 3], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supply` (0xf7a73840) function
        pub fn supply(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 167, 56, 64], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supplyWithPermit` (0x680dd47c) function
        pub fn supply_with_permit(
            &self,
            args: [u8; 32],
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 13, 212, 124], (args, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapBorrowRateMode` (0x1fe3c6f3) function
        pub fn swap_borrow_rate_mode(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 227, 198, 243], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x8e19899e) function
        pub fn withdraw(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([142, 25, 137, 158], args)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IL2Pool<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `borrow` function with signature `borrow(bytes32)` and selector `0xd5eed868`
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
    #[ethcall(name = "borrow", abi = "borrow(bytes32)")]
    pub struct BorrowCall {
        pub args: [u8; 32],
    }
    ///Container type for all input parameters for the `liquidationCall` function with signature `liquidationCall(bytes32,bytes32)` and selector `0xfd21ecff`
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
    #[ethcall(name = "liquidationCall", abi = "liquidationCall(bytes32,bytes32)")]
    pub struct LiquidationCallCall {
        pub args_1: [u8; 32],
        pub args_2: [u8; 32],
    }
    ///Container type for all input parameters for the `rebalanceStableBorrowRate` function with signature `rebalanceStableBorrowRate(bytes32)` and selector `0x427da177`
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
        name = "rebalanceStableBorrowRate",
        abi = "rebalanceStableBorrowRate(bytes32)"
    )]
    pub struct RebalanceStableBorrowRateCall {
        pub args: [u8; 32],
    }
    ///Container type for all input parameters for the `repay` function with signature `repay(bytes32)` and selector `0x563dd613`
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
    #[ethcall(name = "repay", abi = "repay(bytes32)")]
    pub struct RepayCall {
        pub args: [u8; 32],
    }
    ///Container type for all input parameters for the `repayWithATokens` function with signature `repayWithATokens(bytes32)` and selector `0xdc7c0bff`
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
    #[ethcall(name = "repayWithATokens", abi = "repayWithATokens(bytes32)")]
    pub struct RepayWithATokensCall {
        pub args: [u8; 32],
    }
    ///Container type for all input parameters for the `repayWithPermit` function with signature `repayWithPermit(bytes32,bytes32,bytes32)` and selector `0x94b576de`
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
        name = "repayWithPermit",
        abi = "repayWithPermit(bytes32,bytes32,bytes32)"
    )]
    pub struct RepayWithPermitCall {
        pub args: [u8; 32],
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `setUserUseReserveAsCollateral` function with signature `setUserUseReserveAsCollateral(bytes32)` and selector `0x4d013f03`
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
        name = "setUserUseReserveAsCollateral",
        abi = "setUserUseReserveAsCollateral(bytes32)"
    )]
    pub struct SetUserUseReserveAsCollateralCall {
        pub args: [u8; 32],
    }
    ///Container type for all input parameters for the `supply` function with signature `supply(bytes32)` and selector `0xf7a73840`
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
    #[ethcall(name = "supply", abi = "supply(bytes32)")]
    pub struct SupplyCall {
        pub args: [u8; 32],
    }
    ///Container type for all input parameters for the `supplyWithPermit` function with signature `supplyWithPermit(bytes32,bytes32,bytes32)` and selector `0x680dd47c`
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
        name = "supplyWithPermit",
        abi = "supplyWithPermit(bytes32,bytes32,bytes32)"
    )]
    pub struct SupplyWithPermitCall {
        pub args: [u8; 32],
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `swapBorrowRateMode` function with signature `swapBorrowRateMode(bytes32)` and selector `0x1fe3c6f3`
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
    #[ethcall(name = "swapBorrowRateMode", abi = "swapBorrowRateMode(bytes32)")]
    pub struct SwapBorrowRateModeCall {
        pub args: [u8; 32],
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(bytes32)` and selector `0x8e19899e`
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
    #[ethcall(name = "withdraw", abi = "withdraw(bytes32)")]
    pub struct WithdrawCall {
        pub args: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IL2PoolCalls {
        Borrow(BorrowCall),
        LiquidationCall(LiquidationCallCall),
        RebalanceStableBorrowRate(RebalanceStableBorrowRateCall),
        Repay(RepayCall),
        RepayWithATokens(RepayWithATokensCall),
        RepayWithPermit(RepayWithPermitCall),
        SetUserUseReserveAsCollateral(SetUserUseReserveAsCollateralCall),
        Supply(SupplyCall),
        SupplyWithPermit(SupplyWithPermitCall),
        SwapBorrowRateMode(SwapBorrowRateModeCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for IL2PoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BorrowCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Borrow(decoded));
            }
            if let Ok(decoded)
                = <LiquidationCallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LiquidationCall(decoded));
            }
            if let Ok(decoded)
                = <RebalanceStableBorrowRateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RebalanceStableBorrowRate(decoded));
            }
            if let Ok(decoded)
                = <RepayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Repay(decoded));
            }
            if let Ok(decoded)
                = <RepayWithATokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RepayWithATokens(decoded));
            }
            if let Ok(decoded)
                = <RepayWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RepayWithPermit(decoded));
            }
            if let Ok(decoded)
                = <SetUserUseReserveAsCollateralCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetUserUseReserveAsCollateral(decoded));
            }
            if let Ok(decoded)
                = <SupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Supply(decoded));
            }
            if let Ok(decoded)
                = <SupplyWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupplyWithPermit(decoded));
            }
            if let Ok(decoded)
                = <SwapBorrowRateModeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapBorrowRateMode(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IL2PoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Borrow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LiquidationCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceStableBorrowRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Repay(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RepayWithATokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RepayWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUserUseReserveAsCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Supply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupplyWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapBorrowRateMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IL2PoolCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Borrow(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidationCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceStableBorrowRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Repay(element) => ::core::fmt::Display::fmt(element, f),
                Self::RepayWithATokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::RepayWithPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUserUseReserveAsCollateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Supply(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplyWithPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapBorrowRateMode(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BorrowCall> for IL2PoolCalls {
        fn from(value: BorrowCall) -> Self {
            Self::Borrow(value)
        }
    }
    impl ::core::convert::From<LiquidationCallCall> for IL2PoolCalls {
        fn from(value: LiquidationCallCall) -> Self {
            Self::LiquidationCall(value)
        }
    }
    impl ::core::convert::From<RebalanceStableBorrowRateCall> for IL2PoolCalls {
        fn from(value: RebalanceStableBorrowRateCall) -> Self {
            Self::RebalanceStableBorrowRate(value)
        }
    }
    impl ::core::convert::From<RepayCall> for IL2PoolCalls {
        fn from(value: RepayCall) -> Self {
            Self::Repay(value)
        }
    }
    impl ::core::convert::From<RepayWithATokensCall> for IL2PoolCalls {
        fn from(value: RepayWithATokensCall) -> Self {
            Self::RepayWithATokens(value)
        }
    }
    impl ::core::convert::From<RepayWithPermitCall> for IL2PoolCalls {
        fn from(value: RepayWithPermitCall) -> Self {
            Self::RepayWithPermit(value)
        }
    }
    impl ::core::convert::From<SetUserUseReserveAsCollateralCall> for IL2PoolCalls {
        fn from(value: SetUserUseReserveAsCollateralCall) -> Self {
            Self::SetUserUseReserveAsCollateral(value)
        }
    }
    impl ::core::convert::From<SupplyCall> for IL2PoolCalls {
        fn from(value: SupplyCall) -> Self {
            Self::Supply(value)
        }
    }
    impl ::core::convert::From<SupplyWithPermitCall> for IL2PoolCalls {
        fn from(value: SupplyWithPermitCall) -> Self {
            Self::SupplyWithPermit(value)
        }
    }
    impl ::core::convert::From<SwapBorrowRateModeCall> for IL2PoolCalls {
        fn from(value: SwapBorrowRateModeCall) -> Self {
            Self::SwapBorrowRateMode(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for IL2PoolCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `repay` function with signature `repay(bytes32)` and selector `0x563dd613`
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
    pub struct RepayReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `repayWithATokens` function with signature `repayWithATokens(bytes32)` and selector `0xdc7c0bff`
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
    pub struct RepayWithATokensReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `repayWithPermit` function with signature `repayWithPermit(bytes32,bytes32,bytes32)` and selector `0x94b576de`
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
    pub struct RepayWithPermitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `withdraw` function with signature `withdraw(bytes32)` and selector `0x8e19899e`
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
    pub struct WithdrawReturn(pub ::ethers::core::types::U256);
}
