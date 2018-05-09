pragma solidity ^0.4.18;

contract Useless {
    event Receipt(
        address from,
        address to,
        uint amount_in_wei
    );

    function() public payable {
        emit Receipt(msg.sender, this, msg.value);
    }
}