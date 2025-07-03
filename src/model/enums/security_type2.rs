//! Enum for all supported security types 2 in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::SecurityType2;
//! let mic = SecurityType2::BASISSWAP; // Basis Swap
//! ```
//!
//! For the full list of values, see: <https://api.openfigi.com/v3/mapping/values/securityType2>
use serde::{Deserialize, Serialize};

/// Enum for all supported security types 2 values.
#[expect(missing_docs)]
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SecurityType2 {
    #[serde(rename = "2ND LIEN")]
    _2NDLIEN,
    #[serde(rename = "ABS")]
    ABS,
    #[serde(rename = "ABS Other")]
    ABSOther,
    #[serde(rename = "ABS/HG")]
    ABSHG,
    #[serde(rename = "ABS/MEZZ")]
    ABSMEZZ,
    #[serde(rename = "BA")]
    BA,
    #[serde(rename = "Bagged Briquettes")]
    BaggedBriquettes,
    #[serde(rename = "Bagged Pellets")]
    BaggedPellets,
    #[serde(rename = "BANK BILL")]
    BANKBILL,
    #[serde(rename = "BANKERS ACCEPTANCE")]
    BANKERSACCEPTANCE,
    #[serde(rename = "BASIS")]
    BASIS,
    #[serde(rename = "BASIS SWAP")]
    BASISSWAP,
    #[serde(rename = "BASIS_IMM")]
    BASISIMM,
    #[serde(rename = "Bill")]
    Bill,
    #[serde(rename = "Billet 20MN")]
    Billet20MN,
    #[serde(rename = "Billet 3803p")]
    Billet3803p,
    #[serde(rename = "Billet 3803s")]
    Billet3803s,
    #[serde(rename = "Billet 3803sp")]
    Billet3803sp,
    #[serde(rename = "Billet 3805p")]
    Billet3805p,
    #[serde(rename = "Billet 3805s")]
    Billet3805s,
    #[serde(rename = "Billet 3805sp")]
    Billet3805sp,
    #[serde(rename = "Billet A61560")]
    BilletA61560,
    #[serde(rename = "Billet BS4449")]
    BilletBS4449,
    #[serde(rename = "Billet LME Grade 1")]
    BilletLMEGrade1,
    #[serde(rename = "Billet LME Grade 2")]
    BilletLMEGrade2,
    #[serde(rename = "Billet LME Grade 3")]
    BilletLMEGrade3,
    #[serde(rename = "Billet LME Grade 4")]
    BilletLMEGrade4,
    #[serde(rename = "Billet LME Grade 5")]
    BilletLMEGrade5,
    #[serde(rename = "Billet LME Grade 6")]
    BilletLMEGrade6,
    #[serde(rename = "Billet LME Grade 7")]
    BilletLMEGrade7,
    #[serde(rename = "Billet LME Grade 8")]
    BilletLMEGrade8,
    #[serde(rename = "Billet LME Grade 9")]
    BilletLMEGrade9,
    #[serde(rename = "Billet Q235")]
    BilletQ235,
    #[serde(rename = "BN")]
    BN,
    #[serde(rename = "Bond")]
    Bond,
    #[serde(rename = "Bond/Note")]
    BondNote,
    #[serde(rename = "Briquettes")]
    Briquettes,
    #[serde(rename = "BUTTERFLY SWAP")]
    BUTTERFLYSWAP,
    #[serde(rename = "CAPFLOOR")]
    CAPFLOOR,
    #[serde(rename = "CAPFLOOR_RFR")]
    CAPFLOORRFR,
    #[serde(rename = "CAPS & FLOORS")]
    CAPSANDFLOORS,
    #[serde(rename = "CASH RATE")]
    CASHRATE,
    #[serde(rename = "Cathodes")]
    Cathodes,
    #[serde(rename = "Cathodes 100x100mm")]
    Cathodes100x100mm,
    #[serde(rename = "Cathodes 25x25mm")]
    Cathodes25x25mm,
    #[serde(rename = "Cathodes 50x50mm")]
    Cathodes50x50mm,
    #[serde(rename = "CD")]
    CD,
    #[serde(rename = "CDO2")]
    CDO2,
    #[serde(rename = "CDS")]
    CDS,
    #[serde(rename = "CDS(CRP)")]
    CDSCRP,
    #[serde(rename = "Certificate")]
    Certificate,
    #[serde(rename = "CMBS")]
    CMBS,
    #[serde(rename = "CMO")]
    CMO,
    #[serde(rename = "Coarse Grain Powder")]
    CoarseGrainPowder,
    #[serde(rename = "Comdty")]
    Comdty,
    #[serde(rename = "COMMERCIAL PAPER")]
    COMMERCIALPAPER,
    #[serde(rename = "Common Stock")]
    CommonStock,
    #[serde(rename = "CONTRACT FRA")]
    CONTRACTFRA,
    #[serde(rename = "Corp")]
    Corp,
    #[serde(rename = "CP")]
    CP,
    #[serde(rename = "CRE")]
    CRE,
    #[serde(rename = "CROSS")]
    CROSS,
    #[serde(rename = "CRYPTO")]
    CRYPTO,
    #[serde(rename = "Curncy")]
    Curncy,
    #[serde(rename = "Daily Future")]
    DailyFuture,
    #[serde(rename = "DEPOSIT")]
    DEPOSIT,
    #[serde(rename = "Depositary Receipt")]
    DepositaryReceipt,
    #[serde(rename = "Derived")]
    Derived,
    #[serde(rename = "DN")]
    DN,
    #[serde(rename = "Equity")]
    Equity,
    #[serde(rename = "FDIC")]
    FDIC,
    #[serde(rename = "FIXED_FLOAT")]
    FIXEDFLOAT,
    #[serde(rename = "FIXED_FLOAT_FORWARD_STARTING")]
    FIXEDFLOATFORWARDSTARTING,
    #[serde(rename = "FIXING RATE")]
    FIXINGRATE,
    #[serde(rename = "FORWARD")]
    FORWARD,
    #[serde(rename = "FORWARD CROSS")]
    FORWARDCROSS,
    #[serde(rename = "FORWARD CURVE")]
    FORWARDCURVE,
    #[serde(rename = "FRA")]
    FRA,
    #[serde(rename = "Full Plate Cathodes")]
    FullPlateCathodes,
    #[serde(rename = "Future")]
    Future,
    #[serde(rename = "FWD SWAP")]
    FWDSWAP,
    #[serde(rename = "FX Curve")]
    FXCurve,
    #[serde(rename = "Generic")]
    Generic,
    #[serde(rename = "Govt")]
    Govt,
    #[serde(rename = "Granules")]
    Granules,
    #[serde(rename = "Hedged")]
    Hedged,
    #[serde(rename = "HF")]
    HF,
    #[serde(rename = "HY")]
    HY,
    #[serde(rename = "IG")]
    IG,
    #[serde(rename = "IMM FORWARD")]
    IMMFORWARD,
    #[serde(rename = "IMM SWAP")]
    IMMSWAP,
    #[serde(rename = "Index")]
    Index,
    #[serde(rename = "INFL_FIXING_ZERO_COUPON")]
    INFLFIXINGZEROCOUPON,
    #[serde(rename = "INFL_FXFL_ZERO_COUPON")]
    INFLFXFLZEROCOUPON,
    #[serde(rename = "INFLATION SWAP")]
    INFLATIONSWAP,
    #[serde(rename = "INFLATION_SWAP")]
    INFLATIONSWAP2,
    #[serde(rename = "Ingots")]
    Ingots,
    #[serde(rename = "Ingots 226/DIN")]
    Ingots226DIN,
    #[serde(rename = "Ingots A380.1")]
    IngotsA3801,
    #[serde(rename = "Ingots AD12.1")]
    IngotsAD121,
    #[serde(rename = "Ingots D12S/J1S")]
    IngotsD12SJ1S,
    #[serde(rename = "Jumbo")]
    Jumbo,
    #[serde(rename = "Large Sows")]
    LargeSows,
    #[serde(rename = "Large sows 226")]
    Largesows226,
    #[serde(rename = "Large sows A380.1")]
    LargesowsA3801,
    #[serde(rename = "Large sows AD12.1")]
    LargesowsAD121,
    #[serde(rename = "Large sows D12S")]
    LargesowsD12S,
    #[serde(rename = "LL")]
    LL,
    #[serde(rename = "LL08")]
    LL08,
    #[serde(rename = "M-Mkt")]
    MMkt,
    #[serde(rename = "MAC SWAP")]
    MACSWAP,
    #[serde(rename = "MEZZ")]
    MEZZ,
    #[serde(rename = "MML")]
    MML,
    #[serde(rename = "Molybdenum Cntd n RMC(Roasted")]
    MolybdenumCntdnRMCRoasted,
    #[serde(rename = "MONEY MARKET CALL")]
    MONEYMARKETCALL,
    #[serde(rename = "Mtge")]
    Mtge,
    #[serde(rename = "MTN")]
    MTN,
    #[serde(rename = "Muni")]
    Muni,
    #[serde(rename = "MUNI SWAP")]
    MUNISWAP,
    #[serde(rename = "Mutual Fund")]
    MutualFund,
    #[serde(rename = "NDF SWAP")]
    NDFSWAP,
    #[serde(rename = "Nickel Rounds")]
    NickelRounds,
    #[serde(rename = "Nickel Rounds Bag")]
    NickelRoundsBag,
    #[serde(rename = "NON-DELIVERABLE FORWARD")]
    NONDELIVERABLEFORWARD,
    #[serde(rename = "NON-DELIVERABLE IRS SWAP")]
    NONDELIVERABLEIRSSWAP,
    #[serde(rename = "NON-DELIVERABLE OIS SWAP")]
    NONDELIVERABLEOISSWAP,
    #[serde(rename = "Note")]
    Note,
    #[serde(rename = "ONSHORE FORWARD")]
    ONSHOREFORWARD,
    #[serde(rename = "ONSHORE SWAP")]
    ONSHORESWAP,
    #[serde(rename = "Option")]
    Option,
    #[serde(rename = "OPTION VOLATILITY")]
    OPTIONVOLATILITY,
    #[serde(rename = "OTHER")]
    OTHER,
    #[serde(rename = "OVERNIGHT INDEXED SWAP")]
    OVERNIGHTINDEXEDSWAP,
    #[serde(rename = "PAIR")]
    PAIR,
    #[serde(rename = "Partnership Shares")]
    PartnershipShares,
    #[serde(rename = "Pellets")]
    Pellets,
    #[serde(rename = "Pool")]
    Pool,
    #[serde(rename = "PP12")]
    PP12,
    #[serde(rename = "PP20")]
    PP20,
    #[serde(rename = "PP25")]
    PP25,
    #[serde(rename = "PP3.5")]
    PP35,
    #[serde(rename = "Preference")]
    Preference,
    #[serde(rename = "Preferred Stock")]
    PreferredStock,
    #[serde(rename = "PROMISSORY NOTE")]
    PROMISSORYNOTE,
    #[serde(rename = "Prompt Forward")]
    PromptForward,
    #[serde(rename = "PROPERTY SWAP")]
    PROPERTYSWAP,
    #[serde(rename = "QUARTERLY SWAP")]
    QUARTERLYSWAP,
    #[serde(rename = "REIT")]
    REIT,
    #[serde(rename = "REPO")]
    REPO,
    #[serde(rename = "RETURN IDX")]
    RETURNIDX,
    #[serde(rename = "Right")]
    Right,
    #[serde(rename = "RMBS")]
    RMBS,
    #[serde(rename = "Rounds")]
    Rounds,
    #[serde(rename = "Small Sows")]
    SmallSows,
    #[serde(rename = "Small sows 226")]
    Smallsows226,
    #[serde(rename = "Small sows A380.1")]
    SmallsowsA3801,
    #[serde(rename = "Small sows AD12.1")]
    SmallsowsAD121,
    #[serde(rename = "Small sows D12S")]
    SmallsowsD12S,
    #[serde(rename = "SME")]
    SME,
    #[serde(rename = "Sows")]
    Sows,
    #[serde(rename = "SPOT")]
    SPOT,
    #[serde(rename = "SWAP")]
    SWAP,
    #[serde(rename = "SWAP SPREAD")]
    SWAPSPREAD,
    #[serde(rename = "SWAPTION")]
    SWAPTION,
    #[serde(rename = "SWAPTION VOLATILITY")]
    SWAPTIONVOLATILITY,
    #[serde(rename = "T-Bar")]
    TBar,
    #[serde(rename = "T-Bars 226")]
    TBars226,
    #[serde(rename = "T-Bars A380.1")]
    TBarsA3801,
    #[serde(rename = "T-Bars AD12.1")]
    TBarsAD121,
    #[serde(rename = "T-Bars D12S")]
    TBarsD12S,
    #[serde(rename = "TBA")]
    TBA,
    #[serde(rename = "TD")]
    TD,
    #[serde(rename = "TREASURY BILL")]
    TREASURYBILL,
    #[serde(rename = "TRP")]
    TRP,
    #[serde(rename = "Unit")]
    Unit,
    #[serde(rename = "Unit Investment Trust")]
    UnitInvestmentTrust,
    #[serde(rename = "VOLATILITY DERIVATIVE")]
    VOLATILITYDERIVATIVE,
    #[serde(rename = "Warrant")]
    Warrant,
    #[serde(rename = "Whole Loan")]
    WholeLoan,
    #[serde(rename = "Yield Curve")]
    YieldCurve,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_enum_serialization;

    test_enum_serialization!(
        test_serialize_basis_swap,
        SecurityType2,
        BASISSWAP,
        "\"BASIS SWAP\""
    );
    test_enum_serialization!(
        test_serialize_unit_investment_trust,
        SecurityType2,
        UnitInvestmentTrust,
        "\"Unit Investment Trust\""
    );
}
