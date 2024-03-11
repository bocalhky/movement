pragma solidity ^0.8.20;

import {IRiscZeroVerifier} from "./IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol"; // auto-generated contract after running `cargo build`.

/// @title A starter application using RISC Zero.
/// @notice This basic application holds a number, guaranteed to be even.
/// @dev This contract demonstrates one pattern for offloading the computation of an expensive
///      or difficult to implement function to a RISC Zero guest running on Bonsai.
contract EvenNumber {
    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public immutable verifier;
    /// @notice Image ID of the only zkVM binary to accept verification from.
    bytes32 public constant imageId = ImageID.IS_EVEN_ID;

    /// @notice A number that is guaranteed, by the RISC Zero zkVM, to be even.
    ///         It can be set by calling the `set` function.
    uint256 public number;

    /// @notice Initialize the contract, binding it to a specified RISC Zero verifier.
    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
        number = 0;
    }

    /// @notice Set the even number stored on the contract. Requires a RISC Zero proof that the number is even.
    function set(uint256 x, bytes32 postStateDigest, bytes calldata seal) public {
        // Construct the expected journal data. Verify will fail if journal does not match.
        bytes memory journal = abi.encode(x);
        require(verifier.verify(seal, imageId, postStateDigest, sha256(journal)));
        number = x;
    }

    /// @notice Returns the number stored.
    function get() public view returns (uint256) {
        return number;
    }
}