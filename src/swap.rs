use std::task::Poll;

pub struct Pool{
    pub token: &'static str,
    pub eth_amount:u128,
    pub quote_token_amount:u128,
    pub k:u128,
    price:u128,
}

impl Pool{
    pub fn new(eth_amount:u128,quote_token_amount:u128) -> Self{
        let new_pool = Pool { 
            token: "USDC",
            eth_amount,
            quote_token_amount,
            k:eth_amount*quote_token_amount, 
            price:quote_token_amount/eth_amount,
        };
        new_pool.print_status();
        new_pool
    }

    pub fn user_buy_token(&mut self,dETH:u128) -> &mut Self{
        self.eth_amount += dETH;
        self.quote_token_amount = self.k*1000_000/self.eth_amount;
        self.update_knp()
    }

    fn update_knp(&mut self)-> &mut Self{
        self.k = self.eth_amount*self.quote_token_amount;
        self.price = self.quote_token_amount/self.eth_amount;
        self.print_status();
        self
    }

    fn print_status(&self) {
        println!("---this is a pool for eth and {}---",self.token);
        println!("eth:{}",self.eth_amount);
        println!("{}:{}",self.token,self.quote_token_amount);
        println!("K:{}",self.k);
        println!("price:{}",self.price);
    }
}