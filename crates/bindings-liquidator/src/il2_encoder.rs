pub use il2_encoder::*;
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
pub mod il2_encoder {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("encodeBorrowParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("encodeBorrowParams"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interestRateMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeLiquidationCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "encodeLiquidationCall",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtToCover"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiveAToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeRebalanceStableBorrowRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "encodeRebalanceStableBorrowRate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeRepayParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("encodeRepayParams"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interestRateMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeRepayWithATokensParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "encodeRepayWithATokensParams",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interestRateMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeRepayWithPermitParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "encodeRepayWithPermitParams",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interestRateMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permitV"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permitR"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permitS"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                        "encodeSetUserUseReserveAsCollateral",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "encodeSetUserUseReserveAsCollateral",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("useAsCollateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeSupplyParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("encodeSupplyParams"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeSwapBorrowRateMode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "encodeSwapBorrowRateMode",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interestRateMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("encodeWithdrawParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "encodeWithdrawParams",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
    pub static IL2ENCODER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IL2Encoder<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IL2Encoder<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IL2Encoder<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IL2Encoder<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IL2Encoder<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IL2Encoder)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IL2Encoder<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IL2ENCODER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `encodeBorrowParams` (0x1a64acf2) function
        pub fn encode_borrow_params(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            interest_rate_mode: ::ethers::core::types::U256,
            referral_code: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [26, 100, 172, 242],
                    (asset, amount, interest_rate_mode, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeLiquidationCall` (0x88d51852) function
        pub fn encode_liquidation_call(
            &self,
            collateral_asset: ::ethers::core::types::Address,
            debt_asset: ::ethers::core::types::Address,
            user: ::ethers::core::types::Address,
            debt_to_cover: ::ethers::core::types::U256,
            receive_a_token: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ([u8; 32], [u8; 32])> {
            self.0
                .method_hash(
                    [136, 213, 24, 82],
                    (collateral_asset, debt_asset, user, debt_to_cover, receive_a_token),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeRebalanceStableBorrowRate` (0x1a8f6dee) function
        pub fn encode_rebalance_stable_borrow_rate(
            &self,
            asset: ::ethers::core::types::Address,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([26, 143, 109, 238], (asset, user))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeRepayParams` (0x9d2ffc1b) function
        pub fn encode_repay_params(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            interest_rate_mode: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([157, 47, 252, 27], (asset, amount, interest_rate_mode))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeRepayWithATokensParams` (0x8da7fb18) function
        pub fn encode_repay_with_a_tokens_params(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            interest_rate_mode: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([141, 167, 251, 24], (asset, amount, interest_rate_mode))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeRepayWithPermitParams` (0xfed63a93) function
        pub fn encode_repay_with_permit_params(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            interest_rate_mode: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            permit_v: u8,
            permit_r: [u8; 32],
            permit_s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ([u8; 32], [u8; 32], [u8; 32]),
        > {
            self.0
                .method_hash(
                    [254, 214, 58, 147],
                    (
                        asset,
                        amount,
                        interest_rate_mode,
                        deadline,
                        permit_v,
                        permit_r,
                        permit_s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeSetUserUseReserveAsCollateral` (0xfc0eed85) function
        pub fn encode_set_user_use_reserve_as_collateral(
            &self,
            asset: ::ethers::core::types::Address,
            use_as_collateral: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([252, 14, 237, 133], (asset, use_as_collateral))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeSupplyParams` (0xb76398e4) function
        pub fn encode_supply_params(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            referral_code: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([183, 99, 152, 228], (asset, amount, referral_code))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeSwapBorrowRateMode` (0x1fd34797) function
        pub fn encode_swap_borrow_rate_mode(
            &self,
            asset: ::ethers::core::types::Address,
            interest_rate_mode: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([31, 211, 71, 151], (asset, interest_rate_mode))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeWithdrawParams` (0x5cc7bc10) function
        pub fn encode_withdraw_params(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([92, 199, 188, 16], (asset, amount))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IL2Encoder<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `encodeBorrowParams` function with signature `encodeBorrowParams(address,uint256,uint256,uint16)` and selector `0x1a64acf2`
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
        name = "encodeBorrowParams",
        abi = "encodeBorrowParams(address,uint256,uint256,uint16)"
    )]
    pub struct EncodeBorrowParamsCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub interest_rate_mode: ::ethers::core::types::U256,
        pub referral_code: u16,
    }
    ///Container type for all input parameters for the `encodeLiquidationCall` function with signature `encodeLiquidationCall(address,address,address,uint256,bool)` and selector `0x88d51852`
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
        name = "encodeLiquidationCall",
        abi = "encodeLiquidationCall(address,address,address,uint256,bool)"
    )]
    pub struct EncodeLiquidationCallCall {
        pub collateral_asset: ::ethers::core::types::Address,
        pub debt_asset: ::ethers::core::types::Address,
        pub user: ::ethers::core::types::Address,
        pub debt_to_cover: ::ethers::core::types::U256,
        pub receive_a_token: bool,
    }
    ///Container type for all input parameters for the `encodeRebalanceStableBorrowRate` function with signature `encodeRebalanceStableBorrowRate(address,address)` and selector `0x1a8f6dee`
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
        name = "encodeRebalanceStableBorrowRate",
        abi = "encodeRebalanceStableBorrowRate(address,address)"
    )]
    pub struct EncodeRebalanceStableBorrowRateCall {
        pub asset: ::ethers::core::types::Address,
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `encodeRepayParams` function with signature `encodeRepayParams(address,uint256,uint256)` and selector `0x9d2ffc1b`
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
        name = "encodeRepayParams",
        abi = "encodeRepayParams(address,uint256,uint256)"
    )]
    pub struct EncodeRepayParamsCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub interest_rate_mode: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `encodeRepayWithATokensParams` function with signature `encodeRepayWithATokensParams(address,uint256,uint256)` and selector `0x8da7fb18`
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
        name = "encodeRepayWithATokensParams",
        abi = "encodeRepayWithATokensParams(address,uint256,uint256)"
    )]
    pub struct EncodeRepayWithATokensParamsCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub interest_rate_mode: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `encodeRepayWithPermitParams` function with signature `encodeRepayWithPermitParams(address,uint256,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xfed63a93`
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
        name = "encodeRepayWithPermitParams",
        abi = "encodeRepayWithPermitParams(address,uint256,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct EncodeRepayWithPermitParamsCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub interest_rate_mode: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub permit_v: u8,
        pub permit_r: [u8; 32],
        pub permit_s: [u8; 32],
    }
    ///Container type for all input parameters for the `encodeSetUserUseReserveAsCollateral` function with signature `encodeSetUserUseReserveAsCollateral(address,bool)` and selector `0xfc0eed85`
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
        name = "encodeSetUserUseReserveAsCollateral",
        abi = "encodeSetUserUseReserveAsCollateral(address,bool)"
    )]
    pub struct EncodeSetUserUseReserveAsCollateralCall {
        pub asset: ::ethers::core::types::Address,
        pub use_as_collateral: bool,
    }
    ///Container type for all input parameters for the `encodeSupplyParams` function with signature `encodeSupplyParams(address,uint256,uint16)` and selector `0xb76398e4`
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
        name = "encodeSupplyParams",
        abi = "encodeSupplyParams(address,uint256,uint16)"
    )]
    pub struct EncodeSupplyParamsCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub referral_code: u16,
    }
    ///Container type for all input parameters for the `encodeSwapBorrowRateMode` function with signature `encodeSwapBorrowRateMode(address,uint256)` and selector `0x1fd34797`
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
        name = "encodeSwapBorrowRateMode",
        abi = "encodeSwapBorrowRateMode(address,uint256)"
    )]
    pub struct EncodeSwapBorrowRateModeCall {
        pub asset: ::ethers::core::types::Address,
        pub interest_rate_mode: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `encodeWithdrawParams` function with signature `encodeWithdrawParams(address,uint256)` and selector `0x5cc7bc10`
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
        name = "encodeWithdrawParams",
        abi = "encodeWithdrawParams(address,uint256)"
    )]
    pub struct EncodeWithdrawParamsCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IL2EncoderCalls {
        EncodeBorrowParams(EncodeBorrowParamsCall),
        EncodeLiquidationCall(EncodeLiquidationCallCall),
        EncodeRebalanceStableBorrowRate(EncodeRebalanceStableBorrowRateCall),
        EncodeRepayParams(EncodeRepayParamsCall),
        EncodeRepayWithATokensParams(EncodeRepayWithATokensParamsCall),
        EncodeRepayWithPermitParams(EncodeRepayWithPermitParamsCall),
        EncodeSetUserUseReserveAsCollateral(EncodeSetUserUseReserveAsCollateralCall),
        EncodeSupplyParams(EncodeSupplyParamsCall),
        EncodeSwapBorrowRateMode(EncodeSwapBorrowRateModeCall),
        EncodeWithdrawParams(EncodeWithdrawParamsCall),
    }
    impl ::ethers::core::abi::AbiDecode for IL2EncoderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <EncodeBorrowParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EncodeBorrowParams(decoded));
            }
            if let Ok(decoded)
                = <EncodeLiquidationCallCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EncodeLiquidationCall(decoded));
            }
            if let Ok(decoded)
                = <EncodeRebalanceStableBorrowRateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EncodeRebalanceStableBorrowRate(decoded));
            }
            if let Ok(decoded)
                = <EncodeRepayParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EncodeRepayParams(decoded));
            }
            if let Ok(decoded)
                = <EncodeRepayWithATokensParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EncodeRepayWithATokensParams(decoded));
            }
            if let Ok(decoded)
                = <EncodeRepayWithPermitParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EncodeRepayWithPermitParams(decoded));
            }
            if let Ok(decoded)
                = <EncodeSetUserUseReserveAsCollateralCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EncodeSetUserUseReserveAsCollateral(decoded));
            }
            if let Ok(decoded)
                = <EncodeSupplyParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EncodeSupplyParams(decoded));
            }
            if let Ok(decoded)
                = <EncodeSwapBorrowRateModeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EncodeSwapBorrowRateMode(decoded));
            }
            if let Ok(decoded)
                = <EncodeWithdrawParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EncodeWithdrawParams(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IL2EncoderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::EncodeBorrowParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeLiquidationCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeRebalanceStableBorrowRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeRepayParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeRepayWithATokensParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeRepayWithPermitParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeSetUserUseReserveAsCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeSupplyParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeSwapBorrowRateMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeWithdrawParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IL2EncoderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EncodeBorrowParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeLiquidationCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeRebalanceStableBorrowRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeRepayParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncodeRepayWithATokensParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeRepayWithPermitParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeSetUserUseReserveAsCollateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeSupplyParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeSwapBorrowRateMode(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EncodeWithdrawParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<EncodeBorrowParamsCall> for IL2EncoderCalls {
        fn from(value: EncodeBorrowParamsCall) -> Self {
            Self::EncodeBorrowParams(value)
        }
    }
    impl ::core::convert::From<EncodeLiquidationCallCall> for IL2EncoderCalls {
        fn from(value: EncodeLiquidationCallCall) -> Self {
            Self::EncodeLiquidationCall(value)
        }
    }
    impl ::core::convert::From<EncodeRebalanceStableBorrowRateCall> for IL2EncoderCalls {
        fn from(value: EncodeRebalanceStableBorrowRateCall) -> Self {
            Self::EncodeRebalanceStableBorrowRate(value)
        }
    }
    impl ::core::convert::From<EncodeRepayParamsCall> for IL2EncoderCalls {
        fn from(value: EncodeRepayParamsCall) -> Self {
            Self::EncodeRepayParams(value)
        }
    }
    impl ::core::convert::From<EncodeRepayWithATokensParamsCall> for IL2EncoderCalls {
        fn from(value: EncodeRepayWithATokensParamsCall) -> Self {
            Self::EncodeRepayWithATokensParams(value)
        }
    }
    impl ::core::convert::From<EncodeRepayWithPermitParamsCall> for IL2EncoderCalls {
        fn from(value: EncodeRepayWithPermitParamsCall) -> Self {
            Self::EncodeRepayWithPermitParams(value)
        }
    }
    impl ::core::convert::From<EncodeSetUserUseReserveAsCollateralCall>
    for IL2EncoderCalls {
        fn from(value: EncodeSetUserUseReserveAsCollateralCall) -> Self {
            Self::EncodeSetUserUseReserveAsCollateral(value)
        }
    }
    impl ::core::convert::From<EncodeSupplyParamsCall> for IL2EncoderCalls {
        fn from(value: EncodeSupplyParamsCall) -> Self {
            Self::EncodeSupplyParams(value)
        }
    }
    impl ::core::convert::From<EncodeSwapBorrowRateModeCall> for IL2EncoderCalls {
        fn from(value: EncodeSwapBorrowRateModeCall) -> Self {
            Self::EncodeSwapBorrowRateMode(value)
        }
    }
    impl ::core::convert::From<EncodeWithdrawParamsCall> for IL2EncoderCalls {
        fn from(value: EncodeWithdrawParamsCall) -> Self {
            Self::EncodeWithdrawParams(value)
        }
    }
    ///Container type for all return fields from the `encodeBorrowParams` function with signature `encodeBorrowParams(address,uint256,uint256,uint16)` and selector `0x1a64acf2`
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
    pub struct EncodeBorrowParamsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `encodeLiquidationCall` function with signature `encodeLiquidationCall(address,address,address,uint256,bool)` and selector `0x88d51852`
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
    pub struct EncodeLiquidationCallReturn(pub [u8; 32], pub [u8; 32]);
    ///Container type for all return fields from the `encodeRebalanceStableBorrowRate` function with signature `encodeRebalanceStableBorrowRate(address,address)` and selector `0x1a8f6dee`
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
    pub struct EncodeRebalanceStableBorrowRateReturn(pub [u8; 32]);
    ///Container type for all return fields from the `encodeRepayParams` function with signature `encodeRepayParams(address,uint256,uint256)` and selector `0x9d2ffc1b`
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
    pub struct EncodeRepayParamsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `encodeRepayWithATokensParams` function with signature `encodeRepayWithATokensParams(address,uint256,uint256)` and selector `0x8da7fb18`
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
    pub struct EncodeRepayWithATokensParamsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `encodeRepayWithPermitParams` function with signature `encodeRepayWithPermitParams(address,uint256,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xfed63a93`
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
    pub struct EncodeRepayWithPermitParamsReturn(
        pub [u8; 32],
        pub [u8; 32],
        pub [u8; 32],
    );
    ///Container type for all return fields from the `encodeSetUserUseReserveAsCollateral` function with signature `encodeSetUserUseReserveAsCollateral(address,bool)` and selector `0xfc0eed85`
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
    pub struct EncodeSetUserUseReserveAsCollateralReturn(pub [u8; 32]);
    ///Container type for all return fields from the `encodeSupplyParams` function with signature `encodeSupplyParams(address,uint256,uint16)` and selector `0xb76398e4`
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
    pub struct EncodeSupplyParamsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `encodeSwapBorrowRateMode` function with signature `encodeSwapBorrowRateMode(address,uint256)` and selector `0x1fd34797`
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
    pub struct EncodeSwapBorrowRateModeReturn(pub [u8; 32]);
    ///Container type for all return fields from the `encodeWithdrawParams` function with signature `encodeWithdrawParams(address,uint256)` and selector `0x5cc7bc10`
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
    pub struct EncodeWithdrawParamsReturn(pub [u8; 32]);
}
