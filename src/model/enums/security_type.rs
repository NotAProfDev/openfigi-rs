//! Enum for all supported security types in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::SecurityType;
//! let security_type = SecurityType::ABSCard; // ABS Card
//! ```
//!
//! For the full list of values, see: <https://api.openfigi.com/v3/mapping/values/securityType>

use serde::{Deserialize, Serialize};

/// Enum for all supported security types.
#[expect(missing_docs)]
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SecurityType {
    #[serde(rename = "ABS Auto")]
    ABSAuto,
    #[serde(rename = "ABS Card")]
    ABSCard,
    #[serde(rename = "ABS Home")]
    ABSHome,
    #[serde(rename = "ABS Other")]
    ABSOther,
    #[serde(rename = "ACCEPT BANCARIA")]
    ACCEPTBANCARIA,
    #[serde(rename = "ADJ CONV. TO FIXED")]
    ADJCONVTOFIXED,
    #[serde(rename = "ADJ CONV. TO FIXED, OID")]
    ADJCONVTOFIXEDOID,
    #[serde(rename = "ADJUSTABLE")]
    ADJUSTABLE,
    #[serde(rename = "ADJUSTABLE, OID")]
    ADJUSTABLEOID,
    #[serde(rename = "ADR")]
    ADR,
    #[serde(rename = "Agncy ABS Home")]
    AgncyABSHome,
    #[serde(rename = "Agncy ABS Other")]
    AgncyABSOther,
    #[serde(rename = "Agncy CMBS")]
    AgncyCMBS,
    #[serde(rename = "Agncy CMO FLT")]
    AgncyCMOFLT,
    #[serde(rename = "Agncy CMO INV")]
    AgncyCMOINV,
    #[serde(rename = "Agncy CMO IO")]
    AgncyCMOIO,
    #[serde(rename = "Agncy CMO Other")]
    AgncyCMOOther,
    #[serde(rename = "Agncy CMO PO")]
    AgncyCMOPO,
    #[serde(rename = "Agncy CMO Z")]
    AgncyCMOZ,
    #[serde(rename = "Asset-Based")]
    AssetBased,
    #[serde(rename = "ASSET-BASED BRIDGE")]
    ASSETBASEDBRIDGE,
    #[serde(rename = "ASSET-BASED BRIDGE REV")]
    ASSETBASEDBRIDGEREV,
    #[serde(rename = "ASSET-BASED BRIDGE TERM")]
    ASSETBASEDBRIDGETERM,
    #[serde(rename = "ASSET-BASED DELAY-DRAW TERM")]
    ASSETBASEDDELAYDRAWTERM,
    #[serde(rename = "ASSET-BASED DIP")]
    ASSETBASEDDIP,
    #[serde(rename = "ASSET-BASED DIP DELAY-DRAW")]
    ASSETBASEDDIPDELAYDRAW,
    #[serde(rename = "ASSET-BASED DIP REV")]
    ASSETBASEDDIPREV,
    #[serde(rename = "ASSET-BASED DIP TERM")]
    ASSETBASEDDIPTERM,
    #[serde(rename = "ASSET-BASED LOC")]
    ASSETBASEDLOC,
    #[serde(rename = "ASSET-BASED PIK REV")]
    ASSETBASEDPIKREV,
    #[serde(rename = "ASSET-BASED PIK TERM")]
    ASSETBASEDPIKTERM,
    #[serde(rename = "ASSET-BASED REV")]
    ASSETBASEDREV,
    #[serde(rename = "ASSET-BASED TERM")]
    ASSETBASEDTERM,
    #[serde(rename = "AUSTRALIAN")]
    AUSTRALIAN,
    #[serde(rename = "AUSTRALIAN CD")]
    AUSTRALIANCD,
    #[serde(rename = "AUSTRALIAN CP")]
    AUSTRALIANCP,
    #[serde(rename = "Austrian Crt")]
    AustrianCrt,
    #[serde(rename = "BANK ACCEPT BILL")]
    BANKACCEPTBILL,
    #[serde(rename = "BANK BILL")]
    BANKBILL,
    #[serde(rename = "BANK NOTE")]
    BANKNOTE,
    #[serde(rename = "BANKERS ACCEPT")]
    BANKERSACCEPT,
    #[serde(rename = "BANKERS ACCEPTANCE")]
    BANKERSACCEPTANCE,
    #[serde(rename = "BASIS SWAP")]
    BASISSWAP,
    #[serde(rename = "BASIS TRADE ON CLOSE")]
    BASISTRADEONCLOSE,
    #[serde(rename = "Basket WRT")]
    BasketWRT,
    #[serde(rename = "BDR")]
    BDR,
    #[serde(rename = "BEARER DEP NOTE")]
    BEARERDEPNOTE,
    #[serde(rename = "Belgium Cert")]
    BelgiumCert,
    #[serde(rename = "BELGIUM CP")]
    BELGIUMCP,
    #[serde(rename = "BILL OF EXCHANGE")]
    BILLOFEXCHANGE,
    #[serde(rename = "BILLET A ORDRE")]
    BILLETAORDRE,
    #[serde(rename = "Bond")]
    Bond,
    #[serde(rename = "BRAZIL GENERIC")]
    BRAZILGENERIC,
    #[serde(rename = "BRAZILIAN CDI")]
    BRAZILIANCDI,
    #[serde(rename = "BRIDGE")]
    BRIDGE,
    #[serde(rename = "BRIDGE DELAY-DRAW")]
    BRIDGEDELAYDRAW,
    #[serde(rename = "BRIDGE DELAY-DRAW TERM")]
    BRIDGEDELAYDRAWTERM,
    #[serde(rename = "BRIDGE DIP TERM")]
    BRIDGEDIPTERM,
    #[serde(rename = "BRIDGE GUARANTEE FAC")]
    BRIDGEGUARANTEEFAC,
    #[serde(rename = "BRIDGE ISLAMIC")]
    BRIDGEISLAMIC,
    #[serde(rename = "BRIDGE ISLAMIC TERM")]
    BRIDGEISLAMICTERM,
    #[serde(rename = "BRIDGE PIK")]
    BRIDGEPIK,
    #[serde(rename = "BRIDGE PIK REV")]
    BRIDGEPIKREV,
    #[serde(rename = "BRIDGE PIK TERM")]
    BRIDGEPIKTERM,
    #[serde(rename = "BRIDGE REV")]
    BRIDGEREV,
    #[serde(rename = "BRIDGE REV GUARANTEE FAC")]
    BRIDGEREVGUARANTEEFAC,
    #[serde(rename = "BRIDGE STANDBY TERM")]
    BRIDGESTANDBYTERM,
    #[serde(rename = "BRIDGE TERM")]
    BRIDGETERM,
    #[serde(rename = "BRIDGE TERM GUARANTEE FAC")]
    BRIDGETERMGUARANTEEFAC,
    #[serde(rename = "BRIDGE TERM VAT-TRNCH")]
    BRIDGETERMVATTRNCH,
    #[serde(rename = "BRIDGE VAT-TRNCH")]
    BRIDGEVATTRNCH,
    #[serde(rename = "BULLDOG")]
    BULLDOG,
    #[serde(rename = "BUTTERFLY SWAP")]
    BUTTERFLYSWAP,
    #[serde(rename = "CAD INT BEAR CP")]
    CADINTBEARCP,
    #[serde(rename = "CALC_INSTRUMENT")]
    CALCINSTRUMENT,
    #[serde(rename = "Calendar Spread Option")]
    CalendarSpreadOption,
    #[serde(rename = "CALL LOANS")]
    CALLLOANS,
    #[serde(rename = "CALLABLE CP")]
    CALLABLECP,
    #[serde(rename = "CANADIAN")]
    CANADIAN,
    #[serde(rename = "Canadian")]
    Canadian,
    #[serde(rename = "CANADIAN CD")]
    CANADIANCD,
    #[serde(rename = "CANADIAN CP")]
    CANADIANCP,
    #[serde(rename = "Canadian DR")]
    CanadianDR,
    #[serde(rename = "CAPS & FLOORS")]
    CAPSANDFLOORS,
    #[serde(rename = "Car Forward")]
    CarForward,
    #[serde(rename = "CASH")]
    CASH,
    #[serde(rename = "CASH FLOW")]
    CASHFLOW,
    #[serde(rename = "CASH FLOW, OID")]
    CASHFLOWOID,
    #[serde(rename = "CASH RATE")]
    CASHRATE,
    #[serde(rename = "CBLO")]
    CBLO,
    #[serde(rename = "CD")]
    CD,
    #[serde(rename = "CDI")]
    CDI,
    #[serde(rename = "CDR")]
    CDR,
    #[serde(rename = "CEDEAR")]
    CEDEAR,
    #[serde(rename = "CF")]
    CF,
    #[serde(rename = "CHILEAN CD")]
    CHILEANCD,
    #[serde(rename = "CHILEAN DN")]
    CHILEANDN,
    #[serde(rename = "Closed-End Fund")]
    ClosedEndFund,
    #[serde(rename = "CMBS")]
    CMBS,
    #[serde(rename = "Cmdt Fut WRT")]
    CmdtFutWRT,
    #[serde(rename = "Cmdt Idx WRT")]
    CmdtIdxWRT,
    #[serde(rename = "COLLAT CALL NOTE")]
    COLLATCALLNOTE,
    #[serde(rename = "COLOMBIAN CD")]
    COLOMBIANCD,
    #[serde(rename = "COMMERCIAL NOTE")]
    COMMERCIALNOTE,
    #[serde(rename = "COMMERCIAL PAPER")]
    COMMERCIALPAPER,
    #[serde(rename = "Commodity Index")]
    CommodityIndex,
    #[serde(rename = "Common Stock")]
    CommonStock,
    #[serde(rename = "CONTRACT FOR DIFFERENCE")]
    CONTRACTFORDIFFERENCE,
    #[serde(rename = "CONTRACT FRA")]
    CONTRACTFRA,
    #[serde(rename = "Conv Bond")]
    ConvBond,
    #[serde(rename = "Conv Prfd")]
    ConvPrfd,
    #[serde(rename = "Corp Bnd WRT")]
    CorpBndWRT,
    #[serde(rename = "Cover Pool")]
    CoverPool,
    #[serde(rename = "CP-LIKE EXT NOTE")]
    CPLIKEEXTNOTE,
    #[serde(rename = "CPI LINKED")]
    CPILINKED,
    #[serde(rename = "CROSS")]
    CROSS,
    #[serde(rename = "Crypto")]
    Crypto,
    #[serde(rename = "Currency future.")]
    Currencyfuture,
    #[serde(rename = "Currency option.")]
    Currencyoption,
    #[serde(rename = "Currency spot.")]
    Currencyspot,
    #[serde(rename = "Currency WRT")]
    CurrencyWRT,
    #[serde(rename = "CURVE_ROLL")]
    CURVEROLL,
    #[serde(rename = "DELAY-DRAW")]
    DELAYDRAW,
    #[serde(rename = "DELAY-DRAW ISLAMIC")]
    DELAYDRAWISLAMIC,
    #[serde(rename = "DELAY-DRAW ISLAMIC LOC")]
    DELAYDRAWISLAMICLOC,
    #[serde(rename = "DELAY-DRAW ISLAMIC TERM")]
    DELAYDRAWISLAMICTERM,
    #[serde(rename = "DELAY-DRAW LOC")]
    DELAYDRAWLOC,
    #[serde(rename = "DELAY-DRAW PIK TERM")]
    DELAYDRAWPIKTERM,
    #[serde(rename = "DELAY-DRAW STANDBY TERM")]
    DELAYDRAWSTANDBYTERM,
    #[serde(rename = "DELAY-DRAW TERM")]
    DELAYDRAWTERM,
    #[serde(rename = "DELAY-DRAW TERM GUARANTEE F")]
    DELAYDRAWTERMGUARANTEEF,
    #[serde(rename = "DELAY-DRAW TERM VAT-TRNCH")]
    DELAYDRAWTERMVATTRNCH,
    #[serde(rename = "DEPOSIT")]
    DEPOSIT,
    #[serde(rename = "DEPOSIT NOTE")]
    DEPOSITNOTE,
    #[serde(rename = "DIM SUM BRIDGE TERM")]
    DIMSUMBRIDGETERM,
    #[serde(rename = "DIM SUM DELAY-DRAW TERM")]
    DIMSUMDELAYDRAWTERM,
    #[serde(rename = "DIM SUM REV")]
    DIMSUMREV,
    #[serde(rename = "DIM SUM TERM")]
    DIMSUMTERM,
    #[serde(rename = "DIP")]
    DIP,
    #[serde(rename = "DIP DELAY-DRAW ISLAMIC TERM")]
    DIPDELAYDRAWISLAMICTERM,
    #[serde(rename = "DIP DELAY-DRAW PIK TERM")]
    DIPDELAYDRAWPIKTERM,
    #[serde(rename = "DIP DELAY-DRAW TERM")]
    DIPDELAYDRAWTERM,
    #[serde(rename = "DIP LOC")]
    DIPLOC,
    #[serde(rename = "DIP PIK TERM")]
    DIPPIKTERM,
    #[serde(rename = "DIP REV")]
    DIPREV,
    #[serde(rename = "DIP STANDBY LOC")]
    DIPSTANDBYLOC,
    #[serde(rename = "DIP SYNTH LOC")]
    DIPSYNTHLOC,
    #[serde(rename = "DIP TERM")]
    DIPTERM,
    #[serde(rename = "DISCOUNT FIXBIS")]
    DISCOUNTFIXBIS,
    #[serde(rename = "DISCOUNT NOTES")]
    DISCOUNTNOTES,
    #[serde(rename = "DIVIDEND NEUTRAL STOCK FUTURE")]
    DIVIDENDNEUTRALSTOCKFUTURE,
    #[serde(rename = "DOMESTC TIME DEP")]
    DOMESTCTIMEDEP,
    #[serde(rename = "DOMESTIC")]
    DOMESTIC,
    #[serde(rename = "DOMESTIC MTN")]
    DOMESTICMTN,
    #[serde(rename = "Dutch Cert")]
    DutchCert,
    #[serde(rename = "DUTCH CP")]
    DUTCHCP,
    #[serde(rename = "EDR")]
    EDR,
    #[serde(rename = "Equity Index")]
    EquityIndex,
    #[serde(rename = "Equity Option")]
    EquityOption,
    #[serde(rename = "Equity WRT")]
    EquityWRT,
    #[serde(rename = "ETP")]
    ETP,
    #[serde(rename = "EURO CD")]
    EUROCD,
    #[serde(rename = "EURO CP")]
    EUROCP,
    #[serde(rename = "EURO MTN")]
    EUROMTN,
    #[serde(rename = "EURO NON-DOLLAR")]
    EURONONDOLLAR,
    #[serde(rename = "EURO STRUCTRD LN")]
    EUROSTRUCTRDLN,
    #[serde(rename = "EURO TIME DEPST")]
    EUROTIMEDEPST,
    #[serde(rename = "EURO-DOLLAR")]
    EURODOLLAR,
    #[serde(rename = "EURO-ZONE")]
    EUROZONE,
    #[serde(rename = "EXTEND COMM NOTE")]
    EXTENDCOMMNOTE,
    #[serde(rename = "EXTEND. NOTE MTN")]
    EXTENDNOTEMTN,
    #[serde(rename = "FDIC")]
    FDIC,
    #[serde(rename = "FED FUNDS")]
    FEDFUNDS,
    #[serde(rename = "FIDC")]
    FIDC,
    #[serde(rename = "Financial commodity future.")]
    Financialcommodityfuture,
    #[serde(rename = "Financial commodity generic.")]
    Financialcommoditygeneric,
    #[serde(rename = "Financial commodity option.")]
    Financialcommodityoption,
    #[serde(rename = "Financial commodity spot.")]
    Financialcommodityspot,
    #[serde(rename = "Financial index future.")]
    Financialindexfuture,
    #[serde(rename = "Financial index generic.")]
    Financialindexgeneric,
    #[serde(rename = "Financial index option.")]
    Financialindexoption,
    #[serde(rename = "FINNISH CD")]
    FINNISHCD,
    #[serde(rename = "FINNISH CP")]
    FINNISHCP,
    #[serde(rename = "FIXED")]
    FIXED,
    #[serde(rename = "Fixed Income Index")]
    FixedIncomeIndex,
    #[serde(rename = "FIXED, OID")]
    FIXEDOID,
    #[serde(rename = "FIXING RATE")]
    FIXINGRATE,
    #[serde(rename = "FLOATING")]
    FLOATING,
    #[serde(rename = "FLOATING CP")]
    FLOATINGCP,
    #[serde(rename = "FLOATING, OID")]
    FLOATINGOID,
    #[serde(rename = "FNMA FHAVA")]
    FNMAFHAVA,
    #[serde(rename = "Foreign Sh.")]
    ForeignSh,
    #[serde(rename = "FORWARD")]
    FORWARD,
    #[serde(rename = "FORWARD CROSS")]
    FORWARDCROSS,
    #[serde(rename = "FORWARD CURVE")]
    FORWARDCURVE,
    #[serde(rename = "FRA")]
    FRA,
    #[serde(rename = "FRENCH CD")]
    FRENCHCD,
    #[serde(rename = "French Cert")]
    FrenchCert,
    #[serde(rename = "FRENCH CP")]
    FRENCHCP,
    #[serde(rename = "Fund of Funds")]
    FundofFunds,
    #[serde(rename = "Futures Monthly Ticker")]
    FuturesMonthlyTicker,
    #[serde(rename = "FWD SWAP")]
    FWDSWAP,
    #[serde(rename = "FX Curve")]
    FXCurve,
    #[serde(rename = "FX DISCOUNT NOTE")]
    FXDISCOUNTNOTE,
    #[serde(rename = "GDR")]
    GDR,
    #[serde(rename = "Generic currency future.")]
    Genericcurrencyfuture,
    #[serde(rename = "Generic index future.")]
    Genericindexfuture,
    #[serde(rename = "German Cert")]
    GermanCert,
    #[serde(rename = "GERMAN CP")]
    GERMANCP,
    #[serde(rename = "GLOBAL")]
    GLOBAL,
    #[serde(rename = "GUARANTEE FAC")]
    GUARANTEEFAC,
    #[serde(rename = "HB")]
    HB,
    #[serde(rename = "HDR")]
    HDR,
    #[serde(rename = "HONG KONG CD")]
    HONGKONGCD,
    #[serde(rename = "I.R. Fut WRT")]
    IRFutWRT,
    #[serde(rename = "I.R. Swp WRT")]
    IRSwpWRT,
    #[serde(rename = "IDR")]
    IDR,
    #[serde(rename = "IMM FORWARD")]
    IMMFORWARD,
    #[serde(rename = "IMM SWAP")]
    IMMSWAP,
    #[serde(rename = "Index")]
    Index,
    #[serde(rename = "Index Option")]
    IndexOption,
    #[serde(rename = "Index WRT")]
    IndexWRT,
    #[serde(rename = "INDIAN CD")]
    INDIANCD,
    #[serde(rename = "INDIAN CP")]
    INDIANCP,
    #[serde(rename = "INDONESIAN CP")]
    INDONESIANCP,
    #[serde(rename = "Indx Fut WRT")]
    IndxFutWRT,
    #[serde(rename = "INFLATION SWAP")]
    INFLATIONSWAP,
    #[serde(rename = "INT BEAR FIXBIS")]
    INTBEARFIXBIS,
    #[serde(rename = "Int. Rt. WRT")]
    IntRtWRT,
    #[serde(rename = "INTER. APPRECIATION")]
    INTERAPPRECIATION,
    #[serde(rename = "INTER. APPRECIATION, OID")]
    INTERAPPRECIATIONOID,
    #[serde(rename = "ISLAMIC")]
    ISLAMIC,
    #[serde(rename = "ISLAMIC BA")]
    ISLAMICBA,
    #[serde(rename = "ISLAMIC CP")]
    ISLAMICCP,
    #[serde(rename = "ISLAMIC GUARANTEE FAC")]
    ISLAMICGUARANTEEFAC,
    #[serde(rename = "ISLAMIC LOC")]
    ISLAMICLOC,
    #[serde(rename = "ISLAMIC REV")]
    ISLAMICREV,
    #[serde(rename = "ISLAMIC STANDBY")]
    ISLAMICSTANDBY,
    #[serde(rename = "ISLAMIC STANDBY REV")]
    ISLAMICSTANDBYREV,
    #[serde(rename = "ISLAMIC STANDBY TERM")]
    ISLAMICSTANDBYTERM,
    #[serde(rename = "ISLAMIC TERM")]
    ISLAMICTERM,
    #[serde(rename = "ISLAMIC TERM GUARANTEE FAC")]
    ISLAMICTERMGUARANTEEFAC,
    #[serde(rename = "ISLAMIC TERM VAT-TRNCH")]
    ISLAMICTERMVATTRNCH,
    #[serde(rename = "JUMBO CD")]
    JUMBOCD,
    #[serde(rename = "KOREAN CD")]
    KOREANCD,
    #[serde(rename = "KOREAN CP")]
    KOREANCP,
    #[serde(rename = "LEBANESE CP")]
    LEBANESECP,
    #[serde(rename = "LIQUIDITY NOTE")]
    LIQUIDITYNOTE,
    #[serde(rename = "LOC")]
    LOC,
    #[serde(rename = "LOC GUARANTEE FAC")]
    LOCGUARANTEEFAC,
    #[serde(rename = "LOC TERM")]
    LOCTERM,
    #[serde(rename = "Ltd Part")]
    LtdPart,
    #[serde(rename = "MALAYSIAN CP")]
    MALAYSIANCP,
    #[serde(rename = "Managed Account")]
    ManagedAccount,
    #[serde(rename = "MARGIN TERM DEP")]
    MARGINTERMDEP,
    #[serde(rename = "MASTER NOTES")]
    MASTERNOTES,
    #[serde(rename = "MBS 10yr")]
    MBS10yr,
    #[serde(rename = "MBS 15yr")]
    MBS15yr,
    #[serde(rename = "MBS 20yr")]
    MBS20yr,
    #[serde(rename = "MBS 30yr")]
    MBS30yr,
    #[serde(rename = "MBS 35yr")]
    MBS35yr,
    #[serde(rename = "MBS 40yr")]
    MBS40yr,
    #[serde(rename = "MBS 50yr")]
    MBS50yr,
    #[serde(rename = "MBS 5yr")]
    MBS5yr,
    #[serde(rename = "MBS 7yr")]
    MBS7yr,
    #[serde(rename = "MBS ARM")]
    MBSARM,
    #[serde(rename = "MBS balloon")]
    MBSballoon,
    #[serde(rename = "MBS Other")]
    MBSOther,
    #[serde(rename = "MED TERM NOTE")]
    MEDTERMNOTE,
    #[serde(rename = "MEDIUM TERM CD")]
    MEDIUMTERMCD,
    #[serde(rename = "MEDIUM TERM ECD")]
    MEDIUMTERMECD,
    #[serde(rename = "MEXICAN CP")]
    MEXICANCP,
    #[serde(rename = "MEXICAN PAGARE")]
    MEXICANPAGARE,
    #[serde(rename = "Misc.")]
    Misc,
    #[serde(rename = "MLP")]
    MLP,
    #[serde(rename = "MONETARY BILLS")]
    MONETARYBILLS,
    #[serde(rename = "MONEY MARKET CALL")]
    MONEYMARKETCALL,
    #[serde(rename = "MUNI CP")]
    MUNICP,
    #[serde(rename = "MUNI INT BEAR CP")]
    MUNIINTBEARCP,
    #[serde(rename = "MUNI SWAP")]
    MUNISWAP,
    #[serde(rename = "MURABAHA")]
    MURABAHA,
    #[serde(rename = "Mutual Fund")]
    MutualFund,
    #[serde(rename = "MV")]
    MV,
    #[serde(rename = "MX CERT BURSATIL")]
    MXCERTBURSATIL,
    #[serde(rename = "NDF SWAP")]
    NDFSWAP,
    #[serde(rename = "NEG EURO CP")]
    NEGEUROCP,
    #[serde(rename = "NEG INST DEPOSIT")]
    NEGINSTDEPOSIT,
    #[serde(rename = "NEGOTIABLE CD")]
    NEGOTIABLECD,
    #[serde(rename = "NEW ZEALAND CD")]
    NEWZEALANDCD,
    #[serde(rename = "NEW ZEALAND CP")]
    NEWZEALANDCP,
    #[serde(rename = "NON-DELIVERABLE FORWARD")]
    NONDELIVERABLEFORWARD,
    #[serde(rename = "NON-DELIVERABLE IRS SWAP")]
    NONDELIVERABLEIRSSWAP,
    #[serde(rename = "NVDR")]
    NVDR,
    #[serde(rename = "NY Reg Shrs")]
    NYRegShrs,
    #[serde(rename = "OID")]
    OID,
    #[serde(rename = "ONSHORE FORWARD")]
    ONSHOREFORWARD,
    #[serde(rename = "ONSHORE SWAP")]
    ONSHORESWAP,
    #[serde(rename = "Open-End Fund")]
    OpenEndFund,
    #[serde(rename = "OPTION")]
    OPTION,
    #[serde(rename = "Option on Equity Future")]
    OptiononEquityFuture,
    #[serde(rename = "OPTION VOLATILITY")]
    OPTIONVOLATILITY,
    #[serde(rename = "OTHER")]
    OTHER,
    #[serde(rename = "OVER/NIGHT")]
    OVERNIGHT,
    #[serde(rename = "OVERDRAFT")]
    OVERDRAFT,
    #[serde(rename = "OVERNIGHT INDEXED SWAP")]
    OVERNIGHTINDEXEDSWAP,
    #[serde(rename = "PANAMANIAN CP")]
    PANAMANIANCP,
    #[serde(rename = "Participate Cert")]
    ParticipateCert,
    #[serde(rename = "PHILIPPINE CP")]
    PHILIPPINECP,
    #[serde(rename = "Physical commodity forward.")]
    Physicalcommodityforward,
    #[serde(rename = "Physical commodity future.")]
    Physicalcommodityfuture,
    #[serde(rename = "Physical commodity generic.")]
    Physicalcommoditygeneric,
    #[serde(rename = "Physical commodity option.")]
    Physicalcommodityoption,
    #[serde(rename = "Physical commodity spot.")]
    Physicalcommodityspot,
    #[serde(rename = "Physical index future.")]
    Physicalindexfuture,
    #[serde(rename = "Physical index option.")]
    Physicalindexoption,
    #[serde(rename = "PIK")]
    PIK,
    #[serde(rename = "PIK LOC")]
    PIKLOC,
    #[serde(rename = "PIK REV")]
    PIKREV,
    #[serde(rename = "PIK SYNTH LOC")]
    PIKSYNTHLOC,
    #[serde(rename = "PIK TERM")]
    PIKTERM,
    #[serde(rename = "PLAZOS FIJOS")]
    PLAZOSFIJOS,
    #[serde(rename = "PORTUGUESE CP")]
    PORTUGUESECP,
    #[serde(rename = "Preference")]
    Preference,
    #[serde(rename = "Preferred")]
    Preferred,
    #[serde(rename = "PRES")]
    PRES,
    #[serde(rename = "Prfd WRT")]
    PrfdWRT,
    #[serde(rename = "PRIV PLACEMENT")]
    PRIVPLACEMENT,
    #[serde(rename = "PRIVATE")]
    PRIVATE,
    #[serde(rename = "Private Comp")]
    PrivateComp,
    #[serde(rename = "Private-equity backed")]
    Privateequitybacked,
    #[serde(rename = "PROMISSORY NOTE")]
    PROMISSORYNOTE,
    #[serde(rename = "PROV T-BILL")]
    PROVTBILL,
    #[serde(rename = "Prvt CMBS")]
    PrvtCMBS,
    #[serde(rename = "Prvt CMO FLT")]
    PrvtCMOFLT,
    #[serde(rename = "Prvt CMO INV")]
    PrvtCMOINV,
    #[serde(rename = "Prvt CMO IO")]
    PrvtCMOIO,
    #[serde(rename = "Prvt CMO Other")]
    PrvtCMOOther,
    #[serde(rename = "Prvt CMO PO")]
    PrvtCMOPO,
    #[serde(rename = "Prvt CMO Z")]
    PrvtCMOZ,
    #[serde(rename = "PUBLIC")]
    PUBLIC,
    #[serde(rename = "Pvt Eqty Fund")]
    PvtEqtyFund,
    #[serde(rename = "RDC")]
    RDC,
    #[serde(rename = "Receipt")]
    Receipt,
    #[serde(rename = "REIT")]
    REIT,
    #[serde(rename = "REPO")]
    REPO,
    #[serde(rename = "RESERVE-BASED DIP REV")]
    RESERVEBASEDDIPREV,
    #[serde(rename = "RESERVE-BASED REV")]
    RESERVEBASEDREV,
    #[serde(rename = "RESERVE-BASED TERM")]
    RESERVEBASEDTERM,
    #[serde(rename = "RESTRUCTURD DEBT")]
    RESTRUCTURDDEBT,
    #[serde(rename = "RETAIL CD")]
    RETAILCD,
    #[serde(rename = "RETURN IDX")]
    RETURNIDX,
    #[serde(rename = "REV")]
    REV,
    #[serde(rename = "REV GUARANTEE FAC")]
    REVGUARANTEEFAC,
    #[serde(rename = "REV VAT-TRNCH")]
    REVVATTRNCH,
    #[serde(rename = "Revolver")]
    Revolver,
    #[serde(rename = "Right")]
    Right,
    #[serde(rename = "Royalty Trst")]
    RoyaltyTrst,
    #[serde(rename = "S.TERM LOAN NOTE")]
    STERMLOANNOTE,
    #[serde(rename = "SAMURAI")]
    SAMURAI,
    #[serde(rename = "Savings Plan")]
    SavingsPlan,
    #[serde(rename = "Savings Share")]
    SavingsShare,
    #[serde(rename = "SBA Pool")]
    SBAPool,
    #[serde(rename = "SDR")]
    SDR,
    #[serde(rename = "Sec Lending")]
    SecLending,
    #[serde(rename = "SHOGUN")]
    SHOGUN,
    #[serde(rename = "SHORT TERM BN")]
    SHORTTERMBN,
    #[serde(rename = "SHORT TERM DN")]
    SHORTTERMDN,
    #[serde(rename = "SINGAPORE CP")]
    SINGAPORECP,
    #[serde(rename = "Singapore DR")]
    SingaporeDR,
    #[serde(rename = "SINGLE STOCK DIVIDEND FUTURE")]
    SINGLESTOCKDIVIDENDFUTURE,
    #[serde(rename = "SINGLE STOCK FORWARD")]
    SINGLESTOCKFORWARD,
    #[serde(rename = "SINGLE STOCK FUTURE")]
    SINGLESTOCKFUTURE,
    #[serde(rename = "SINGLE STOCK FUTURE SPREAD")]
    SINGLESTOCKFUTURESPREAD,
    #[serde(rename = "SN")]
    SN,
    #[serde(rename = "SPANISH CP")]
    SPANISHCP,
    #[serde(rename = "SPECIAL LMMK PGM")]
    SPECIALLMMKPGM,
    #[serde(rename = "SPOT")]
    SPOT,
    #[serde(rename = "Spot index.")]
    Spotindex,
    #[serde(rename = "STANDBY")]
    STANDBY,
    #[serde(rename = "STANDBY LOC")]
    STANDBYLOC,
    #[serde(rename = "STANDBY LOC GUARANTEE FAC")]
    STANDBYLOCGUARANTEEFAC,
    #[serde(rename = "STANDBY REV")]
    STANDBYREV,
    #[serde(rename = "STANDBY TERM")]
    STANDBYTERM,
    #[serde(rename = "Stapled Security")]
    StapledSecurity,
    #[serde(rename = "STERLING CD")]
    STERLINGCD,
    #[serde(rename = "STERLING CP")]
    STERLINGCP,
    #[serde(rename = "Strategy Trade.")]
    StrategyTrade,
    #[serde(rename = "SWAP")]
    SWAP,
    #[serde(rename = "SWAP SPREAD")]
    SWAPSPREAD,
    #[serde(rename = "SWAPTION VOLATILITY")]
    SWAPTIONVOLATILITY,
    #[serde(rename = "SWEDISH CP")]
    SWEDISHCP,
    #[serde(rename = "SWINGLINE")]
    SWINGLINE,
    #[serde(rename = "Swiss Cert")]
    SwissCert,
    #[serde(rename = "SYNTH LOC")]
    SYNTHLOC,
    #[serde(rename = "SYNTH REV")]
    SYNTHREV,
    #[serde(rename = "SYNTH TERM")]
    SYNTHTERM,
    #[serde(rename = "Synthetic Term")]
    SyntheticTerm,
    #[serde(rename = "TAIWAN CP")]
    TAIWANCP,
    #[serde(rename = "TAIWAN CP GUAR")]
    TAIWANCPGUAR,
    #[serde(rename = "TAIWAN NEGO CD")]
    TAIWANNEGOCD,
    #[serde(rename = "TAIWAN TIME DEPO")]
    TAIWANTIMEDEPO,
    #[serde(rename = "TAX CREDIT")]
    TAXCREDIT,
    #[serde(rename = "TAX CREDIT, OID")]
    TAXCREDITOID,
    #[serde(rename = "TDR")]
    TDR,
    #[serde(rename = "TERM")]
    TERM,
    #[serde(rename = "Term")]
    Term,
    #[serde(rename = "TERM DEPOSITS")]
    TERMDEPOSITS,
    #[serde(rename = "TERM GUARANTEE FAC")]
    TERMGUARANTEEFAC,
    #[serde(rename = "TERM REV")]
    TERMREV,
    #[serde(rename = "TERM VAT-TRNCH")]
    TERMVATTRNCH,
    #[serde(rename = "THAILAND CP")]
    THAILANDCP,
    #[serde(rename = "TLTRO TERM")]
    TLTROTERM,
    #[serde(rename = "Tracking Stk")]
    TrackingStk,
    #[serde(rename = "TREASURY BILL")]
    TREASURYBILL,
    #[serde(rename = "U.S. CD")]
    USCD,
    #[serde(rename = "U.S. CP")]
    USCP,
    #[serde(rename = "U.S. INT BEAR CP")]
    USINTBEARCP,
    #[serde(rename = "UIT")]
    UIT,
    #[serde(rename = "UK GILT STOCK")]
    UKGILTSTOCK,
    #[serde(rename = "UMBS MBS Other")]
    UMBSMBSOther,
    #[serde(rename = "Unit")]
    Unit,
    #[serde(rename = "Unit Inv Tst")]
    UnitInvTst,
    #[serde(rename = "UNITRANCHE")]
    UNITRANCHE,
    #[serde(rename = "UNITRANCHE ASSET-BASED REV")]
    UNITRANCHEASSETBASEDREV,
    #[serde(rename = "UNITRANCHE DELAY-DRAW PIK T")]
    UNITRANCHEDELAYDRAWPIKT,
    #[serde(rename = "UNITRANCHE DELAY-DRAW TERM")]
    UNITRANCHEDELAYDRAWTERM,
    #[serde(rename = "UNITRANCHE PIK TERM")]
    UNITRANCHEPIKTERM,
    #[serde(rename = "UNITRANCHE REV")]
    UNITRANCHEREV,
    #[serde(rename = "UNITRANCHE TERM")]
    UNITRANCHETERM,
    #[serde(rename = "US DOMESTIC")]
    USDOMESTIC,
    #[serde(rename = "US GOVERNMENT")]
    USGOVERNMENT,
    #[serde(rename = "US NON-DOLLAR")]
    USNONDOLLAR,
    #[serde(rename = "VAR RATE DEM OBL")]
    VARRATEDEMOBL,
    #[serde(rename = "VAT-TRNCH")]
    VATTRNCH,
    #[serde(rename = "VENEZUELAN CP")]
    VENEZUELANCP,
    #[serde(rename = "VIETNAMESE CD")]
    VIETNAMESECD,
    #[serde(rename = "VOLATILITY DERIVATIVE")]
    VOLATILITYDERIVATIVE,
    #[serde(rename = "Warrant")]
    Warrant,
    #[serde(rename = "YANKEE")]
    YANKEE,
    #[serde(rename = "YANKEE CD")]
    YANKEECD,
    #[serde(rename = "YEN CD")]
    YENCD,
    #[serde(rename = "YEN CP")]
    YENCP,
    #[serde(rename = "Yield Curve")]
    YieldCurve,
    #[serde(rename = "ZERO COUPON")]
    ZEROCOUPON,
    #[serde(rename = "ZERO COUPON, OID")]
    ZEROCOUPONOID,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_abs_card() {
        let val = SecurityType::ABSCard;
        let serialized = serde_json::to_string(&val).unwrap();
        assert_eq!(serialized, "\"ABS Card\"");
    }

    #[test]
    fn test_deserialize_abs_card() {
        let json = "\"ABS Card\"";
        let deserialized: SecurityType = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, SecurityType::ABSCard);
    }

    #[test]
    fn test_serialize_zero_coupon_oid() {
        let val = SecurityType::ZEROCOUPONOID;
        let serialized = serde_json::to_string(&val).unwrap();
        assert_eq!(serialized, "\"ZERO COUPON, OID\"");
    }

    #[test]
    fn test_deserialize_zero_coupon_oid() {
        let json = "\"ZERO COUPON, OID\"";
        let deserialized: SecurityType = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, SecurityType::ZEROCOUPONOID);
    }
}
