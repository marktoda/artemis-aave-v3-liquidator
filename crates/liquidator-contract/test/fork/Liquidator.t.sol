// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC20} from "solmate/tokens/ERC20.sol";
import {Test, console2} from "forge-std/Test.sol";
import {MockOracle} from "../util/MockOracle.sol";
import {Liquidator} from "../../src/Liquidator.sol";
import {IL2Pool} from "../../src/interfaces/IL2Pool.sol";
import {IL2Encoder} from "../../src/interfaces/IL2Encoder.sol";
import {IAaveOracle} from "../../src/interfaces/IAaveOracle.sol";
import {IPoolAddressesProvider} from "../../src/interfaces/IAddressesProvider.sol";
import {IPoolDataProvider} from "../../src/interfaces/IPoolDataProvider.sol";
import {IQuoterV2} from "../../src/interfaces/IQuoterV2.sol";

contract LiquidatorTest is Test {
    ERC20 constant aweth = ERC20(0xD4a0e0b9149BCee3C920d2E00b5dE09138fd8bb7);
    ERC20 constant ausdc = ERC20(0x0a1d576f3eFeF75b330424287a95A366e8281D54);
    ERC20 constant weth = ERC20(0x4200000000000000000000000000000000000006);
    ERC20 constant usdc = ERC20(0xd9aAEc86B65D86f6A7B5B1b0c42FFA531710b6CA);
    IQuoterV2 constant quoter = IQuoterV2(0x3d4e44Eb1374240CE5F1B871ab261CD16335B76a);
    IL2Encoder constant encoder = IL2Encoder(0x39e97c588B2907Fb67F44fea256Ae3BA064207C5);
    // IAaveOracle constant oracle = IAaveOracle(0x2Cc0Fc26eD4563A5ce5e8bdcfe1A2878676Ae156);
    IPoolDataProvider constant dataProvider = IPoolDataProvider(0x2d8A3C5677189723C4cB8873CfC9C8976FDF38Ac);
    MockOracle oracle;
    IPoolAddressesProvider constant addressesProvider =
        IPoolAddressesProvider(0xe20fCBdBfFC4Dd138cE8b2E6FBb6CB49777ad64D);
    address constant aaveAdmin = 0xA9F30e6ED4098e9439B2ac8aEA2d3fc26BcEbb45;
    uint256 constant wethUnit = 10 ** 18;
    uint256 constant usdcUnit = 10 ** 6;
    Liquidator liquidator;
    IL2Pool pool;
    address user;

    function setUp() public {
        vm.createSelectFork(vm.envString("FORK_URL"));
        oracle = new MockOracle();
        user = makeAddr("user");
        liquidator = new Liquidator();
        liquidator.approvePool(address(weth));
        liquidator.approvePool(address(usdc));
        pool = liquidator.pool();
        deal(address(weth), user, 100 ether);
        deal(address(usdc), user, 10000 * 10 ** 6);

        oracle.setAssetPrice(address(weth), 1700 ether);
        oracle.setAssetPrice(address(usdc), 1 ether);
        oracle.setEthUsdPrice(1700 ether);
        vm.startPrank(aaveAdmin);
        addressesProvider.setPriceOracle(address(oracle));
        vm.stopPrank();
    }

    function testLiquidationWethCollateral() public {
        vm.startPrank(user);
        weth.approve(address(pool), type(uint256).max);
        pool.supply(encoder.encodeSupplyParams(address(weth), 1 ether, 0));
        pool.borrow(encoder.encodeBorrowParams(address(usdc), 1200 * 10 ** 6, 2, 0));
        vm.stopPrank();
        uint256 collateralAssetPrice = 1200 ether;
        uint256 debtAssetPrice = oracle.getAssetPrice(address(usdc));
        oracle.setAssetPrice(address(weth), collateralAssetPrice);
        uint256 userCollateralBalance = aweth.balanceOf(user);
        // user now has a liquidatable position with a 100% close factor

        (,,, uint256 bonus,,,,,,) = dataProvider.getReserveConfigurationData(address(weth));
        uint256 protocolFee = dataProvider.getLiquidationProtocolFee(address(weth));
        (, uint256 currentStableDebt, uint256 currentVariableDebt,,,,,,) =
            dataProvider.getUserReserveData(address(usdc), user);
        uint256 debtToCover = currentStableDebt + currentVariableDebt;

        uint256 baseCollateral = (debtAssetPrice * debtToCover * wethUnit) / (collateralAssetPrice * usdcUnit);
        uint256 collateralToLiquidate = percentMul(baseCollateral, bonus);

        if (collateralToLiquidate > userCollateralBalance) {
            collateralToLiquidate = userCollateralBalance;
            debtToCover = (collateralAssetPrice * collateralToLiquidate * usdcUnit)
                / percentDiv((debtAssetPrice * wethUnit), bonus);
        }

        uint256 bonusCollateral = collateralToLiquidate - percentDiv(collateralToLiquidate, bonus);

        uint256 liquidationProtocolFees = percentMul(bonusCollateral, protocolFee);
        uint256 expectedLiquidationReward = collateralToLiquidate - liquidationProtocolFees;
        (uint256 wethRequired,,,) = quoter.quoteExactOutputSingle(
            IQuoterV2.QuoteExactOutputSingleParams({
                tokenIn: address(weth),
                tokenOut: address(usdc),
                amount: debtToCover,
                fee: 500,
                sqrtPriceLimitX96: 4295128740
            })
        );
        uint256 expectedGain = expectedLiquidationReward - wethRequired;

        (bytes32 arg1, bytes32 arg2) =
            encoder.encodeLiquidationCall(address(weth), address(usdc), user, debtToCover, false);
        liquidator.liquidate(address(weth), address(usdc), 500, debtToCover, arg1, arg2);
        assertEq(weth.balanceOf(address(liquidator)), expectedGain);
    }

    function testLiquidationUsdcCollateral() public {
        vm.startPrank(user);
        usdc.approve(address(pool), type(uint256).max);
        pool.supply(encoder.encodeSupplyParams(address(usdc), 2500 * 10 ** 6, 0));
        pool.borrow(encoder.encodeBorrowParams(address(weth), 1 ether, 2, 0));
        vm.stopPrank();
        uint256 debtAssetPrice = 2900 ether;
        uint256 collateralAssetPrice = oracle.getAssetPrice(address(usdc));
        oracle.setAssetPrice(address(weth), debtAssetPrice);
        uint256 userCollateralBalance = ausdc.balanceOf(user);
        // user now has a liquidatable position with a 100% close factor

        (,,, uint256 bonus,,,,,,) = dataProvider.getReserveConfigurationData(address(usdc));
        uint256 protocolFee = dataProvider.getLiquidationProtocolFee(address(usdc));
        (, uint256 currentStableDebt, uint256 currentVariableDebt,,,,,,) =
            dataProvider.getUserReserveData(address(weth), user);
        uint256 debtToCover = currentStableDebt + currentVariableDebt;

        uint256 baseCollateral = (debtAssetPrice * debtToCover * usdcUnit) / (collateralAssetPrice * wethUnit);
        uint256 collateralToLiquidate = percentMul(baseCollateral, bonus);

        if (collateralToLiquidate > userCollateralBalance) {
            collateralToLiquidate = userCollateralBalance;
            debtToCover = (collateralAssetPrice * collateralToLiquidate * wethUnit)
                / percentDiv((debtAssetPrice * usdcUnit), bonus);
        }

        uint256 bonusCollateral = collateralToLiquidate - percentDiv(collateralToLiquidate, bonus);

        uint256 liquidationProtocolFees = percentMul(bonusCollateral, protocolFee);
        uint256 expectedLiquidationReward = collateralToLiquidate - liquidationProtocolFees;
        (uint256 usdcRequired,,,) = quoter.quoteExactOutputSingle(
            IQuoterV2.QuoteExactOutputSingleParams({
                tokenIn: address(usdc),
                tokenOut: address(weth),
                amount: debtToCover,
                fee: 500,
                sqrtPriceLimitX96: 1461446703485210103287273052203988822378723970341
            })
        );
        uint256 expectedGain = expectedLiquidationReward - usdcRequired;

        (bytes32 arg1, bytes32 arg2) =
            encoder.encodeLiquidationCall(address(usdc), address(weth), user, debtToCover, false);
        liquidator.liquidate(address(usdc), address(weth), 500, debtToCover, arg1, arg2);
        assertEq(usdc.balanceOf(address(liquidator)), expectedGain);
    }

    function percentMul(uint256 a, uint256 bps) internal pure returns (uint256) {
        return (5000 + (a * bps)) / 10000;
    }

    function percentDiv(uint256 a, uint256 bps) internal pure returns (uint256) {
        uint256 halfBps = (bps / 2);
        return (halfBps + (a * 10000)) / bps;
    }
}
