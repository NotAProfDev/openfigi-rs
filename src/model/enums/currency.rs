//! Enum for all supported currency codes in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::Currency;
//! let currency = Currency::USD; // US Dollar
//! ```
//!
//! For the full list of values, see: <https://api.openfigi.com/v3/mapping/values/currency>

use serde::{Deserialize, Serialize};

/// Enum for all supported currency codes.
#[expect(missing_docs)]
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "***")]
    AAA,
    #[serde(rename = "ADP")]
    ADP,
    #[serde(rename = "AED")]
    AED,
    #[serde(rename = "AFN")]
    AFN,
    #[serde(rename = "ALL")]
    ALL,
    #[serde(rename = "AMD")]
    AMD,
    #[serde(rename = "ANG")]
    ANG,
    #[serde(rename = "AOA")]
    AOA,
    #[serde(rename = "ARS")]
    ARS,
    #[serde(rename = "ATS")]
    ATS,
    #[serde(rename = "AUD")]
    AUD,
    #[serde(rename = "AUd")]
    AUd,
    #[serde(rename = "AWG")]
    AWG,
    #[serde(rename = "AZM")]
    AZM,
    #[serde(rename = "AZN")]
    AZN,
    #[serde(rename = "BAM")]
    BAM,
    #[serde(rename = "BBD")]
    BBD,
    #[serde(rename = "BDT")]
    BDT,
    #[serde(rename = "BEF")]
    BEF,
    #[serde(rename = "BGN")]
    BGN,
    #[serde(rename = "BHD")]
    BHD,
    #[serde(rename = "BIF")]
    BIF,
    #[serde(rename = "BMD")]
    BMD,
    #[serde(rename = "BND")]
    BND,
    #[serde(rename = "BOB")]
    BOB,
    #[serde(rename = "BRL")]
    BRL,
    #[serde(rename = "BRl")]
    BRl,
    #[serde(rename = "BSD")]
    BSD,
    #[serde(rename = "BTN")]
    BTN,
    #[serde(rename = "BWP")]
    BWP,
    #[serde(rename = "BWp")]
    BWp,
    #[serde(rename = "BYN")]
    BYN,
    #[serde(rename = "BYR")]
    BYR,
    #[serde(rename = "BYS")]
    BYS,
    #[serde(rename = "BZD")]
    BZD,
    #[serde(rename = "CAD")]
    CAD,
    #[serde(rename = "CAd")]
    CAd,
    #[serde(rename = "CDF")]
    CDF,
    #[serde(rename = "CER")]
    CER,
    #[serde(rename = "CHF")]
    CHF,
    #[serde(rename = "CHf")]
    CHf,
    #[serde(rename = "CLF")]
    CLF,
    #[serde(rename = "CLP")]
    CLP,
    #[serde(rename = "CNH")]
    CNH,
    #[serde(rename = "CNT")]
    CNT,
    #[serde(rename = "CNY")]
    CNY,
    #[serde(rename = "COP")]
    COP,
    #[serde(rename = "COU")]
    COU,
    #[serde(rename = "CRC")]
    CRC,
    #[serde(rename = "CRS")]
    CRS,
    #[serde(rename = "CUP")]
    CUP,
    #[serde(rename = "CVE")]
    CVE,
    #[serde(rename = "CYP")]
    CYP,
    #[serde(rename = "CZK")]
    CZK,
    #[serde(rename = "DEM")]
    DEM,
    #[serde(rename = "DJF")]
    DJF,
    #[serde(rename = "DKK")]
    DKK,
    #[serde(rename = "DOP")]
    DOP,
    #[serde(rename = "DZD")]
    DZD,
    #[serde(rename = "ECS")]
    ECS,
    #[serde(rename = "EEK")]
    EEK,
    #[serde(rename = "EES")]
    EES,
    #[serde(rename = "EGD")]
    EGD,
    #[serde(rename = "EGP")]
    EGP,
    #[serde(rename = "ERN")]
    ERN,
    #[serde(rename = "ESP")]
    ESP,
    #[serde(rename = "ETB")]
    ETB,
    #[serde(rename = "EUA")]
    EUA,
    #[serde(rename = "EUR")]
    EUR,
    #[serde(rename = "EUr")]
    EUr,
    #[serde(rename = "FIM")]
    FIM,
    #[serde(rename = "FJD")]
    FJD,
    #[serde(rename = "FKP")]
    FKP,
    #[serde(rename = "FRF")]
    FRF,
    #[serde(rename = "GBP")]
    GBP,
    #[serde(rename = "GBp")]
    GBp,
    #[serde(rename = "GEL")]
    GEL,
    #[serde(rename = "GHC")]
    GHC,
    #[serde(rename = "GHS")]
    GHS,
    #[serde(rename = "GIP")]
    GIP,
    #[serde(rename = "GLD")]
    GLD,
    #[serde(rename = "GMD")]
    GMD,
    #[serde(rename = "GNF")]
    GNF,
    #[serde(rename = "GRD")]
    GRD,
    #[serde(rename = "GTQ")]
    GTQ,
    #[serde(rename = "GWP")]
    GWP,
    #[serde(rename = "GYD")]
    GYD,
    #[serde(rename = "HKD")]
    HKD,
    #[serde(rename = "HNL")]
    HNL,
    #[serde(rename = "HRK")]
    HRK,
    #[serde(rename = "HTG")]
    HTG,
    #[serde(rename = "HUF")]
    HUF,
    #[serde(rename = "IDR")]
    IDR,
    #[serde(rename = "IEP")]
    IEP,
    #[serde(rename = "ILS")]
    ILS,
    #[serde(rename = "ILs")]
    ILs,
    #[serde(rename = "INR")]
    INR,
    #[serde(rename = "IQD")]
    IQD,
    #[serde(rename = "IRR")]
    IRR,
    #[serde(rename = "ISK")]
    ISK,
    #[serde(rename = "ITL")]
    ITL,
    #[serde(rename = "JEP")]
    JEP,
    #[serde(rename = "JMD")]
    JMD,
    #[serde(rename = "JOD")]
    JOD,
    #[serde(rename = "JPY")]
    JPY,
    #[serde(rename = "KES")]
    KES,
    #[serde(rename = "KGS")]
    KGS,
    #[serde(rename = "KHR")]
    KHR,
    #[serde(rename = "KMF")]
    KMF,
    #[serde(rename = "KPW")]
    KPW,
    #[serde(rename = "KRW")]
    KRW,
    #[serde(rename = "KWD")]
    KWD,
    #[serde(rename = "KWd")]
    KWd,
    #[serde(rename = "KYD")]
    KYD,
    #[serde(rename = "KZT")]
    KZT,
    #[serde(rename = "LAK")]
    LAK,
    #[serde(rename = "LBP")]
    LBP,
    #[serde(rename = "LKR")]
    LKR,
    #[serde(rename = "LRD")]
    LRD,
    #[serde(rename = "LSL")]
    LSL,
    #[serde(rename = "LTL")]
    LTL,
    #[serde(rename = "LUF")]
    LUF,
    #[serde(rename = "LVL")]
    LVL,
    #[serde(rename = "LYD")]
    LYD,
    #[serde(rename = "MAD")]
    MAD,
    #[serde(rename = "MDL")]
    MDL,
    #[serde(rename = "MGA")]
    MGA,
    #[serde(rename = "MGF")]
    MGF,
    #[serde(rename = "MKD")]
    MKD,
    #[serde(rename = "MLF")]
    MLF,
    #[serde(rename = "MMK")]
    MMK,
    #[serde(rename = "MNT")]
    MNT,
    #[serde(rename = "MOP")]
    MOP,
    #[serde(rename = "MRO")]
    MRO,
    #[serde(rename = "MRU")]
    MRU,
    #[serde(rename = "MTL")]
    MTL,
    #[serde(rename = "MULTI")]
    MULTI,
    #[serde(rename = "MUR")]
    MUR,
    #[serde(rename = "MVR")]
    MVR,
    #[serde(rename = "MWK")]
    MWK,
    #[serde(rename = "MWk")]
    MWk,
    #[serde(rename = "MXN")]
    MXN,
    #[serde(rename = "MYR")]
    MYR,
    #[serde(rename = "MYr")]
    MYr,
    #[serde(rename = "MZM")]
    MZM,
    #[serde(rename = "MZN")]
    MZN,
    #[serde(rename = "NAD")]
    NAD,
    #[serde(rename = "NAd")]
    NAd,
    #[serde(rename = "NGN")]
    NGN,
    #[serde(rename = "NIC")]
    NIC,
    #[serde(rename = "NID")]
    NID,
    #[serde(rename = "NIO")]
    NIO,
    #[serde(rename = "NLG")]
    NLG,
    #[serde(rename = "NOK")]
    NOK,
    #[serde(rename = "NPR")]
    NPR,
    #[serde(rename = "NZD")]
    NZD,
    #[serde(rename = "OMR")]
    OMR,
    #[serde(rename = "PAB")]
    PAB,
    #[serde(rename = "PEN")]
    PEN,
    #[serde(rename = "PGK")]
    PGK,
    #[serde(rename = "PHP")]
    PHP,
    #[serde(rename = "PKR")]
    PKR,
    #[serde(rename = "PLD")]
    PLD,
    #[serde(rename = "PLN")]
    PLN,
    #[serde(rename = "PTE")]
    PTE,
    #[serde(rename = "PYG")]
    PYG,
    #[serde(rename = "QAR")]
    QAR,
    #[serde(rename = "ROL")]
    ROL,
    #[serde(rename = "RON")]
    RON,
    #[serde(rename = "RSD")]
    RSD,
    #[serde(rename = "RUB")]
    RUB,
    #[serde(rename = "RWF")]
    RWF,
    #[serde(rename = "SAR")]
    SAR,
    #[serde(rename = "SBD")]
    SBD,
    #[serde(rename = "SCR")]
    SCR,
    #[serde(rename = "SDD")]
    SDD,
    #[serde(rename = "SDG")]
    SDG,
    #[serde(rename = "SDP")]
    SDP,
    #[serde(rename = "SDR")]
    SDR,
    #[serde(rename = "SEK")]
    SEK,
    #[serde(rename = "SGD")]
    SGD,
    #[serde(rename = "SGd")]
    SGd,
    #[serde(rename = "SHP")]
    SHP,
    #[serde(rename = "SIT")]
    SIT,
    #[serde(rename = "SKK")]
    SKK,
    #[serde(rename = "SLE")]
    SLE,
    #[serde(rename = "SLL")]
    SLL,
    #[serde(rename = "SLV")]
    SLV,
    #[serde(rename = "SOS")]
    SOS,
    #[serde(rename = "SPL")]
    SPL,
    #[serde(rename = "SRD")]
    SRD,
    #[serde(rename = "SRG")]
    SRG,
    #[serde(rename = "SSP")]
    SSP,
    #[serde(rename = "STD")]
    STD,
    #[serde(rename = "STN")]
    STN,
    #[serde(rename = "SVC")]
    SVC,
    #[serde(rename = "SYP")]
    SYP,
    #[serde(rename = "SZL")]
    SZL,
    #[serde(rename = "SZl")]
    SZl,
    #[serde(rename = "THB")]
    THB,
    #[serde(rename = "THO")]
    THO,
    #[serde(rename = "TJS")]
    TJS,
    #[serde(rename = "TMM")]
    TMM,
    #[serde(rename = "TMT")]
    TMT,
    #[serde(rename = "TND")]
    TND,
    #[serde(rename = "TOP")]
    TOP,
    #[serde(rename = "TPE")]
    TPE,
    #[serde(rename = "TRL")]
    TRL,
    #[serde(rename = "TRY")]
    TRY,
    #[serde(rename = "TTD")]
    TTD,
    #[serde(rename = "TVD")]
    TVD,
    #[serde(rename = "TWD")]
    TWD,
    #[serde(rename = "TZS")]
    TZS,
    #[serde(rename = "UAH")]
    UAH,
    #[serde(rename = "UDI")]
    UDI,
    #[serde(rename = "UGX")]
    UGX,
    #[serde(rename = "US")]
    US,
    #[serde(rename = "USD")]
    USD,
    #[serde(rename = "USd")]
    USd,
    #[serde(rename = "UVR")]
    UVR,
    #[serde(rename = "UYI")]
    UYI,
    #[serde(rename = "UYU")]
    UYU,
    #[serde(rename = "UYW")]
    UYW,
    #[serde(rename = "UZS")]
    UZS,
    #[serde(rename = "VEB")]
    VEB,
    #[serde(rename = "VEE")]
    VEE,
    #[serde(rename = "VEF")]
    VEF,
    #[serde(rename = "VES")]
    VES,
    #[serde(rename = "VND")]
    VND,
    #[serde(rename = "VUV")]
    VUV,
    #[serde(rename = "WST")]
    WST,
    #[serde(rename = "X0S")]
    X0S,
    #[serde(rename = "X1S")]
    X1S,
    #[serde(rename = "X2S")]
    X2S,
    #[serde(rename = "X3S")]
    X3S,
    #[serde(rename = "X4S")]
    X4S,
    #[serde(rename = "X5S")]
    X5S,
    #[serde(rename = "X6S")]
    X6S,
    #[serde(rename = "X7S")]
    X7S,
    #[serde(rename = "X8S")]
    X8S,
    #[serde(rename = "X9S")]
    X9S,
    #[serde(rename = "XAD")]
    XAD,
    #[serde(rename = "XAF")]
    XAF,
    #[serde(rename = "XAG")]
    XAG,
    #[serde(rename = "XAL")]
    XAL,
    #[serde(rename = "XAO")]
    XAO,
    #[serde(rename = "XAS")]
    XAS,
    #[serde(rename = "XAU")]
    XAU,
    #[serde(rename = "XAV")]
    XAV,
    #[serde(rename = "XBA")]
    XBA,
    #[serde(rename = "XBI")]
    XBI,
    #[serde(rename = "XBN")]
    XBN,
    #[serde(rename = "XBS")]
    XBS,
    #[serde(rename = "XBT")]
    XBT,
    #[serde(rename = "XBW")]
    XBW,
    #[serde(rename = "XCD")]
    XCD,
    #[serde(rename = "XCG")]
    XCG,
    #[serde(rename = "XCR")]
    XCR,
    #[serde(rename = "XCS")]
    XCS,
    #[serde(rename = "XCU")]
    XCU,
    #[serde(rename = "XDG")]
    XDG,
    #[serde(rename = "XDH")]
    XDH,
    #[serde(rename = "XDI")]
    XDI,
    #[serde(rename = "XDO")]
    XDO,
    #[serde(rename = "XDR")]
    XDR,
    #[serde(rename = "XDT")]
    XDT,
    #[serde(rename = "XEG")]
    XEG,
    #[serde(rename = "XEN")]
    XEN,
    #[serde(rename = "XEO")]
    XEO,
    #[serde(rename = "XET")]
    XET,
    #[serde(rename = "XEU")]
    XEU,
    #[serde(rename = "XFI")]
    XFI,
    #[serde(rename = "XFL")]
    XFL,
    #[serde(rename = "XFM")]
    XFM,
    #[serde(rename = "XFT")]
    XFT,
    #[serde(rename = "XGZ")]
    XGZ,
    #[serde(rename = "XHB")]
    XHB,
    #[serde(rename = "XIC")]
    XIC,
    #[serde(rename = "XIN")]
    XIN,
    #[serde(rename = "XIO")]
    XIO,
    #[serde(rename = "XLC")]
    XLC,
    #[serde(rename = "XLI")]
    XLI,
    #[serde(rename = "XLM")]
    XLM,
    #[serde(rename = "XLU")]
    XLU,
    #[serde(rename = "XMA")]
    XMA,
    #[serde(rename = "XMK")]
    XMK,
    #[serde(rename = "XMN")]
    XMN,
    #[serde(rename = "XMR")]
    XMR,
    #[serde(rename = "XNI")]
    XNI,
    #[serde(rename = "XOF")]
    XOF,
    #[serde(rename = "XPB")]
    XPB,
    #[serde(rename = "XPD")]
    XPD,
    #[serde(rename = "XPF")]
    XPF,
    #[serde(rename = "XPT")]
    XPT,
    #[serde(rename = "XRA")]
    XRA,
    #[serde(rename = "XRH")]
    XRH,
    #[serde(rename = "XRI")]
    XRI,
    #[serde(rename = "XRP")]
    XRP,
    #[serde(rename = "XRU")]
    XRU,
    #[serde(rename = "XSA")]
    XSA,
    #[serde(rename = "XSN")]
    XSN,
    #[serde(rename = "XSO")]
    XSO,
    #[serde(rename = "XST")]
    XST,
    #[serde(rename = "XSU")]
    XSU,
    #[serde(rename = "XTH")]
    XTH,
    #[serde(rename = "XTK")]
    XTK,
    #[serde(rename = "XTR")]
    XTR,
    #[serde(rename = "XUC")]
    XUC,
    #[serde(rename = "XUN")]
    XUN,
    #[serde(rename = "XUT")]
    XUT,
    #[serde(rename = "XVC")]
    XVC,
    #[serde(rename = "XVV")]
    XVV,
    #[serde(rename = "XXT")]
    XXT,
    #[serde(rename = "XZC")]
    XZC,
    #[serde(rename = "XZI")]
    XZI,
    #[serde(rename = "YER")]
    YER,
    #[serde(rename = "ZAR")]
    ZAR,
    #[serde(rename = "ZAr")]
    ZAr,
    #[serde(rename = "ZMK")]
    ZMK,
    #[serde(rename = "ZMW")]
    ZMW,
    #[serde(rename = "ZWD")]
    ZWD,
    #[serde(rename = "ZWd")]
    ZWd,
    #[serde(rename = "ZWF")]
    ZWF,
    #[serde(rename = "ZWG")]
    ZWG,
    #[serde(rename = "ZWg")]
    ZWg,
    #[serde(rename = "ZWL")]
    ZWL,
    #[serde(rename = "ZWN")]
    ZWN,
    #[serde(rename = "ZWR")]
    ZWR,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_enum_serialization;

    test_enum_serialization!(test_serialize_usd, Currency, USD, "\"USD\"");
    test_enum_serialization!(test_serialize_aud, Currency, AUd, "\"AUd\"");
}
