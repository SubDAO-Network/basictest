#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

pub use self::basictest::Basictest;

#[ink::contract]
mod basictest {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Basictest {
        /// Stores a single `bool` value on the storage.
        value: bool,
        owner: AccountId,
        name: String,
    }

    impl Basictest {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value, name: String::default(), owner: Default::default() }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }


        #[ink(message)]
        pub fn init_base(&mut self, name: String) {
            self.set_name(name);
            let caller = self.env().caller();
            self.set_owner(caller);
        }

        #[ink(message)]
        pub fn set_name(&mut self, name: String) {
            self.name = String::from(name);
        }

        #[ink(message)]
        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        #[ink(message)]
        pub fn set_owner(&mut self, owner: AccountId) {

            let caller = self.env().caller();

            if self.owner == AccountId::default() || caller == self.owner {
                self.owner = owner;
            }
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_lang as ink;
        
        /// We test if the default constructor does its job.
        #[test]
        fn default_works() {
            let basictest = Basictest::default();
            assert_eq!(basictest.get(), false);
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
            let mut basictest = Basictest::new(false);
            assert_eq!(basictest.get(), false);
            basictest.flip();
            assert_eq!(basictest.get(), true);
        }

        #[test]
        fn test_name() {
            let mut base = Basictest::default();

            base.set_name("Hello".to_string());

            let dbg_msg = format!("name is {}", base.get_name());
            ink_env::debug_println( &dbg_msg );

            assert_eq!(base.get_name(), "Hello");
        }


        #[ink::test]
        fn test_owner() {

            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

            let mut base = Basictest::default();

            base.set_owner(accounts.alice);

            assert_eq!(base.get_owner(), accounts.alice);
        }

        #[ink::test]
        fn test_change_owner() {

            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

            let mut base = Basictest::default();

            base.set_owner(accounts.alice);

            assert_eq!(base.get_owner(), accounts.alice);

            base.set_owner(accounts.bob);

            assert_eq!(base.get_owner(), accounts.bob);
        }
    }
}
