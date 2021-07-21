use crate::{self as pallet_exchange};
use crate::mock::sp_api_hidden_includes_construct_runtime::hidden_include::traits::GenesisBuild;

use sp_core::H256;
use frame_support::parameter_types;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup, Zero}, testing::Header,
};
use frame_system;

use orml_traits::parameter_type_with_key;
use orml_currencies::BasicCurrencyAdapter;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type AccountId = u64;
type Block = frame_system::mocking::MockBlock<TestRuntime>;
type BlockNumber = u64;
type Amount = i128;
type Balance = u128;
type CurrencyId = u128;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;

pub const NATIVE: CurrencyId = 0;
pub const DOT: CurrencyId = 1;
pub const BTC: CurrencyId = 2;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for TestRuntime {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 500;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for TestRuntime {
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<TestRuntime>;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_id: CurrencyId| -> Balance {
		Zero::zero()
	};
}

impl orml_tokens::Config for TestRuntime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type ExistentialDeposits = ExistentialDeposits;
	// TODO: investigate the proper OnDust setup
	// type OnDust = orml_tokens::TransferDust<Runtime, Balance>;
	type OnDust = ();
	type MaxLocks = MaxLocks;
	type WeightInfo = ();
}

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = 0;
}

impl orml_currencies::Config for TestRuntime {
	type Event = Event;
	type MultiCurrency = Tokens;
	type NativeCurrency = BasicCurrencyAdapter<TestRuntime, Balances, Amount, BlockNumber>;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type WeightInfo = ();
}

impl pallet_exchange::Config for TestRuntime {
	type Event = Event;
	type Currency = Currencies;
}

frame_support::construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
		Currencies: orml_currencies::{Pallet, Call, Event<T>},
		Exchange: pallet_exchange::{Pallet, Call, Storage, Event<T>},
	}
);

pub const ENDOWED_AMT: u128 = 1000;

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap();

	orml_tokens::GenesisConfig::<TestRuntime> {
		balances: vec![
			(ALICE, NATIVE, ENDOWED_AMT),
			(ALICE, DOT, ENDOWED_AMT),
			(BOB, BTC, ENDOWED_AMT),
		]
	}.assimilate_storage(&mut t).unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
