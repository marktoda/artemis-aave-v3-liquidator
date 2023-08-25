pub use initializable_admin_upgradeability_proxy::*;
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
pub mod initializable_admin_upgradeability_proxy {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("admin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("admin"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("changeAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
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
                (
                    ::std::borrow::ToOwned::to_owned("implementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("implementation"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("logic"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_logic"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upgradeTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgradeTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newImplementation"),
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
                (
                    ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
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
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static INITIALIZABLEADMINUPGRADEABILITYPROXY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\t\xFE\x80a\0 `\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0pW`\x005`\xE0\x1C\x80c\x8F(9p\x11a\0NW\x80c\x8F(9p\x14a\0\xDEW\x80c\xCFz\x1Dw\x14a\0\xFEW\x80c\xD1\xF5x\x94\x14a\x01\x11W\x80c\xF8Q\xA4@\x14a\x01$Wa\0pV[\x80c6Y\xCF\xE6\x14a\0zW\x80cO\x1E\xF2\x86\x14a\0\x9AW\x80c\\`\xDA\x1B\x14a\0\xADW[a\0xa\x019V[\0[4\x80\x15a\0\x86W`\0\x80\xFD[Pa\0xa\0\x956`\x04a\x07\x0EV[a\x01aV[a\0xa\0\xA86`\x04a\x070V[a\x01\x9EV[4\x80\x15a\0\xB9W`\0\x80\xFD[Pa\0\xC2a\x02MV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xEAW`\0\x80\xFD[Pa\0xa\0\xF96`\x04a\x07\x0EV[a\x02\x9DV[a\0xa\x01\x0C6`\x04a\x08VV[a\x03\xAFV[a\0xa\x01\x1F6`\x04a\x08\xB4V[a\x04AV[4\x80\x15a\x010W`\0\x80\xFD[Pa\0\xC2a\x05/V[a\x01Aa\x05tV[a\x01_a\x01Z`\0\x80Q` a\t\xA9\x839\x81Q\x91RT\x90V[a\x05|V[V[`\0\x80Q` a\t\x89\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x01\x96Wa\x01\x93\x81a\x05\xA0V[PV[a\x01\x93a\x019V[`\0\x80Q` a\t\x89\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x02@Wa\x01\xD0\x83a\x05\xA0V[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Qa\x01\xEC\x92\x91\x90a\t\x02V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x02'W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02,V[``\x91P[PP\x90P\x80a\x02:W`\0\x80\xFD[PPPPV[a\x02Ha\x019V[PPPV[`\0a\x02e`\0\x80Q` a\t\x89\x839\x81Q\x91RT\x90V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x02\x92WP`\0\x80Q` a\t\xA9\x839\x81Q\x91RT\x90V[a\x02\x9Aa\x019V[\x90V[`\0\x80Q` a\t\x89\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x01\x96W`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FCannot change the admin of a pro`D\x82\x01Ruxy to the zero address`P\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x7F~dMyB/\x17\xC0\x1EH\x94\xB5\xF4\xF5\x88\xD31\xEB\xFA(e=B\xAE\x83-\xC5\x9E8\xC9y\x8Fa\x03w`\0\x80Q` a\t\x89\x839\x81Q\x91RT\x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x84\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1a\x01\x93\x81`\0\x80Q` a\t\x89\x839\x81Q\x91RUV[`\0a\x03\xC7`\0\x80Q` a\t\xA9\x839\x81Q\x91RT\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\xDAW`\0\x80\xFD[a\x03\xE4\x83\x82a\x04AV[a\x04\x0F`\x01\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x04a\t\x12V[`\0\x80Q` a\t\x89\x839\x81Q\x91R\x14a\x04+Wa\x04+a\t7V[a\x02H\x82`\0\x80Q` a\t\x89\x839\x81Q\x91RUV[`\0a\x04Y`\0\x80Q` a\t\xA9\x839\x81Q\x91RT\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04lW`\0\x80\xFD[a\x04\x97`\x01\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBDa\t\x12V[`\0\x80Q` a\t\xA9\x839\x81Q\x91R\x14a\x04\xB3Wa\x04\xB3a\t7V[a\x04\xBC\x82a\x05\xE0V[\x80Q\x15a\x05+W`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Qa\x04\xDD\x91\x90a\tMV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x05\x18W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05\x1DV[``\x91P[PP\x90P\x80a\x02HW`\0\x80\xFD[PPV[`\0a\x05G`\0\x80Q` a\t\x89\x839\x81Q\x91RT\x90V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x02\x92WP`\0\x80Q` a\t\x89\x839\x81Q\x91RT\x90V[a\x01_a\x06fV[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15a\x05\x9BW=`\0\xF3[=`\0\xFD[a\x05\xA9\x81a\x05\xE0V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[\x80;a\x06TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FCannot set a proxy implementatio`D\x82\x01R\x7Fn to a non-contract address\0\0\0\0\0`d\x82\x01R`\x84\x01a\x037V[`\0\x80Q` a\t\xA9\x839\x81Q\x91RUV[`\0\x80Q` a\t\x89\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x01_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FCannot call fallback function fr`D\x82\x01Rq7\xB6\x90:42\x90897\xBC<\x900\xB26\xB4\xB7`q\x1B`d\x82\x01R`\x84\x01a\x037V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\tW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x07 W`\0\x80\xFD[a\x07)\x82a\x06\xF2V[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x07EW`\0\x80\xFD[a\x07N\x84a\x06\xF2V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07kW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x07\x7FW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x07\x8EW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x07\xA0W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x07\xDAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xF5Wa\x07\xF5a\x07\xB3V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x08\x1DWa\x08\x1Da\x07\xB3V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x086W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x08kW`\0\x80\xFD[a\x08t\x84a\x06\xF2V[\x92Pa\x08\x82` \x85\x01a\x06\xF2V[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x9EW`\0\x80\xFD[a\x08\xAA\x86\x82\x87\x01a\x07\xC9V[\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x08\xC7W`\0\x80\xFD[a\x08\xD0\x83a\x06\xF2V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xECW`\0\x80\xFD[a\x08\xF8\x85\x82\x86\x01a\x07\xC9V[\x91PP\x92P\x92\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x82\x82\x10\x15a\t2WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x03\x90V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x82Q`\0[\x81\x81\x10\x15a\tnW` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\tTV[\x81\x81\x11\x15a\t}W`\0\x82\x85\x01R[P\x91\x90\x91\x01\x92\x91PPV\xFE\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x036\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA2dipfsX\"\x12 YJ\xCF<\xCFV\xDB\\7\xE9\xB3\xC2\x14\xE3O<6^\xB9>\xE4\xCE*?\xBE\xFA\xF3\xFB\x87\xFA\xFB\x96dsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static INITIALIZABLEADMINUPGRADEABILITYPROXY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0pW`\x005`\xE0\x1C\x80c\x8F(9p\x11a\0NW\x80c\x8F(9p\x14a\0\xDEW\x80c\xCFz\x1Dw\x14a\0\xFEW\x80c\xD1\xF5x\x94\x14a\x01\x11W\x80c\xF8Q\xA4@\x14a\x01$Wa\0pV[\x80c6Y\xCF\xE6\x14a\0zW\x80cO\x1E\xF2\x86\x14a\0\x9AW\x80c\\`\xDA\x1B\x14a\0\xADW[a\0xa\x019V[\0[4\x80\x15a\0\x86W`\0\x80\xFD[Pa\0xa\0\x956`\x04a\x07\x0EV[a\x01aV[a\0xa\0\xA86`\x04a\x070V[a\x01\x9EV[4\x80\x15a\0\xB9W`\0\x80\xFD[Pa\0\xC2a\x02MV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xEAW`\0\x80\xFD[Pa\0xa\0\xF96`\x04a\x07\x0EV[a\x02\x9DV[a\0xa\x01\x0C6`\x04a\x08VV[a\x03\xAFV[a\0xa\x01\x1F6`\x04a\x08\xB4V[a\x04AV[4\x80\x15a\x010W`\0\x80\xFD[Pa\0\xC2a\x05/V[a\x01Aa\x05tV[a\x01_a\x01Z`\0\x80Q` a\t\xA9\x839\x81Q\x91RT\x90V[a\x05|V[V[`\0\x80Q` a\t\x89\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x01\x96Wa\x01\x93\x81a\x05\xA0V[PV[a\x01\x93a\x019V[`\0\x80Q` a\t\x89\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x02@Wa\x01\xD0\x83a\x05\xA0V[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Qa\x01\xEC\x92\x91\x90a\t\x02V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x02'W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02,V[``\x91P[PP\x90P\x80a\x02:W`\0\x80\xFD[PPPPV[a\x02Ha\x019V[PPPV[`\0a\x02e`\0\x80Q` a\t\x89\x839\x81Q\x91RT\x90V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x02\x92WP`\0\x80Q` a\t\xA9\x839\x81Q\x91RT\x90V[a\x02\x9Aa\x019V[\x90V[`\0\x80Q` a\t\x89\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x01\x96W`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FCannot change the admin of a pro`D\x82\x01Ruxy to the zero address`P\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x7F~dMyB/\x17\xC0\x1EH\x94\xB5\xF4\xF5\x88\xD31\xEB\xFA(e=B\xAE\x83-\xC5\x9E8\xC9y\x8Fa\x03w`\0\x80Q` a\t\x89\x839\x81Q\x91RT\x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x84\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1a\x01\x93\x81`\0\x80Q` a\t\x89\x839\x81Q\x91RUV[`\0a\x03\xC7`\0\x80Q` a\t\xA9\x839\x81Q\x91RT\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\xDAW`\0\x80\xFD[a\x03\xE4\x83\x82a\x04AV[a\x04\x0F`\x01\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x04a\t\x12V[`\0\x80Q` a\t\x89\x839\x81Q\x91R\x14a\x04+Wa\x04+a\t7V[a\x02H\x82`\0\x80Q` a\t\x89\x839\x81Q\x91RUV[`\0a\x04Y`\0\x80Q` a\t\xA9\x839\x81Q\x91RT\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04lW`\0\x80\xFD[a\x04\x97`\x01\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBDa\t\x12V[`\0\x80Q` a\t\xA9\x839\x81Q\x91R\x14a\x04\xB3Wa\x04\xB3a\t7V[a\x04\xBC\x82a\x05\xE0V[\x80Q\x15a\x05+W`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Qa\x04\xDD\x91\x90a\tMV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x05\x18W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05\x1DV[``\x91P[PP\x90P\x80a\x02HW`\0\x80\xFD[PPV[`\0a\x05G`\0\x80Q` a\t\x89\x839\x81Q\x91RT\x90V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x02\x92WP`\0\x80Q` a\t\x89\x839\x81Q\x91RT\x90V[a\x01_a\x06fV[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15a\x05\x9BW=`\0\xF3[=`\0\xFD[a\x05\xA9\x81a\x05\xE0V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[\x80;a\x06TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FCannot set a proxy implementatio`D\x82\x01R\x7Fn to a non-contract address\0\0\0\0\0`d\x82\x01R`\x84\x01a\x037V[`\0\x80Q` a\t\xA9\x839\x81Q\x91RUV[`\0\x80Q` a\t\x89\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x01_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FCannot call fallback function fr`D\x82\x01Rq7\xB6\x90:42\x90897\xBC<\x900\xB26\xB4\xB7`q\x1B`d\x82\x01R`\x84\x01a\x037V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\tW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x07 W`\0\x80\xFD[a\x07)\x82a\x06\xF2V[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x07EW`\0\x80\xFD[a\x07N\x84a\x06\xF2V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07kW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x07\x7FW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x07\x8EW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x07\xA0W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x07\xDAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xF5Wa\x07\xF5a\x07\xB3V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x08\x1DWa\x08\x1Da\x07\xB3V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x086W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x08kW`\0\x80\xFD[a\x08t\x84a\x06\xF2V[\x92Pa\x08\x82` \x85\x01a\x06\xF2V[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x9EW`\0\x80\xFD[a\x08\xAA\x86\x82\x87\x01a\x07\xC9V[\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x08\xC7W`\0\x80\xFD[a\x08\xD0\x83a\x06\xF2V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xECW`\0\x80\xFD[a\x08\xF8\x85\x82\x86\x01a\x07\xC9V[\x91PP\x92P\x92\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x82\x82\x10\x15a\t2WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x03\x90V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x82Q`\0[\x81\x81\x10\x15a\tnW` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\tTV[\x81\x81\x11\x15a\t}W`\0\x82\x85\x01R[P\x91\x90\x91\x01\x92\x91PPV\xFE\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x036\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA2dipfsX\"\x12 YJ\xCF<\xCFV\xDB\\7\xE9\xB3\xC2\x14\xE3O<6^\xB9>\xE4\xCE*?\xBE\xFA\xF3\xFB\x87\xFA\xFB\x96dsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static INITIALIZABLEADMINUPGRADEABILITYPROXY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct InitializableAdminUpgradeabilityProxy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for InitializableAdminUpgradeabilityProxy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for InitializableAdminUpgradeabilityProxy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for InitializableAdminUpgradeabilityProxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for InitializableAdminUpgradeabilityProxy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(InitializableAdminUpgradeabilityProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> InitializableAdminUpgradeabilityProxy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    INITIALIZABLEADMINUPGRADEABILITYPROXY_ABI.clone(),
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
                INITIALIZABLEADMINUPGRADEABILITYPROXY_ABI.clone(),
                INITIALIZABLEADMINUPGRADEABILITYPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `admin` (0xf851a440) function
        pub fn admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeAdmin` (0x8f283970) function
        pub fn change_admin(
            &self,
            new_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 40, 57, 112], new_admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `implementation` (0x5c60da1b) function
        pub fn implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([92, 96, 218, 27], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xcf7a1d77) function
        pub fn initialize_with_logic_and_admin(
            &self,
            logic: ::ethers::core::types::Address,
            admin: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 122, 29, 119], (logic, admin, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xd1f57894) function
        pub fn initialize(
            &self,
            logic: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 245, 120, 148], (logic, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeTo` (0x3659cfe6) function
        pub fn upgrade_to(
            &self,
            new_implementation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 89, 207, 230], new_implementation)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeToAndCall` (0x4f1ef286) function
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AdminChanged` event
        pub fn admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpgradedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializableAdminUpgradeabilityProxyEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for InitializableAdminUpgradeabilityProxy<M> {
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
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        pub previous_admin: ::ethers::core::types::Address,
        pub new_admin: ::ethers::core::types::Address,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum InitializableAdminUpgradeabilityProxyEvents {
        AdminChangedFilter(AdminChangedFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode
    for InitializableAdminUpgradeabilityProxyEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(
                    InitializableAdminUpgradeabilityProxyEvents::AdminChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(
                    InitializableAdminUpgradeabilityProxyEvents::UpgradedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for InitializableAdminUpgradeabilityProxyEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminChangedFilter>
    for InitializableAdminUpgradeabilityProxyEvents {
        fn from(value: AdminChangedFilter) -> Self {
            Self::AdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter>
    for InitializableAdminUpgradeabilityProxyEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `admin` function with signature `admin()` and selector `0xf851a440`
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
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    ///Container type for all input parameters for the `changeAdmin` function with signature `changeAdmin(address)` and selector `0x8f283970`
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
    #[ethcall(name = "changeAdmin", abi = "changeAdmin(address)")]
    pub struct ChangeAdminCall {
        pub new_admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `implementation` function with signature `implementation()` and selector `0x5c60da1b`
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
    #[ethcall(name = "implementation", abi = "implementation()")]
    pub struct ImplementationCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,bytes)` and selector `0xcf7a1d77`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address,bytes)")]
    pub struct InitializeWithLogicAndAdminCall {
        pub logic: ::ethers::core::types::Address,
        pub admin: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,bytes)` and selector `0xd1f57894`
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
    #[ethcall(name = "initialize", abi = "initialize(address,bytes)")]
    pub struct InitializeCall {
        pub logic: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `upgradeTo` function with signature `upgradeTo(address)` and selector `0x3659cfe6`
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
    #[ethcall(name = "upgradeTo", abi = "upgradeTo(address)")]
    pub struct UpgradeToCall {
        pub new_implementation: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`
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
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum InitializableAdminUpgradeabilityProxyCalls {
        Admin(AdminCall),
        ChangeAdmin(ChangeAdminCall),
        Implementation(ImplementationCall),
        InitializeWithLogicAndAdmin(InitializeWithLogicAndAdminCall),
        Initialize(InitializeCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
    }
    impl ::ethers::core::abi::AbiDecode for InitializableAdminUpgradeabilityProxyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Admin(decoded));
            }
            if let Ok(decoded)
                = <ChangeAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChangeAdmin(decoded));
            }
            if let Ok(decoded)
                = <ImplementationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Implementation(decoded));
            }
            if let Ok(decoded)
                = <InitializeWithLogicAndAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InitializeWithLogicAndAdmin(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <UpgradeToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpgradeTo(decoded));
            }
            if let Ok(decoded)
                = <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for InitializableAdminUpgradeabilityProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Admin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ChangeAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Implementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializeWithLogicAndAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeToAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for InitializableAdminUpgradeabilityProxyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::Implementation(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializeWithLogicAndAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminCall>
    for InitializableAdminUpgradeabilityProxyCalls {
        fn from(value: AdminCall) -> Self {
            Self::Admin(value)
        }
    }
    impl ::core::convert::From<ChangeAdminCall>
    for InitializableAdminUpgradeabilityProxyCalls {
        fn from(value: ChangeAdminCall) -> Self {
            Self::ChangeAdmin(value)
        }
    }
    impl ::core::convert::From<ImplementationCall>
    for InitializableAdminUpgradeabilityProxyCalls {
        fn from(value: ImplementationCall) -> Self {
            Self::Implementation(value)
        }
    }
    impl ::core::convert::From<InitializeWithLogicAndAdminCall>
    for InitializableAdminUpgradeabilityProxyCalls {
        fn from(value: InitializeWithLogicAndAdminCall) -> Self {
            Self::InitializeWithLogicAndAdmin(value)
        }
    }
    impl ::core::convert::From<InitializeCall>
    for InitializableAdminUpgradeabilityProxyCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<UpgradeToCall>
    for InitializableAdminUpgradeabilityProxyCalls {
        fn from(value: UpgradeToCall) -> Self {
            Self::UpgradeTo(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall>
    for InitializableAdminUpgradeabilityProxyCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    ///Container type for all return fields from the `admin` function with signature `admin()` and selector `0xf851a440`
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
    pub struct AdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `implementation` function with signature `implementation()` and selector `0x5c60da1b`
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
    pub struct ImplementationReturn(pub ::ethers::core::types::Address);
}
