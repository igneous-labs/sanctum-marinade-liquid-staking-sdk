pub enum MarinadeError {
    DepositAmountIsTooLow,
    ProgramIsPaused,
    StakingIsCapped,
    CalculationFailure,
    TooLowDelegationInDepositingStake,
    WrongValidatorAccountOrIndex,
    WithdrawStakeAccountIsNotEnabled,
    StakeAccountIsEmergencyUnstaking,
    WithdrawStakeLamportsIsTooLow,
    SelectedStakeAccountHasNotEnoughFunds,
    StakeAccountRemainderTooLow,
}
