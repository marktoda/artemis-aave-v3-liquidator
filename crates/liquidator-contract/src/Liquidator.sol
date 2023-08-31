// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Owned} from "solmate/auth/Owned.sol";
import {ERC20} from "solmate/tokens/ERC20.sol";
import {IL2Pool} from "./interfaces/IL2Pool.sol";
import {IUniswapV3SwapCallback} from "./interfaces/IUniswapV3SwapCallback.sol";
import {IUniswapV3PoolActions} from "./interfaces/IUniswapV3PoolActions.sol";
import {PoolAddress} from "./lib/PoolAddress.sol";

uint160 constant MIN_SQRT_RATIO = 4295128739;
/// @dev The maximum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MAX_TICK)
uint160 constant MAX_SQRT_RATIO = 1461446703485210103287273052203988822378723970342;

contract Liquidator is Owned(msg.sender), IUniswapV3SwapCallback {
    // ERC20 constant weth = ERC20(0x4200000000000000000000000000000000000006);
    // ERC20 constant usdc = ERC20(0xd9aAEc86B65D86f6A7B5B1b0c42FFA531710b6CA);
    // IUniswapV3PoolActions constant uniswapPool = IUniswapV3PoolActions(0x4C36388bE6F416A29C8d8Eee81C771cE6bE14B18);
    address private constant uniswapV3Factory = 0x33128a8fC17869897dcE68Ed026d694621f6FDfD;
    IL2Pool public constant pool = IL2Pool(0x8F44Fd754285aa6A2b8B9B97739B79746e0475a7);

    constructor() {}

    function liquidate(
        address collateral,
        address debt,
        uint24 uniswapFee,
        uint256 debtToCover,
        bytes32 liquidationArg1,
        bytes32 liquidationArg2
    ) external onlyOwner returns (int256 collateralGain) {
        address uniswapPool =
            PoolAddress.computeAddress(uniswapV3Factory, PoolAddress.getPoolKey(collateral, debt, uniswapFee));
        uint256 collateralBalance = ERC20(collateral).balanceOf(address(this));

        // collateral in, debt out
        bool zeroForOne = collateral < debt;

        IUniswapV3PoolActions(uniswapPool).swap(
            address(this),
            zeroForOne,
            // debtToCover is the amount of outputs we need so we do exact output
            -int256(debtToCover),
            // price is irrelevant
            zeroForOne ? MIN_SQRT_RATIO + 1 : MAX_SQRT_RATIO - 1,
            abi.encode(collateral, debt, uniswapFee, liquidationArg1, liquidationArg2)
        );

        collateralGain = int256(ERC20(collateral).balanceOf(address(this))) - int256(collateralBalance);
    }

    function uniswapV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external override {
        (address collateral, address debt, uint24 uniswapFee, bytes32 liquidationArg1, bytes32 liquidationArg2) =
            abi.decode(data, (address, address, uint24, bytes32, bytes32));

        verifyCallback(uniswapV3Factory, PoolAddress.getPoolKey(collateral, debt, uniswapFee));

        // we expect to get the opposite token returned
        pool.liquidationCall(liquidationArg1, liquidationArg2);

        (address token0, address token1) = collateral < debt ? (collateral, debt) : (debt, collateral);
        if (amount0Delta > 0) {
            ERC20(token0).transfer(msg.sender, uint256(amount0Delta));
        } else if (amount1Delta > 0) {
            ERC20(token1).transfer(msg.sender, uint256(amount1Delta));
        }
    }

    function approvePool(address token) external onlyOwner {
        ERC20(token).approve(address(pool), type(uint256).max);
    }

    function recover(address token, uint256 amount) external onlyOwner {
        if (token == address(0)) {
            payable(msg.sender).transfer(amount);
            return;
        }
        ERC20(token).transfer(msg.sender, amount);
    }

    function verifyCallback(address factory, PoolAddress.PoolKey memory poolKey) internal view {
        address p = PoolAddress.computeAddress(factory, poolKey);
        require(msg.sender == p, "invalid pool");
    }
}
