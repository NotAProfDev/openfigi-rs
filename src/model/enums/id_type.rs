//! Enum for all supported idType values in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::IdType;
//! let id_type = IdType::IdIsin; // International Securities Identification Number
//! ```
//!
//! For the full list of values, see: <https://api.openfigi.com/v3/mapping/values/idType>

use serde::{Deserialize, Serialize};

/// Enum for all supported idType values.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IdType {
    /// ISIN - International Securities Identification Number.
    /// Example: `[{"idType":"ID_ISIN","idValue":"XX1234567890"}]`
    IdIsin,
    /// Unique Bloomberg Identifier - A legacy, internal Bloomberg identifier.
    /// Example: `[{"idType":"ID_BB_UNIQUE","idValue":"EQ0010080100001000"}]`
    IdBbUnique,
    /// Sedol Number - Stock Exchange Daily Official List.
    /// Example: `[{"idType":"ID_SEDOL","idValue":"1234567"}]`
    IdSedol,
    /// Common Code - A nine digit identification number.
    /// Example: `[{"idType":"ID_COMMON","idValue":"123456789"}]`
    IdCommon,
    /// Wertpapierkennnummer/WKN - German securities identification code.
    /// Example: `[{"idType":"ID_WERTPAPIER","idValue":"123456"}]`
    IdWertpapier,
    /// CUSIP - Committee on Uniform Securities Identification Procedures.
    /// Example: `[{"idType":"ID_CUSIP","idValue":"123456789"}]`
    IdCusip,
    /// CINS - CUSIP International Numbering System.
    /// Example: `[{"idType":"ID_CINS","idValue":"A12345678"}]`
    IdCins,
    /// A legacy Bloomberg identifier.
    /// Example: `[{"idType":"ID_BB","idValue":"123456789"}]`
    IdBb,
    /// A legacy Bloomberg identifier (8 characters only).
    /// Example: `[{"idType":"ID_BB_8_CHR","idValue":"12345678"}]`
    IdBb8Chr,
    /// Trace eligible bond identifier issued by FINRA.
    /// Example: `[{"idType":"ID_TRACE","idValue":"12345678"}]`
    IdTrace,
    /// Italian Identifier Number - The Italian Identification number consisting of five or six digits.
    /// Example: `[{"idType":"ID_ITALY","idValue":"123456"}]`
    IdItaly,
    /// Common Code - A nine digit identification number.
    /// Example: `[{"idType":"ID_EXCH_SYMBOL","idValue":"IBM123"}]`
    IdExchSymbol,
    /// Full Exchange Symbol - Contains the exchange symbol for futures, options, indices inclusive of base symbol and other security elements.
    /// Example: `[{"idType":"ID_FULL_EXCHANGE_SYMBOL", "idValue":"ABC 123456789"}]`
    IdFullExchangeSymbol,
    /// Composite Financial Instrument Global Identifier - The Composite Financial Instrument Global Identifier (FIGI) enables users to link multiple FIGIs at the trading venue level within the same country or market in order to obtain an aggregated view for an instrument within that country or market.
    /// Example: `[{"idType":"COMPOSITE_ID_BB_GLOBAL", "idValue":"BBG000BLNNH6"}]`
    CompositeIdBbGlobal,
    /// Share Class Financial Instrument Global Identifier - A Share Class level Financial Instrument Global Identifier is assigned to an instrument that is traded in more than one country. This enables users to link multiple Composite FIGIs for the same instrument in order to obtain an aggregated view for that instrument across all countries (globally).
    /// Example: `[{"idType":"ID_BB_GLOBAL_SHARE_CLASS_LEVEL", "idValue":"BBG001S5S399"}]`
    IdBbGlobalShareClassLevel,
    /// Financial Instrument Global Identifier (FIGI) - An identifier that is assigned to instruments of all asset classes and is unique to an individual instrument. Once issued, the FIGI assigned to an instrument will not change.
    /// Example: `[{"idType":"ID_BB_GLOBAL", "idValue":"BBG0000362Y4"}]`
    IdBbGlobal,
    /// Security ID Number Description - Descriptor for a financial instrument. Similar to the ticker field, but will provide additional metadata data.
    /// Example: `[{"idType":"ID_BB_SEC_NUM_DES", "idValue":"IBM 7 10/30/25"}]`
    IdBbSecNumDes,
    /// Ticker - Ticker is a specific identifier for a financial instrument that reflects common usage.
    /// Example: `[{"idType":"TICKER", "idValue":"IBM"}]`
    Ticker,
    /// An indistinct identifier which may be linked to multiple instruments. May need to be combined with other values to identify a unique instrument.
    /// Example: `[{"idType":"BASE_TICKER","idValue":"IBM"}]`
    BaseTicker,
    /// CUSIP (8 Characters Only) - Committee on Uniform Securities Identification Procedures.
    /// Example: `[{"idType":"ID_CUSIP_8_CHR","idValue":"12345678"}]`
    IdCusip8Chr,
    /// OCC Symbol - A twenty-one character option symbol standardized by the Options Clearing Corporation (OCC) to identify a U.S. option.
    /// Example: `[{"idType":"OCC_SYMBOL","idValue":"ABC 123456789123456789"}]`
    OccSymbol,
    /// Unique Identifier for Future Option - Bloomberg unique ticker with logic for index, currency, single stock futures, commodities and commodity options.
    /// Example: `[{"idType":"UNIQUE_ID_FUT_OPT","idValue":"ABCD=A1 Equity"}]`
    UniqueIdFutOpt,
    /// OPRA Symbol - Option symbol standardized by the Options Price Reporting Authority (OPRA) to identify a U.S. option.
    /// Example: `[{"idType":"OPRA_SYMBOL","idValue":"ABC A123A456789"}]`
    OpraSymbol,
    /// Trading System Identifier - Unique identifier for the instrument as used on the source trading system.
    /// Example: `[{"idType":"TRADING_SYSTEM_IDENTIFIER","idValue":"FZH18 IBMG"}]`
    TradingSystemIdentifier,
    /// An exchange venue specific code to identify fixed income instruments primarily traded in Asia.
    /// Example: `[{"idType":"ID_SHORT_CODE","idValue":"123456.AB"}]`
    IdShortCode,
    /// Index code assigned by the index provider for the purpose of identifying the security.
    /// Example: `[{"idType":"VENDOR_INDEX_CODE","idValue":"123456"}]`
    VendorIndexCode,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_id_isin() {
        let id = IdType::IdIsin;
        let serialized = serde_json::to_string(&id).unwrap();
        assert_eq!(serialized, "\"ID_ISIN\"");
    }

    #[test]
    fn test_deserialize_id_isin() {
        let json = "\"ID_ISIN\"";
        let deserialized: IdType = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, IdType::IdIsin);
    }

    #[test]
    fn test_serialize_vendor_index_code() {
        let id = IdType::VendorIndexCode;
        let serialized = serde_json::to_string(&id).unwrap();
        assert_eq!(serialized, "\"VENDOR_INDEX_CODE\"");
    }

    #[test]
    fn test_deserialize_vendor_index_code() {
        let json = "\"VENDOR_INDEX_CODE\"";
        let deserialized: IdType = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, IdType::VendorIndexCode);
    }
}
