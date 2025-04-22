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
