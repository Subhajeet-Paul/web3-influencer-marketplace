#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol, Vec};

#[contract]
pub struct InfluencerMarketplace;

#[contractimpl]
impl InfluencerMarketplace {

    // Register influencer
    pub fn register_influencer(env: Env, influencer: Address, niche: Symbol) {
        influencer.require_auth();

        let key = (symbol_short!("INF"), influencer.clone());
        env.storage().instance().set(&key, &niche);
    }

    // Create campaign by brand
    pub fn create_campaign(
        env: Env,
        brand: Address,
        campaign_id: u64,
        budget: i128,
    ) {
        brand.require_auth();

        let key = (symbol_short!("CMP"), campaign_id);
        env.storage().instance().set(&key, &(brand, budget));
    }

    // Apply to campaign
    pub fn apply_campaign(
        env: Env,
        influencer: Address,
        campaign_id: u64,
    ) {
        influencer.require_auth();

        let key = (symbol_short!("APP"), campaign_id);
        let mut applicants: Vec<Address> =
            env.storage().instance().get(&key).unwrap_or(Vec::new(&env));

        applicants.push_back(influencer);
        env.storage().instance().set(&key, &applicants);
    }

    // Select influencer
    pub fn select_influencer(
        env: Env,
        brand: Address,
        campaign_id: u64,
        influencer: Address,
    ) {
        brand.require_auth();

        let key = (symbol_short!("SEL"), campaign_id);
        env.storage().instance().set(&key, &influencer);
    }

    // Get selected influencer
    pub fn get_selected(env: Env, campaign_id: u64) -> Address {
        let key = (symbol_short!("SEL"), campaign_id);
        env.storage().instance().get(&key).unwrap()
    }
}