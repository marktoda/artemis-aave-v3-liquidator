pub use errors::*;
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
pub mod errors {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ACL_ADMIN_CANNOT_BE_ZERO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ACL_ADMIN_CANNOT_BE_ZERO",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ADDRESSES_PROVIDER_ALREADY_ADDED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ADDRESSES_PROVIDER_ALREADY_ADDED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "ADDRESSES_PROVIDER_NOT_REGISTERED",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ADDRESSES_PROVIDER_NOT_REGISTERED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "ASSET_NOT_BORROWABLE_IN_ISOLATION",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ASSET_NOT_BORROWABLE_IN_ISOLATION",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ASSET_NOT_LISTED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ASSET_NOT_LISTED"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BORROWING_NOT_ENABLED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BORROWING_NOT_ENABLED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BORROW_CAP_EXCEEDED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BORROW_CAP_EXCEEDED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BRIDGE_PROTOCOL_FEE_INVALID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BRIDGE_PROTOCOL_FEE_INVALID",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CALLER_MUST_BE_POOL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CALLER_MUST_BE_POOL",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CALLER_NOT_ATOKEN"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CALLER_NOT_ATOKEN"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CALLER_NOT_BRIDGE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CALLER_NOT_BRIDGE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CALLER_NOT_EMERGENCY_ADMIN"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CALLER_NOT_EMERGENCY_ADMIN",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CALLER_NOT_POOL_ADMIN"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CALLER_NOT_POOL_ADMIN",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CALLER_NOT_POOL_CONFIGURATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CALLER_NOT_POOL_CONFIGURATOR",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "CALLER_NOT_POOL_OR_EMERGENCY_ADMIN",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CALLER_NOT_POOL_OR_EMERGENCY_ADMIN",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CALLER_NOT_RISK_OR_POOL_ADMIN"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CALLER_NOT_RISK_OR_POOL_ADMIN",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("COLLATERAL_BALANCE_IS_ZERO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "COLLATERAL_BALANCE_IS_ZERO",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("COLLATERAL_CANNOT_BE_LIQUIDATED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "COLLATERAL_CANNOT_BE_LIQUIDATED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "COLLATERAL_CANNOT_COVER_NEW_BORROW",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "COLLATERAL_CANNOT_COVER_NEW_BORROW",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "COLLATERAL_SAME_AS_BORROWING_CURRENCY",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "COLLATERAL_SAME_AS_BORROWING_CURRENCY",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DEBT_CEILING_EXCEEDED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DEBT_CEILING_EXCEEDED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DEBT_CEILING_NOT_ZERO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DEBT_CEILING_NOT_ZERO",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EMODE_CATEGORY_RESERVED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EMODE_CATEGORY_RESERVED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FLASHLOAN_DISABLED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FLASHLOAN_DISABLED"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FLASHLOAN_PREMIUM_INVALID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FLASHLOAN_PREMIUM_INVALID",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "HEALTH_FACTOR_NOT_BELOW_THRESHOLD",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HEALTH_FACTOR_NOT_BELOW_THRESHOLD",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INCONSISTENT_EMODE_CATEGORY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INCONSISTENT_EMODE_CATEGORY",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INCONSISTENT_FLASHLOAN_PARAMS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INCONSISTENT_FLASHLOAN_PARAMS",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INCONSISTENT_PARAMS_LENGTH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INCONSISTENT_PARAMS_LENGTH",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_ADDRESSES_PROVIDER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_ADDRESSES_PROVIDER",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_ADDRESSES_PROVIDER_ID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_ADDRESSES_PROVIDER_ID",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_AMOUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INVALID_AMOUNT"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_BORROW_CAP"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INVALID_BORROW_CAP"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_BURN_AMOUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_BURN_AMOUNT",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_DEBT_CEILING"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_DEBT_CEILING",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_DECIMALS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INVALID_DECIMALS"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_EMODE_CATEGORY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_EMODE_CATEGORY",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "INVALID_EMODE_CATEGORY_ASSIGNMENT",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_EMODE_CATEGORY_ASSIGNMENT",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_EMODE_CATEGORY_PARAMS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_EMODE_CATEGORY_PARAMS",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_EXPIRATION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INVALID_EXPIRATION"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "INVALID_FLASHLOAN_EXECUTOR_RETURN",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_FLASHLOAN_EXECUTOR_RETURN",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "INVALID_INTEREST_RATE_MODE_SELECTED",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_INTEREST_RATE_MODE_SELECTED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_LIQUIDATION_PROTOCOL_FEE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_LIQUIDATION_PROTOCOL_FEE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_LIQ_BONUS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INVALID_LIQ_BONUS"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_LIQ_THRESHOLD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_LIQ_THRESHOLD",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_LTV"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INVALID_LTV"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_MINT_AMOUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_MINT_AMOUNT",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_OPTIMAL_USAGE_RATIO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_OPTIMAL_USAGE_RATIO",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_RESERVE_FACTOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_RESERVE_FACTOR",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_RESERVE_INDEX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_RESERVE_INDEX",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_RESERVE_PARAMS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_RESERVE_PARAMS",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_SIGNATURE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INVALID_SIGNATURE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_SUPPLY_CAP"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INVALID_SUPPLY_CAP"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_UNBACKED_MINT_CAP"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_UNBACKED_MINT_CAP",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LTV_VALIDATION_FAILED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LTV_VALIDATION_FAILED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NOT_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("NOT_CONTRACT"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "NOT_ENOUGH_AVAILABLE_USER_BALANCE",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NOT_ENOUGH_AVAILABLE_USER_BALANCE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NO_DEBT_OF_SELECTED_TYPE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NO_DEBT_OF_SELECTED_TYPE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NO_MORE_RESERVES_ALLOWED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NO_MORE_RESERVES_ALLOWED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NO_OUTSTANDING_STABLE_DEBT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NO_OUTSTANDING_STABLE_DEBT",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NO_OUTSTANDING_VARIABLE_DEBT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NO_OUTSTANDING_VARIABLE_DEBT",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OPERATION_NOT_SUPPORTED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OPERATION_NOT_SUPPORTED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("POOL_ADDRESSES_DO_NOT_MATCH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "POOL_ADDRESSES_DO_NOT_MATCH",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "PRICE_ORACLE_SENTINEL_CHECK_FAILED",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRICE_ORACLE_SENTINEL_CHECK_FAILED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RESERVE_ALREADY_ADDED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RESERVE_ALREADY_ADDED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RESERVE_ALREADY_INITIALIZED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RESERVE_ALREADY_INITIALIZED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RESERVE_DEBT_NOT_ZERO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RESERVE_DEBT_NOT_ZERO",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RESERVE_FROZEN"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("RESERVE_FROZEN"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RESERVE_INACTIVE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("RESERVE_INACTIVE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RESERVE_LIQUIDITY_NOT_ZERO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RESERVE_LIQUIDITY_NOT_ZERO",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RESERVE_PAUSED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("RESERVE_PAUSED"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SILOED_BORROWING_VIOLATION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SILOED_BORROWING_VIOLATION",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("STABLE_BORROWING_ENABLED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "STABLE_BORROWING_ENABLED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("STABLE_BORROWING_NOT_ENABLED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "STABLE_BORROWING_NOT_ENABLED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("STABLE_DEBT_NOT_ZERO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "STABLE_DEBT_NOT_ZERO",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SUPPLY_CAP_EXCEEDED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SUPPLY_CAP_EXCEEDED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UNBACKED_MINT_CAP_EXCEEDED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UNBACKED_MINT_CAP_EXCEEDED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UNDERLYING_BALANCE_ZERO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UNDERLYING_BALANCE_ZERO",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UNDERLYING_CANNOT_BE_RESCUED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UNDERLYING_CANNOT_BE_RESCUED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                        "USER_IN_ISOLATION_MODE_OR_LTV_ZERO",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "USER_IN_ISOLATION_MODE_OR_LTV_ZERO",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VARIABLE_DEBT_SUPPLY_NOT_ZERO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "VARIABLE_DEBT_SUPPLY_NOT_ZERO",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZERO_ADDRESS_NOT_VALID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ZERO_ADDRESS_NOT_VALID",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
    pub static ERRORS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x11/a\0:`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14a\0-WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\x04\xF8W`\x005`\xE0\x1C\x80c\x8A\xA3\xCAL\x11a\x02\x98W\x80c\xBA\xD80\x8C\x11a\x01qW\x80c\xDD\x1D\xD9_\x11a\0\xE3W\x80c\xF0\x7Fg\x85\x11a\0\x9CW\x80c\xF0\x7Fg\x85\x14a\x0F\xDEW\x80c\xF1\x07'\xDB\x14a\x0F\xFFW\x80c\xF4y\xEA\x11\x14a\x10 W\x80c\xFA\x16:\x83\x14a\x10AW\x80c\xFA\xE8'\x91\x14a\x10bW\x80c\xFD\x18(\xFF\x14a\x10\x83W`\0\x80\xFD[\x80c\xDD\x1D\xD9_\x14a\x0F\x19W\x80c\xDE$\x94\x8C\x14a\x0F:W\x80c\xE0/\x07\xEE\x14a\x0F[W\x80c\xE3\xFA \xF5\x14a\x0F{W\x80c\xE4\xDD\x8Bt\x14a\x0F\x9CW\x80c\xE9\x81H:\x14a\x0F\xBDW`\0\x80\xFD[\x80c\xD1K\xB1z\x11a\x015W\x80c\xD1K\xB1z\x14a\x0ESW\x80c\xD1\xCD\x8B\x1D\x14a\x0EtW\x80c\xD6\xF9\xFC\xDE\x14a\x0E\x95W\x80c\xD9\xAD\xDA\x85\x14a\x0E\xB6W\x80c\xDC\x19\x1B\xD9\x14a\x0E\xD7W\x80c\xDC\xC5m\xB6\x14a\x0E\xF8W`\0\x80\xFD[\x80c\xBA\xD80\x8C\x14a\r\xAEW\x80c\xC0\x8A\x11F\x14a\r\xCFW\x80c\xC8c\x80\x82\x14a\r\xF0W\x80c\xC8\x990\x1A\x14a\x0E\x11W\x80c\xCD#6|\x14a\x0E2W`\0\x80\xFD[\x80c\xA4\x86\x8D\xCA\x11a\x02\nW\x80c\xB0Q\0T\x11a\x01\xCEW\x80c\xB0Q\0T\x14a\x0C\xE9W\x80c\xB4\xA4W0\x14a\r\nW\x80c\xB5\xE7\x93f\x14a\r+W\x80c\xB6\x87t\xE9\x14a\rKW\x80c\xB7\xF5\xE2$\x14a\rlW\x80c\xB8pA\xC2\x14a\r\x8DW`\0\x80\xFD[\x80c\xA4\x86\x8D\xCA\x14a\x0CEW\x80c\xA8\xC9xS\x14a\x0CfW\x80c\xAB\x88<\xA0\x14a\x0C\x87W\x80c\xAB\xD3Q\xB1\x14a\x0C\xA8W\x80c\xACu26\x14a\x0C\xC9W`\0\x80\xFD[\x80c\x95&3\xC5\x11a\x02\\W\x80c\x95&3\xC5\x14a\x0B\x7FW\x80c\x95'\xE9\xD9\x14a\x0B\xA0W\x80c\x99\xCES\xF3\x14a\x0B\xC1W\x80c\xA2y|\x80\x14a\x0B\xE2W\x80c\xA2\xE9v\xC6\x14a\x0C\x03W\x80c\xA3@*8\x14a\x0C$W`\0\x80\xFD[\x80c\x8A\xA3\xCAL\x14a\n\xDAW\x80c\x8B\x8B\x98\xD7\x14a\n\xFBW\x80c\x8E\xDAF\xBD\x14a\x0B\x1CW\x80c\x8Fw\"\xB2\x14a\x0B=W\x80c\x94\xF9\xFD\x8A\x14a\x0B^W`\0\x80\xFD[\x80cN:\xED7\x11a\x03\xD5W\x80cl\xD3\xCF\xBC\x11a\x03GW\x80cz\xA0v~\x11a\x03\0W\x80cz\xA0v~\x14a\n\x14W\x80c\x7F\xEAo6\x14a\n5W\x80c\x85\x96\xAA\xD5\x14a\nVW\x80c\x89_}\xC8\x14a\nwW\x80c\x89\xC5\xD4_\x14a\n\x98W\x80c\x8A4@\0\x14a\n\xB9W`\0\x80\xFD[\x80cl\xD3\xCF\xBC\x14a\tNW\x80cq/Sj\x14a\toW\x80cs\xDE\xA5\xE3\x14a\t\x90W\x80ctE\x9B\x14\x14a\t\xB1W\x80ct\x7F\xA5V\x14a\t\xD2W\x80cv\xAE\x8F\xCA\x14a\t\xF3W`\0\x80\xFD[\x80c]\x9Cv\xC0\x11a\x03\x99W\x80c]\x9Cv\xC0\x14a\x08\x89W\x80c`\xC3\xDE\x80\x14a\x08\xAAW\x80ca\xC1\x11\xD2\x14a\x08\xCAW\x80ce\xA8;\xAB\x14a\x08\xEBW\x80ce\xE7\xEFL\x14a\t\x0CW\x80ck?|\xC7\x14a\t-W`\0\x80\xFD[\x80cN:\xED7\x14a\x07\xE5W\x80cN\xF9\x99\xFF\x14a\x08\x06W\x80cOwd{\x14a\x08'W\x80cQ&tP\x14a\x08GW\x80cR\xBA\x9D\xBE\x14a\x08hW`\0\x80\xFD[\x80c.\xED\x17\xE8\x11a\x04nW\x80cG\xBA\x93\xD8\x11a\x042W\x80cG\xBA\x93\xD8\x14a\x07 W\x80cG\xCF\x15#\x14a\x07AW\x80cH\x07\x02\xAE\x14a\x07bW\x80cH\\\x8F\xF6\x14a\x07\x83W\x80cM\x86\xF3\x93\x14a\x07\xA3W\x80cN\x01\xE3\xC1\x14a\x07\xC4W`\0\x80\xFD[\x80c.\xED\x17\xE8\x14a\x06{W\x80c3Wc\xDE\x14a\x06\x9CW\x80c6n\xB5M\x14a\x06\xBDW\x80c7\x93\x07\x82\x14a\x06\xDEW\x80cG\x1D\xF6\x85\x14a\x06\xFFW`\0\x80\xFD[\x80c\x1A\xBB\xB0\x01\x11a\x04\xC0W\x80c\x1A\xBB\xB0\x01\x14a\x05\xB7W\x80c\"\xA74F\x14a\x05\xD8W\x80c&\xBB\xD0S\x14a\x05\xF9W\x80c&\xE7\xB3\x12\x14a\x06\x1AW\x80c)&\xC9q\x14a\x06:W\x80c,\x8E;L\x14a\x06[W`\0\x80\xFD[\x80c\x08M\xFA\r\x14a\x04\xFDW\x80c\x11\xD7\xB0\x06\x14a\x054W\x80c\x12\xDC\xAD\xE8\x14a\x05TW\x80c\x14\xDC\xFB\xBC\x14a\x05uW\x80c\x19\x8Djk\x14a\x05\x96W[`\0\x80\xFD[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06'`\xF3\x1B\x81RP\x81V[`@Qa\x05+\x91\x90a\x10\xA4V[`@Q\x80\x91\x03\x90\xF3[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`9`\xF8\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x0CM`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1C\x1B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x07\x07`\xF3\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a87`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a47`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a69`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`3`\xF8\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\r\r`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`5`\xF8\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03S`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03#`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a35`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x18\x99`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a23`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1B\x99`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a21`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1B\x19`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x19`\xF9\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a31`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x0C\xCD`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a83`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x033`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x1B`\xF9\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a25`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a27`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a17`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x07`\xFB\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03\x13`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a53`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a55`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1A\x99`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06G`\xF3\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03C`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a49`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a41`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a19`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a15`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x19\x19`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a13`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03c`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1A\x1B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a33`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a37`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a91`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03\x83`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03s`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06\xA7`\xF3\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\rM`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a45`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a65`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a63`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a43`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a11`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a79`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a67`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a71`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a85`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x0C\x8D`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a51`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1A\x19`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\r`\xFA\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a29`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x19\x99`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a57`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1B\x9B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06\xE7`\xF3\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a59`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x0E\r`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1C\x19`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a77`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\r\xCD`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06\xC7`\xF3\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a61`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a39`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a73`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\r\x8D`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a89`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`7`\xF8\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x19\x9B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a81`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03\x93`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06g`\xF3\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1A\x9B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x18\x9B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1B\x1B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x19\x1B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a75`\xF0\x1B\x81RP\x81V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x10\xD1W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x10\xB5V[\x81\x81\x11\x15a\x10\xE3W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xEC&\xB86\xB0\xD9\xBF\xB9\xDE\xDA\x93]Sc\xCB\x1C\x90V\xF7\xCD\xED#\xF3\xB9\xD7~\xF2\xDE\xBE\xC2G\x8BdsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static ERRORS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\x04\xF8W`\x005`\xE0\x1C\x80c\x8A\xA3\xCAL\x11a\x02\x98W\x80c\xBA\xD80\x8C\x11a\x01qW\x80c\xDD\x1D\xD9_\x11a\0\xE3W\x80c\xF0\x7Fg\x85\x11a\0\x9CW\x80c\xF0\x7Fg\x85\x14a\x0F\xDEW\x80c\xF1\x07'\xDB\x14a\x0F\xFFW\x80c\xF4y\xEA\x11\x14a\x10 W\x80c\xFA\x16:\x83\x14a\x10AW\x80c\xFA\xE8'\x91\x14a\x10bW\x80c\xFD\x18(\xFF\x14a\x10\x83W`\0\x80\xFD[\x80c\xDD\x1D\xD9_\x14a\x0F\x19W\x80c\xDE$\x94\x8C\x14a\x0F:W\x80c\xE0/\x07\xEE\x14a\x0F[W\x80c\xE3\xFA \xF5\x14a\x0F{W\x80c\xE4\xDD\x8Bt\x14a\x0F\x9CW\x80c\xE9\x81H:\x14a\x0F\xBDW`\0\x80\xFD[\x80c\xD1K\xB1z\x11a\x015W\x80c\xD1K\xB1z\x14a\x0ESW\x80c\xD1\xCD\x8B\x1D\x14a\x0EtW\x80c\xD6\xF9\xFC\xDE\x14a\x0E\x95W\x80c\xD9\xAD\xDA\x85\x14a\x0E\xB6W\x80c\xDC\x19\x1B\xD9\x14a\x0E\xD7W\x80c\xDC\xC5m\xB6\x14a\x0E\xF8W`\0\x80\xFD[\x80c\xBA\xD80\x8C\x14a\r\xAEW\x80c\xC0\x8A\x11F\x14a\r\xCFW\x80c\xC8c\x80\x82\x14a\r\xF0W\x80c\xC8\x990\x1A\x14a\x0E\x11W\x80c\xCD#6|\x14a\x0E2W`\0\x80\xFD[\x80c\xA4\x86\x8D\xCA\x11a\x02\nW\x80c\xB0Q\0T\x11a\x01\xCEW\x80c\xB0Q\0T\x14a\x0C\xE9W\x80c\xB4\xA4W0\x14a\r\nW\x80c\xB5\xE7\x93f\x14a\r+W\x80c\xB6\x87t\xE9\x14a\rKW\x80c\xB7\xF5\xE2$\x14a\rlW\x80c\xB8pA\xC2\x14a\r\x8DW`\0\x80\xFD[\x80c\xA4\x86\x8D\xCA\x14a\x0CEW\x80c\xA8\xC9xS\x14a\x0CfW\x80c\xAB\x88<\xA0\x14a\x0C\x87W\x80c\xAB\xD3Q\xB1\x14a\x0C\xA8W\x80c\xACu26\x14a\x0C\xC9W`\0\x80\xFD[\x80c\x95&3\xC5\x11a\x02\\W\x80c\x95&3\xC5\x14a\x0B\x7FW\x80c\x95'\xE9\xD9\x14a\x0B\xA0W\x80c\x99\xCES\xF3\x14a\x0B\xC1W\x80c\xA2y|\x80\x14a\x0B\xE2W\x80c\xA2\xE9v\xC6\x14a\x0C\x03W\x80c\xA3@*8\x14a\x0C$W`\0\x80\xFD[\x80c\x8A\xA3\xCAL\x14a\n\xDAW\x80c\x8B\x8B\x98\xD7\x14a\n\xFBW\x80c\x8E\xDAF\xBD\x14a\x0B\x1CW\x80c\x8Fw\"\xB2\x14a\x0B=W\x80c\x94\xF9\xFD\x8A\x14a\x0B^W`\0\x80\xFD[\x80cN:\xED7\x11a\x03\xD5W\x80cl\xD3\xCF\xBC\x11a\x03GW\x80cz\xA0v~\x11a\x03\0W\x80cz\xA0v~\x14a\n\x14W\x80c\x7F\xEAo6\x14a\n5W\x80c\x85\x96\xAA\xD5\x14a\nVW\x80c\x89_}\xC8\x14a\nwW\x80c\x89\xC5\xD4_\x14a\n\x98W\x80c\x8A4@\0\x14a\n\xB9W`\0\x80\xFD[\x80cl\xD3\xCF\xBC\x14a\tNW\x80cq/Sj\x14a\toW\x80cs\xDE\xA5\xE3\x14a\t\x90W\x80ctE\x9B\x14\x14a\t\xB1W\x80ct\x7F\xA5V\x14a\t\xD2W\x80cv\xAE\x8F\xCA\x14a\t\xF3W`\0\x80\xFD[\x80c]\x9Cv\xC0\x11a\x03\x99W\x80c]\x9Cv\xC0\x14a\x08\x89W\x80c`\xC3\xDE\x80\x14a\x08\xAAW\x80ca\xC1\x11\xD2\x14a\x08\xCAW\x80ce\xA8;\xAB\x14a\x08\xEBW\x80ce\xE7\xEFL\x14a\t\x0CW\x80ck?|\xC7\x14a\t-W`\0\x80\xFD[\x80cN:\xED7\x14a\x07\xE5W\x80cN\xF9\x99\xFF\x14a\x08\x06W\x80cOwd{\x14a\x08'W\x80cQ&tP\x14a\x08GW\x80cR\xBA\x9D\xBE\x14a\x08hW`\0\x80\xFD[\x80c.\xED\x17\xE8\x11a\x04nW\x80cG\xBA\x93\xD8\x11a\x042W\x80cG\xBA\x93\xD8\x14a\x07 W\x80cG\xCF\x15#\x14a\x07AW\x80cH\x07\x02\xAE\x14a\x07bW\x80cH\\\x8F\xF6\x14a\x07\x83W\x80cM\x86\xF3\x93\x14a\x07\xA3W\x80cN\x01\xE3\xC1\x14a\x07\xC4W`\0\x80\xFD[\x80c.\xED\x17\xE8\x14a\x06{W\x80c3Wc\xDE\x14a\x06\x9CW\x80c6n\xB5M\x14a\x06\xBDW\x80c7\x93\x07\x82\x14a\x06\xDEW\x80cG\x1D\xF6\x85\x14a\x06\xFFW`\0\x80\xFD[\x80c\x1A\xBB\xB0\x01\x11a\x04\xC0W\x80c\x1A\xBB\xB0\x01\x14a\x05\xB7W\x80c\"\xA74F\x14a\x05\xD8W\x80c&\xBB\xD0S\x14a\x05\xF9W\x80c&\xE7\xB3\x12\x14a\x06\x1AW\x80c)&\xC9q\x14a\x06:W\x80c,\x8E;L\x14a\x06[W`\0\x80\xFD[\x80c\x08M\xFA\r\x14a\x04\xFDW\x80c\x11\xD7\xB0\x06\x14a\x054W\x80c\x12\xDC\xAD\xE8\x14a\x05TW\x80c\x14\xDC\xFB\xBC\x14a\x05uW\x80c\x19\x8Djk\x14a\x05\x96W[`\0\x80\xFD[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06'`\xF3\x1B\x81RP\x81V[`@Qa\x05+\x91\x90a\x10\xA4V[`@Q\x80\x91\x03\x90\xF3[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`9`\xF8\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x0CM`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1C\x1B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x07\x07`\xF3\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a87`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a47`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a69`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`3`\xF8\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\r\r`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`5`\xF8\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03S`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03#`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a35`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x18\x99`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a23`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1B\x99`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a21`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1B\x19`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x19`\xF9\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a31`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x0C\xCD`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a83`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x033`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x1B`\xF9\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a25`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a27`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a17`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\x07`\xFB\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03\x13`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a53`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a55`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1A\x99`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06G`\xF3\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03C`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a49`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a41`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a19`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a15`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x19\x19`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a13`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03c`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1A\x1B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a33`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a37`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a91`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03\x83`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03s`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06\xA7`\xF3\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\rM`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a45`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a65`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a63`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a43`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a11`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a79`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a67`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a71`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a85`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x0C\x8D`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a51`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1A\x19`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\r`\xFA\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a29`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x19\x99`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a57`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1B\x9B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06\xE7`\xF3\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a59`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x0E\r`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1C\x19`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a77`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\r\xCD`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06\xC7`\xF3\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a61`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a39`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a73`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\r\x8D`\xF2\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a89`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`7`\xF8\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x19\x9B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a81`\xF0\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x03\x93`\xF4\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06g`\xF3\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1A\x9B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x18\x9B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1B\x1B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x19\x1B`\xF1\x1B\x81RP\x81V[a\x05\x1E`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a75`\xF0\x1B\x81RP\x81V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x10\xD1W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x10\xB5V[\x81\x81\x11\x15a\x10\xE3W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xEC&\xB86\xB0\xD9\xBF\xB9\xDE\xDA\x93]Sc\xCB\x1C\x90V\xF7\xCD\xED#\xF3\xB9\xD7~\xF2\xDE\xBE\xC2G\x8BdsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static ERRORS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Errors<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Errors<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Errors<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Errors<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Errors<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Errors)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Errors<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERRORS_ABI.clone(),
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
                ERRORS_ABI.clone(),
                ERRORS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ACL_ADMIN_CANNOT_BE_ZERO` (0xfd1828ff) function
        pub fn acl_admin_cannot_be_zero(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([253, 24, 40, 255], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ADDRESSES_PROVIDER_ALREADY_ADDED` (0x14dcfbbc) function
        pub fn addresses_provider_already_added(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([20, 220, 251, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ADDRESSES_PROVIDER_NOT_REGISTERED` (0xe02f07ee) function
        pub fn addresses_provider_not_registered(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([224, 47, 7, 238], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE` (0xf07f6785) function
        pub fn amount_bigger_than_max_loan_size_stable(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([240, 127, 103, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ASSET_NOT_BORROWABLE_IN_ISOLATION` (0x8596aad5) function
        pub fn asset_not_borrowable_in_isolation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([133, 150, 170, 213], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ASSET_NOT_LISTED` (0xcd23367c) function
        pub fn asset_not_listed(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([205, 35, 54, 124], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BORROWING_NOT_ENABLED` (0x4ef999ff) function
        pub fn borrowing_not_enabled(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([78, 249, 153, 255], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BORROW_CAP_EXCEEDED` (0x2eed17e8) function
        pub fn borrow_cap_exceeded(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([46, 237, 23, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BRIDGE_PROTOCOL_FEE_INVALID` (0x7aa0767e) function
        pub fn bridge_protocol_fee_invalid(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([122, 160, 118, 126], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CALLER_MUST_BE_POOL` (0x471df685) function
        pub fn caller_must_be_pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([71, 29, 246, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN` (0x2c8e3b4c) function
        pub fn caller_not_asset_listing_or_pool_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([44, 142, 59, 76], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CALLER_NOT_ATOKEN` (0xa2e976c6) function
        pub fn caller_not_atoken(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([162, 233, 118, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CALLER_NOT_BRIDGE` (0x4f77647b) function
        pub fn caller_not_bridge(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([79, 119, 100, 123], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CALLER_NOT_EMERGENCY_ADMIN` (0x485c8ff6) function
        pub fn caller_not_emergency_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([72, 92, 143, 246], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CALLER_NOT_POOL_ADMIN` (0xac753236) function
        pub fn caller_not_pool_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([172, 117, 50, 54], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CALLER_NOT_POOL_CONFIGURATOR` (0x61c111d2) function
        pub fn caller_not_pool_configurator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([97, 193, 17, 210], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CALLER_NOT_POOL_OR_EMERGENCY_ADMIN` (0x26e7b312) function
        pub fn caller_not_pool_or_emergency_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([38, 231, 179, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CALLER_NOT_RISK_OR_POOL_ADMIN` (0xb5e79366) function
        pub fn caller_not_risk_or_pool_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([181, 231, 147, 102], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `COLLATERAL_BALANCE_IS_ZERO` (0x4e01e3c1) function
        pub fn collateral_balance_is_zero(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([78, 1, 227, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `COLLATERAL_CANNOT_BE_LIQUIDATED` (0x895f7dc8) function
        pub fn collateral_cannot_be_liquidated(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([137, 95, 125, 200], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `COLLATERAL_CANNOT_COVER_NEW_BORROW` (0xe3fa20f5) function
        pub fn collateral_cannot_cover_new_borrow(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([227, 250, 32, 245], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `COLLATERAL_SAME_AS_BORROWING_CURRENCY` (0x8a344000) function
        pub fn collateral_same_as_borrowing_currency(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([138, 52, 64, 0], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEBT_CEILING_EXCEEDED` (0x65a83bab) function
        pub fn debt_ceiling_exceeded(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([101, 168, 59, 171], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEBT_CEILING_NOT_ZERO` (0xe4dd8b74) function
        pub fn debt_ceiling_not_zero(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([228, 221, 139, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `EMODE_CATEGORY_RESERVED` (0xf479ea11) function
        pub fn emode_category_reserved(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([244, 121, 234, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FLASHLOAN_DISABLED` (0x8aa3ca4c) function
        pub fn flashloan_disabled(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([138, 163, 202, 76], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FLASHLOAN_PREMIUM_INVALID` (0x747fa556) function
        pub fn flashloan_premium_invalid(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([116, 127, 165, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD` (0x366eb54d) function
        pub fn health_factor_lower_than_liquidation_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([54, 110, 181, 77], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HEALTH_FACTOR_NOT_BELOW_THRESHOLD` (0x952633c5) function
        pub fn health_factor_not_below_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 38, 51, 197], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INCONSISTENT_EMODE_CATEGORY` (0x8f7722b2) function
        pub fn inconsistent_emode_category(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([143, 119, 34, 178], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INCONSISTENT_FLASHLOAN_PARAMS` (0x73dea5e3) function
        pub fn inconsistent_flashloan_params(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([115, 222, 165, 227], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INCONSISTENT_PARAMS_LENGTH` (0xbad8308c) function
        pub fn inconsistent_params_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([186, 216, 48, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET` (0x2926c971) function
        pub fn interest_rate_rebalance_conditions_not_met(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([41, 38, 201, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_ADDRESSES_PROVIDER` (0x37930782) function
        pub fn invalid_addresses_provider(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([55, 147, 7, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_ADDRESSES_PROVIDER_ID` (0x60c3de80) function
        pub fn invalid_addresses_provider_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([96, 195, 222, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_AMOUNT` (0xfae82791) function
        pub fn invalid_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([250, 232, 39, 145], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_BORROW_CAP` (0xd6f9fcde) function
        pub fn invalid_borrow_cap(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([214, 249, 252, 222], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_BURN_AMOUNT` (0x51267450) function
        pub fn invalid_burn_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([81, 38, 116, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_DEBT_CEILING` (0xdcc56db6) function
        pub fn invalid_debt_ceiling(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([220, 197, 109, 182], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_DECIMALS` (0xfa163a83) function
        pub fn invalid_decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([250, 22, 58, 131], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_EMODE_CATEGORY` (0xa8c97853) function
        pub fn invalid_emode_category(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([168, 201, 120, 83], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_EMODE_CATEGORY_ASSIGNMENT` (0x5d9c76c0) function
        pub fn invalid_emode_category_assignment(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([93, 156, 118, 192], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_EMODE_CATEGORY_PARAMS` (0x47cf1523) function
        pub fn invalid_emode_category_params(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([71, 207, 21, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_EXPIRATION` (0xc08a1146) function
        pub fn invalid_expiration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([192, 138, 17, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_FLASHLOAN_EXECUTOR_RETURN` (0x7fea6f36) function
        pub fn invalid_flashloan_executor_return(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([127, 234, 111, 54], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_INTEREST_RATE_MODE_SELECTED` (0x89c5d45f) function
        pub fn invalid_interest_rate_mode_selected(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([137, 197, 212, 95], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_LIQUIDATION_PROTOCOL_FEE` (0x8eda46bd) function
        pub fn invalid_liquidation_protocol_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([142, 218, 70, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_LIQ_BONUS` (0x9527e9d9) function
        pub fn invalid_liq_bonus(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 39, 233, 217], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_LIQ_THRESHOLD` (0xdd1dd95f) function
        pub fn invalid_liq_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([221, 29, 217, 95], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_LTV` (0x99ce53f3) function
        pub fn invalid_ltv(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([153, 206, 83, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_MINT_AMOUNT` (0xabd351b1) function
        pub fn invalid_mint_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([171, 211, 81, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO` (0xc899301a) function
        pub fn invalid_optimal_stable_to_total_debt_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 153, 48, 26], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_OPTIMAL_USAGE_RATIO` (0x4e3aed37) function
        pub fn invalid_optimal_usage_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([78, 58, 237, 55], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_RESERVE_FACTOR` (0xa4868dca) function
        pub fn invalid_reserve_factor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([164, 134, 141, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_RESERVE_INDEX` (0xd1cd8b1d) function
        pub fn invalid_reserve_index(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([209, 205, 139, 29], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_RESERVE_PARAMS` (0x335763de) function
        pub fn invalid_reserve_params(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([51, 87, 99, 222], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_SIGNATURE` (0xa3402a38) function
        pub fn invalid_signature(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([163, 64, 42, 56], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_SUPPLY_CAP` (0x26bbd053) function
        pub fn invalid_supply_cap(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([38, 187, 208, 83], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_UNBACKED_MINT_CAP` (0x47ba93d8) function
        pub fn invalid_unbacked_mint_cap(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([71, 186, 147, 216], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `LTV_VALIDATION_FAILED` (0xb87041c2) function
        pub fn ltv_validation_failed(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([184, 112, 65, 194], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `NOT_CONTRACT` (0x11d7b006) function
        pub fn not_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([17, 215, 176, 6], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `NOT_ENOUGH_AVAILABLE_USER_BALANCE` (0xb7f5e224) function
        pub fn not_enough_available_user_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([183, 245, 226, 36], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `NO_DEBT_OF_SELECTED_TYPE` (0xdc191bd9) function
        pub fn no_debt_of_selected_type(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([220, 25, 27, 217], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF` (0x712f536a) function
        pub fn no_explicit_amount_to_repay_on_behalf(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([113, 47, 83, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `NO_MORE_RESERVES_ALLOWED` (0x76ae8fca) function
        pub fn no_more_reserves_allowed(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([118, 174, 143, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `NO_OUTSTANDING_STABLE_DEBT` (0x74459b14) function
        pub fn no_outstanding_stable_debt(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([116, 69, 155, 20], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `NO_OUTSTANDING_VARIABLE_DEBT` (0xb4a45730) function
        pub fn no_outstanding_variable_debt(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([180, 164, 87, 48], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OPERATION_NOT_SUPPORTED` (0x8b8b98d7) function
        pub fn operation_not_supported(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([139, 139, 152, 215], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `POOL_ADDRESSES_DO_NOT_MATCH` (0x1abbb001) function
        pub fn pool_addresses_do_not_match(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([26, 187, 176, 1], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PRICE_ORACLE_SENTINEL_CHECK_FAILED` (0xc8638082) function
        pub fn price_oracle_sentinel_check_failed(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 99, 128, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RESERVE_ALREADY_ADDED` (0x12dcade8) function
        pub fn reserve_already_added(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([18, 220, 173, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RESERVE_ALREADY_INITIALIZED` (0xd9adda85) function
        pub fn reserve_already_initialized(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([217, 173, 218, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RESERVE_DEBT_NOT_ZERO` (0xe981483a) function
        pub fn reserve_debt_not_zero(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([233, 129, 72, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RESERVE_FROZEN` (0x6cd3cfbc) function
        pub fn reserve_frozen(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([108, 211, 207, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RESERVE_INACTIVE` (0x52ba9dbe) function
        pub fn reserve_inactive(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([82, 186, 157, 190], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RESERVE_LIQUIDITY_NOT_ZERO` (0x084dfa0d) function
        pub fn reserve_liquidity_not_zero(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([8, 77, 250, 13], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RESERVE_PAUSED` (0xb68774e9) function
        pub fn reserve_paused(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([182, 135, 116, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SILOED_BORROWING_VIOLATION` (0xde24948c) function
        pub fn siloed_borrowing_violation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([222, 36, 148, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER` (0x22a73446) function
        pub fn specified_currency_not_borrowed_by_user(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([34, 167, 52, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STABLE_BORROWING_ENABLED` (0x198d6a6b) function
        pub fn stable_borrowing_enabled(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([25, 141, 106, 107], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STABLE_BORROWING_NOT_ENABLED` (0x4d86f393) function
        pub fn stable_borrowing_not_enabled(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([77, 134, 243, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STABLE_DEBT_NOT_ZERO` (0x65e7ef4c) function
        pub fn stable_debt_not_zero(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([101, 231, 239, 76], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SUPPLY_CAP_EXCEEDED` (0xb0510054) function
        pub fn supply_cap_exceeded(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([176, 81, 0, 84], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UNBACKED_MINT_CAP_EXCEEDED` (0x6b3f7cc7) function
        pub fn unbacked_mint_cap_exceeded(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([107, 63, 124, 199], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UNDERLYING_BALANCE_ZERO` (0xa2797c80) function
        pub fn underlying_balance_zero(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([162, 121, 124, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UNDERLYING_CANNOT_BE_RESCUED` (0xab883ca0) function
        pub fn underlying_cannot_be_rescued(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([171, 136, 60, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO` (0x94f9fd8a) function
        pub fn underlying_claimable_rights_not_zero(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([148, 249, 253, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `USER_IN_ISOLATION_MODE_OR_LTV_ZERO` (0x480702ae) function
        pub fn user_in_isolation_mode_or_ltv_zero(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([72, 7, 2, 174], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `VARIABLE_DEBT_SUPPLY_NOT_ZERO` (0xf10727db) function
        pub fn variable_debt_supply_not_zero(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([241, 7, 39, 219], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ZERO_ADDRESS_NOT_VALID` (0xd14bb17a) function
        pub fn zero_address_not_valid(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([209, 75, 177, 122], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Errors<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `ACL_ADMIN_CANNOT_BE_ZERO` function with signature `ACL_ADMIN_CANNOT_BE_ZERO()` and selector `0xfd1828ff`
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
    #[ethcall(name = "ACL_ADMIN_CANNOT_BE_ZERO", abi = "ACL_ADMIN_CANNOT_BE_ZERO()")]
    pub struct AclAdminCannotBeZeroCall;
    ///Container type for all input parameters for the `ADDRESSES_PROVIDER_ALREADY_ADDED` function with signature `ADDRESSES_PROVIDER_ALREADY_ADDED()` and selector `0x14dcfbbc`
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
        name = "ADDRESSES_PROVIDER_ALREADY_ADDED",
        abi = "ADDRESSES_PROVIDER_ALREADY_ADDED()"
    )]
    pub struct AddressesProviderAlreadyAddedCall;
    ///Container type for all input parameters for the `ADDRESSES_PROVIDER_NOT_REGISTERED` function with signature `ADDRESSES_PROVIDER_NOT_REGISTERED()` and selector `0xe02f07ee`
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
        name = "ADDRESSES_PROVIDER_NOT_REGISTERED",
        abi = "ADDRESSES_PROVIDER_NOT_REGISTERED()"
    )]
    pub struct AddressesProviderNotRegisteredCall;
    ///Container type for all input parameters for the `AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE` function with signature `AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE()` and selector `0xf07f6785`
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
        name = "AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE",
        abi = "AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE()"
    )]
    pub struct AmountBiggerThanMaxLoanSizeStableCall;
    ///Container type for all input parameters for the `ASSET_NOT_BORROWABLE_IN_ISOLATION` function with signature `ASSET_NOT_BORROWABLE_IN_ISOLATION()` and selector `0x8596aad5`
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
        name = "ASSET_NOT_BORROWABLE_IN_ISOLATION",
        abi = "ASSET_NOT_BORROWABLE_IN_ISOLATION()"
    )]
    pub struct AssetNotBorrowableInIsolationCall;
    ///Container type for all input parameters for the `ASSET_NOT_LISTED` function with signature `ASSET_NOT_LISTED()` and selector `0xcd23367c`
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
    #[ethcall(name = "ASSET_NOT_LISTED", abi = "ASSET_NOT_LISTED()")]
    pub struct AssetNotListedCall;
    ///Container type for all input parameters for the `BORROWING_NOT_ENABLED` function with signature `BORROWING_NOT_ENABLED()` and selector `0x4ef999ff`
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
    #[ethcall(name = "BORROWING_NOT_ENABLED", abi = "BORROWING_NOT_ENABLED()")]
    pub struct BorrowingNotEnabledCall;
    ///Container type for all input parameters for the `BORROW_CAP_EXCEEDED` function with signature `BORROW_CAP_EXCEEDED()` and selector `0x2eed17e8`
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
    #[ethcall(name = "BORROW_CAP_EXCEEDED", abi = "BORROW_CAP_EXCEEDED()")]
    pub struct BorrowCapExceededCall;
    ///Container type for all input parameters for the `BRIDGE_PROTOCOL_FEE_INVALID` function with signature `BRIDGE_PROTOCOL_FEE_INVALID()` and selector `0x7aa0767e`
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
        name = "BRIDGE_PROTOCOL_FEE_INVALID",
        abi = "BRIDGE_PROTOCOL_FEE_INVALID()"
    )]
    pub struct BridgeProtocolFeeInvalidCall;
    ///Container type for all input parameters for the `CALLER_MUST_BE_POOL` function with signature `CALLER_MUST_BE_POOL()` and selector `0x471df685`
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
    #[ethcall(name = "CALLER_MUST_BE_POOL", abi = "CALLER_MUST_BE_POOL()")]
    pub struct CallerMustBePoolCall;
    ///Container type for all input parameters for the `CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN` function with signature `CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN()` and selector `0x2c8e3b4c`
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
        name = "CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN",
        abi = "CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN()"
    )]
    pub struct CallerNotAssetListingOrPoolAdminCall;
    ///Container type for all input parameters for the `CALLER_NOT_ATOKEN` function with signature `CALLER_NOT_ATOKEN()` and selector `0xa2e976c6`
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
    #[ethcall(name = "CALLER_NOT_ATOKEN", abi = "CALLER_NOT_ATOKEN()")]
    pub struct CallerNotAtokenCall;
    ///Container type for all input parameters for the `CALLER_NOT_BRIDGE` function with signature `CALLER_NOT_BRIDGE()` and selector `0x4f77647b`
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
    #[ethcall(name = "CALLER_NOT_BRIDGE", abi = "CALLER_NOT_BRIDGE()")]
    pub struct CallerNotBridgeCall;
    ///Container type for all input parameters for the `CALLER_NOT_EMERGENCY_ADMIN` function with signature `CALLER_NOT_EMERGENCY_ADMIN()` and selector `0x485c8ff6`
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
    #[ethcall(name = "CALLER_NOT_EMERGENCY_ADMIN", abi = "CALLER_NOT_EMERGENCY_ADMIN()")]
    pub struct CallerNotEmergencyAdminCall;
    ///Container type for all input parameters for the `CALLER_NOT_POOL_ADMIN` function with signature `CALLER_NOT_POOL_ADMIN()` and selector `0xac753236`
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
    #[ethcall(name = "CALLER_NOT_POOL_ADMIN", abi = "CALLER_NOT_POOL_ADMIN()")]
    pub struct CallerNotPoolAdminCall;
    ///Container type for all input parameters for the `CALLER_NOT_POOL_CONFIGURATOR` function with signature `CALLER_NOT_POOL_CONFIGURATOR()` and selector `0x61c111d2`
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
        name = "CALLER_NOT_POOL_CONFIGURATOR",
        abi = "CALLER_NOT_POOL_CONFIGURATOR()"
    )]
    pub struct CallerNotPoolConfiguratorCall;
    ///Container type for all input parameters for the `CALLER_NOT_POOL_OR_EMERGENCY_ADMIN` function with signature `CALLER_NOT_POOL_OR_EMERGENCY_ADMIN()` and selector `0x26e7b312`
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
        name = "CALLER_NOT_POOL_OR_EMERGENCY_ADMIN",
        abi = "CALLER_NOT_POOL_OR_EMERGENCY_ADMIN()"
    )]
    pub struct CallerNotPoolOrEmergencyAdminCall;
    ///Container type for all input parameters for the `CALLER_NOT_RISK_OR_POOL_ADMIN` function with signature `CALLER_NOT_RISK_OR_POOL_ADMIN()` and selector `0xb5e79366`
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
        name = "CALLER_NOT_RISK_OR_POOL_ADMIN",
        abi = "CALLER_NOT_RISK_OR_POOL_ADMIN()"
    )]
    pub struct CallerNotRiskOrPoolAdminCall;
    ///Container type for all input parameters for the `COLLATERAL_BALANCE_IS_ZERO` function with signature `COLLATERAL_BALANCE_IS_ZERO()` and selector `0x4e01e3c1`
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
    #[ethcall(name = "COLLATERAL_BALANCE_IS_ZERO", abi = "COLLATERAL_BALANCE_IS_ZERO()")]
    pub struct CollateralBalanceIsZeroCall;
    ///Container type for all input parameters for the `COLLATERAL_CANNOT_BE_LIQUIDATED` function with signature `COLLATERAL_CANNOT_BE_LIQUIDATED()` and selector `0x895f7dc8`
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
        name = "COLLATERAL_CANNOT_BE_LIQUIDATED",
        abi = "COLLATERAL_CANNOT_BE_LIQUIDATED()"
    )]
    pub struct CollateralCannotBeLiquidatedCall;
    ///Container type for all input parameters for the `COLLATERAL_CANNOT_COVER_NEW_BORROW` function with signature `COLLATERAL_CANNOT_COVER_NEW_BORROW()` and selector `0xe3fa20f5`
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
        name = "COLLATERAL_CANNOT_COVER_NEW_BORROW",
        abi = "COLLATERAL_CANNOT_COVER_NEW_BORROW()"
    )]
    pub struct CollateralCannotCoverNewBorrowCall;
    ///Container type for all input parameters for the `COLLATERAL_SAME_AS_BORROWING_CURRENCY` function with signature `COLLATERAL_SAME_AS_BORROWING_CURRENCY()` and selector `0x8a344000`
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
        name = "COLLATERAL_SAME_AS_BORROWING_CURRENCY",
        abi = "COLLATERAL_SAME_AS_BORROWING_CURRENCY()"
    )]
    pub struct CollateralSameAsBorrowingCurrencyCall;
    ///Container type for all input parameters for the `DEBT_CEILING_EXCEEDED` function with signature `DEBT_CEILING_EXCEEDED()` and selector `0x65a83bab`
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
    #[ethcall(name = "DEBT_CEILING_EXCEEDED", abi = "DEBT_CEILING_EXCEEDED()")]
    pub struct DebtCeilingExceededCall;
    ///Container type for all input parameters for the `DEBT_CEILING_NOT_ZERO` function with signature `DEBT_CEILING_NOT_ZERO()` and selector `0xe4dd8b74`
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
    #[ethcall(name = "DEBT_CEILING_NOT_ZERO", abi = "DEBT_CEILING_NOT_ZERO()")]
    pub struct DebtCeilingNotZeroCall;
    ///Container type for all input parameters for the `EMODE_CATEGORY_RESERVED` function with signature `EMODE_CATEGORY_RESERVED()` and selector `0xf479ea11`
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
    #[ethcall(name = "EMODE_CATEGORY_RESERVED", abi = "EMODE_CATEGORY_RESERVED()")]
    pub struct EmodeCategoryReservedCall;
    ///Container type for all input parameters for the `FLASHLOAN_DISABLED` function with signature `FLASHLOAN_DISABLED()` and selector `0x8aa3ca4c`
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
    #[ethcall(name = "FLASHLOAN_DISABLED", abi = "FLASHLOAN_DISABLED()")]
    pub struct FlashloanDisabledCall;
    ///Container type for all input parameters for the `FLASHLOAN_PREMIUM_INVALID` function with signature `FLASHLOAN_PREMIUM_INVALID()` and selector `0x747fa556`
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
    #[ethcall(name = "FLASHLOAN_PREMIUM_INVALID", abi = "FLASHLOAN_PREMIUM_INVALID()")]
    pub struct FlashloanPremiumInvalidCall;
    ///Container type for all input parameters for the `HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD` function with signature `HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD()` and selector `0x366eb54d`
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
        name = "HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD",
        abi = "HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD()"
    )]
    pub struct HealthFactorLowerThanLiquidationThresholdCall;
    ///Container type for all input parameters for the `HEALTH_FACTOR_NOT_BELOW_THRESHOLD` function with signature `HEALTH_FACTOR_NOT_BELOW_THRESHOLD()` and selector `0x952633c5`
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
        name = "HEALTH_FACTOR_NOT_BELOW_THRESHOLD",
        abi = "HEALTH_FACTOR_NOT_BELOW_THRESHOLD()"
    )]
    pub struct HealthFactorNotBelowThresholdCall;
    ///Container type for all input parameters for the `INCONSISTENT_EMODE_CATEGORY` function with signature `INCONSISTENT_EMODE_CATEGORY()` and selector `0x8f7722b2`
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
        name = "INCONSISTENT_EMODE_CATEGORY",
        abi = "INCONSISTENT_EMODE_CATEGORY()"
    )]
    pub struct InconsistentEmodeCategoryCall;
    ///Container type for all input parameters for the `INCONSISTENT_FLASHLOAN_PARAMS` function with signature `INCONSISTENT_FLASHLOAN_PARAMS()` and selector `0x73dea5e3`
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
        name = "INCONSISTENT_FLASHLOAN_PARAMS",
        abi = "INCONSISTENT_FLASHLOAN_PARAMS()"
    )]
    pub struct InconsistentFlashloanParamsCall;
    ///Container type for all input parameters for the `INCONSISTENT_PARAMS_LENGTH` function with signature `INCONSISTENT_PARAMS_LENGTH()` and selector `0xbad8308c`
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
    #[ethcall(name = "INCONSISTENT_PARAMS_LENGTH", abi = "INCONSISTENT_PARAMS_LENGTH()")]
    pub struct InconsistentParamsLengthCall;
    ///Container type for all input parameters for the `INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET` function with signature `INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET()` and selector `0x2926c971`
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
        name = "INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET",
        abi = "INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET()"
    )]
    pub struct InterestRateRebalanceConditionsNotMetCall;
    ///Container type for all input parameters for the `INVALID_ADDRESSES_PROVIDER` function with signature `INVALID_ADDRESSES_PROVIDER()` and selector `0x37930782`
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
    #[ethcall(name = "INVALID_ADDRESSES_PROVIDER", abi = "INVALID_ADDRESSES_PROVIDER()")]
    pub struct InvalidAddressesProviderCall;
    ///Container type for all input parameters for the `INVALID_ADDRESSES_PROVIDER_ID` function with signature `INVALID_ADDRESSES_PROVIDER_ID()` and selector `0x60c3de80`
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
        name = "INVALID_ADDRESSES_PROVIDER_ID",
        abi = "INVALID_ADDRESSES_PROVIDER_ID()"
    )]
    pub struct InvalidAddressesProviderIdCall;
    ///Container type for all input parameters for the `INVALID_AMOUNT` function with signature `INVALID_AMOUNT()` and selector `0xfae82791`
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
    #[ethcall(name = "INVALID_AMOUNT", abi = "INVALID_AMOUNT()")]
    pub struct InvalidAmountCall;
    ///Container type for all input parameters for the `INVALID_BORROW_CAP` function with signature `INVALID_BORROW_CAP()` and selector `0xd6f9fcde`
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
    #[ethcall(name = "INVALID_BORROW_CAP", abi = "INVALID_BORROW_CAP()")]
    pub struct InvalidBorrowCapCall;
    ///Container type for all input parameters for the `INVALID_BURN_AMOUNT` function with signature `INVALID_BURN_AMOUNT()` and selector `0x51267450`
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
    #[ethcall(name = "INVALID_BURN_AMOUNT", abi = "INVALID_BURN_AMOUNT()")]
    pub struct InvalidBurnAmountCall;
    ///Container type for all input parameters for the `INVALID_DEBT_CEILING` function with signature `INVALID_DEBT_CEILING()` and selector `0xdcc56db6`
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
    #[ethcall(name = "INVALID_DEBT_CEILING", abi = "INVALID_DEBT_CEILING()")]
    pub struct InvalidDebtCeilingCall;
    ///Container type for all input parameters for the `INVALID_DECIMALS` function with signature `INVALID_DECIMALS()` and selector `0xfa163a83`
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
    #[ethcall(name = "INVALID_DECIMALS", abi = "INVALID_DECIMALS()")]
    pub struct InvalidDecimalsCall;
    ///Container type for all input parameters for the `INVALID_EMODE_CATEGORY` function with signature `INVALID_EMODE_CATEGORY()` and selector `0xa8c97853`
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
    #[ethcall(name = "INVALID_EMODE_CATEGORY", abi = "INVALID_EMODE_CATEGORY()")]
    pub struct InvalidEmodeCategoryCall;
    ///Container type for all input parameters for the `INVALID_EMODE_CATEGORY_ASSIGNMENT` function with signature `INVALID_EMODE_CATEGORY_ASSIGNMENT()` and selector `0x5d9c76c0`
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
        name = "INVALID_EMODE_CATEGORY_ASSIGNMENT",
        abi = "INVALID_EMODE_CATEGORY_ASSIGNMENT()"
    )]
    pub struct InvalidEmodeCategoryAssignmentCall;
    ///Container type for all input parameters for the `INVALID_EMODE_CATEGORY_PARAMS` function with signature `INVALID_EMODE_CATEGORY_PARAMS()` and selector `0x47cf1523`
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
        name = "INVALID_EMODE_CATEGORY_PARAMS",
        abi = "INVALID_EMODE_CATEGORY_PARAMS()"
    )]
    pub struct InvalidEmodeCategoryParamsCall;
    ///Container type for all input parameters for the `INVALID_EXPIRATION` function with signature `INVALID_EXPIRATION()` and selector `0xc08a1146`
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
    #[ethcall(name = "INVALID_EXPIRATION", abi = "INVALID_EXPIRATION()")]
    pub struct InvalidExpirationCall;
    ///Container type for all input parameters for the `INVALID_FLASHLOAN_EXECUTOR_RETURN` function with signature `INVALID_FLASHLOAN_EXECUTOR_RETURN()` and selector `0x7fea6f36`
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
        name = "INVALID_FLASHLOAN_EXECUTOR_RETURN",
        abi = "INVALID_FLASHLOAN_EXECUTOR_RETURN()"
    )]
    pub struct InvalidFlashloanExecutorReturnCall;
    ///Container type for all input parameters for the `INVALID_INTEREST_RATE_MODE_SELECTED` function with signature `INVALID_INTEREST_RATE_MODE_SELECTED()` and selector `0x89c5d45f`
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
        name = "INVALID_INTEREST_RATE_MODE_SELECTED",
        abi = "INVALID_INTEREST_RATE_MODE_SELECTED()"
    )]
    pub struct InvalidInterestRateModeSelectedCall;
    ///Container type for all input parameters for the `INVALID_LIQUIDATION_PROTOCOL_FEE` function with signature `INVALID_LIQUIDATION_PROTOCOL_FEE()` and selector `0x8eda46bd`
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
        name = "INVALID_LIQUIDATION_PROTOCOL_FEE",
        abi = "INVALID_LIQUIDATION_PROTOCOL_FEE()"
    )]
    pub struct InvalidLiquidationProtocolFeeCall;
    ///Container type for all input parameters for the `INVALID_LIQ_BONUS` function with signature `INVALID_LIQ_BONUS()` and selector `0x9527e9d9`
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
    #[ethcall(name = "INVALID_LIQ_BONUS", abi = "INVALID_LIQ_BONUS()")]
    pub struct InvalidLiqBonusCall;
    ///Container type for all input parameters for the `INVALID_LIQ_THRESHOLD` function with signature `INVALID_LIQ_THRESHOLD()` and selector `0xdd1dd95f`
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
    #[ethcall(name = "INVALID_LIQ_THRESHOLD", abi = "INVALID_LIQ_THRESHOLD()")]
    pub struct InvalidLiqThresholdCall;
    ///Container type for all input parameters for the `INVALID_LTV` function with signature `INVALID_LTV()` and selector `0x99ce53f3`
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
    #[ethcall(name = "INVALID_LTV", abi = "INVALID_LTV()")]
    pub struct InvalidLtvCall;
    ///Container type for all input parameters for the `INVALID_MINT_AMOUNT` function with signature `INVALID_MINT_AMOUNT()` and selector `0xabd351b1`
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
    #[ethcall(name = "INVALID_MINT_AMOUNT", abi = "INVALID_MINT_AMOUNT()")]
    pub struct InvalidMintAmountCall;
    ///Container type for all input parameters for the `INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO` function with signature `INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO()` and selector `0xc899301a`
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
        name = "INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO",
        abi = "INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO()"
    )]
    pub struct InvalidOptimalStableToTotalDebtRatioCall;
    ///Container type for all input parameters for the `INVALID_OPTIMAL_USAGE_RATIO` function with signature `INVALID_OPTIMAL_USAGE_RATIO()` and selector `0x4e3aed37`
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
        name = "INVALID_OPTIMAL_USAGE_RATIO",
        abi = "INVALID_OPTIMAL_USAGE_RATIO()"
    )]
    pub struct InvalidOptimalUsageRatioCall;
    ///Container type for all input parameters for the `INVALID_RESERVE_FACTOR` function with signature `INVALID_RESERVE_FACTOR()` and selector `0xa4868dca`
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
    #[ethcall(name = "INVALID_RESERVE_FACTOR", abi = "INVALID_RESERVE_FACTOR()")]
    pub struct InvalidReserveFactorCall;
    ///Container type for all input parameters for the `INVALID_RESERVE_INDEX` function with signature `INVALID_RESERVE_INDEX()` and selector `0xd1cd8b1d`
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
    #[ethcall(name = "INVALID_RESERVE_INDEX", abi = "INVALID_RESERVE_INDEX()")]
    pub struct InvalidReserveIndexCall;
    ///Container type for all input parameters for the `INVALID_RESERVE_PARAMS` function with signature `INVALID_RESERVE_PARAMS()` and selector `0x335763de`
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
    #[ethcall(name = "INVALID_RESERVE_PARAMS", abi = "INVALID_RESERVE_PARAMS()")]
    pub struct InvalidReserveParamsCall;
    ///Container type for all input parameters for the `INVALID_SIGNATURE` function with signature `INVALID_SIGNATURE()` and selector `0xa3402a38`
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
    #[ethcall(name = "INVALID_SIGNATURE", abi = "INVALID_SIGNATURE()")]
    pub struct InvalidSignatureCall;
    ///Container type for all input parameters for the `INVALID_SUPPLY_CAP` function with signature `INVALID_SUPPLY_CAP()` and selector `0x26bbd053`
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
    #[ethcall(name = "INVALID_SUPPLY_CAP", abi = "INVALID_SUPPLY_CAP()")]
    pub struct InvalidSupplyCapCall;
    ///Container type for all input parameters for the `INVALID_UNBACKED_MINT_CAP` function with signature `INVALID_UNBACKED_MINT_CAP()` and selector `0x47ba93d8`
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
    #[ethcall(name = "INVALID_UNBACKED_MINT_CAP", abi = "INVALID_UNBACKED_MINT_CAP()")]
    pub struct InvalidUnbackedMintCapCall;
    ///Container type for all input parameters for the `LTV_VALIDATION_FAILED` function with signature `LTV_VALIDATION_FAILED()` and selector `0xb87041c2`
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
    #[ethcall(name = "LTV_VALIDATION_FAILED", abi = "LTV_VALIDATION_FAILED()")]
    pub struct LtvValidationFailedCall;
    ///Container type for all input parameters for the `NOT_CONTRACT` function with signature `NOT_CONTRACT()` and selector `0x11d7b006`
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
    #[ethcall(name = "NOT_CONTRACT", abi = "NOT_CONTRACT()")]
    pub struct NotContractCall;
    ///Container type for all input parameters for the `NOT_ENOUGH_AVAILABLE_USER_BALANCE` function with signature `NOT_ENOUGH_AVAILABLE_USER_BALANCE()` and selector `0xb7f5e224`
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
        name = "NOT_ENOUGH_AVAILABLE_USER_BALANCE",
        abi = "NOT_ENOUGH_AVAILABLE_USER_BALANCE()"
    )]
    pub struct NotEnoughAvailableUserBalanceCall;
    ///Container type for all input parameters for the `NO_DEBT_OF_SELECTED_TYPE` function with signature `NO_DEBT_OF_SELECTED_TYPE()` and selector `0xdc191bd9`
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
    #[ethcall(name = "NO_DEBT_OF_SELECTED_TYPE", abi = "NO_DEBT_OF_SELECTED_TYPE()")]
    pub struct NoDebtOfSelectedTypeCall;
    ///Container type for all input parameters for the `NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF` function with signature `NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF()` and selector `0x712f536a`
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
        name = "NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF",
        abi = "NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF()"
    )]
    pub struct NoExplicitAmountToRepayOnBehalfCall;
    ///Container type for all input parameters for the `NO_MORE_RESERVES_ALLOWED` function with signature `NO_MORE_RESERVES_ALLOWED()` and selector `0x76ae8fca`
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
    #[ethcall(name = "NO_MORE_RESERVES_ALLOWED", abi = "NO_MORE_RESERVES_ALLOWED()")]
    pub struct NoMoreReservesAllowedCall;
    ///Container type for all input parameters for the `NO_OUTSTANDING_STABLE_DEBT` function with signature `NO_OUTSTANDING_STABLE_DEBT()` and selector `0x74459b14`
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
    #[ethcall(name = "NO_OUTSTANDING_STABLE_DEBT", abi = "NO_OUTSTANDING_STABLE_DEBT()")]
    pub struct NoOutstandingStableDebtCall;
    ///Container type for all input parameters for the `NO_OUTSTANDING_VARIABLE_DEBT` function with signature `NO_OUTSTANDING_VARIABLE_DEBT()` and selector `0xb4a45730`
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
        name = "NO_OUTSTANDING_VARIABLE_DEBT",
        abi = "NO_OUTSTANDING_VARIABLE_DEBT()"
    )]
    pub struct NoOutstandingVariableDebtCall;
    ///Container type for all input parameters for the `OPERATION_NOT_SUPPORTED` function with signature `OPERATION_NOT_SUPPORTED()` and selector `0x8b8b98d7`
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
    #[ethcall(name = "OPERATION_NOT_SUPPORTED", abi = "OPERATION_NOT_SUPPORTED()")]
    pub struct OperationNotSupportedCall;
    ///Container type for all input parameters for the `POOL_ADDRESSES_DO_NOT_MATCH` function with signature `POOL_ADDRESSES_DO_NOT_MATCH()` and selector `0x1abbb001`
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
        name = "POOL_ADDRESSES_DO_NOT_MATCH",
        abi = "POOL_ADDRESSES_DO_NOT_MATCH()"
    )]
    pub struct PoolAddressesDoNotMatchCall;
    ///Container type for all input parameters for the `PRICE_ORACLE_SENTINEL_CHECK_FAILED` function with signature `PRICE_ORACLE_SENTINEL_CHECK_FAILED()` and selector `0xc8638082`
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
        name = "PRICE_ORACLE_SENTINEL_CHECK_FAILED",
        abi = "PRICE_ORACLE_SENTINEL_CHECK_FAILED()"
    )]
    pub struct PriceOracleSentinelCheckFailedCall;
    ///Container type for all input parameters for the `RESERVE_ALREADY_ADDED` function with signature `RESERVE_ALREADY_ADDED()` and selector `0x12dcade8`
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
    #[ethcall(name = "RESERVE_ALREADY_ADDED", abi = "RESERVE_ALREADY_ADDED()")]
    pub struct ReserveAlreadyAddedCall;
    ///Container type for all input parameters for the `RESERVE_ALREADY_INITIALIZED` function with signature `RESERVE_ALREADY_INITIALIZED()` and selector `0xd9adda85`
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
        name = "RESERVE_ALREADY_INITIALIZED",
        abi = "RESERVE_ALREADY_INITIALIZED()"
    )]
    pub struct ReserveAlreadyInitializedCall;
    ///Container type for all input parameters for the `RESERVE_DEBT_NOT_ZERO` function with signature `RESERVE_DEBT_NOT_ZERO()` and selector `0xe981483a`
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
    #[ethcall(name = "RESERVE_DEBT_NOT_ZERO", abi = "RESERVE_DEBT_NOT_ZERO()")]
    pub struct ReserveDebtNotZeroCall;
    ///Container type for all input parameters for the `RESERVE_FROZEN` function with signature `RESERVE_FROZEN()` and selector `0x6cd3cfbc`
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
    #[ethcall(name = "RESERVE_FROZEN", abi = "RESERVE_FROZEN()")]
    pub struct ReserveFrozenCall;
    ///Container type for all input parameters for the `RESERVE_INACTIVE` function with signature `RESERVE_INACTIVE()` and selector `0x52ba9dbe`
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
    #[ethcall(name = "RESERVE_INACTIVE", abi = "RESERVE_INACTIVE()")]
    pub struct ReserveInactiveCall;
    ///Container type for all input parameters for the `RESERVE_LIQUIDITY_NOT_ZERO` function with signature `RESERVE_LIQUIDITY_NOT_ZERO()` and selector `0x084dfa0d`
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
    #[ethcall(name = "RESERVE_LIQUIDITY_NOT_ZERO", abi = "RESERVE_LIQUIDITY_NOT_ZERO()")]
    pub struct ReserveLiquidityNotZeroCall;
    ///Container type for all input parameters for the `RESERVE_PAUSED` function with signature `RESERVE_PAUSED()` and selector `0xb68774e9`
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
    #[ethcall(name = "RESERVE_PAUSED", abi = "RESERVE_PAUSED()")]
    pub struct ReservePausedCall;
    ///Container type for all input parameters for the `SILOED_BORROWING_VIOLATION` function with signature `SILOED_BORROWING_VIOLATION()` and selector `0xde24948c`
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
    #[ethcall(name = "SILOED_BORROWING_VIOLATION", abi = "SILOED_BORROWING_VIOLATION()")]
    pub struct SiloedBorrowingViolationCall;
    ///Container type for all input parameters for the `SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER` function with signature `SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER()` and selector `0x22a73446`
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
        name = "SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER",
        abi = "SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER()"
    )]
    pub struct SpecifiedCurrencyNotBorrowedByUserCall;
    ///Container type for all input parameters for the `STABLE_BORROWING_ENABLED` function with signature `STABLE_BORROWING_ENABLED()` and selector `0x198d6a6b`
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
    #[ethcall(name = "STABLE_BORROWING_ENABLED", abi = "STABLE_BORROWING_ENABLED()")]
    pub struct StableBorrowingEnabledCall;
    ///Container type for all input parameters for the `STABLE_BORROWING_NOT_ENABLED` function with signature `STABLE_BORROWING_NOT_ENABLED()` and selector `0x4d86f393`
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
        name = "STABLE_BORROWING_NOT_ENABLED",
        abi = "STABLE_BORROWING_NOT_ENABLED()"
    )]
    pub struct StableBorrowingNotEnabledCall;
    ///Container type for all input parameters for the `STABLE_DEBT_NOT_ZERO` function with signature `STABLE_DEBT_NOT_ZERO()` and selector `0x65e7ef4c`
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
    #[ethcall(name = "STABLE_DEBT_NOT_ZERO", abi = "STABLE_DEBT_NOT_ZERO()")]
    pub struct StableDebtNotZeroCall;
    ///Container type for all input parameters for the `SUPPLY_CAP_EXCEEDED` function with signature `SUPPLY_CAP_EXCEEDED()` and selector `0xb0510054`
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
    #[ethcall(name = "SUPPLY_CAP_EXCEEDED", abi = "SUPPLY_CAP_EXCEEDED()")]
    pub struct SupplyCapExceededCall;
    ///Container type for all input parameters for the `UNBACKED_MINT_CAP_EXCEEDED` function with signature `UNBACKED_MINT_CAP_EXCEEDED()` and selector `0x6b3f7cc7`
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
    #[ethcall(name = "UNBACKED_MINT_CAP_EXCEEDED", abi = "UNBACKED_MINT_CAP_EXCEEDED()")]
    pub struct UnbackedMintCapExceededCall;
    ///Container type for all input parameters for the `UNDERLYING_BALANCE_ZERO` function with signature `UNDERLYING_BALANCE_ZERO()` and selector `0xa2797c80`
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
    #[ethcall(name = "UNDERLYING_BALANCE_ZERO", abi = "UNDERLYING_BALANCE_ZERO()")]
    pub struct UnderlyingBalanceZeroCall;
    ///Container type for all input parameters for the `UNDERLYING_CANNOT_BE_RESCUED` function with signature `UNDERLYING_CANNOT_BE_RESCUED()` and selector `0xab883ca0`
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
        name = "UNDERLYING_CANNOT_BE_RESCUED",
        abi = "UNDERLYING_CANNOT_BE_RESCUED()"
    )]
    pub struct UnderlyingCannotBeRescuedCall;
    ///Container type for all input parameters for the `UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO` function with signature `UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO()` and selector `0x94f9fd8a`
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
        name = "UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO",
        abi = "UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO()"
    )]
    pub struct UnderlyingClaimableRightsNotZeroCall;
    ///Container type for all input parameters for the `USER_IN_ISOLATION_MODE_OR_LTV_ZERO` function with signature `USER_IN_ISOLATION_MODE_OR_LTV_ZERO()` and selector `0x480702ae`
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
        name = "USER_IN_ISOLATION_MODE_OR_LTV_ZERO",
        abi = "USER_IN_ISOLATION_MODE_OR_LTV_ZERO()"
    )]
    pub struct UserInIsolationModeOrLtvZeroCall;
    ///Container type for all input parameters for the `VARIABLE_DEBT_SUPPLY_NOT_ZERO` function with signature `VARIABLE_DEBT_SUPPLY_NOT_ZERO()` and selector `0xf10727db`
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
        name = "VARIABLE_DEBT_SUPPLY_NOT_ZERO",
        abi = "VARIABLE_DEBT_SUPPLY_NOT_ZERO()"
    )]
    pub struct VariableDebtSupplyNotZeroCall;
    ///Container type for all input parameters for the `ZERO_ADDRESS_NOT_VALID` function with signature `ZERO_ADDRESS_NOT_VALID()` and selector `0xd14bb17a`
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
    #[ethcall(name = "ZERO_ADDRESS_NOT_VALID", abi = "ZERO_ADDRESS_NOT_VALID()")]
    pub struct ZeroAddressNotValidCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ErrorsCalls {
        AclAdminCannotBeZero(AclAdminCannotBeZeroCall),
        AddressesProviderAlreadyAdded(AddressesProviderAlreadyAddedCall),
        AddressesProviderNotRegistered(AddressesProviderNotRegisteredCall),
        AmountBiggerThanMaxLoanSizeStable(AmountBiggerThanMaxLoanSizeStableCall),
        AssetNotBorrowableInIsolation(AssetNotBorrowableInIsolationCall),
        AssetNotListed(AssetNotListedCall),
        BorrowingNotEnabled(BorrowingNotEnabledCall),
        BorrowCapExceeded(BorrowCapExceededCall),
        BridgeProtocolFeeInvalid(BridgeProtocolFeeInvalidCall),
        CallerMustBePool(CallerMustBePoolCall),
        CallerNotAssetListingOrPoolAdmin(CallerNotAssetListingOrPoolAdminCall),
        CallerNotAtoken(CallerNotAtokenCall),
        CallerNotBridge(CallerNotBridgeCall),
        CallerNotEmergencyAdmin(CallerNotEmergencyAdminCall),
        CallerNotPoolAdmin(CallerNotPoolAdminCall),
        CallerNotPoolConfigurator(CallerNotPoolConfiguratorCall),
        CallerNotPoolOrEmergencyAdmin(CallerNotPoolOrEmergencyAdminCall),
        CallerNotRiskOrPoolAdmin(CallerNotRiskOrPoolAdminCall),
        CollateralBalanceIsZero(CollateralBalanceIsZeroCall),
        CollateralCannotBeLiquidated(CollateralCannotBeLiquidatedCall),
        CollateralCannotCoverNewBorrow(CollateralCannotCoverNewBorrowCall),
        CollateralSameAsBorrowingCurrency(CollateralSameAsBorrowingCurrencyCall),
        DebtCeilingExceeded(DebtCeilingExceededCall),
        DebtCeilingNotZero(DebtCeilingNotZeroCall),
        EmodeCategoryReserved(EmodeCategoryReservedCall),
        FlashloanDisabled(FlashloanDisabledCall),
        FlashloanPremiumInvalid(FlashloanPremiumInvalidCall),
        HealthFactorLowerThanLiquidationThreshold(
            HealthFactorLowerThanLiquidationThresholdCall,
        ),
        HealthFactorNotBelowThreshold(HealthFactorNotBelowThresholdCall),
        InconsistentEmodeCategory(InconsistentEmodeCategoryCall),
        InconsistentFlashloanParams(InconsistentFlashloanParamsCall),
        InconsistentParamsLength(InconsistentParamsLengthCall),
        InterestRateRebalanceConditionsNotMet(InterestRateRebalanceConditionsNotMetCall),
        InvalidAddressesProvider(InvalidAddressesProviderCall),
        InvalidAddressesProviderId(InvalidAddressesProviderIdCall),
        InvalidAmount(InvalidAmountCall),
        InvalidBorrowCap(InvalidBorrowCapCall),
        InvalidBurnAmount(InvalidBurnAmountCall),
        InvalidDebtCeiling(InvalidDebtCeilingCall),
        InvalidDecimals(InvalidDecimalsCall),
        InvalidEmodeCategory(InvalidEmodeCategoryCall),
        InvalidEmodeCategoryAssignment(InvalidEmodeCategoryAssignmentCall),
        InvalidEmodeCategoryParams(InvalidEmodeCategoryParamsCall),
        InvalidExpiration(InvalidExpirationCall),
        InvalidFlashloanExecutorReturn(InvalidFlashloanExecutorReturnCall),
        InvalidInterestRateModeSelected(InvalidInterestRateModeSelectedCall),
        InvalidLiquidationProtocolFee(InvalidLiquidationProtocolFeeCall),
        InvalidLiqBonus(InvalidLiqBonusCall),
        InvalidLiqThreshold(InvalidLiqThresholdCall),
        InvalidLtv(InvalidLtvCall),
        InvalidMintAmount(InvalidMintAmountCall),
        InvalidOptimalStableToTotalDebtRatio(InvalidOptimalStableToTotalDebtRatioCall),
        InvalidOptimalUsageRatio(InvalidOptimalUsageRatioCall),
        InvalidReserveFactor(InvalidReserveFactorCall),
        InvalidReserveIndex(InvalidReserveIndexCall),
        InvalidReserveParams(InvalidReserveParamsCall),
        InvalidSignature(InvalidSignatureCall),
        InvalidSupplyCap(InvalidSupplyCapCall),
        InvalidUnbackedMintCap(InvalidUnbackedMintCapCall),
        LtvValidationFailed(LtvValidationFailedCall),
        NotContract(NotContractCall),
        NotEnoughAvailableUserBalance(NotEnoughAvailableUserBalanceCall),
        NoDebtOfSelectedType(NoDebtOfSelectedTypeCall),
        NoExplicitAmountToRepayOnBehalf(NoExplicitAmountToRepayOnBehalfCall),
        NoMoreReservesAllowed(NoMoreReservesAllowedCall),
        NoOutstandingStableDebt(NoOutstandingStableDebtCall),
        NoOutstandingVariableDebt(NoOutstandingVariableDebtCall),
        OperationNotSupported(OperationNotSupportedCall),
        PoolAddressesDoNotMatch(PoolAddressesDoNotMatchCall),
        PriceOracleSentinelCheckFailed(PriceOracleSentinelCheckFailedCall),
        ReserveAlreadyAdded(ReserveAlreadyAddedCall),
        ReserveAlreadyInitialized(ReserveAlreadyInitializedCall),
        ReserveDebtNotZero(ReserveDebtNotZeroCall),
        ReserveFrozen(ReserveFrozenCall),
        ReserveInactive(ReserveInactiveCall),
        ReserveLiquidityNotZero(ReserveLiquidityNotZeroCall),
        ReservePaused(ReservePausedCall),
        SiloedBorrowingViolation(SiloedBorrowingViolationCall),
        SpecifiedCurrencyNotBorrowedByUser(SpecifiedCurrencyNotBorrowedByUserCall),
        StableBorrowingEnabled(StableBorrowingEnabledCall),
        StableBorrowingNotEnabled(StableBorrowingNotEnabledCall),
        StableDebtNotZero(StableDebtNotZeroCall),
        SupplyCapExceeded(SupplyCapExceededCall),
        UnbackedMintCapExceeded(UnbackedMintCapExceededCall),
        UnderlyingBalanceZero(UnderlyingBalanceZeroCall),
        UnderlyingCannotBeRescued(UnderlyingCannotBeRescuedCall),
        UnderlyingClaimableRightsNotZero(UnderlyingClaimableRightsNotZeroCall),
        UserInIsolationModeOrLtvZero(UserInIsolationModeOrLtvZeroCall),
        VariableDebtSupplyNotZero(VariableDebtSupplyNotZeroCall),
        ZeroAddressNotValid(ZeroAddressNotValidCall),
    }
    impl ::ethers::core::abi::AbiDecode for ErrorsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AclAdminCannotBeZeroCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AclAdminCannotBeZero(decoded));
            }
            if let Ok(decoded)
                = <AddressesProviderAlreadyAddedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddressesProviderAlreadyAdded(decoded));
            }
            if let Ok(decoded)
                = <AddressesProviderNotRegisteredCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddressesProviderNotRegistered(decoded));
            }
            if let Ok(decoded)
                = <AmountBiggerThanMaxLoanSizeStableCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AmountBiggerThanMaxLoanSizeStable(decoded));
            }
            if let Ok(decoded)
                = <AssetNotBorrowableInIsolationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AssetNotBorrowableInIsolation(decoded));
            }
            if let Ok(decoded)
                = <AssetNotListedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetNotListed(decoded));
            }
            if let Ok(decoded)
                = <BorrowingNotEnabledCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BorrowingNotEnabled(decoded));
            }
            if let Ok(decoded)
                = <BorrowCapExceededCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BorrowCapExceeded(decoded));
            }
            if let Ok(decoded)
                = <BridgeProtocolFeeInvalidCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BridgeProtocolFeeInvalid(decoded));
            }
            if let Ok(decoded)
                = <CallerMustBePoolCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CallerMustBePool(decoded));
            }
            if let Ok(decoded)
                = <CallerNotAssetListingOrPoolAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CallerNotAssetListingOrPoolAdmin(decoded));
            }
            if let Ok(decoded)
                = <CallerNotAtokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallerNotAtoken(decoded));
            }
            if let Ok(decoded)
                = <CallerNotBridgeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallerNotBridge(decoded));
            }
            if let Ok(decoded)
                = <CallerNotEmergencyAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CallerNotEmergencyAdmin(decoded));
            }
            if let Ok(decoded)
                = <CallerNotPoolAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CallerNotPoolAdmin(decoded));
            }
            if let Ok(decoded)
                = <CallerNotPoolConfiguratorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CallerNotPoolConfigurator(decoded));
            }
            if let Ok(decoded)
                = <CallerNotPoolOrEmergencyAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CallerNotPoolOrEmergencyAdmin(decoded));
            }
            if let Ok(decoded)
                = <CallerNotRiskOrPoolAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CallerNotRiskOrPoolAdmin(decoded));
            }
            if let Ok(decoded)
                = <CollateralBalanceIsZeroCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CollateralBalanceIsZero(decoded));
            }
            if let Ok(decoded)
                = <CollateralCannotBeLiquidatedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CollateralCannotBeLiquidated(decoded));
            }
            if let Ok(decoded)
                = <CollateralCannotCoverNewBorrowCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CollateralCannotCoverNewBorrow(decoded));
            }
            if let Ok(decoded)
                = <CollateralSameAsBorrowingCurrencyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CollateralSameAsBorrowingCurrency(decoded));
            }
            if let Ok(decoded)
                = <DebtCeilingExceededCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DebtCeilingExceeded(decoded));
            }
            if let Ok(decoded)
                = <DebtCeilingNotZeroCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DebtCeilingNotZero(decoded));
            }
            if let Ok(decoded)
                = <EmodeCategoryReservedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EmodeCategoryReserved(decoded));
            }
            if let Ok(decoded)
                = <FlashloanDisabledCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FlashloanDisabled(decoded));
            }
            if let Ok(decoded)
                = <FlashloanPremiumInvalidCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FlashloanPremiumInvalid(decoded));
            }
            if let Ok(decoded)
                = <HealthFactorLowerThanLiquidationThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::HealthFactorLowerThanLiquidationThreshold(decoded));
            }
            if let Ok(decoded)
                = <HealthFactorNotBelowThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::HealthFactorNotBelowThreshold(decoded));
            }
            if let Ok(decoded)
                = <InconsistentEmodeCategoryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InconsistentEmodeCategory(decoded));
            }
            if let Ok(decoded)
                = <InconsistentFlashloanParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InconsistentFlashloanParams(decoded));
            }
            if let Ok(decoded)
                = <InconsistentParamsLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InconsistentParamsLength(decoded));
            }
            if let Ok(decoded)
                = <InterestRateRebalanceConditionsNotMetCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InterestRateRebalanceConditionsNotMet(decoded));
            }
            if let Ok(decoded)
                = <InvalidAddressesProviderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidAddressesProvider(decoded));
            }
            if let Ok(decoded)
                = <InvalidAddressesProviderIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidAddressesProviderId(decoded));
            }
            if let Ok(decoded)
                = <InvalidAmountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidAmount(decoded));
            }
            if let Ok(decoded)
                = <InvalidBorrowCapCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidBorrowCap(decoded));
            }
            if let Ok(decoded)
                = <InvalidBurnAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidBurnAmount(decoded));
            }
            if let Ok(decoded)
                = <InvalidDebtCeilingCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidDebtCeiling(decoded));
            }
            if let Ok(decoded)
                = <InvalidDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidDecimals(decoded));
            }
            if let Ok(decoded)
                = <InvalidEmodeCategoryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidEmodeCategory(decoded));
            }
            if let Ok(decoded)
                = <InvalidEmodeCategoryAssignmentCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidEmodeCategoryAssignment(decoded));
            }
            if let Ok(decoded)
                = <InvalidEmodeCategoryParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidEmodeCategoryParams(decoded));
            }
            if let Ok(decoded)
                = <InvalidExpirationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidExpiration(decoded));
            }
            if let Ok(decoded)
                = <InvalidFlashloanExecutorReturnCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidFlashloanExecutorReturn(decoded));
            }
            if let Ok(decoded)
                = <InvalidInterestRateModeSelectedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidInterestRateModeSelected(decoded));
            }
            if let Ok(decoded)
                = <InvalidLiquidationProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidLiquidationProtocolFee(decoded));
            }
            if let Ok(decoded)
                = <InvalidLiqBonusCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidLiqBonus(decoded));
            }
            if let Ok(decoded)
                = <InvalidLiqThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidLiqThreshold(decoded));
            }
            if let Ok(decoded)
                = <InvalidLtvCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidLtv(decoded));
            }
            if let Ok(decoded)
                = <InvalidMintAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidMintAmount(decoded));
            }
            if let Ok(decoded)
                = <InvalidOptimalStableToTotalDebtRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidOptimalStableToTotalDebtRatio(decoded));
            }
            if let Ok(decoded)
                = <InvalidOptimalUsageRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidOptimalUsageRatio(decoded));
            }
            if let Ok(decoded)
                = <InvalidReserveFactorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidReserveFactor(decoded));
            }
            if let Ok(decoded)
                = <InvalidReserveIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidReserveIndex(decoded));
            }
            if let Ok(decoded)
                = <InvalidReserveParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidReserveParams(decoded));
            }
            if let Ok(decoded)
                = <InvalidSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidSignature(decoded));
            }
            if let Ok(decoded)
                = <InvalidSupplyCapCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidSupplyCap(decoded));
            }
            if let Ok(decoded)
                = <InvalidUnbackedMintCapCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidUnbackedMintCap(decoded));
            }
            if let Ok(decoded)
                = <LtvValidationFailedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LtvValidationFailed(decoded));
            }
            if let Ok(decoded)
                = <NotContractCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotContract(decoded));
            }
            if let Ok(decoded)
                = <NotEnoughAvailableUserBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NotEnoughAvailableUserBalance(decoded));
            }
            if let Ok(decoded)
                = <NoDebtOfSelectedTypeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NoDebtOfSelectedType(decoded));
            }
            if let Ok(decoded)
                = <NoExplicitAmountToRepayOnBehalfCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NoExplicitAmountToRepayOnBehalf(decoded));
            }
            if let Ok(decoded)
                = <NoMoreReservesAllowedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NoMoreReservesAllowed(decoded));
            }
            if let Ok(decoded)
                = <NoOutstandingStableDebtCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NoOutstandingStableDebt(decoded));
            }
            if let Ok(decoded)
                = <NoOutstandingVariableDebtCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NoOutstandingVariableDebt(decoded));
            }
            if let Ok(decoded)
                = <OperationNotSupportedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OperationNotSupported(decoded));
            }
            if let Ok(decoded)
                = <PoolAddressesDoNotMatchCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PoolAddressesDoNotMatch(decoded));
            }
            if let Ok(decoded)
                = <PriceOracleSentinelCheckFailedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PriceOracleSentinelCheckFailed(decoded));
            }
            if let Ok(decoded)
                = <ReserveAlreadyAddedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ReserveAlreadyAdded(decoded));
            }
            if let Ok(decoded)
                = <ReserveAlreadyInitializedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ReserveAlreadyInitialized(decoded));
            }
            if let Ok(decoded)
                = <ReserveDebtNotZeroCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ReserveDebtNotZero(decoded));
            }
            if let Ok(decoded)
                = <ReserveFrozenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReserveFrozen(decoded));
            }
            if let Ok(decoded)
                = <ReserveInactiveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReserveInactive(decoded));
            }
            if let Ok(decoded)
                = <ReserveLiquidityNotZeroCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ReserveLiquidityNotZero(decoded));
            }
            if let Ok(decoded)
                = <ReservePausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReservePaused(decoded));
            }
            if let Ok(decoded)
                = <SiloedBorrowingViolationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SiloedBorrowingViolation(decoded));
            }
            if let Ok(decoded)
                = <SpecifiedCurrencyNotBorrowedByUserCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SpecifiedCurrencyNotBorrowedByUser(decoded));
            }
            if let Ok(decoded)
                = <StableBorrowingEnabledCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::StableBorrowingEnabled(decoded));
            }
            if let Ok(decoded)
                = <StableBorrowingNotEnabledCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::StableBorrowingNotEnabled(decoded));
            }
            if let Ok(decoded)
                = <StableDebtNotZeroCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::StableDebtNotZero(decoded));
            }
            if let Ok(decoded)
                = <SupplyCapExceededCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupplyCapExceeded(decoded));
            }
            if let Ok(decoded)
                = <UnbackedMintCapExceededCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UnbackedMintCapExceeded(decoded));
            }
            if let Ok(decoded)
                = <UnderlyingBalanceZeroCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UnderlyingBalanceZero(decoded));
            }
            if let Ok(decoded)
                = <UnderlyingCannotBeRescuedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UnderlyingCannotBeRescued(decoded));
            }
            if let Ok(decoded)
                = <UnderlyingClaimableRightsNotZeroCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UnderlyingClaimableRightsNotZero(decoded));
            }
            if let Ok(decoded)
                = <UserInIsolationModeOrLtvZeroCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UserInIsolationModeOrLtvZero(decoded));
            }
            if let Ok(decoded)
                = <VariableDebtSupplyNotZeroCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VariableDebtSupplyNotZero(decoded));
            }
            if let Ok(decoded)
                = <ZeroAddressNotValidCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ZeroAddressNotValid(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ErrorsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AclAdminCannotBeZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressesProviderAlreadyAdded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressesProviderNotRegistered(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AmountBiggerThanMaxLoanSizeStable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssetNotBorrowableInIsolation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssetNotListed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BorrowingNotEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BorrowCapExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BridgeProtocolFeeInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerMustBePool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerNotAssetListingOrPoolAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerNotAtoken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerNotBridge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerNotEmergencyAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerNotPoolAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerNotPoolConfigurator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerNotPoolOrEmergencyAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerNotRiskOrPoolAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CollateralBalanceIsZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CollateralCannotBeLiquidated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CollateralCannotCoverNewBorrow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CollateralSameAsBorrowingCurrency(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DebtCeilingExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DebtCeilingNotZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmodeCategoryReserved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashloanDisabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashloanPremiumInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HealthFactorLowerThanLiquidationThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HealthFactorNotBelowThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InconsistentEmodeCategory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InconsistentFlashloanParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InconsistentParamsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InterestRateRebalanceConditionsNotMet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAddressesProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAddressesProviderId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidBorrowCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidBurnAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidDebtCeiling(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidEmodeCategory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidEmodeCategoryAssignment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidEmodeCategoryParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidExpiration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFlashloanExecutorReturn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInterestRateModeSelected(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidLiquidationProtocolFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidLiqBonus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidLiqThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidLtv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMintAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidOptimalStableToTotalDebtRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidOptimalUsageRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReserveFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReserveIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReserveParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSupplyCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidUnbackedMintCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LtvValidationFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughAvailableUserBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoDebtOfSelectedType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoExplicitAmountToRepayOnBehalf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoMoreReservesAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoOutstandingStableDebt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoOutstandingVariableDebt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperationNotSupported(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolAddressesDoNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PriceOracleSentinelCheckFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveAlreadyAdded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveAlreadyInitialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveDebtNotZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveFrozen(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveInactive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReserveLiquidityNotZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReservePaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SiloedBorrowingViolation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpecifiedCurrencyNotBorrowedByUser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StableBorrowingEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StableBorrowingNotEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StableDebtNotZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupplyCapExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnbackedMintCapExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnderlyingBalanceZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnderlyingCannotBeRescued(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnderlyingClaimableRightsNotZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserInIsolationModeOrLtvZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VariableDebtSupplyNotZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroAddressNotValid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ErrorsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AclAdminCannotBeZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddressesProviderAlreadyAdded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddressesProviderNotRegistered(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AmountBiggerThanMaxLoanSizeStable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssetNotBorrowableInIsolation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssetNotListed(element) => ::core::fmt::Display::fmt(element, f),
                Self::BorrowingNotEnabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BorrowCapExceeded(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeProtocolFeeInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallerMustBePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallerNotAssetListingOrPoolAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallerNotAtoken(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallerNotBridge(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallerNotEmergencyAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallerNotPoolAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallerNotPoolConfigurator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallerNotPoolOrEmergencyAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallerNotRiskOrPoolAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CollateralBalanceIsZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CollateralCannotBeLiquidated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CollateralCannotCoverNewBorrow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CollateralSameAsBorrowingCurrency(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DebtCeilingExceeded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DebtCeilingNotZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EmodeCategoryReserved(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FlashloanDisabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::FlashloanPremiumInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HealthFactorLowerThanLiquidationThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HealthFactorNotBelowThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InconsistentEmodeCategory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InconsistentFlashloanParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InconsistentParamsLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InterestRateRebalanceConditionsNotMet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidAddressesProvider(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidAddressesProviderId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidBorrowCap(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidBurnAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidDebtCeiling(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidEmodeCategory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidEmodeCategoryAssignment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidEmodeCategoryParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidExpiration(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidFlashloanExecutorReturn(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidInterestRateModeSelected(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidLiquidationProtocolFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidLiqBonus(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidLiqThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidLtv(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidMintAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidOptimalStableToTotalDebtRatio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidOptimalUsageRatio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidReserveFactor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidReserveIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidReserveParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSupplyCap(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUnbackedMintCap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LtvValidationFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotEnoughAvailableUserBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoDebtOfSelectedType(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoExplicitAmountToRepayOnBehalf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoMoreReservesAllowed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoOutstandingStableDebt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoOutstandingVariableDebt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperationNotSupported(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolAddressesDoNotMatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PriceOracleSentinelCheckFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveAlreadyAdded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveAlreadyInitialized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveDebtNotZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveInactive(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveLiquidityNotZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReservePaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::SiloedBorrowingViolation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SpecifiedCurrencyNotBorrowedByUser(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StableBorrowingEnabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StableBorrowingNotEnabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StableDebtNotZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplyCapExceeded(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnbackedMintCapExceeded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnderlyingBalanceZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnderlyingCannotBeRescued(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnderlyingClaimableRightsNotZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UserInIsolationModeOrLtvZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VariableDebtSupplyNotZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ZeroAddressNotValid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AclAdminCannotBeZeroCall> for ErrorsCalls {
        fn from(value: AclAdminCannotBeZeroCall) -> Self {
            Self::AclAdminCannotBeZero(value)
        }
    }
    impl ::core::convert::From<AddressesProviderAlreadyAddedCall> for ErrorsCalls {
        fn from(value: AddressesProviderAlreadyAddedCall) -> Self {
            Self::AddressesProviderAlreadyAdded(value)
        }
    }
    impl ::core::convert::From<AddressesProviderNotRegisteredCall> for ErrorsCalls {
        fn from(value: AddressesProviderNotRegisteredCall) -> Self {
            Self::AddressesProviderNotRegistered(value)
        }
    }
    impl ::core::convert::From<AmountBiggerThanMaxLoanSizeStableCall> for ErrorsCalls {
        fn from(value: AmountBiggerThanMaxLoanSizeStableCall) -> Self {
            Self::AmountBiggerThanMaxLoanSizeStable(value)
        }
    }
    impl ::core::convert::From<AssetNotBorrowableInIsolationCall> for ErrorsCalls {
        fn from(value: AssetNotBorrowableInIsolationCall) -> Self {
            Self::AssetNotBorrowableInIsolation(value)
        }
    }
    impl ::core::convert::From<AssetNotListedCall> for ErrorsCalls {
        fn from(value: AssetNotListedCall) -> Self {
            Self::AssetNotListed(value)
        }
    }
    impl ::core::convert::From<BorrowingNotEnabledCall> for ErrorsCalls {
        fn from(value: BorrowingNotEnabledCall) -> Self {
            Self::BorrowingNotEnabled(value)
        }
    }
    impl ::core::convert::From<BorrowCapExceededCall> for ErrorsCalls {
        fn from(value: BorrowCapExceededCall) -> Self {
            Self::BorrowCapExceeded(value)
        }
    }
    impl ::core::convert::From<BridgeProtocolFeeInvalidCall> for ErrorsCalls {
        fn from(value: BridgeProtocolFeeInvalidCall) -> Self {
            Self::BridgeProtocolFeeInvalid(value)
        }
    }
    impl ::core::convert::From<CallerMustBePoolCall> for ErrorsCalls {
        fn from(value: CallerMustBePoolCall) -> Self {
            Self::CallerMustBePool(value)
        }
    }
    impl ::core::convert::From<CallerNotAssetListingOrPoolAdminCall> for ErrorsCalls {
        fn from(value: CallerNotAssetListingOrPoolAdminCall) -> Self {
            Self::CallerNotAssetListingOrPoolAdmin(value)
        }
    }
    impl ::core::convert::From<CallerNotAtokenCall> for ErrorsCalls {
        fn from(value: CallerNotAtokenCall) -> Self {
            Self::CallerNotAtoken(value)
        }
    }
    impl ::core::convert::From<CallerNotBridgeCall> for ErrorsCalls {
        fn from(value: CallerNotBridgeCall) -> Self {
            Self::CallerNotBridge(value)
        }
    }
    impl ::core::convert::From<CallerNotEmergencyAdminCall> for ErrorsCalls {
        fn from(value: CallerNotEmergencyAdminCall) -> Self {
            Self::CallerNotEmergencyAdmin(value)
        }
    }
    impl ::core::convert::From<CallerNotPoolAdminCall> for ErrorsCalls {
        fn from(value: CallerNotPoolAdminCall) -> Self {
            Self::CallerNotPoolAdmin(value)
        }
    }
    impl ::core::convert::From<CallerNotPoolConfiguratorCall> for ErrorsCalls {
        fn from(value: CallerNotPoolConfiguratorCall) -> Self {
            Self::CallerNotPoolConfigurator(value)
        }
    }
    impl ::core::convert::From<CallerNotPoolOrEmergencyAdminCall> for ErrorsCalls {
        fn from(value: CallerNotPoolOrEmergencyAdminCall) -> Self {
            Self::CallerNotPoolOrEmergencyAdmin(value)
        }
    }
    impl ::core::convert::From<CallerNotRiskOrPoolAdminCall> for ErrorsCalls {
        fn from(value: CallerNotRiskOrPoolAdminCall) -> Self {
            Self::CallerNotRiskOrPoolAdmin(value)
        }
    }
    impl ::core::convert::From<CollateralBalanceIsZeroCall> for ErrorsCalls {
        fn from(value: CollateralBalanceIsZeroCall) -> Self {
            Self::CollateralBalanceIsZero(value)
        }
    }
    impl ::core::convert::From<CollateralCannotBeLiquidatedCall> for ErrorsCalls {
        fn from(value: CollateralCannotBeLiquidatedCall) -> Self {
            Self::CollateralCannotBeLiquidated(value)
        }
    }
    impl ::core::convert::From<CollateralCannotCoverNewBorrowCall> for ErrorsCalls {
        fn from(value: CollateralCannotCoverNewBorrowCall) -> Self {
            Self::CollateralCannotCoverNewBorrow(value)
        }
    }
    impl ::core::convert::From<CollateralSameAsBorrowingCurrencyCall> for ErrorsCalls {
        fn from(value: CollateralSameAsBorrowingCurrencyCall) -> Self {
            Self::CollateralSameAsBorrowingCurrency(value)
        }
    }
    impl ::core::convert::From<DebtCeilingExceededCall> for ErrorsCalls {
        fn from(value: DebtCeilingExceededCall) -> Self {
            Self::DebtCeilingExceeded(value)
        }
    }
    impl ::core::convert::From<DebtCeilingNotZeroCall> for ErrorsCalls {
        fn from(value: DebtCeilingNotZeroCall) -> Self {
            Self::DebtCeilingNotZero(value)
        }
    }
    impl ::core::convert::From<EmodeCategoryReservedCall> for ErrorsCalls {
        fn from(value: EmodeCategoryReservedCall) -> Self {
            Self::EmodeCategoryReserved(value)
        }
    }
    impl ::core::convert::From<FlashloanDisabledCall> for ErrorsCalls {
        fn from(value: FlashloanDisabledCall) -> Self {
            Self::FlashloanDisabled(value)
        }
    }
    impl ::core::convert::From<FlashloanPremiumInvalidCall> for ErrorsCalls {
        fn from(value: FlashloanPremiumInvalidCall) -> Self {
            Self::FlashloanPremiumInvalid(value)
        }
    }
    impl ::core::convert::From<HealthFactorLowerThanLiquidationThresholdCall>
    for ErrorsCalls {
        fn from(value: HealthFactorLowerThanLiquidationThresholdCall) -> Self {
            Self::HealthFactorLowerThanLiquidationThreshold(value)
        }
    }
    impl ::core::convert::From<HealthFactorNotBelowThresholdCall> for ErrorsCalls {
        fn from(value: HealthFactorNotBelowThresholdCall) -> Self {
            Self::HealthFactorNotBelowThreshold(value)
        }
    }
    impl ::core::convert::From<InconsistentEmodeCategoryCall> for ErrorsCalls {
        fn from(value: InconsistentEmodeCategoryCall) -> Self {
            Self::InconsistentEmodeCategory(value)
        }
    }
    impl ::core::convert::From<InconsistentFlashloanParamsCall> for ErrorsCalls {
        fn from(value: InconsistentFlashloanParamsCall) -> Self {
            Self::InconsistentFlashloanParams(value)
        }
    }
    impl ::core::convert::From<InconsistentParamsLengthCall> for ErrorsCalls {
        fn from(value: InconsistentParamsLengthCall) -> Self {
            Self::InconsistentParamsLength(value)
        }
    }
    impl ::core::convert::From<InterestRateRebalanceConditionsNotMetCall>
    for ErrorsCalls {
        fn from(value: InterestRateRebalanceConditionsNotMetCall) -> Self {
            Self::InterestRateRebalanceConditionsNotMet(value)
        }
    }
    impl ::core::convert::From<InvalidAddressesProviderCall> for ErrorsCalls {
        fn from(value: InvalidAddressesProviderCall) -> Self {
            Self::InvalidAddressesProvider(value)
        }
    }
    impl ::core::convert::From<InvalidAddressesProviderIdCall> for ErrorsCalls {
        fn from(value: InvalidAddressesProviderIdCall) -> Self {
            Self::InvalidAddressesProviderId(value)
        }
    }
    impl ::core::convert::From<InvalidAmountCall> for ErrorsCalls {
        fn from(value: InvalidAmountCall) -> Self {
            Self::InvalidAmount(value)
        }
    }
    impl ::core::convert::From<InvalidBorrowCapCall> for ErrorsCalls {
        fn from(value: InvalidBorrowCapCall) -> Self {
            Self::InvalidBorrowCap(value)
        }
    }
    impl ::core::convert::From<InvalidBurnAmountCall> for ErrorsCalls {
        fn from(value: InvalidBurnAmountCall) -> Self {
            Self::InvalidBurnAmount(value)
        }
    }
    impl ::core::convert::From<InvalidDebtCeilingCall> for ErrorsCalls {
        fn from(value: InvalidDebtCeilingCall) -> Self {
            Self::InvalidDebtCeiling(value)
        }
    }
    impl ::core::convert::From<InvalidDecimalsCall> for ErrorsCalls {
        fn from(value: InvalidDecimalsCall) -> Self {
            Self::InvalidDecimals(value)
        }
    }
    impl ::core::convert::From<InvalidEmodeCategoryCall> for ErrorsCalls {
        fn from(value: InvalidEmodeCategoryCall) -> Self {
            Self::InvalidEmodeCategory(value)
        }
    }
    impl ::core::convert::From<InvalidEmodeCategoryAssignmentCall> for ErrorsCalls {
        fn from(value: InvalidEmodeCategoryAssignmentCall) -> Self {
            Self::InvalidEmodeCategoryAssignment(value)
        }
    }
    impl ::core::convert::From<InvalidEmodeCategoryParamsCall> for ErrorsCalls {
        fn from(value: InvalidEmodeCategoryParamsCall) -> Self {
            Self::InvalidEmodeCategoryParams(value)
        }
    }
    impl ::core::convert::From<InvalidExpirationCall> for ErrorsCalls {
        fn from(value: InvalidExpirationCall) -> Self {
            Self::InvalidExpiration(value)
        }
    }
    impl ::core::convert::From<InvalidFlashloanExecutorReturnCall> for ErrorsCalls {
        fn from(value: InvalidFlashloanExecutorReturnCall) -> Self {
            Self::InvalidFlashloanExecutorReturn(value)
        }
    }
    impl ::core::convert::From<InvalidInterestRateModeSelectedCall> for ErrorsCalls {
        fn from(value: InvalidInterestRateModeSelectedCall) -> Self {
            Self::InvalidInterestRateModeSelected(value)
        }
    }
    impl ::core::convert::From<InvalidLiquidationProtocolFeeCall> for ErrorsCalls {
        fn from(value: InvalidLiquidationProtocolFeeCall) -> Self {
            Self::InvalidLiquidationProtocolFee(value)
        }
    }
    impl ::core::convert::From<InvalidLiqBonusCall> for ErrorsCalls {
        fn from(value: InvalidLiqBonusCall) -> Self {
            Self::InvalidLiqBonus(value)
        }
    }
    impl ::core::convert::From<InvalidLiqThresholdCall> for ErrorsCalls {
        fn from(value: InvalidLiqThresholdCall) -> Self {
            Self::InvalidLiqThreshold(value)
        }
    }
    impl ::core::convert::From<InvalidLtvCall> for ErrorsCalls {
        fn from(value: InvalidLtvCall) -> Self {
            Self::InvalidLtv(value)
        }
    }
    impl ::core::convert::From<InvalidMintAmountCall> for ErrorsCalls {
        fn from(value: InvalidMintAmountCall) -> Self {
            Self::InvalidMintAmount(value)
        }
    }
    impl ::core::convert::From<InvalidOptimalStableToTotalDebtRatioCall>
    for ErrorsCalls {
        fn from(value: InvalidOptimalStableToTotalDebtRatioCall) -> Self {
            Self::InvalidOptimalStableToTotalDebtRatio(value)
        }
    }
    impl ::core::convert::From<InvalidOptimalUsageRatioCall> for ErrorsCalls {
        fn from(value: InvalidOptimalUsageRatioCall) -> Self {
            Self::InvalidOptimalUsageRatio(value)
        }
    }
    impl ::core::convert::From<InvalidReserveFactorCall> for ErrorsCalls {
        fn from(value: InvalidReserveFactorCall) -> Self {
            Self::InvalidReserveFactor(value)
        }
    }
    impl ::core::convert::From<InvalidReserveIndexCall> for ErrorsCalls {
        fn from(value: InvalidReserveIndexCall) -> Self {
            Self::InvalidReserveIndex(value)
        }
    }
    impl ::core::convert::From<InvalidReserveParamsCall> for ErrorsCalls {
        fn from(value: InvalidReserveParamsCall) -> Self {
            Self::InvalidReserveParams(value)
        }
    }
    impl ::core::convert::From<InvalidSignatureCall> for ErrorsCalls {
        fn from(value: InvalidSignatureCall) -> Self {
            Self::InvalidSignature(value)
        }
    }
    impl ::core::convert::From<InvalidSupplyCapCall> for ErrorsCalls {
        fn from(value: InvalidSupplyCapCall) -> Self {
            Self::InvalidSupplyCap(value)
        }
    }
    impl ::core::convert::From<InvalidUnbackedMintCapCall> for ErrorsCalls {
        fn from(value: InvalidUnbackedMintCapCall) -> Self {
            Self::InvalidUnbackedMintCap(value)
        }
    }
    impl ::core::convert::From<LtvValidationFailedCall> for ErrorsCalls {
        fn from(value: LtvValidationFailedCall) -> Self {
            Self::LtvValidationFailed(value)
        }
    }
    impl ::core::convert::From<NotContractCall> for ErrorsCalls {
        fn from(value: NotContractCall) -> Self {
            Self::NotContract(value)
        }
    }
    impl ::core::convert::From<NotEnoughAvailableUserBalanceCall> for ErrorsCalls {
        fn from(value: NotEnoughAvailableUserBalanceCall) -> Self {
            Self::NotEnoughAvailableUserBalance(value)
        }
    }
    impl ::core::convert::From<NoDebtOfSelectedTypeCall> for ErrorsCalls {
        fn from(value: NoDebtOfSelectedTypeCall) -> Self {
            Self::NoDebtOfSelectedType(value)
        }
    }
    impl ::core::convert::From<NoExplicitAmountToRepayOnBehalfCall> for ErrorsCalls {
        fn from(value: NoExplicitAmountToRepayOnBehalfCall) -> Self {
            Self::NoExplicitAmountToRepayOnBehalf(value)
        }
    }
    impl ::core::convert::From<NoMoreReservesAllowedCall> for ErrorsCalls {
        fn from(value: NoMoreReservesAllowedCall) -> Self {
            Self::NoMoreReservesAllowed(value)
        }
    }
    impl ::core::convert::From<NoOutstandingStableDebtCall> for ErrorsCalls {
        fn from(value: NoOutstandingStableDebtCall) -> Self {
            Self::NoOutstandingStableDebt(value)
        }
    }
    impl ::core::convert::From<NoOutstandingVariableDebtCall> for ErrorsCalls {
        fn from(value: NoOutstandingVariableDebtCall) -> Self {
            Self::NoOutstandingVariableDebt(value)
        }
    }
    impl ::core::convert::From<OperationNotSupportedCall> for ErrorsCalls {
        fn from(value: OperationNotSupportedCall) -> Self {
            Self::OperationNotSupported(value)
        }
    }
    impl ::core::convert::From<PoolAddressesDoNotMatchCall> for ErrorsCalls {
        fn from(value: PoolAddressesDoNotMatchCall) -> Self {
            Self::PoolAddressesDoNotMatch(value)
        }
    }
    impl ::core::convert::From<PriceOracleSentinelCheckFailedCall> for ErrorsCalls {
        fn from(value: PriceOracleSentinelCheckFailedCall) -> Self {
            Self::PriceOracleSentinelCheckFailed(value)
        }
    }
    impl ::core::convert::From<ReserveAlreadyAddedCall> for ErrorsCalls {
        fn from(value: ReserveAlreadyAddedCall) -> Self {
            Self::ReserveAlreadyAdded(value)
        }
    }
    impl ::core::convert::From<ReserveAlreadyInitializedCall> for ErrorsCalls {
        fn from(value: ReserveAlreadyInitializedCall) -> Self {
            Self::ReserveAlreadyInitialized(value)
        }
    }
    impl ::core::convert::From<ReserveDebtNotZeroCall> for ErrorsCalls {
        fn from(value: ReserveDebtNotZeroCall) -> Self {
            Self::ReserveDebtNotZero(value)
        }
    }
    impl ::core::convert::From<ReserveFrozenCall> for ErrorsCalls {
        fn from(value: ReserveFrozenCall) -> Self {
            Self::ReserveFrozen(value)
        }
    }
    impl ::core::convert::From<ReserveInactiveCall> for ErrorsCalls {
        fn from(value: ReserveInactiveCall) -> Self {
            Self::ReserveInactive(value)
        }
    }
    impl ::core::convert::From<ReserveLiquidityNotZeroCall> for ErrorsCalls {
        fn from(value: ReserveLiquidityNotZeroCall) -> Self {
            Self::ReserveLiquidityNotZero(value)
        }
    }
    impl ::core::convert::From<ReservePausedCall> for ErrorsCalls {
        fn from(value: ReservePausedCall) -> Self {
            Self::ReservePaused(value)
        }
    }
    impl ::core::convert::From<SiloedBorrowingViolationCall> for ErrorsCalls {
        fn from(value: SiloedBorrowingViolationCall) -> Self {
            Self::SiloedBorrowingViolation(value)
        }
    }
    impl ::core::convert::From<SpecifiedCurrencyNotBorrowedByUserCall> for ErrorsCalls {
        fn from(value: SpecifiedCurrencyNotBorrowedByUserCall) -> Self {
            Self::SpecifiedCurrencyNotBorrowedByUser(value)
        }
    }
    impl ::core::convert::From<StableBorrowingEnabledCall> for ErrorsCalls {
        fn from(value: StableBorrowingEnabledCall) -> Self {
            Self::StableBorrowingEnabled(value)
        }
    }
    impl ::core::convert::From<StableBorrowingNotEnabledCall> for ErrorsCalls {
        fn from(value: StableBorrowingNotEnabledCall) -> Self {
            Self::StableBorrowingNotEnabled(value)
        }
    }
    impl ::core::convert::From<StableDebtNotZeroCall> for ErrorsCalls {
        fn from(value: StableDebtNotZeroCall) -> Self {
            Self::StableDebtNotZero(value)
        }
    }
    impl ::core::convert::From<SupplyCapExceededCall> for ErrorsCalls {
        fn from(value: SupplyCapExceededCall) -> Self {
            Self::SupplyCapExceeded(value)
        }
    }
    impl ::core::convert::From<UnbackedMintCapExceededCall> for ErrorsCalls {
        fn from(value: UnbackedMintCapExceededCall) -> Self {
            Self::UnbackedMintCapExceeded(value)
        }
    }
    impl ::core::convert::From<UnderlyingBalanceZeroCall> for ErrorsCalls {
        fn from(value: UnderlyingBalanceZeroCall) -> Self {
            Self::UnderlyingBalanceZero(value)
        }
    }
    impl ::core::convert::From<UnderlyingCannotBeRescuedCall> for ErrorsCalls {
        fn from(value: UnderlyingCannotBeRescuedCall) -> Self {
            Self::UnderlyingCannotBeRescued(value)
        }
    }
    impl ::core::convert::From<UnderlyingClaimableRightsNotZeroCall> for ErrorsCalls {
        fn from(value: UnderlyingClaimableRightsNotZeroCall) -> Self {
            Self::UnderlyingClaimableRightsNotZero(value)
        }
    }
    impl ::core::convert::From<UserInIsolationModeOrLtvZeroCall> for ErrorsCalls {
        fn from(value: UserInIsolationModeOrLtvZeroCall) -> Self {
            Self::UserInIsolationModeOrLtvZero(value)
        }
    }
    impl ::core::convert::From<VariableDebtSupplyNotZeroCall> for ErrorsCalls {
        fn from(value: VariableDebtSupplyNotZeroCall) -> Self {
            Self::VariableDebtSupplyNotZero(value)
        }
    }
    impl ::core::convert::From<ZeroAddressNotValidCall> for ErrorsCalls {
        fn from(value: ZeroAddressNotValidCall) -> Self {
            Self::ZeroAddressNotValid(value)
        }
    }
    ///Container type for all return fields from the `ACL_ADMIN_CANNOT_BE_ZERO` function with signature `ACL_ADMIN_CANNOT_BE_ZERO()` and selector `0xfd1828ff`
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
    pub struct AclAdminCannotBeZeroReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ADDRESSES_PROVIDER_ALREADY_ADDED` function with signature `ADDRESSES_PROVIDER_ALREADY_ADDED()` and selector `0x14dcfbbc`
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
    pub struct AddressesProviderAlreadyAddedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ADDRESSES_PROVIDER_NOT_REGISTERED` function with signature `ADDRESSES_PROVIDER_NOT_REGISTERED()` and selector `0xe02f07ee`
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
    pub struct AddressesProviderNotRegisteredReturn(pub ::std::string::String);
    ///Container type for all return fields from the `AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE` function with signature `AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE()` and selector `0xf07f6785`
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
    pub struct AmountBiggerThanMaxLoanSizeStableReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ASSET_NOT_BORROWABLE_IN_ISOLATION` function with signature `ASSET_NOT_BORROWABLE_IN_ISOLATION()` and selector `0x8596aad5`
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
    pub struct AssetNotBorrowableInIsolationReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ASSET_NOT_LISTED` function with signature `ASSET_NOT_LISTED()` and selector `0xcd23367c`
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
    pub struct AssetNotListedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `BORROWING_NOT_ENABLED` function with signature `BORROWING_NOT_ENABLED()` and selector `0x4ef999ff`
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
    pub struct BorrowingNotEnabledReturn(pub ::std::string::String);
    ///Container type for all return fields from the `BORROW_CAP_EXCEEDED` function with signature `BORROW_CAP_EXCEEDED()` and selector `0x2eed17e8`
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
    pub struct BorrowCapExceededReturn(pub ::std::string::String);
    ///Container type for all return fields from the `BRIDGE_PROTOCOL_FEE_INVALID` function with signature `BRIDGE_PROTOCOL_FEE_INVALID()` and selector `0x7aa0767e`
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
    pub struct BridgeProtocolFeeInvalidReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CALLER_MUST_BE_POOL` function with signature `CALLER_MUST_BE_POOL()` and selector `0x471df685`
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
    pub struct CallerMustBePoolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN` function with signature `CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN()` and selector `0x2c8e3b4c`
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
    pub struct CallerNotAssetListingOrPoolAdminReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CALLER_NOT_ATOKEN` function with signature `CALLER_NOT_ATOKEN()` and selector `0xa2e976c6`
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
    pub struct CallerNotAtokenReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CALLER_NOT_BRIDGE` function with signature `CALLER_NOT_BRIDGE()` and selector `0x4f77647b`
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
    pub struct CallerNotBridgeReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CALLER_NOT_EMERGENCY_ADMIN` function with signature `CALLER_NOT_EMERGENCY_ADMIN()` and selector `0x485c8ff6`
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
    pub struct CallerNotEmergencyAdminReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CALLER_NOT_POOL_ADMIN` function with signature `CALLER_NOT_POOL_ADMIN()` and selector `0xac753236`
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
    pub struct CallerNotPoolAdminReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CALLER_NOT_POOL_CONFIGURATOR` function with signature `CALLER_NOT_POOL_CONFIGURATOR()` and selector `0x61c111d2`
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
    pub struct CallerNotPoolConfiguratorReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CALLER_NOT_POOL_OR_EMERGENCY_ADMIN` function with signature `CALLER_NOT_POOL_OR_EMERGENCY_ADMIN()` and selector `0x26e7b312`
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
    pub struct CallerNotPoolOrEmergencyAdminReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CALLER_NOT_RISK_OR_POOL_ADMIN` function with signature `CALLER_NOT_RISK_OR_POOL_ADMIN()` and selector `0xb5e79366`
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
    pub struct CallerNotRiskOrPoolAdminReturn(pub ::std::string::String);
    ///Container type for all return fields from the `COLLATERAL_BALANCE_IS_ZERO` function with signature `COLLATERAL_BALANCE_IS_ZERO()` and selector `0x4e01e3c1`
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
    pub struct CollateralBalanceIsZeroReturn(pub ::std::string::String);
    ///Container type for all return fields from the `COLLATERAL_CANNOT_BE_LIQUIDATED` function with signature `COLLATERAL_CANNOT_BE_LIQUIDATED()` and selector `0x895f7dc8`
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
    pub struct CollateralCannotBeLiquidatedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `COLLATERAL_CANNOT_COVER_NEW_BORROW` function with signature `COLLATERAL_CANNOT_COVER_NEW_BORROW()` and selector `0xe3fa20f5`
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
    pub struct CollateralCannotCoverNewBorrowReturn(pub ::std::string::String);
    ///Container type for all return fields from the `COLLATERAL_SAME_AS_BORROWING_CURRENCY` function with signature `COLLATERAL_SAME_AS_BORROWING_CURRENCY()` and selector `0x8a344000`
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
    pub struct CollateralSameAsBorrowingCurrencyReturn(pub ::std::string::String);
    ///Container type for all return fields from the `DEBT_CEILING_EXCEEDED` function with signature `DEBT_CEILING_EXCEEDED()` and selector `0x65a83bab`
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
    pub struct DebtCeilingExceededReturn(pub ::std::string::String);
    ///Container type for all return fields from the `DEBT_CEILING_NOT_ZERO` function with signature `DEBT_CEILING_NOT_ZERO()` and selector `0xe4dd8b74`
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
    pub struct DebtCeilingNotZeroReturn(pub ::std::string::String);
    ///Container type for all return fields from the `EMODE_CATEGORY_RESERVED` function with signature `EMODE_CATEGORY_RESERVED()` and selector `0xf479ea11`
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
    pub struct EmodeCategoryReservedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `FLASHLOAN_DISABLED` function with signature `FLASHLOAN_DISABLED()` and selector `0x8aa3ca4c`
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
    pub struct FlashloanDisabledReturn(pub ::std::string::String);
    ///Container type for all return fields from the `FLASHLOAN_PREMIUM_INVALID` function with signature `FLASHLOAN_PREMIUM_INVALID()` and selector `0x747fa556`
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
    pub struct FlashloanPremiumInvalidReturn(pub ::std::string::String);
    ///Container type for all return fields from the `HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD` function with signature `HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD()` and selector `0x366eb54d`
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
    pub struct HealthFactorLowerThanLiquidationThresholdReturn(
        pub ::std::string::String,
    );
    ///Container type for all return fields from the `HEALTH_FACTOR_NOT_BELOW_THRESHOLD` function with signature `HEALTH_FACTOR_NOT_BELOW_THRESHOLD()` and selector `0x952633c5`
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
    pub struct HealthFactorNotBelowThresholdReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INCONSISTENT_EMODE_CATEGORY` function with signature `INCONSISTENT_EMODE_CATEGORY()` and selector `0x8f7722b2`
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
    pub struct InconsistentEmodeCategoryReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INCONSISTENT_FLASHLOAN_PARAMS` function with signature `INCONSISTENT_FLASHLOAN_PARAMS()` and selector `0x73dea5e3`
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
    pub struct InconsistentFlashloanParamsReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INCONSISTENT_PARAMS_LENGTH` function with signature `INCONSISTENT_PARAMS_LENGTH()` and selector `0xbad8308c`
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
    pub struct InconsistentParamsLengthReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET` function with signature `INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET()` and selector `0x2926c971`
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
    pub struct InterestRateRebalanceConditionsNotMetReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_ADDRESSES_PROVIDER` function with signature `INVALID_ADDRESSES_PROVIDER()` and selector `0x37930782`
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
    pub struct InvalidAddressesProviderReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_ADDRESSES_PROVIDER_ID` function with signature `INVALID_ADDRESSES_PROVIDER_ID()` and selector `0x60c3de80`
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
    pub struct InvalidAddressesProviderIdReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_AMOUNT` function with signature `INVALID_AMOUNT()` and selector `0xfae82791`
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
    pub struct InvalidAmountReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_BORROW_CAP` function with signature `INVALID_BORROW_CAP()` and selector `0xd6f9fcde`
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
    pub struct InvalidBorrowCapReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_BURN_AMOUNT` function with signature `INVALID_BURN_AMOUNT()` and selector `0x51267450`
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
    pub struct InvalidBurnAmountReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_DEBT_CEILING` function with signature `INVALID_DEBT_CEILING()` and selector `0xdcc56db6`
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
    pub struct InvalidDebtCeilingReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_DECIMALS` function with signature `INVALID_DECIMALS()` and selector `0xfa163a83`
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
    pub struct InvalidDecimalsReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_EMODE_CATEGORY` function with signature `INVALID_EMODE_CATEGORY()` and selector `0xa8c97853`
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
    pub struct InvalidEmodeCategoryReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_EMODE_CATEGORY_ASSIGNMENT` function with signature `INVALID_EMODE_CATEGORY_ASSIGNMENT()` and selector `0x5d9c76c0`
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
    pub struct InvalidEmodeCategoryAssignmentReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_EMODE_CATEGORY_PARAMS` function with signature `INVALID_EMODE_CATEGORY_PARAMS()` and selector `0x47cf1523`
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
    pub struct InvalidEmodeCategoryParamsReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_EXPIRATION` function with signature `INVALID_EXPIRATION()` and selector `0xc08a1146`
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
    pub struct InvalidExpirationReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_FLASHLOAN_EXECUTOR_RETURN` function with signature `INVALID_FLASHLOAN_EXECUTOR_RETURN()` and selector `0x7fea6f36`
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
    pub struct InvalidFlashloanExecutorReturnReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_INTEREST_RATE_MODE_SELECTED` function with signature `INVALID_INTEREST_RATE_MODE_SELECTED()` and selector `0x89c5d45f`
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
    pub struct InvalidInterestRateModeSelectedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_LIQUIDATION_PROTOCOL_FEE` function with signature `INVALID_LIQUIDATION_PROTOCOL_FEE()` and selector `0x8eda46bd`
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
    pub struct InvalidLiquidationProtocolFeeReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_LIQ_BONUS` function with signature `INVALID_LIQ_BONUS()` and selector `0x9527e9d9`
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
    pub struct InvalidLiqBonusReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_LIQ_THRESHOLD` function with signature `INVALID_LIQ_THRESHOLD()` and selector `0xdd1dd95f`
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
    pub struct InvalidLiqThresholdReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_LTV` function with signature `INVALID_LTV()` and selector `0x99ce53f3`
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
    pub struct InvalidLtvReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_MINT_AMOUNT` function with signature `INVALID_MINT_AMOUNT()` and selector `0xabd351b1`
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
    pub struct InvalidMintAmountReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO` function with signature `INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO()` and selector `0xc899301a`
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
    pub struct InvalidOptimalStableToTotalDebtRatioReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_OPTIMAL_USAGE_RATIO` function with signature `INVALID_OPTIMAL_USAGE_RATIO()` and selector `0x4e3aed37`
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
    pub struct InvalidOptimalUsageRatioReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_RESERVE_FACTOR` function with signature `INVALID_RESERVE_FACTOR()` and selector `0xa4868dca`
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
    pub struct InvalidReserveFactorReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_RESERVE_INDEX` function with signature `INVALID_RESERVE_INDEX()` and selector `0xd1cd8b1d`
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
    pub struct InvalidReserveIndexReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_RESERVE_PARAMS` function with signature `INVALID_RESERVE_PARAMS()` and selector `0x335763de`
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
    pub struct InvalidReserveParamsReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_SIGNATURE` function with signature `INVALID_SIGNATURE()` and selector `0xa3402a38`
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
    pub struct InvalidSignatureReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_SUPPLY_CAP` function with signature `INVALID_SUPPLY_CAP()` and selector `0x26bbd053`
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
    pub struct InvalidSupplyCapReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_UNBACKED_MINT_CAP` function with signature `INVALID_UNBACKED_MINT_CAP()` and selector `0x47ba93d8`
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
    pub struct InvalidUnbackedMintCapReturn(pub ::std::string::String);
    ///Container type for all return fields from the `LTV_VALIDATION_FAILED` function with signature `LTV_VALIDATION_FAILED()` and selector `0xb87041c2`
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
    pub struct LtvValidationFailedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `NOT_CONTRACT` function with signature `NOT_CONTRACT()` and selector `0x11d7b006`
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
    pub struct NotContractReturn(pub ::std::string::String);
    ///Container type for all return fields from the `NOT_ENOUGH_AVAILABLE_USER_BALANCE` function with signature `NOT_ENOUGH_AVAILABLE_USER_BALANCE()` and selector `0xb7f5e224`
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
    pub struct NotEnoughAvailableUserBalanceReturn(pub ::std::string::String);
    ///Container type for all return fields from the `NO_DEBT_OF_SELECTED_TYPE` function with signature `NO_DEBT_OF_SELECTED_TYPE()` and selector `0xdc191bd9`
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
    pub struct NoDebtOfSelectedTypeReturn(pub ::std::string::String);
    ///Container type for all return fields from the `NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF` function with signature `NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF()` and selector `0x712f536a`
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
    pub struct NoExplicitAmountToRepayOnBehalfReturn(pub ::std::string::String);
    ///Container type for all return fields from the `NO_MORE_RESERVES_ALLOWED` function with signature `NO_MORE_RESERVES_ALLOWED()` and selector `0x76ae8fca`
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
    pub struct NoMoreReservesAllowedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `NO_OUTSTANDING_STABLE_DEBT` function with signature `NO_OUTSTANDING_STABLE_DEBT()` and selector `0x74459b14`
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
    pub struct NoOutstandingStableDebtReturn(pub ::std::string::String);
    ///Container type for all return fields from the `NO_OUTSTANDING_VARIABLE_DEBT` function with signature `NO_OUTSTANDING_VARIABLE_DEBT()` and selector `0xb4a45730`
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
    pub struct NoOutstandingVariableDebtReturn(pub ::std::string::String);
    ///Container type for all return fields from the `OPERATION_NOT_SUPPORTED` function with signature `OPERATION_NOT_SUPPORTED()` and selector `0x8b8b98d7`
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
    pub struct OperationNotSupportedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `POOL_ADDRESSES_DO_NOT_MATCH` function with signature `POOL_ADDRESSES_DO_NOT_MATCH()` and selector `0x1abbb001`
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
    pub struct PoolAddressesDoNotMatchReturn(pub ::std::string::String);
    ///Container type for all return fields from the `PRICE_ORACLE_SENTINEL_CHECK_FAILED` function with signature `PRICE_ORACLE_SENTINEL_CHECK_FAILED()` and selector `0xc8638082`
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
    pub struct PriceOracleSentinelCheckFailedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `RESERVE_ALREADY_ADDED` function with signature `RESERVE_ALREADY_ADDED()` and selector `0x12dcade8`
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
    pub struct ReserveAlreadyAddedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `RESERVE_ALREADY_INITIALIZED` function with signature `RESERVE_ALREADY_INITIALIZED()` and selector `0xd9adda85`
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
    pub struct ReserveAlreadyInitializedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `RESERVE_DEBT_NOT_ZERO` function with signature `RESERVE_DEBT_NOT_ZERO()` and selector `0xe981483a`
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
    pub struct ReserveDebtNotZeroReturn(pub ::std::string::String);
    ///Container type for all return fields from the `RESERVE_FROZEN` function with signature `RESERVE_FROZEN()` and selector `0x6cd3cfbc`
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
    pub struct ReserveFrozenReturn(pub ::std::string::String);
    ///Container type for all return fields from the `RESERVE_INACTIVE` function with signature `RESERVE_INACTIVE()` and selector `0x52ba9dbe`
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
    pub struct ReserveInactiveReturn(pub ::std::string::String);
    ///Container type for all return fields from the `RESERVE_LIQUIDITY_NOT_ZERO` function with signature `RESERVE_LIQUIDITY_NOT_ZERO()` and selector `0x084dfa0d`
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
    pub struct ReserveLiquidityNotZeroReturn(pub ::std::string::String);
    ///Container type for all return fields from the `RESERVE_PAUSED` function with signature `RESERVE_PAUSED()` and selector `0xb68774e9`
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
    pub struct ReservePausedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `SILOED_BORROWING_VIOLATION` function with signature `SILOED_BORROWING_VIOLATION()` and selector `0xde24948c`
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
    pub struct SiloedBorrowingViolationReturn(pub ::std::string::String);
    ///Container type for all return fields from the `SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER` function with signature `SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER()` and selector `0x22a73446`
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
    pub struct SpecifiedCurrencyNotBorrowedByUserReturn(pub ::std::string::String);
    ///Container type for all return fields from the `STABLE_BORROWING_ENABLED` function with signature `STABLE_BORROWING_ENABLED()` and selector `0x198d6a6b`
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
    pub struct StableBorrowingEnabledReturn(pub ::std::string::String);
    ///Container type for all return fields from the `STABLE_BORROWING_NOT_ENABLED` function with signature `STABLE_BORROWING_NOT_ENABLED()` and selector `0x4d86f393`
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
    pub struct StableBorrowingNotEnabledReturn(pub ::std::string::String);
    ///Container type for all return fields from the `STABLE_DEBT_NOT_ZERO` function with signature `STABLE_DEBT_NOT_ZERO()` and selector `0x65e7ef4c`
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
    pub struct StableDebtNotZeroReturn(pub ::std::string::String);
    ///Container type for all return fields from the `SUPPLY_CAP_EXCEEDED` function with signature `SUPPLY_CAP_EXCEEDED()` and selector `0xb0510054`
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
    pub struct SupplyCapExceededReturn(pub ::std::string::String);
    ///Container type for all return fields from the `UNBACKED_MINT_CAP_EXCEEDED` function with signature `UNBACKED_MINT_CAP_EXCEEDED()` and selector `0x6b3f7cc7`
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
    pub struct UnbackedMintCapExceededReturn(pub ::std::string::String);
    ///Container type for all return fields from the `UNDERLYING_BALANCE_ZERO` function with signature `UNDERLYING_BALANCE_ZERO()` and selector `0xa2797c80`
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
    pub struct UnderlyingBalanceZeroReturn(pub ::std::string::String);
    ///Container type for all return fields from the `UNDERLYING_CANNOT_BE_RESCUED` function with signature `UNDERLYING_CANNOT_BE_RESCUED()` and selector `0xab883ca0`
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
    pub struct UnderlyingCannotBeRescuedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO` function with signature `UNDERLYING_CLAIMABLE_RIGHTS_NOT_ZERO()` and selector `0x94f9fd8a`
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
    pub struct UnderlyingClaimableRightsNotZeroReturn(pub ::std::string::String);
    ///Container type for all return fields from the `USER_IN_ISOLATION_MODE_OR_LTV_ZERO` function with signature `USER_IN_ISOLATION_MODE_OR_LTV_ZERO()` and selector `0x480702ae`
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
    pub struct UserInIsolationModeOrLtvZeroReturn(pub ::std::string::String);
    ///Container type for all return fields from the `VARIABLE_DEBT_SUPPLY_NOT_ZERO` function with signature `VARIABLE_DEBT_SUPPLY_NOT_ZERO()` and selector `0xf10727db`
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
    pub struct VariableDebtSupplyNotZeroReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ZERO_ADDRESS_NOT_VALID` function with signature `ZERO_ADDRESS_NOT_VALID()` and selector `0xd14bb17a`
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
    pub struct ZeroAddressNotValidReturn(pub ::std::string::String);
}
