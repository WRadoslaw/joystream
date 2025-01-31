#![cfg(test)]

pub use frame_system;

use common::locks::{
    BoundStakingAccountLockId, CandidacyLockId, CouncilorLockId, InvitedMemberLockId, VotingLockId,
};
use frame_support::traits::{LockIdentifier, OnFinalize, OnInitialize, WithdrawReasons};
use frame_support::{
    parameter_types,
    traits::{ConstU16, ConstU32, ConstU64, EnsureOneOf},
    PalletId,
};
use frame_system::{EnsureRoot, EnsureSigned};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    DispatchResult,
};

use crate::{BalanceOf, CouncilOriginValidator, MemberOriginValidator, MembershipInfoProvider};

use frame_support::dispatch::DispatchError;
use sp_std::convert::{TryFrom, TryInto};

use staking_handler::{LockComparator, StakingHandler, StakingManager};

use crate as proposals_discussion;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        Balances: balances,
        Membership: membership::{Pallet, Call, Storage, Event<T>},
        Timestamp: pallet_timestamp,
        Council: council::{Pallet, Call, Storage, Event<T>},
        Referendum: referendum::<Instance1>::{Pallet, Call, Storage, Event<T>},
        Discussions: proposals_discussion::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MinimumPeriod: u64 = 5;
}

parameter_types! {
    pub const ThreadTitleLengthLimit: u32 = 200;
    pub const PostLengthLimit: u32 = 2000;
}

parameter_types! {
    pub const ExistentialDeposit: u32 = 10;
    pub const TransferFee: u32 = 0;
    pub const CreationFee: u32 = 0;
    pub const MaxWhiteListSize: u32 = 4;
    pub const DefaultMembershipPrice: u64 = 100;
    pub const DefaultInitialInvitationBalance: u64 = 100;
    pub const DefaultMemberInvitesCount: u32 = 2;
    pub const ReferralCutMaximumPercent: u8 = 50;
    pub const CandidateStake: u64 = 100;
    pub const PostLifeTime: u64 = 10;
    pub const PostDeposit: u64 = 100;
    pub const ProposalsDiscussionModuleId: PalletId = PalletId(*b"mo:propo");
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u128;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type MaxLocks = ();
    type MaxReserves = ConstU32<2>;
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

impl common::membership::MembershipTypes for Test {
    type MemberId = u64;
    type ActorId = u64;
}

parameter_types! {
    pub const ScreenedMemberMaxInitialBalance: u64 = 500;
}

impl membership::Config for Test {
    type Event = Event;
    type DefaultMembershipPrice = DefaultMembershipPrice;
    type WorkingGroup = Wg;
    type WeightInfo = ();
    type DefaultInitialInvitationBalance = ();
    type InvitedMemberStakingHandler = staking_handler::StakingManager<Self, InvitedMemberLockId>;
    type ReferralCutMaximumPercent = ReferralCutMaximumPercent;
    type StakingCandidateStakingHandler =
        staking_handler::StakingManager<Self, BoundStakingAccountLockId>;
    type CandidateStake = CandidateStake;
    type DefaultMemberInvitesCount = DefaultMemberInvitesCount;
}

impl LockComparator<<Test as balances::Config>::Balance> for Test {
    fn are_locks_conflicting(
        _new_lock: &LockIdentifier,
        _existing_locks: &[LockIdentifier],
    ) -> bool {
        false
    }
}

pub struct Wg;
impl common::working_group::WorkingGroupBudgetHandler<u128, u64> for Wg {
    fn get_budget() -> u64 {
        unimplemented!()
    }

    fn set_budget(_new_value: u64) {
        unimplemented!()
    }

    fn try_withdraw(_account_id: &u128, _amount: u64) -> DispatchResult {
        unimplemented!()
    }
}

impl common::working_group::WorkingGroupAuthenticator<Test> for Wg {
    fn ensure_worker_origin(
        _origin: <Test as frame_system::Config>::Origin,
        _worker_id: &<Test as common::membership::MembershipTypes>::ActorId,
    ) -> DispatchResult {
        unimplemented!();
    }

    fn ensure_leader_origin(_origin: <Test as frame_system::Config>::Origin) -> DispatchResult {
        unimplemented!()
    }

    fn get_leader_member_id() -> Option<<Test as common::membership::MembershipTypes>::MemberId> {
        unimplemented!()
    }

    fn get_worker_member_id(
        _: &<Test as common::membership::MembershipTypes>::ActorId,
    ) -> Option<<Test as common::membership::MembershipTypes>::MemberId> {
        unimplemented!()
    }

    fn is_leader_account_id(_account_id: &<Test as frame_system::Config>::AccountId) -> bool {
        unimplemented!()
    }

    fn is_worker_account_id(
        _account_id: &<Test as frame_system::Config>::AccountId,
        _worker_id: &<Test as common::membership::MembershipTypes>::ActorId,
    ) -> bool {
        unimplemented!()
    }

    fn worker_exists(_worker_id: &<Test as common::membership::MembershipTypes>::ActorId) -> bool {
        unimplemented!();
    }

    fn ensure_worker_exists(
        _worker_id: &<Test as common::membership::MembershipTypes>::ActorId,
    ) -> DispatchResult {
        unimplemented!();
    }
}

impl crate::Config for Test {
    type Event = Event;
    type AuthorOriginValidator = ();
    type MembershipInfoProvider = ();
    type CouncilOriginValidator = CouncilMock;
    type ThreadId = u64;
    type PostId = u64;
    type MaxWhiteListSize = MaxWhiteListSize;
    type WeightInfo = ();
    type PostLifeTime = PostLifeTime;
    type PostDeposit = PostDeposit;
    type ModuleId = ProposalsDiscussionModuleId;
}

impl MemberOriginValidator<Origin, u64, u128> for () {
    fn ensure_member_controller_account_origin(
        origin: Origin,
        actor_id: u64,
    ) -> Result<u128, DispatchError> {
        if frame_system::ensure_none(origin.clone()).is_ok() {
            return Ok(1);
        }

        if actor_id < 12 {
            if let Ok(account_id) = frame_system::ensure_signed(origin) {
                return Ok(account_id);
            } else {
                return Ok(actor_id.into());
            }
        }

        if actor_id == 12 && frame_system::ensure_signed(origin).unwrap_or_default() == 12 {
            return Ok(12);
        }

        Err(DispatchError::Other("Invalid author"))
    }

    fn is_member_controller_account(_member_id: &u64, _account_id: &u128) -> bool {
        unimplemented!()
    }
}

impl MembershipInfoProvider<Test> for () {
    fn controller_account_id(member_id: u64) -> Result<u128, DispatchError> {
        if member_id <= 12 {
            Ok(member_id.into())
        } else {
            Err(DispatchError::Other("member does not exist"))
        }
    }
}

parameter_types! {
    pub const MinNumberOfExtraCandidates: u32 = 1;
    pub const AnnouncingPeriodDuration: u64 = 15;
    pub const IdlePeriodDuration: u64 = 27;
    pub const CouncilSize: u32 = 4;
    pub const MinCandidateStake: u64 = 11000;
    pub const ElectedMemberRewardPeriod: u64 = 10;
    pub const BudgetRefillAmount: u64 = 1000;
    // intentionally high number that prevents side-effecting tests other than  budget refill tests
    pub const BudgetRefillPeriod: u64 = 1000;
}

type ReferendumInstance = referendum::Instance1;

impl council::Config for Test {
    type Event = Event;

    type Referendum = referendum::Module<Test, ReferendumInstance>;

    type MinNumberOfExtraCandidates = MinNumberOfExtraCandidates;
    type CouncilSize = CouncilSize;
    type AnnouncingPeriodDuration = AnnouncingPeriodDuration;
    type IdlePeriodDuration = IdlePeriodDuration;
    type MinCandidateStake = MinCandidateStake;

    type CandidacyLock = StakingManager<Self, CandidacyLockId>;
    type CouncilorLock = StakingManager<Self, CouncilorLockId>;

    type ElectedMemberRewardPeriod = ElectedMemberRewardPeriod;

    type BudgetRefillPeriod = BudgetRefillPeriod;

    type StakingAccountValidator = membership::Module<Test>;
    type WeightInfo = ();

    fn new_council_elected(_: &[council::CouncilMemberOf<Self>]) {}

    type MemberOriginValidator = ();
}

pub struct CouncilMock;
impl CouncilOriginValidator<Origin, u64, u128> for CouncilMock {
    fn ensure_member_consulate(origin: Origin, actor_id: u64) -> DispatchResult {
        if actor_id == 2 && frame_system::ensure_signed(origin).unwrap_or_default() == 2 {
            return Ok(());
        }

        Err(DispatchError::Other("Not a council"))
    }
}

parameter_types! {
    pub const VoteStageDuration: u64 = 19;
    pub const RevealStageDuration: u64 = 23;
    pub const MinimumVotingStake: u64 = 10000;
    pub const MaxSaltLength: u64 = 32; // use some multiple of 8 for ez testing
    pub const MaxWinnerTargetCount: u32 = 10;
}

impl referendum::Config<ReferendumInstance> for Test {
    type Event = Event;

    type MaxSaltLength = MaxSaltLength;

    type StakingHandler = staking_handler::StakingManager<Self, VotingLockId>;
    type ManagerOrigin = EnsureOneOf<EnsureSigned<Self::AccountId>, EnsureRoot<Self::AccountId>>;

    type VotePower = u64;

    type VoteStageDuration = VoteStageDuration;
    type RevealStageDuration = RevealStageDuration;

    type MinimumStake = MinimumVotingStake;

    type WeightInfo = ();

    type MaxWinnerTargetCount = MaxWinnerTargetCount;

    fn calculate_vote_power(
        _: &<Self as frame_system::Config>::AccountId,
        _: &Self::Balance,
    ) -> Self::VotePower {
        1
    }

    fn can_unlock_vote_stake(
        _: &referendum::CastVote<Self::Hash, Self::Balance, Self::MemberId>,
    ) -> bool {
        true
    }

    fn process_results(winners: &[referendum::OptionResult<Self::MemberId, Self::VotePower>]) {
        let tmp_winners: Vec<referendum::OptionResult<Self::MemberId, Self::VotePower>> = winners
            .iter()
            .map(|item| referendum::OptionResult {
                option_id: item.option_id,
                vote_power: item.vote_power,
            })
            .collect();
        <council::Module<Test> as council::ReferendumConnection<Test>>::recieve_referendum_results(
            tmp_winners.as_slice(),
        );
    }

    fn is_valid_option_id(_: &u64) -> bool {
        true
    }

    fn get_option_power(_: &u64) -> Self::VotePower {
        1
    }

    fn increase_option_power(_: &u64, _: &Self::VotePower) {}
}

pub fn initial_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    crate::GenesisConfig::default()
        .assimilate_storage::<Test>(&mut t)
        .unwrap();

    council::GenesisConfig::<Test>::default()
        .assimilate_storage(&mut t)
        .unwrap();

    t.into()
}

// Recommendation from Parity on testing on_finalize
// https://substrate.dev/docs/en/next/development/module/tests
pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        <System as OnFinalize<u64>>::on_finalize(System::block_number());
        <Discussions as OnFinalize<u64>>::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        <System as OnInitialize<u64>>::on_initialize(System::block_number());
        <Discussions as OnInitialize<u64>>::on_initialize(System::block_number());
    }
}

pub fn ed() -> BalanceOf<Test> {
    ExistentialDeposit::get().into()
}

pub fn set_invitation_lock(
    who: &<Test as frame_system::Config>::AccountId,
    amount: BalanceOf<Test>,
) {
    <Test as membership::Config>::InvitedMemberStakingHandler::lock_with_reasons(
        &who,
        amount,
        WithdrawReasons::except(WithdrawReasons::TRANSACTION_PAYMENT),
    );
}

pub fn set_staking_candidate_lock(
    who: &<Test as frame_system::Config>::AccountId,
    amount: BalanceOf<Test>,
) {
    <Test as membership::Config>::StakingCandidateStakingHandler::lock(&who, amount);
}
