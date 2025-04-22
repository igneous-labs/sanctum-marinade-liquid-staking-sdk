pub enum MarinadeError {
    DepositAmountIsTooLow,
    ProgramIsPaused,
    StakingIsCapped,

    // TODO: This is not an actual `MarinadeError`, we're defining this to have a error to throw when working with `Floor/Ratio`
    CalculationError,
}
