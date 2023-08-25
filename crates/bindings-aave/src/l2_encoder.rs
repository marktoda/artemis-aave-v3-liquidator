pub use l2_encoder::*;
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
pub mod l2_encoder {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("pool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IPool"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("POOL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("POOL"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("encodeSupplyWithPermitParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "encodeSupplyWithPermitParams",
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
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
    pub static L2ENCODER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x11\x118\x03\x80a\x11\x11\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x10@a\0\xD1`\09`\0\x81\x81a\x01K\x01R\x81\x81a\x02+\x01R\x81\x81a\x02\xFD\x01R\x81\x81a\x03\xA0\x01R\x81\x81a\x04S\x01R\x81\x81a\x05%\x01R\x81\x81a\x06\r\x01R\x81\x81a\x06\xA7\x01R\x81\x81a\x07\x9F\x01R\x81\x81a\x08\x81\x01Ra\t@\x01Ra\x10@`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB4W`\x005`\xE0\x1C\x80c\x88\xD5\x18R\x11a\0qW\x80c\x88\xD5\x18R\x14a\x01\x85W\x80c\x8D\xA7\xFB\x18\x14a\x01\xADW\x80c\x9D/\xFC\x1B\x14a\x01\xC0W\x80c\xB7c\x98\xE4\x14a\x01\xD3W\x80c\xFC\x0E\xED\x85\x14a\x01\xE6W\x80c\xFE\xD6:\x93\x14a\x01\xF4W`\0\x80\xFD[\x80c\x1Ad\xAC\xF2\x14a\0\xB9W\x80c\x1A\x8Fm\xEE\x14a\0\xDFW\x80c\x1F\xD3G\x97\x14a\0\xF2W\x80c\\\xC7\xBC\x10\x14a\x01\x05W\x80cg\x1A\x7F\xAE\x14a\x01\x18W\x80cu5\xD2F\x14a\x01FW[`\0\x80\xFD[a\0\xCCa\0\xC76`\x04a\x0B\x81V[a\x02\x07V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCCa\0\xED6`\x04a\x0B\xCBV[a\x02\xD9V[a\0\xCCa\x01\x006`\x04a\x0C\x04V[a\x03|V[a\0\xCCa\x01\x136`\x04a\x0C\x04V[a\x04/V[a\x01+a\x01&6`\x04a\x0CFV[a\x04\xFDV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\0\xD6V[a\x01m\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xD6V[a\x01\x98a\x01\x936`\x04a\x0C\xC4V[a\x05\xE7V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xD6V[a\0\xCCa\x01\xBB6`\x04a\r(V[a\x07fV[a\0\xCCa\x01\xCE6`\x04a\r(V[a\x07{V[a\0\xCCa\x01\xE16`\x04a\r]V[a\x08]V[a\0\xCCa\0\xED6`\x04a\r\x9FV[a\x01+a\x02\x026`\x04a\r\xD4V[a\t\x18V[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x97\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0a\x02\xAA\x87a\n!V[\x90P`\0a\x02\xB7\x87a\n\x93V[`\x10\x92\x90\x92\x1B`\x90\x92\x90\x92\x1B`\x98\x96\x90\x96\x1B\x95\x90\x95\x01\x01\x01\x96\x95PPPPPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03i\x91\x90a\x0E\xE7V[`\xE0\x01Q`\x10\x84\x90\x1B\x01\x91PP\x92\x91PPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x0C\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0a\x04\x1F\x85a\n\x93V[`\x10\x1B\x91\x90\x91\x01\x95\x94PPPPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xBF\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0`\0\x19\x85\x14a\x04\xE0Wa\x04\xDB\x85a\n!V[a\x04\x1FV[Pq\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x01\x94\x93PPPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x82\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x91\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0a\x05\xA4\x8Ca\n!V[\x90P`\0a\x05\xB1\x8Ba\n\xF4V[\x90P`\0\x8A`\xC0\x1B\x82`\xA0\x1B\x01\x8D`\x90\x1B\x01\x83`\x10\x1B\x01\x84\x01\x90P\x80\x8A\x8A\x97P\x97P\x97PPPPPP\x97P\x97P\x97\x94PPPPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06{\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x92\x93P\x90\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x15\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0`\0\x19\x89\x14a\x076Wa\x071\x89a\n!V[a\x07?V[`\x01`\x01`\x80\x1B\x03[`\x10\x92\x90\x92\x1B\x93\x90\x93\x01` \x8A\x90\x1B\x01\x95P`\x80\x87\x90\x1B\x01\x93PPPP\x95P\x95\x93PPPPV[`\0a\x07s\x84\x84\x84a\x07{V[\x94\x93PPPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x0B\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0`\0\x19\x86\x14a\x08,Wa\x08'\x86a\n!V[a\x085V[`\x01`\x01`\x80\x1B\x03[\x90P`\0a\x08B\x86a\n\x93V[`\x90\x1B`\x10\x92\x90\x92\x1B\x91\x90\x91\x01\x91\x90\x91\x01\x96\x95PPPPPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xED\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0a\t\0\x86a\n!V[`\x10\x1B`\x90\x86\x90\x1B\x01\x91\x90\x91\x01\x92PPP\x93\x92PPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x82\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xAC\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0`\0\x19\x8C\x14a\t\xCDWa\t\xC8\x8Ca\n!V[a\t\xD6V[`\x01`\x01`\x80\x1B\x03[\x90P`\0a\t\xE3\x8Ca\n\x93V[\x90P`\0a\t\xF0\x8Ca\n\xF4V[`\xB8\x9B\x90\x9B\x1B`\x98\x9B\x90\x9B\x1B\x9A\x90\x9A\x01`\x90\x91\x90\x91\x1B\x01`\x10\x91\x90\x91\x1B\x01\x01\x9B\x95\x9AP\x93\x98P\x93\x96PPPPPPPV[`\0`\x01`\x01`\x80\x1B\x03\x82\x11\x15a\n\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[P\x90V[`\0`\xFF\x82\x11\x15a\n\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FSafeCast: value doesn't fit in 8`D\x82\x01Rd bits`\xD8\x1B`d\x82\x01R`\x84\x01a\n\x86V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\n\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\n\x86V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0BnW`\0\x80\xFD[PV[a\xFF\xFF\x81\x16\x81\x14a\x0BnW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0B\x97W`\0\x80\xFD[\x845a\x0B\xA2\x81a\x0BYV[\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015a\x0B\xC0\x81a\x0BqV[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xDEW`\0\x80\xFD[\x825a\x0B\xE9\x81a\x0BYV[\x91P` \x83\x015a\x0B\xF9\x81a\x0BYV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x17W`\0\x80\xFD[\x825a\x0C\"\x81a\x0BYV[\x94` \x93\x90\x93\x015\x93PPPV[\x805`\xFF\x81\x16\x81\x14a\x0CAW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x0CaW`\0\x80\xFD[\x875a\x0Cl\x81a\x0BYV[\x96P` \x88\x015\x95P`@\x88\x015a\x0C\x83\x81a\x0BqV[\x94P``\x88\x015\x93Pa\x0C\x98`\x80\x89\x01a\x0C0V[\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[\x805\x80\x15\x15\x81\x14a\x0CAW`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x0C\xDCW`\0\x80\xFD[\x855a\x0C\xE7\x81a\x0BYV[\x94P` \x86\x015a\x0C\xF7\x81a\x0BYV[\x93P`@\x86\x015a\r\x07\x81a\x0BYV[\x92P``\x86\x015\x91Pa\r\x1C`\x80\x87\x01a\x0C\xB4V[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\r=W`\0\x80\xFD[\x835a\rH\x81a\x0BYV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\rrW`\0\x80\xFD[\x835a\r}\x81a\x0BYV[\x92P` \x84\x015\x91P`@\x84\x015a\r\x94\x81a\x0BqV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\r\xB2W`\0\x80\xFD[\x825a\r\xBD\x81a\x0BYV[\x91Pa\r\xCB` \x84\x01a\x0C\xB4V[\x90P\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\r\xEFW`\0\x80\xFD[\x875a\r\xFA\x81a\x0BYV[\x96P` \x88\x015\x95P`@\x88\x015\x94P``\x88\x015\x93Pa\x0C\x98`\x80\x89\x01a\x0C0V[`@Qa\x01\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0EOWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0` \x82\x84\x03\x12\x15a\x0EgW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0E\x98WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x91Q\x82RP\x91\x90PV[\x80Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x0CAW`\0\x80\xFD[\x80Qd\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0CAW`\0\x80\xFD[\x80Qa\x0CA\x81a\x0BqV[\x80Qa\x0CA\x81a\x0BYV[`\0a\x01\xE0\x82\x84\x03\x12\x15a\x0E\xFAW`\0\x80\xFD[a\x0F\x02a\x0E\x1DV[a\x0F\x0C\x84\x84a\x0EUV[\x81Ra\x0F\x1A` \x84\x01a\x0E\xA5V[` \x82\x01Ra\x0F+`@\x84\x01a\x0E\xA5V[`@\x82\x01Ra\x0F<``\x84\x01a\x0E\xA5V[``\x82\x01Ra\x0FM`\x80\x84\x01a\x0E\xA5V[`\x80\x82\x01Ra\x0F^`\xA0\x84\x01a\x0E\xA5V[`\xA0\x82\x01Ra\x0Fo`\xC0\x84\x01a\x0E\xBCV[`\xC0\x82\x01Ra\x0F\x80`\xE0\x84\x01a\x0E\xD1V[`\xE0\x82\x01Ra\x01\0a\x0F\x93\x81\x85\x01a\x0E\xDCV[\x90\x82\x01Ra\x01 a\x0F\xA5\x84\x82\x01a\x0E\xDCV[\x90\x82\x01Ra\x01@a\x0F\xB7\x84\x82\x01a\x0E\xDCV[\x90\x82\x01Ra\x01`a\x0F\xC9\x84\x82\x01a\x0E\xDCV[\x90\x82\x01Ra\x01\x80a\x0F\xDB\x84\x82\x01a\x0E\xA5V[\x90\x82\x01Ra\x01\xA0a\x0F\xED\x84\x82\x01a\x0E\xA5V[\x90\x82\x01Ra\x01\xC0a\x0F\xFF\x84\x82\x01a\x0E\xA5V[\x90\x82\x01R\x93\x92PPPV\xFE\xA2dipfsX\"\x12 -\x83y(\xED\xF2\x0C\xAD\xD4!},\x89\xD6\xF7\x15\xF2[\xC1\xA3\x9B&\xBA\xAA\xCB\xF6\xF4x\\\xC7\x93\xB7dsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static L2ENCODER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB4W`\x005`\xE0\x1C\x80c\x88\xD5\x18R\x11a\0qW\x80c\x88\xD5\x18R\x14a\x01\x85W\x80c\x8D\xA7\xFB\x18\x14a\x01\xADW\x80c\x9D/\xFC\x1B\x14a\x01\xC0W\x80c\xB7c\x98\xE4\x14a\x01\xD3W\x80c\xFC\x0E\xED\x85\x14a\x01\xE6W\x80c\xFE\xD6:\x93\x14a\x01\xF4W`\0\x80\xFD[\x80c\x1Ad\xAC\xF2\x14a\0\xB9W\x80c\x1A\x8Fm\xEE\x14a\0\xDFW\x80c\x1F\xD3G\x97\x14a\0\xF2W\x80c\\\xC7\xBC\x10\x14a\x01\x05W\x80cg\x1A\x7F\xAE\x14a\x01\x18W\x80cu5\xD2F\x14a\x01FW[`\0\x80\xFD[a\0\xCCa\0\xC76`\x04a\x0B\x81V[a\x02\x07V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCCa\0\xED6`\x04a\x0B\xCBV[a\x02\xD9V[a\0\xCCa\x01\x006`\x04a\x0C\x04V[a\x03|V[a\0\xCCa\x01\x136`\x04a\x0C\x04V[a\x04/V[a\x01+a\x01&6`\x04a\x0CFV[a\x04\xFDV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\0\xD6V[a\x01m\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xD6V[a\x01\x98a\x01\x936`\x04a\x0C\xC4V[a\x05\xE7V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xD6V[a\0\xCCa\x01\xBB6`\x04a\r(V[a\x07fV[a\0\xCCa\x01\xCE6`\x04a\r(V[a\x07{V[a\0\xCCa\x01\xE16`\x04a\r]V[a\x08]V[a\0\xCCa\0\xED6`\x04a\r\x9FV[a\x01+a\x02\x026`\x04a\r\xD4V[a\t\x18V[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x97\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0a\x02\xAA\x87a\n!V[\x90P`\0a\x02\xB7\x87a\n\x93V[`\x10\x92\x90\x92\x1B`\x90\x92\x90\x92\x1B`\x98\x96\x90\x96\x1B\x95\x90\x95\x01\x01\x01\x96\x95PPPPPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03i\x91\x90a\x0E\xE7V[`\xE0\x01Q`\x10\x84\x90\x1B\x01\x91PP\x92\x91PPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x0C\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0a\x04\x1F\x85a\n\x93V[`\x10\x1B\x91\x90\x91\x01\x95\x94PPPPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xBF\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0`\0\x19\x85\x14a\x04\xE0Wa\x04\xDB\x85a\n!V[a\x04\x1FV[Pq\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x01\x94\x93PPPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x82\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x91\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0a\x05\xA4\x8Ca\n!V[\x90P`\0a\x05\xB1\x8Ba\n\xF4V[\x90P`\0\x8A`\xC0\x1B\x82`\xA0\x1B\x01\x8D`\x90\x1B\x01\x83`\x10\x1B\x01\x84\x01\x90P\x80\x8A\x8A\x97P\x97P\x97PPPPPP\x97P\x97P\x97\x94PPPPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06{\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x92\x93P\x90\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x15\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0`\0\x19\x89\x14a\x076Wa\x071\x89a\n!V[a\x07?V[`\x01`\x01`\x80\x1B\x03[`\x10\x92\x90\x92\x1B\x93\x90\x93\x01` \x8A\x90\x1B\x01\x95P`\x80\x87\x90\x1B\x01\x93PPPP\x95P\x95\x93PPPPV[`\0a\x07s\x84\x84\x84a\x07{V[\x94\x93PPPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x0B\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0`\0\x19\x86\x14a\x08,Wa\x08'\x86a\n!V[a\x085V[`\x01`\x01`\x80\x1B\x03[\x90P`\0a\x08B\x86a\n\x93V[`\x90\x1B`\x10\x92\x90\x92\x1B\x91\x90\x91\x01\x91\x90\x91\x01\x96\x95PPPPPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xED\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0a\t\0\x86a\n!V[`\x10\x1B`\x90\x86\x90\x1B\x01\x91\x90\x91\x01\x92PPP\x93\x92PPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x82\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xAC\x91\x90a\x0E\xE7V[`\xE0\x81\x01Q\x90\x91P`\0`\0\x19\x8C\x14a\t\xCDWa\t\xC8\x8Ca\n!V[a\t\xD6V[`\x01`\x01`\x80\x1B\x03[\x90P`\0a\t\xE3\x8Ca\n\x93V[\x90P`\0a\t\xF0\x8Ca\n\xF4V[`\xB8\x9B\x90\x9B\x1B`\x98\x9B\x90\x9B\x1B\x9A\x90\x9A\x01`\x90\x91\x90\x91\x1B\x01`\x10\x91\x90\x91\x1B\x01\x01\x9B\x95\x9AP\x93\x98P\x93\x96PPPPPPPV[`\0`\x01`\x01`\x80\x1B\x03\x82\x11\x15a\n\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[P\x90V[`\0`\xFF\x82\x11\x15a\n\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FSafeCast: value doesn't fit in 8`D\x82\x01Rd bits`\xD8\x1B`d\x82\x01R`\x84\x01a\n\x86V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\n\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\n\x86V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0BnW`\0\x80\xFD[PV[a\xFF\xFF\x81\x16\x81\x14a\x0BnW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0B\x97W`\0\x80\xFD[\x845a\x0B\xA2\x81a\x0BYV[\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015a\x0B\xC0\x81a\x0BqV[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xDEW`\0\x80\xFD[\x825a\x0B\xE9\x81a\x0BYV[\x91P` \x83\x015a\x0B\xF9\x81a\x0BYV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x17W`\0\x80\xFD[\x825a\x0C\"\x81a\x0BYV[\x94` \x93\x90\x93\x015\x93PPPV[\x805`\xFF\x81\x16\x81\x14a\x0CAW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x0CaW`\0\x80\xFD[\x875a\x0Cl\x81a\x0BYV[\x96P` \x88\x015\x95P`@\x88\x015a\x0C\x83\x81a\x0BqV[\x94P``\x88\x015\x93Pa\x0C\x98`\x80\x89\x01a\x0C0V[\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[\x805\x80\x15\x15\x81\x14a\x0CAW`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x0C\xDCW`\0\x80\xFD[\x855a\x0C\xE7\x81a\x0BYV[\x94P` \x86\x015a\x0C\xF7\x81a\x0BYV[\x93P`@\x86\x015a\r\x07\x81a\x0BYV[\x92P``\x86\x015\x91Pa\r\x1C`\x80\x87\x01a\x0C\xB4V[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\r=W`\0\x80\xFD[\x835a\rH\x81a\x0BYV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\rrW`\0\x80\xFD[\x835a\r}\x81a\x0BYV[\x92P` \x84\x015\x91P`@\x84\x015a\r\x94\x81a\x0BqV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\r\xB2W`\0\x80\xFD[\x825a\r\xBD\x81a\x0BYV[\x91Pa\r\xCB` \x84\x01a\x0C\xB4V[\x90P\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\r\xEFW`\0\x80\xFD[\x875a\r\xFA\x81a\x0BYV[\x96P` \x88\x015\x95P`@\x88\x015\x94P``\x88\x015\x93Pa\x0C\x98`\x80\x89\x01a\x0C0V[`@Qa\x01\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0EOWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0` \x82\x84\x03\x12\x15a\x0EgW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0E\x98WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x91Q\x82RP\x91\x90PV[\x80Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x0CAW`\0\x80\xFD[\x80Qd\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0CAW`\0\x80\xFD[\x80Qa\x0CA\x81a\x0BqV[\x80Qa\x0CA\x81a\x0BYV[`\0a\x01\xE0\x82\x84\x03\x12\x15a\x0E\xFAW`\0\x80\xFD[a\x0F\x02a\x0E\x1DV[a\x0F\x0C\x84\x84a\x0EUV[\x81Ra\x0F\x1A` \x84\x01a\x0E\xA5V[` \x82\x01Ra\x0F+`@\x84\x01a\x0E\xA5V[`@\x82\x01Ra\x0F<``\x84\x01a\x0E\xA5V[``\x82\x01Ra\x0FM`\x80\x84\x01a\x0E\xA5V[`\x80\x82\x01Ra\x0F^`\xA0\x84\x01a\x0E\xA5V[`\xA0\x82\x01Ra\x0Fo`\xC0\x84\x01a\x0E\xBCV[`\xC0\x82\x01Ra\x0F\x80`\xE0\x84\x01a\x0E\xD1V[`\xE0\x82\x01Ra\x01\0a\x0F\x93\x81\x85\x01a\x0E\xDCV[\x90\x82\x01Ra\x01 a\x0F\xA5\x84\x82\x01a\x0E\xDCV[\x90\x82\x01Ra\x01@a\x0F\xB7\x84\x82\x01a\x0E\xDCV[\x90\x82\x01Ra\x01`a\x0F\xC9\x84\x82\x01a\x0E\xDCV[\x90\x82\x01Ra\x01\x80a\x0F\xDB\x84\x82\x01a\x0E\xA5V[\x90\x82\x01Ra\x01\xA0a\x0F\xED\x84\x82\x01a\x0E\xA5V[\x90\x82\x01Ra\x01\xC0a\x0F\xFF\x84\x82\x01a\x0E\xA5V[\x90\x82\x01R\x93\x92PPPV\xFE\xA2dipfsX\"\x12 -\x83y(\xED\xF2\x0C\xAD\xD4!},\x89\xD6\xF7\x15\xF2[\xC1\xA3\x9B&\xBA\xAA\xCB\xF6\xF4x\\\xC7\x93\xB7dsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static L2ENCODER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct L2Encoder<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for L2Encoder<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for L2Encoder<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for L2Encoder<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for L2Encoder<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(L2Encoder)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> L2Encoder<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    L2ENCODER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                L2ENCODER_ABI.clone(),
                L2ENCODER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `POOL` (0x7535d246) function
        pub fn pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([117, 53, 210, 70], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `encodeSupplyWithPermitParams` (0x671a7fae) function
        pub fn encode_supply_with_permit_params(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            referral_code: u16,
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
                    [103, 26, 127, 174],
                    (
                        asset,
                        amount,
                        referral_code,
                        deadline,
                        permit_v,
                        permit_r,
                        permit_s,
                    ),
                )
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
    for L2Encoder<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `POOL` function with signature `POOL()` and selector `0x7535d246`
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
    #[ethcall(name = "POOL", abi = "POOL()")]
    pub struct PoolCall;
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
    ///Container type for all input parameters for the `encodeSupplyWithPermitParams` function with signature `encodeSupplyWithPermitParams(address,uint256,uint16,uint256,uint8,bytes32,bytes32)` and selector `0x671a7fae`
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
        name = "encodeSupplyWithPermitParams",
        abi = "encodeSupplyWithPermitParams(address,uint256,uint16,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct EncodeSupplyWithPermitParamsCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub referral_code: u16,
        pub deadline: ::ethers::core::types::U256,
        pub permit_v: u8,
        pub permit_r: [u8; 32],
        pub permit_s: [u8; 32],
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
    pub enum L2EncoderCalls {
        Pool(PoolCall),
        EncodeBorrowParams(EncodeBorrowParamsCall),
        EncodeLiquidationCall(EncodeLiquidationCallCall),
        EncodeRebalanceStableBorrowRate(EncodeRebalanceStableBorrowRateCall),
        EncodeRepayParams(EncodeRepayParamsCall),
        EncodeRepayWithATokensParams(EncodeRepayWithATokensParamsCall),
        EncodeRepayWithPermitParams(EncodeRepayWithPermitParamsCall),
        EncodeSetUserUseReserveAsCollateral(EncodeSetUserUseReserveAsCollateralCall),
        EncodeSupplyParams(EncodeSupplyParamsCall),
        EncodeSupplyWithPermitParams(EncodeSupplyWithPermitParamsCall),
        EncodeSwapBorrowRateMode(EncodeSwapBorrowRateModeCall),
        EncodeWithdrawParams(EncodeWithdrawParamsCall),
    }
    impl ::ethers::core::abi::AbiDecode for L2EncoderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <PoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pool(decoded));
            }
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
                = <EncodeSupplyWithPermitParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EncodeSupplyWithPermitParams(decoded));
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
    impl ::ethers::core::abi::AbiEncode for L2EncoderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::EncodeSupplyWithPermitParams(element) => {
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
    impl ::core::fmt::Display for L2EncoderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::EncodeSupplyWithPermitParams(element) => {
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
    impl ::core::convert::From<PoolCall> for L2EncoderCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<EncodeBorrowParamsCall> for L2EncoderCalls {
        fn from(value: EncodeBorrowParamsCall) -> Self {
            Self::EncodeBorrowParams(value)
        }
    }
    impl ::core::convert::From<EncodeLiquidationCallCall> for L2EncoderCalls {
        fn from(value: EncodeLiquidationCallCall) -> Self {
            Self::EncodeLiquidationCall(value)
        }
    }
    impl ::core::convert::From<EncodeRebalanceStableBorrowRateCall> for L2EncoderCalls {
        fn from(value: EncodeRebalanceStableBorrowRateCall) -> Self {
            Self::EncodeRebalanceStableBorrowRate(value)
        }
    }
    impl ::core::convert::From<EncodeRepayParamsCall> for L2EncoderCalls {
        fn from(value: EncodeRepayParamsCall) -> Self {
            Self::EncodeRepayParams(value)
        }
    }
    impl ::core::convert::From<EncodeRepayWithATokensParamsCall> for L2EncoderCalls {
        fn from(value: EncodeRepayWithATokensParamsCall) -> Self {
            Self::EncodeRepayWithATokensParams(value)
        }
    }
    impl ::core::convert::From<EncodeRepayWithPermitParamsCall> for L2EncoderCalls {
        fn from(value: EncodeRepayWithPermitParamsCall) -> Self {
            Self::EncodeRepayWithPermitParams(value)
        }
    }
    impl ::core::convert::From<EncodeSetUserUseReserveAsCollateralCall>
    for L2EncoderCalls {
        fn from(value: EncodeSetUserUseReserveAsCollateralCall) -> Self {
            Self::EncodeSetUserUseReserveAsCollateral(value)
        }
    }
    impl ::core::convert::From<EncodeSupplyParamsCall> for L2EncoderCalls {
        fn from(value: EncodeSupplyParamsCall) -> Self {
            Self::EncodeSupplyParams(value)
        }
    }
    impl ::core::convert::From<EncodeSupplyWithPermitParamsCall> for L2EncoderCalls {
        fn from(value: EncodeSupplyWithPermitParamsCall) -> Self {
            Self::EncodeSupplyWithPermitParams(value)
        }
    }
    impl ::core::convert::From<EncodeSwapBorrowRateModeCall> for L2EncoderCalls {
        fn from(value: EncodeSwapBorrowRateModeCall) -> Self {
            Self::EncodeSwapBorrowRateMode(value)
        }
    }
    impl ::core::convert::From<EncodeWithdrawParamsCall> for L2EncoderCalls {
        fn from(value: EncodeWithdrawParamsCall) -> Self {
            Self::EncodeWithdrawParams(value)
        }
    }
    ///Container type for all return fields from the `POOL` function with signature `POOL()` and selector `0x7535d246`
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
    pub struct PoolReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `encodeSupplyWithPermitParams` function with signature `encodeSupplyWithPermitParams(address,uint256,uint16,uint256,uint8,bytes32,bytes32)` and selector `0x671a7fae`
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
    pub struct EncodeSupplyWithPermitParamsReturn(
        pub [u8; 32],
        pub [u8; 32],
        pub [u8; 32],
    );
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
