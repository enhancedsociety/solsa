pragma solidity ^0.4.18;

import "./A.sol";

contract A_Test is A {    
    function echidna_one_is_one() public pure returns (bool){
        return one == 1;
    }
}