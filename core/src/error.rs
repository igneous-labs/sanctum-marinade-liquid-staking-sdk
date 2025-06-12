use core::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MarinadeError {
    DepositAmountIsTooLow,
    ProgramIsPaused,
    StakingIsCapped,
    CalculationFailure,
    TooLowDelegationInDepositingStake,
    WithdrawStakeAccountIsNotEnabled,
    StakeAccountIsEmergencyUnstaking,
    WithdrawStakeLamportsIsTooLow,
    SelectedStakeAccountHasNotEnoughFunds,
    StakeAccountRemainderTooLow,
}

impl Display for MarinadeError {
    // Display=Debug, since this is just a simple str enum
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Error for MarinadeError {}
