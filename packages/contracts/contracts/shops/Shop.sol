/// SPDX-License-Identifier: MIT 
pragma solidity >=0.8.0 <0.9.0;
/// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0 <0.9.0;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/utils/Counters.sol";


/**
 * @title ReShop
 * @dev The application NFT for shop interfaces minted by the platform auto imported onto Reaction
 *
 */
contract ReShopNFT {
    using Counters for Counters.Counter;
    Counters.Counter public versions;
    mapping(uint256 => string) public builds;
    address public appOwner;

    constructor(string memory name_, string memory symbol_, string memory tokenURI) ERC721(name_, symbol_) {
        appOwner = msg.sender;
        mint(tokenURI);
    }

    /// Fetch the previous build version given the latest release
    function getPreviousBuild(uint256 versionNumber)
        public
        view
        returns (string memory)
    {
        return builds[versionNumber];
    }
    /// Implements the method of generation, minting new applications accordingly
    function mint(string memory tokenURI) private returns (uint256) {
        versions.increment();
        uint256 tokenId = 1;
        uint256 currentVersion = versions.current();
        _mint(msg.sender, tokenId);
        _setTokenURI(tokenId, tokenURI);
        builds[currentVersion] = tokenURI;
        return tokenId;
    }
    /// Implements a standard method of transfer
    function transferFrom(
        address from,
        address to,
        uint256 tokenId
    ) public virtual override {
        require(
            _isApprovedOrOwner(_msgSender(), tokenId),
            "ERC721: caller is not token owner nor approved"
        );

        _transfer(from, to, tokenId);
        appOwner = to;
    }
    /// This function enables owners to update their applications to the latest release
    function updateApp(string memory newTokenURI) public {
        require(
            msg.sender == appOwner,
            "Only the app owner can make this change"
        );
        uint256 currentVersion = versions.current();
        _setTokenURI(1, newTokenURI);
        builds[currentVersion + 1] = newTokenURI;
        versions.increment();
    }
}