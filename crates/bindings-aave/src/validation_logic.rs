pub use validation_logic::*;
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
pub mod validation_logic {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "HEALTH_FACTOR_LIQUIDATION_THRESHOLD",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HEALTH_FACTOR_LIQUIDATION_THRESHOLD",
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
                        "ISOLATED_COLLATERAL_SUPPLIER_ROLE",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ISOLATED_COLLATERAL_SUPPLIER_ROLE",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned(
                        "MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD",
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
                        "REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD",
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
    pub static VALIDATIONLOGIC_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE8a\09`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14a\0,WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10`QW`\x005`\xE0\x1C\x80c+\x019\xFA\x14`VW\x80cV\x1C\xBE\xC9\x14`\x8EW\x80c\xAB\xFC\xC8j\x14`\x9CW\x80c\xC3R\\(\x14`\xA4W[`\0\x80\xFD[`|\x7F\xD1\xD2\xCF\x86\x90\x16\x11*\x9A\xF1\x10{\xCFC\xC3u\x9D\xAF\"\xCFsJ\xADG\xD0\xC9\xC7&\xE3;\xC7\x82\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`|g\r/\x13\xF7x\x9F\0\0\x81V[`|a#(\x81V[`|g\r\xE0\xB6\xB3\xA7d\0\0\x81V\xFE\xA2dipfsX\"\x12 \xB6\xF2\xD6^\0\x97\x9C\xFF\x9C\x8AF\xFFYs\xC0~\x9F\x02\x07\x81hs*a\x0C&\xE2\xC7G\xF5\xD7\x16dsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static VALIDATIONLOGIC_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10`QW`\x005`\xE0\x1C\x80c+\x019\xFA\x14`VW\x80cV\x1C\xBE\xC9\x14`\x8EW\x80c\xAB\xFC\xC8j\x14`\x9CW\x80c\xC3R\\(\x14`\xA4W[`\0\x80\xFD[`|\x7F\xD1\xD2\xCF\x86\x90\x16\x11*\x9A\xF1\x10{\xCFC\xC3u\x9D\xAF\"\xCFsJ\xADG\xD0\xC9\xC7&\xE3;\xC7\x82\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`|g\r/\x13\xF7x\x9F\0\0\x81V[`|a#(\x81V[`|g\r\xE0\xB6\xB3\xA7d\0\0\x81V\xFE\xA2dipfsX\"\x12 \xB6\xF2\xD6^\0\x97\x9C\xFF\x9C\x8AF\xFFYs\xC0~\x9F\x02\x07\x81hs*a\x0C&\xE2\xC7G\xF5\xD7\x16dsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static VALIDATIONLOGIC_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ValidationLogic<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ValidationLogic<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ValidationLogic<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ValidationLogic<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ValidationLogic<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ValidationLogic))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ValidationLogic<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    VALIDATIONLOGIC_ABI.clone(),
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
                VALIDATIONLOGIC_ABI.clone(),
                VALIDATIONLOGIC_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `HEALTH_FACTOR_LIQUIDATION_THRESHOLD` (0xc3525c28) function
        pub fn health_factor_liquidation_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([195, 82, 92, 40], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ISOLATED_COLLATERAL_SUPPLIER_ROLE` (0x2b0139fa) function
        pub fn isolated_collateral_supplier_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([43, 1, 57, 250], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD` (0x561cbec9) function
        pub fn minimum_health_factor_liquidation_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([86, 28, 190, 201], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD` (0xabfcc86a) function
        pub fn rebalance_up_liquidity_rate_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([171, 252, 200, 106], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ValidationLogic<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `HEALTH_FACTOR_LIQUIDATION_THRESHOLD` function with signature `HEALTH_FACTOR_LIQUIDATION_THRESHOLD()` and selector `0xc3525c28`
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
        name = "HEALTH_FACTOR_LIQUIDATION_THRESHOLD",
        abi = "HEALTH_FACTOR_LIQUIDATION_THRESHOLD()"
    )]
    pub struct HealthFactorLiquidationThresholdCall;
    ///Container type for all input parameters for the `ISOLATED_COLLATERAL_SUPPLIER_ROLE` function with signature `ISOLATED_COLLATERAL_SUPPLIER_ROLE()` and selector `0x2b0139fa`
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
        name = "ISOLATED_COLLATERAL_SUPPLIER_ROLE",
        abi = "ISOLATED_COLLATERAL_SUPPLIER_ROLE()"
    )]
    pub struct IsolatedCollateralSupplierRoleCall;
    ///Container type for all input parameters for the `MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD` function with signature `MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD()` and selector `0x561cbec9`
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
        name = "MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD",
        abi = "MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD()"
    )]
    pub struct MinimumHealthFactorLiquidationThresholdCall;
    ///Container type for all input parameters for the `REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD` function with signature `REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD()` and selector `0xabfcc86a`
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
        name = "REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD",
        abi = "REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD()"
    )]
    pub struct RebalanceUpLiquidityRateThresholdCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ValidationLogicCalls {
        HealthFactorLiquidationThreshold(HealthFactorLiquidationThresholdCall),
        IsolatedCollateralSupplierRole(IsolatedCollateralSupplierRoleCall),
        MinimumHealthFactorLiquidationThreshold(
            MinimumHealthFactorLiquidationThresholdCall,
        ),
        RebalanceUpLiquidityRateThreshold(RebalanceUpLiquidityRateThresholdCall),
    }
    impl ::ethers::core::abi::AbiDecode for ValidationLogicCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <HealthFactorLiquidationThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::HealthFactorLiquidationThreshold(decoded));
            }
            if let Ok(decoded)
                = <IsolatedCollateralSupplierRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsolatedCollateralSupplierRole(decoded));
            }
            if let Ok(decoded)
                = <MinimumHealthFactorLiquidationThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MinimumHealthFactorLiquidationThreshold(decoded));
            }
            if let Ok(decoded)
                = <RebalanceUpLiquidityRateThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RebalanceUpLiquidityRateThreshold(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ValidationLogicCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::HealthFactorLiquidationThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsolatedCollateralSupplierRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinimumHealthFactorLiquidationThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceUpLiquidityRateThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ValidationLogicCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::HealthFactorLiquidationThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsolatedCollateralSupplierRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinimumHealthFactorLiquidationThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RebalanceUpLiquidityRateThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<HealthFactorLiquidationThresholdCall>
    for ValidationLogicCalls {
        fn from(value: HealthFactorLiquidationThresholdCall) -> Self {
            Self::HealthFactorLiquidationThreshold(value)
        }
    }
    impl ::core::convert::From<IsolatedCollateralSupplierRoleCall>
    for ValidationLogicCalls {
        fn from(value: IsolatedCollateralSupplierRoleCall) -> Self {
            Self::IsolatedCollateralSupplierRole(value)
        }
    }
    impl ::core::convert::From<MinimumHealthFactorLiquidationThresholdCall>
    for ValidationLogicCalls {
        fn from(value: MinimumHealthFactorLiquidationThresholdCall) -> Self {
            Self::MinimumHealthFactorLiquidationThreshold(value)
        }
    }
    impl ::core::convert::From<RebalanceUpLiquidityRateThresholdCall>
    for ValidationLogicCalls {
        fn from(value: RebalanceUpLiquidityRateThresholdCall) -> Self {
            Self::RebalanceUpLiquidityRateThreshold(value)
        }
    }
    ///Container type for all return fields from the `HEALTH_FACTOR_LIQUIDATION_THRESHOLD` function with signature `HEALTH_FACTOR_LIQUIDATION_THRESHOLD()` and selector `0xc3525c28`
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
    pub struct HealthFactorLiquidationThresholdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ISOLATED_COLLATERAL_SUPPLIER_ROLE` function with signature `ISOLATED_COLLATERAL_SUPPLIER_ROLE()` and selector `0x2b0139fa`
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
    pub struct IsolatedCollateralSupplierRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD` function with signature `MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD()` and selector `0x561cbec9`
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
    pub struct MinimumHealthFactorLiquidationThresholdReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD` function with signature `REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD()` and selector `0xabfcc86a`
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
    pub struct RebalanceUpLiquidityRateThresholdReturn(pub ::ethers::core::types::U256);
}
