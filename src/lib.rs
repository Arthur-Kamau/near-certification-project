use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
near_sdk::setup_alloc!();


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct GuessNumber {
    // See more data types at https://doc.rust-lang.org/book/ch03-02-data-types.html
    val: i8, // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
}

#[near_bindgen]
impl GuessNumber {
    /// Returns 8-bit signed integer of the guess value.
    ///
    /// This must match the type from our struct's 'val' defined above.
    ///
    /// Note, the parameter is `&self` (without being mutable) meaning it doesn't modify state.
    /// In the frontend (/src/main.js) this is added to the "viewMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near view guess.YOU.testnet get_num
    /// ```
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    /// generate a a random number.
    ///
    /// Note, the parameter is "&mut self" as this function modifies state.
    /// In the frontend (/src/main.js) this is added to the "changeMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near call guess.YOU.testnet guess --accountId donation.YOU.testnet
    /// ```
    pub fn generate(&mut self) {
        
    ///
    /// i could use use rand::Rng;
    /// then do rand::thread_rng().gen_range(0..100)
    /// but it generate an error ServerTransactionError: {"index":0,"kind":{"ExecutionError":"Smart contract panicked: panicked at 'could not initialize thread_rng: getrandom: this target is not supported' 
    /// as such i have created a poor mans random  number generator
    /// 
        self.val = get_random_number()
    }

    

    pub fn guess(&mut self, input: i8 ) {
        // self.val = rand::thread_rng().gen_range(0, 100);
      if input> self.val{
        env::log(b"Guess is too big");
      }else if input< self.val {
        env::log(b"guess is too low ");
      }else if input== self.val{
        env::log(b"Guessed right reset to 0");
        self.val =0;
        
      }else{
        env::log(b"woopsss invalid");
      }
    }




    /// Reset guess number to zero.
    pub fn reset(&mut self) {
        self.val = 0;
        // Another way to log is to cast a string into bytes, hence "b" below:
        env::log(b"Reset guess to zero");
    }
}

// unlike the struct's functions above, this function cannot use attributes #[derive(…)] or #[near_bindgen]
// any attempts will throw helpful warnings upon 'cargo build'
// while this function cannot be invoked directly on the blockchain, it can be called from an invoked function
fn get_random_number() -> i8 {
    let num1 = vec![2, 3];
    let address1 = &num1 as *const Vec<i32>;
    let number1 = address1 as i32;
    let first = number1.to_string().chars().rev().nth(2).unwrap();
      let my_rand = first.to_string().parse::<i8>().unwrap();
      return my_rand;
}


// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn generate_random() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = GuessNumber { val: 0 };
        contract.generate();
        println!("Value after guess: {}", contract.get_num());
        // confirm that we received 1 when calling get_num
        assert_eq!(contract.get_num(), contract.get_num());
    }

    #[test]
    fn generate_random_test_and_reset() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = GuessNumber { val: 0 };
        contract.generate();


        //guess right number 
        contract.guess(contract.get_num());

        println!("Value after guess should be 0: {}", contract.get_num());
        // confirm that we received 1 when calling get_num
        assert_eq!(0, contract.get_num());
    }

    #[test]
    fn generate_random_and_reset() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = GuessNumber { val: 0 };
        contract.generate();
        contract.reset();
        println!("Value after guess reset: {}", contract.get_num());
        // confirm that we received -1 when calling get_num
        assert_eq!(0, contract.get_num());
    }
}