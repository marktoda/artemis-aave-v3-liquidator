pub use price_oracle::*;
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
pub mod price_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("getEthUsdPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getEthUsdPrice"),
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
                    ::std::borrow::ToOwned::to_owned("setAssetPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAssetPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
                    ::std::borrow::ToOwned::to_owned("setEthUsdPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setEthUsdPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AssetPriceUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssetPriceUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
                    ::std::borrow::ToOwned::to_owned("EthPriceUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EthPriceUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PRICEORACLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x02\n\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80cQ2?r\x14a\0QW\x80c\xA0\xA8\x04^\x14a\0fW\x80c\xB3Yo\x07\x14a\0|W\x80c\xB9Q\x88:\x14a\0\xA5W[`\0\x80\xFD[a\0da\0_6`\x04a\x01oV[a\0\xB8V[\0[`\x01T[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0ja\0\x8A6`\x04a\x01\x99V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\0da\0\xB36`\x04a\x01\xBBV[a\x01\x13V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x84\x90U\x81Q\x92\x83R\x82\x01\x83\x90RB\x82\x82\x01RQ\x7F\xCEn\x0BW6{\xAE\x95\xCAq\x98\xE1\x17/e>\xA6Jd\\\x16\xABXkL\xEF\xA9#{\xFC-\x92\x91\x81\x90\x03``\x01\x90\xA1PPV[`\x01\x81\x90U`@\x80Q\x82\x81RB` \x82\x01R\x7F\xB4\xF3Yw\x93\x9F\xA8\xB5\xFF\xE5R\xD5\x17\xA8\xFFR#\x04k\x1F\xDD>\xE0\x06\x8A\xE3\x8D\x1E+\x8D\0\x16\x91\x01`@Q\x80\x91\x03\x90\xA1PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01jW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x01\x82W`\0\x80\xFD[a\x01\x8B\x83a\x01SV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x01\xABW`\0\x80\xFD[a\x01\xB4\x82a\x01SV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x01\xCDW`\0\x80\xFD[P5\x91\x90PV\xFE\xA2dipfsX\"\x12 kS3*v\xF1@\xDA9\xF9\xDD\xEE\x9CC\xCC\x94\xC8\xF8uL\x17\xE5\xC1^c\x07W\x02\x07 xLdsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static PRICEORACLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80cQ2?r\x14a\0QW\x80c\xA0\xA8\x04^\x14a\0fW\x80c\xB3Yo\x07\x14a\0|W\x80c\xB9Q\x88:\x14a\0\xA5W[`\0\x80\xFD[a\0da\0_6`\x04a\x01oV[a\0\xB8V[\0[`\x01T[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0ja\0\x8A6`\x04a\x01\x99V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\0da\0\xB36`\x04a\x01\xBBV[a\x01\x13V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x84\x90U\x81Q\x92\x83R\x82\x01\x83\x90RB\x82\x82\x01RQ\x7F\xCEn\x0BW6{\xAE\x95\xCAq\x98\xE1\x17/e>\xA6Jd\\\x16\xABXkL\xEF\xA9#{\xFC-\x92\x91\x81\x90\x03``\x01\x90\xA1PPV[`\x01\x81\x90U`@\x80Q\x82\x81RB` \x82\x01R\x7F\xB4\xF3Yw\x93\x9F\xA8\xB5\xFF\xE5R\xD5\x17\xA8\xFFR#\x04k\x1F\xDD>\xE0\x06\x8A\xE3\x8D\x1E+\x8D\0\x16\x91\x01`@Q\x80\x91\x03\x90\xA1PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01jW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x01\x82W`\0\x80\xFD[a\x01\x8B\x83a\x01SV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x01\xABW`\0\x80\xFD[a\x01\xB4\x82a\x01SV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x01\xCDW`\0\x80\xFD[P5\x91\x90PV\xFE\xA2dipfsX\"\x12 kS3*v\xF1@\xDA9\xF9\xDD\xEE\x9CC\xCC\x94\xC8\xF8uL\x17\xE5\xC1^c\x07W\x02\x07 xLdsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static PRICEORACLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PriceOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PriceOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PriceOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PriceOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PriceOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PriceOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PriceOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PRICEORACLE_ABI.clone(),
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
                PRICEORACLE_ABI.clone(),
                PRICEORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `getEthUsdPrice` (0xa0a8045e) function
        pub fn get_eth_usd_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([160, 168, 4, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAssetPrice` (0x51323f72) function
        pub fn set_asset_price(
            &self,
            asset: ::ethers::core::types::Address,
            price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 50, 63, 114], (asset, price))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEthUsdPrice` (0xb951883a) function
        pub fn set_eth_usd_price(
            &self,
            price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 81, 136, 58], price)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AssetPriceUpdated` event
        pub fn asset_price_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssetPriceUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EthPriceUpdated` event
        pub fn eth_price_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EthPriceUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PriceOracleEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PriceOracle<M> {
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
    #[ethevent(
        name = "AssetPriceUpdated",
        abi = "AssetPriceUpdated(address,uint256,uint256)"
    )]
    pub struct AssetPriceUpdatedFilter {
        pub asset: ::ethers::core::types::Address,
        pub price: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
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
    #[ethevent(name = "EthPriceUpdated", abi = "EthPriceUpdated(uint256,uint256)")]
    pub struct EthPriceUpdatedFilter {
        pub price: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PriceOracleEvents {
        AssetPriceUpdatedFilter(AssetPriceUpdatedFilter),
        EthPriceUpdatedFilter(EthPriceUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for PriceOracleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssetPriceUpdatedFilter::decode_log(log) {
                return Ok(PriceOracleEvents::AssetPriceUpdatedFilter(decoded));
            }
            if let Ok(decoded) = EthPriceUpdatedFilter::decode_log(log) {
                return Ok(PriceOracleEvents::EthPriceUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PriceOracleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetPriceUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EthPriceUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AssetPriceUpdatedFilter> for PriceOracleEvents {
        fn from(value: AssetPriceUpdatedFilter) -> Self {
            Self::AssetPriceUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<EthPriceUpdatedFilter> for PriceOracleEvents {
        fn from(value: EthPriceUpdatedFilter) -> Self {
            Self::EthPriceUpdatedFilter(value)
        }
    }
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
    ///Container type for all input parameters for the `getEthUsdPrice` function with signature `getEthUsdPrice()` and selector `0xa0a8045e`
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
    #[ethcall(name = "getEthUsdPrice", abi = "getEthUsdPrice()")]
    pub struct GetEthUsdPriceCall;
    ///Container type for all input parameters for the `setAssetPrice` function with signature `setAssetPrice(address,uint256)` and selector `0x51323f72`
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
    #[ethcall(name = "setAssetPrice", abi = "setAssetPrice(address,uint256)")]
    pub struct SetAssetPriceCall {
        pub asset: ::ethers::core::types::Address,
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setEthUsdPrice` function with signature `setEthUsdPrice(uint256)` and selector `0xb951883a`
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
    #[ethcall(name = "setEthUsdPrice", abi = "setEthUsdPrice(uint256)")]
    pub struct SetEthUsdPriceCall {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PriceOracleCalls {
        GetAssetPrice(GetAssetPriceCall),
        GetEthUsdPrice(GetEthUsdPriceCall),
        SetAssetPrice(SetAssetPriceCall),
        SetEthUsdPrice(SetEthUsdPriceCall),
    }
    impl ::ethers::core::abi::AbiDecode for PriceOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetAssetPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAssetPrice(decoded));
            }
            if let Ok(decoded)
                = <GetEthUsdPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetEthUsdPrice(decoded));
            }
            if let Ok(decoded)
                = <SetAssetPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetAssetPrice(decoded));
            }
            if let Ok(decoded)
                = <SetEthUsdPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetEthUsdPrice(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PriceOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetAssetPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetEthUsdPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAssetPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEthUsdPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PriceOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetAssetPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEthUsdPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAssetPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEthUsdPrice(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetAssetPriceCall> for PriceOracleCalls {
        fn from(value: GetAssetPriceCall) -> Self {
            Self::GetAssetPrice(value)
        }
    }
    impl ::core::convert::From<GetEthUsdPriceCall> for PriceOracleCalls {
        fn from(value: GetEthUsdPriceCall) -> Self {
            Self::GetEthUsdPrice(value)
        }
    }
    impl ::core::convert::From<SetAssetPriceCall> for PriceOracleCalls {
        fn from(value: SetAssetPriceCall) -> Self {
            Self::SetAssetPrice(value)
        }
    }
    impl ::core::convert::From<SetEthUsdPriceCall> for PriceOracleCalls {
        fn from(value: SetEthUsdPriceCall) -> Self {
            Self::SetEthUsdPrice(value)
        }
    }
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
    ///Container type for all return fields from the `getEthUsdPrice` function with signature `getEthUsdPrice()` and selector `0xa0a8045e`
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
    pub struct GetEthUsdPriceReturn(pub ::ethers::core::types::U256);
}
