use scrypto::prelude::*;

mod attributes;
use attributes::Color;
use attributes::Eyes;
use attributes::Hat;

#[derive(NonFungibleData)]
struct GumballNftData {
    color: Color,
    hat: Hat,
    eyes: Eyes,
}

blueprint! {

    struct BoredGumballClub {
        gumball_nfts: Vault,
        gumball_nft_def: ResourceAddress,
        collected_xrd: Vault,
        price_random: Decimal,
        price_specific: Decimal,
        admin_badge: ResourceAddress,
        minting_authority: Vault,
        num_nft_minted: u64
    }

    impl BoredGumballClub {

        pub fn instantiate_club(price_random: Decimal, price_specific: Decimal) -> (ComponentAddress, Bucket) {
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Club Admin Badge")
                .initial_supply(1);

            let minting_authority: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "NFT minter authority")
                .metadata("description", "Badge that has the authority to mint new gumball NFTs")
                .initial_supply(1);

            let gumball_address: ResourceAddress =  ResourceBuilder::new_non_fungible()
                .metadata("name", "Bored Gumball Club NFT")
                .mintable(rule!(require(minting_authority.resource_address())),LOCKED)
                .no_initial_supply();

            let component = Self {
                gumball_nfts: Vault::new(gumball_address),
                gumball_nft_def: gumball_address,
                collected_xrd: Vault::new(RADIX_TOKEN),
                price_random: price_random,
                price_specific: price_specific,
                admin_badge: admin_badge.resource_address(),
                minting_authority: Vault::with_bucket(minting_authority),
                num_nft_minted: 0,
            }
            .instantiate();

            let access_rules = AccessRules::new()
                .method("mint_nft", rule!(require(admin_badge.resource_address())))
                .default(rule!(allow_all));

            (component.add_access_check(access_rules).globalize(), admin_badge)
        }

        pub fn mint_nft(&mut self, color: Color, hat: Hat, eyes: Eyes) {
            let nft_id = NonFungibleId::from_u64(self.num_nft_minted + 1);

            let nft_data: GumballNftData = GumballNftData {
                color,
                hat,
                eyes
            };

            let new_nft: Bucket = self.minting_authority.authorize(|| {
                return borrow_resource_manager!(self.gumball_nft_def)
                    .mint_non_fungible(&nft_id, nft_data);
            });

            self.gumball_nfts.put(new_nft);
            self.num_nft_minted += 1;
        }

        pub fn buy_random(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            self.collected_xrd.put(payment.take(self.price_random));
            let nft = self.gumball_nfts.take(1);
            (nft, payment)
        }

        pub fn buy_specific(&mut self, mut payment: Bucket, id: u64) -> (Bucket, Bucket) {
            self.collected_xrd.put(payment.take(self.price_specific));
            let nft = self.gumball_nfts.take_non_fungible(&NonFungibleId::from_u64(id));
            (nft, payment)
        }

    }

}
