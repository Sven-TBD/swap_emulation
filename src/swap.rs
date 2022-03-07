use std::task::Poll;

struct Pool{
    pub token: &'static str,
    pub mut eth_amount:u32,
    pub mut usdc_amount:u32,
    pub k:u32,
}

impl Pool{
    fn initialize(eth_amount:u32,usdc_amount:u32) -> Pool{
        Pool { 
            token: "USDC",
            eth_amount,
            usdc_amount,
            k:eth_amount*usdc_amount 
        }
    }

    fn user_buy_token(dETH:u32,token_pool:Pool) -> Pool{
        token_pool.eth_amount+dETH
    }
}