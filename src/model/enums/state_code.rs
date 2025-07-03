//! Enum for all supported state codes in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::StateCode;
//! let mic = StateCode::CA; // California
//! ```
//!
//! For the full list of values, see: <https://api.openfigi.com/v3/mapping/values/stateCode>

use serde::{Deserialize, Serialize};

/// Enum for all supported state codes.
#[expect(missing_docs)]
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StateCode {
    #[serde(rename = "AB")]
    AB,
    #[serde(rename = "AC")]
    AC,
    #[serde(rename = "AH")]
    AH,
    #[serde(rename = "AK")]
    AK,
    #[serde(rename = "AL")]
    AL,
    #[serde(rename = "AM")]
    AM,
    #[serde(rename = "AR")]
    AR,
    #[serde(rename = "AS")]
    AS,
    #[serde(rename = "AT")]
    AT,
    #[serde(rename = "AZ")]
    AZ,
    #[serde(rename = "BC")]
    BC,
    #[serde(rename = "BJ")]
    BJ,
    #[serde(rename = "CA")]
    CA,
    #[serde(rename = "CB")]
    CB,
    #[serde(rename = "CO")]
    CO,
    #[serde(rename = "CQ")]
    CQ,
    #[serde(rename = "CT")]
    CT,
    #[serde(rename = "CZ")]
    CZ,
    #[serde(rename = "DC")]
    DC,
    #[serde(rename = "DE")]
    DE,
    #[serde(rename = "EH")]
    EH,
    #[serde(rename = "FH")]
    FH,
    #[serde(rename = "FI")]
    FI,
    #[serde(rename = "FJ")]
    FJ,
    #[serde(rename = "FL")]
    FL,
    #[serde(rename = "FO")]
    FO,
    #[serde(rename = "FS")]
    FS,
    #[serde(rename = "GA")]
    GA,
    #[serde(rename = "GD")]
    GD,
    #[serde(rename = "GF")]
    GF,
    #[serde(rename = "GM")]
    GM,
    #[serde(rename = "GS")]
    GS,
    #[serde(rename = "GU")]
    GU,
    #[serde(rename = "GX")]
    GX,
    #[serde(rename = "GZ")]
    GZ,
    #[serde(rename = "HA")]
    HA,
    #[serde(rename = "HB")]
    HB,
    #[serde(rename = "HE")]
    HE,
    #[serde(rename = "HG")]
    HG,
    #[serde(rename = "HI")]
    HI,
    #[serde(rename = "HL")]
    HL,
    #[serde(rename = "HN")]
    HN,
    #[serde(rename = "HO")]
    HO,
    #[serde(rename = "HS")]
    HS,
    #[serde(rename = "IA")]
    IA,
    #[serde(rename = "ID")]
    ID,
    #[serde(rename = "IG")]
    IG,
    #[serde(rename = "IK")]
    IK,
    #[serde(rename = "IL")]
    IL,
    #[serde(rename = "IN")]
    IN,
    #[serde(rename = "IT")]
    IT,
    #[serde(rename = "JL")]
    JL,
    #[serde(rename = "JS")]
    JS,
    #[serde(rename = "JX")]
    JX,
    #[serde(rename = "KA")]
    KA,
    #[serde(rename = "KC")]
    KC,
    #[serde(rename = "KN")]
    KN,
    #[serde(rename = "KO")]
    KO,
    #[serde(rename = "KS")]
    KS,
    #[serde(rename = "KT")]
    KT,
    #[serde(rename = "KU")]
    KU,
    #[serde(rename = "KY")]
    KY,
    #[serde(rename = "LA")]
    LA,
    #[serde(rename = "LN")]
    LN,
    #[serde(rename = "MA")]
    MA,
    #[serde(rename = "MB")]
    MB,
    #[serde(rename = "MD")]
    MD,
    #[serde(rename = "ME")]
    ME,
    #[serde(rename = "MG")]
    MG,
    #[serde(rename = "MI")]
    MI,
    #[serde(rename = "MN")]
    MN,
    #[serde(rename = "MO")]
    MO,
    #[serde(rename = "MS")]
    MS,
    #[serde(rename = "MT")]
    MT,
    #[serde(rename = "MZ")]
    MZ,
    #[serde(rename = "NB")]
    NB,
    #[serde(rename = "NC")]
    NC,
    #[serde(rename = "ND")]
    ND,
    #[serde(rename = "NE")]
    NE,
    #[serde(rename = "NG")]
    NG,
    #[serde(rename = "NH")]
    NH,
    #[serde(rename = "NJ")]
    NJ,
    #[serde(rename = "NL")]
    NL,
    #[serde(rename = "NM")]
    NM,
    #[serde(rename = "NN")]
    NN,
    #[serde(rename = "NR")]
    NR,
    #[serde(rename = "NS")]
    NS,
    #[serde(rename = "NT")]
    NT,
    #[serde(rename = "NU")]
    NU,
    #[serde(rename = "NV")]
    NV,
    #[serde(rename = "NW")]
    NW,
    #[serde(rename = "NX")]
    NX,
    #[serde(rename = "NY")]
    NY,
    #[serde(rename = "OH")]
    OH,
    #[serde(rename = "OK")]
    OK,
    #[serde(rename = "ON")]
    ON,
    #[serde(rename = "OR")]
    OR,
    #[serde(rename = "OS")]
    OS,
    #[serde(rename = "OT")]
    OT,
    #[serde(rename = "OY")]
    OY,
    #[serde(rename = "PA")]
    PA,
    #[serde(rename = "PE")]
    PE,
    #[serde(rename = "PR")]
    PR,
    #[serde(rename = "QC")]
    QC,
    #[serde(rename = "QH")]
    QH,
    #[serde(rename = "QL")]
    QL,
    #[serde(rename = "RI")]
    RI,
    #[serde(rename = "SA")]
    SA,
    #[serde(rename = "SC")]
    SC,
    #[serde(rename = "SD")]
    SD,
    #[serde(rename = "SH")]
    SH,
    #[serde(rename = "SI")]
    SI,
    #[serde(rename = "SK")]
    SK,
    #[serde(rename = "SN")]
    SN,
    #[serde(rename = "ST")]
    ST,
    #[serde(rename = "SX")]
    SX,
    #[serde(rename = "SZ")]
    SZ,
    #[serde(rename = "TA")]
    TA,
    #[serde(rename = "TG")]
    TG,
    #[serde(rename = "TJ")]
    TJ,
    #[serde(rename = "TK")]
    TK,
    #[serde(rename = "TN")]
    TN,
    #[serde(rename = "TS")]
    TS,
    #[serde(rename = "TT")]
    TT,
    #[serde(rename = "TX")]
    TX,
    #[serde(rename = "TY")]
    TY,
    #[serde(rename = "UT")]
    UT,
    #[serde(rename = "VA")]
    VA,
    #[serde(rename = "VI")]
    VI,
    #[serde(rename = "VT")]
    VT,
    #[serde(rename = "WA")]
    WA,
    #[serde(rename = "WI")]
    WI,
    #[serde(rename = "WK")]
    WK,
    #[serde(rename = "WV")]
    WV,
    #[serde(rename = "WY")]
    WY,
    #[serde(rename = "XJ")]
    XJ,
    #[serde(rename = "XZ")]
    XZ,
    #[serde(rename = "YA")]
    YA,
    #[serde(rename = "YN")]
    YN,
    #[serde(rename = "YT")]
    YT,
    #[serde(rename = "YU")]
    YU,
    #[serde(rename = "ZJ")]
    ZJ,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_enum_serialization;

    test_enum_serialization!(test_serialize_ab, StateCode, CA, "\"CA\"");
    test_enum_serialization!(test_serialize_ac, StateCode, YA, "\"YA\"");
}
