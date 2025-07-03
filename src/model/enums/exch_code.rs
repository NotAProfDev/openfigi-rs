//! Enum for all supported exchange codes in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::ExchCode;
//! let exch_code = ExchCode::FRANKFURT; // Frankfurt Stock Exchange
//! ```
//!
//! For the full list of values, see: <https://api.openfigi.com/v3/mapping/values/exchCode>

use serde::{Deserialize, Serialize};

/// Enum for all supported exchange codes.
#[expect(missing_docs)]
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ExchCode {
    #[serde(rename = "A0")]
    A0,
    #[serde(rename = "AA")]
    AA,
    #[serde(rename = "AB")]
    AB,
    #[serde(rename = "ABIDJAN")]
    ABIDJAN,
    #[serde(rename = "ABU DHABI")]
    ABUDHABI,
    #[serde(rename = "AC")]
    AC,
    #[serde(rename = "ACE")]
    ACE,
    #[serde(rename = "AD")]
    AD,
    #[serde(rename = "ADE")]
    ADE,
    #[serde(rename = "ADX")]
    ADX,
    #[serde(rename = "AEQUITAS NEO LIT")]
    AEQUITASNEOLIT,
    #[serde(rename = "AF")]
    AF,
    #[serde(rename = "AFE")]
    AFE,
    #[serde(rename = "AG")]
    AG,
    #[serde(rename = "AH")]
    AH,
    #[serde(rename = "AI")]
    AI,
    #[serde(rename = "AIAF")]
    AIAF,
    #[serde(rename = "AJ")]
    AJ,
    #[serde(rename = "AL")]
    AL,
    #[serde(rename = "ALCN")]
    ALCN,
    #[serde(rename = "ALGIERS")]
    ALGIERS,
    #[serde(rename = "ALL GERMAN SE")]
    ALLGERMANSE,
    #[serde(rename = "AM")]
    AM,
    #[serde(rename = "AME")]
    AME,
    #[serde(rename = "AMMAN FIN MKT")]
    AMMANFINMKT,
    #[serde(rename = "ANTWERP")]
    ANTWERP,
    #[serde(rename = "AO")]
    AO,
    #[serde(rename = "AP")]
    AP,
    #[serde(rename = "APX")]
    APX,
    #[serde(rename = "AQ")]
    AQ,
    #[serde(rename = "Aquis")]
    Aquis,
    #[serde(rename = "AR")]
    AR,
    #[serde(rename = "ARMENIA")]
    ARMENIA,
    #[serde(rename = "AS")]
    AS,
    #[serde(rename = "ASP")]
    ASP,
    #[serde(rename = "ASUNCION")]
    ASUNCION,
    #[serde(rename = "ASX")]
    ASX,
    #[serde(rename = "AT")]
    AT,
    #[serde(rename = "ATA")]
    ATA,
    #[serde(rename = "ATHENS")]
    ATHENS,
    #[serde(rename = "AU")]
    AU,
    #[serde(rename = "AUSTRALIA")]
    AUSTRALIA,
    #[serde(rename = "AV")]
    AV,
    #[serde(rename = "AW")]
    AW,
    #[serde(rename = "AX")]
    AX,
    #[serde(rename = "AY")]
    AY,
    #[serde(rename = "AZ")]
    AZ,
    #[serde(rename = "B1")]
    B1,
    #[serde(rename = "B2")]
    B2,
    #[serde(rename = "B3")]
    B3,
    #[serde(rename = "B4")]
    B4,
    #[serde(rename = "BA")]
    BA,
    #[serde(rename = "BAHAMAS")]
    BAHAMAS,
    #[serde(rename = "BAHRAIN")]
    BAHRAIN,
    #[serde(rename = "BAKU")]
    BAKU,
    #[serde(rename = "BANGALORE")]
    BANGALORE,
    #[serde(rename = "BANJA LUKA")]
    BANJALUKA,
    #[serde(rename = "BARBADOS")]
    BARBADOS,
    #[serde(rename = "BARCELONA")]
    BARCELONA,
    #[serde(rename = "BATS")]
    BATS,
    #[serde(rename = "BB")]
    BB,
    #[serde(rename = "BBOX")]
    BBOX,
    #[serde(rename = "bbox")]
    Bbox,
    #[serde(rename = "bbsp")]
    Bbsp,
    #[serde(rename = "BBX")]
    BBX,
    #[serde(rename = "BC")]
    BC,
    #[serde(rename = "BCEX")]
    BCEX,
    #[serde(rename = "BCF")]
    BCF,
    #[serde(rename = "BD")]
    BD,
    #[serde(rename = "BDP")]
    BDP,
    #[serde(rename = "BEIJING")]
    BEIJING,
    #[serde(rename = "BEIRUT")]
    BEIRUT,
    #[serde(rename = "BELARUS")]
    BELARUS,
    #[serde(rename = "BELGRADE")]
    BELGRADE,
    #[serde(rename = "BEQU")]
    BEQU,
    #[serde(rename = "bequ")]
    Bequ,
    #[serde(rename = "BERLIN")]
    BERLIN,
    #[serde(rename = "BERMUDA")]
    BERMUDA,
    #[serde(rename = "BERN")]
    BERN,
    #[serde(rename = "BEVSA")]
    BEVSA,
    #[serde(rename = "BF")]
    BF,
    #[serde(rename = "BFLY")]
    BFLY,
    #[serde(rename = "bfly")]
    Bfly,
    #[serde(rename = "BFNX")]
    BFNX,
    #[serde(rename = "bfnx")]
    Bfnx,
    #[serde(rename = "BFO")]
    BFO,
    #[serde(rename = "BFRX")]
    BFRX,
    #[serde(rename = "bfrx")]
    Bfrx,
    #[serde(rename = "BFX")]
    BFX,
    #[serde(rename = "BG")]
    BG,
    #[serde(rename = "BGC")]
    BGC,
    #[serde(rename = "BGON")]
    BGON,
    #[serde(rename = "bgon")]
    Bgon,
    #[serde(rename = "BH")]
    BH,
    #[serde(rename = "BI")]
    BI,
    #[serde(rename = "BIDS")]
    BIDS,
    #[serde(rename = "BILBAO")]
    BILBAO,
    #[serde(rename = "BINC")]
    BINC,
    #[serde(rename = "binc")]
    Binc,
    #[serde(rename = "BITZ")]
    BITZ,
    #[serde(rename = "BIVA")]
    BIVA,
    #[serde(rename = "BJEX")]
    BJEX,
    #[serde(rename = "BK")]
    BK,
    #[serde(rename = "BL3P")]
    BL3P,
    #[serde(rename = "blc2")]
    Blc2,
    #[serde(rename = "BLCR")]
    BLCR,
    #[serde(rename = "blcr")]
    Blcr,
    #[serde(rename = "BM")]
    BM,
    #[serde(rename = "BMF")]
    BMF,
    #[serde(rename = "BN")]
    BN,
    #[serde(rename = "BNCE")]
    BNCE,
    #[serde(rename = "bnce")]
    Bnce,
    #[serde(rename = "BNDX")]
    BNDX,
    #[serde(rename = "BNF")]
    BNF,
    #[serde(rename = "BNUS")]
    BNUS,
    #[serde(rename = "bnus")]
    Bnus,
    #[serde(rename = "BO")]
    BO,
    #[serde(rename = "Bodiva")]
    Bodiva,
    #[serde(rename = "BOLSA CENTROAMER")]
    BOLSACENTROAMER,
    #[serde(rename = "BOLSA NACL VALOR")]
    BOLSANACLVALOR,
    #[serde(rename = "Bondvision")]
    Bondvision,
    #[serde(rename = "BORSA ISTANBUL")]
    BORSAISTANBUL,
    #[serde(rename = "BOTSWANA")]
    BOTSWANA,
    #[serde(rename = "BOV")]
    BOV,
    #[serde(rename = "BP")]
    BP,
    #[serde(rename = "Bpm")]
    Bpm,
    #[serde(rename = "bpnd")]
    Bpnd,
    #[serde(rename = "BPVB")]
    BPVB,
    #[serde(rename = "BQ")]
    BQ,
    #[serde(rename = "BR")]
    BR,
    #[serde(rename = "BRATISLAVA")]
    BRATISLAVA,
    #[serde(rename = "BRJ")]
    BRJ,
    #[serde(rename = "BS")]
    BS,
    #[serde(rename = "BSE")]
    BSE,
    #[serde(rename = "BT")]
    BT,
    #[serde(rename = "BTBA")]
    BTBA,
    #[serde(rename = "btba")]
    Btba,
    #[serde(rename = "BTBY")]
    BTBY,
    #[serde(rename = "BTCA")]
    BTCA,
    #[serde(rename = "btcb")]
    Btcb,
    #[serde(rename = "bthb")]
    Bthb,
    #[serde(rename = "btmx")]
    Btmx,
    #[serde(rename = "BTRK")]
    BTRK,
    #[serde(rename = "btrk")]
    Btrk,
    #[serde(rename = "BTRX")]
    BTRX,
    #[serde(rename = "btrx")]
    Btrx,
    #[serde(rename = "BTS")]
    BTS,
    #[serde(rename = "BTSO")]
    BTSO,
    #[serde(rename = "btso")]
    Btso,
    #[serde(rename = "BU")]
    BU,
    #[serde(rename = "BUCHAREST")]
    BUCHAREST,
    #[serde(rename = "BUDAPEST")]
    BUDAPEST,
    #[serde(rename = "BUENOS AIRES")]
    BUENOSAIRES,
    #[serde(rename = "BULGARIA")]
    BULGARIA,
    #[serde(rename = "BURGUNDY")]
    BURGUNDY,
    #[serde(rename = "BURSA MALAYSIA")]
    BURSAMALAYSIA,
    #[serde(rename = "BV")]
    BV,
    #[serde(rename = "BVL")]
    BVL,
    #[serde(rename = "BW")]
    BW,
    #[serde(rename = "BX")]
    BX,
    #[serde(rename = "BX - SWISS")]
    BXSWISS,
    #[serde(rename = "BY")]
    BY,
    #[serde(rename = "BZ")]
    BZ,
    #[serde(rename = "C1")]
    C1,
    #[serde(rename = "C2")]
    C2,
    #[serde(rename = "C3")]
    C3,
    #[serde(rename = "CA")]
    CA,
    #[serde(rename = "CARACAS")]
    CARACAS,
    #[serde(rename = "CASABLANCA")]
    CASABLANCA,
    #[serde(rename = "CAYMAN ISLANDS")]
    CAYMANISLANDS,
    #[serde(rename = "CB")]
    CB,
    #[serde(rename = "CBD")]
    CBD,
    #[serde(rename = "CBF")]
    CBF,
    #[serde(rename = "CBO")]
    CBO,
    #[serde(rename = "CBOE")]
    CBOE,
    #[serde(rename = "CBSE")]
    CBSE,
    #[serde(rename = "cbse")]
    Cbse,
    #[serde(rename = "CBT")]
    CBT,
    #[serde(rename = "CC")]
    CC,
    #[serde(rename = "ccck")]
    Ccck,
    #[serde(rename = "CCO")]
    CCO,
    #[serde(rename = "CCT")]
    CCT,
    #[serde(rename = "CCX")]
    CCX,
    #[serde(rename = "CD")]
    CD,
    #[serde(rename = "CDE")]
    CDE,
    #[serde(rename = "CE")]
    CE,
    #[serde(rename = "CEG")]
    CEG,
    #[serde(rename = "CENT ANOTACIONE")]
    CENTANOTACIONE,
    #[serde(rename = "CEXI")]
    CEXI,
    #[serde(rename = "cexi")]
    Cexi,
    #[serde(rename = "CF")]
    CF,
    #[serde(rename = "CFF")]
    CFF,
    #[serde(rename = "CFLR")]
    CFLR,
    #[serde(rename = "CG")]
    CG,
    #[serde(rename = "CH")]
    CH,
    #[serde(rename = "CHANNEL ISLANDS")]
    CHANNELISLANDS,
    #[serde(rename = "CHI-X")]
    CHIX,
    #[serde(rename = "Chi-X Australia")]
    ChiXAustralia,
    #[serde(rename = "CHICAGO")]
    CHICAGO,
    #[serde(rename = "CHINA INTERBANK")]
    CHINAINTERBANK,
    #[serde(rename = "CHONGWA ASSET EX")]
    CHONGWAASSETEX,
    #[serde(rename = "CI")]
    CI,
    #[serde(rename = "CJ")]
    CJ,
    #[serde(rename = "CK")]
    CK,
    #[serde(rename = "CL")]
    CL,
    #[serde(rename = "CM")]
    CM,
    #[serde(rename = "CME")]
    CME,
    #[serde(rename = "CMF")]
    CMF,
    #[serde(rename = "CMX")]
    CMX,
    #[serde(rename = "CN")]
    CN,
    #[serde(rename = "CNEX")]
    CNEX,
    #[serde(rename = "cnex")]
    Cnex,
    #[serde(rename = "CNGG")]
    CNGG,
    #[serde(rename = "CNMT")]
    CNMT,
    #[serde(rename = "CNSX")]
    CNSX,
    #[serde(rename = "CO")]
    CO,
    #[serde(rename = "COLOMBIA")]
    COLOMBIA,
    #[serde(rename = "COLOMBO")]
    COLOMBO,
    #[serde(rename = "cone")]
    Cone,
    #[serde(rename = "COP")]
    COP,
    #[serde(rename = "CP")]
    CP,
    #[serde(rename = "CQ")]
    CQ,
    #[serde(rename = "CR")]
    CR,
    #[serde(rename = "CRCO")]
    CRCO,
    #[serde(rename = "crco")]
    Crco,
    #[serde(rename = "crv2")]
    Crv2,
    #[serde(rename = "CS")]
    CS,
    #[serde(rename = "CSE")]
    CSE,
    #[serde(rename = "CT")]
    CT,
    #[serde(rename = "CU")]
    CU,
    #[serde(rename = "CUCY")]
    CUCY,
    #[serde(rename = "cucy")]
    Cucy,
    #[serde(rename = "CURV")]
    CURV,
    #[serde(rename = "curv")]
    Curv,
    #[serde(rename = "CV")]
    CV,
    #[serde(rename = "CW")]
    CW,
    #[serde(rename = "CX")]
    CX,
    #[serde(rename = "CY")]
    CY,
    #[serde(rename = "CYPRUS")]
    CYPRUS,
    #[serde(rename = "CZ")]
    CZ,
    #[serde(rename = "DAR-ES-SALAAM")]
    DARESSALAAM,
    #[serde(rename = "DB")]
    DB,
    #[serde(rename = "DBS Digital")]
    DBSDigital,
    #[serde(rename = "DC")]
    DC,
    #[serde(rename = "DCE")]
    DCE,
    #[serde(rename = "DD")]
    DD,
    #[serde(rename = "DE")]
    DE,
    #[serde(rename = "DEB")]
    DEB,
    #[serde(rename = "delt")]
    Delt,
    #[serde(rename = "DF")]
    DF,
    #[serde(rename = "DFX")]
    DFX,
    #[serde(rename = "DG")]
    DG,
    #[serde(rename = "DGC")]
    DGC,
    #[serde(rename = "DH")]
    DH,
    #[serde(rename = "DHAKA")]
    DHAKA,
    #[serde(rename = "DJ")]
    DJ,
    #[serde(rename = "DK")]
    DK,
    #[serde(rename = "DL")]
    DL,
    #[serde(rename = "DM")]
    DM,
    #[serde(rename = "DME")]
    DME,
    #[serde(rename = "DN")]
    DN,
    #[serde(rename = "DOUALA")]
    DOUALA,
    #[serde(rename = "drbt")]
    Drbt,
    #[serde(rename = "DS")]
    DS,
    #[serde(rename = "DT")]
    DT,
    #[serde(rename = "DU")]
    DU,
    #[serde(rename = "DUBAI FINL MKT")]
    DUBAIFINLMKT,
    #[serde(rename = "DUBLIN")]
    DUBLIN,
    #[serde(rename = "DUSSELDORF")]
    DUSSELDORF,
    #[serde(rename = "DV")]
    DV,
    #[serde(rename = "DVX")]
    DVX,
    #[serde(rename = "DX")]
    DX,
    #[serde(rename = "E1")]
    E1,
    #[serde(rename = "E2")]
    E2,
    #[serde(rename = "EA")]
    EA,
    #[serde(rename = "EAST CARIBBEAN")]
    EASTCARIBBEAN,
    #[serde(rename = "EB")]
    EB,
    #[serde(rename = "EC")]
    EC,
    #[serde(rename = "ED")]
    ED,
    #[serde(rename = "EDX")]
    EDX,
    #[serde(rename = "EEE")]
    EEE,
    #[serde(rename = "EG")]
    EG,
    #[serde(rename = "EGX")]
    EGX,
    #[serde(rename = "EI")]
    EI,
    #[serde(rename = "EK")]
    EK,
    #[serde(rename = "EL")]
    EL,
    #[serde(rename = "EL SALVADOR")]
    ELSALVADOR,
    #[serde(rename = "ELECTRONIC CHILE")]
    ELECTRONICCHILE,
    #[serde(rename = "ELX")]
    ELX,
    #[serde(rename = "EM")]
    EM,
    #[serde(rename = "EN")]
    EN,
    #[serde(rename = "EO")]
    EO,
    #[serde(rename = "EOC")]
    EOC,
    #[serde(rename = "EOE")]
    EOE,
    #[serde(rename = "EOP")]
    EOP,
    #[serde(rename = "EP")]
    EP,
    #[serde(rename = "EQ")]
    EQ,
    #[serde(rename = "ERI")]
    ERI,
    #[serde(rename = "ERIS")]
    ERIS,
    #[serde(rename = "eris")]
    Eris,
    #[serde(rename = "ES")]
    ES,
    #[serde(rename = "ESWATINI")]
    ESWATINI,
    #[serde(rename = "ET")]
    ET,
    #[serde(rename = "EU")]
    EU,
    #[serde(rename = "EUROMTF")]
    EUROMTF,
    #[serde(rename = "EUROMTS")]
    EUROMTS,
    #[serde(rename = "EURONEXT-AMSTER")]
    EURONEXTAMSTER,
    #[serde(rename = "EURONEXT-BRUSS")]
    EURONEXTBRUSS,
    #[serde(rename = "EURONEXT-DUBLIN")]
    EURONEXTDUBLIN,
    #[serde(rename = "EURONEXT-GRW-MIL")]
    EURONEXTGRWMIL,
    #[serde(rename = "EURONEXT-LISBON")]
    EURONEXTLISBON,
    #[serde(rename = "EURONEXT-MILAN")]
    EURONEXTMILAN,
    #[serde(rename = "EURONEXT-PARIS")]
    EURONEXTPARIS,
    #[serde(rename = "EUROTLX")]
    EUROTLX,
    #[serde(rename = "EUS")]
    EUS,
    #[serde(rename = "EUWAX STUTTGART")]
    EUWAXSTUTTGART,
    #[serde(rename = "EUX")]
    EUX,
    #[serde(rename = "EX")]
    EX,
    #[serde(rename = "Extra MOT")]
    ExtraMOT,
    #[serde(rename = "Extra MOT Pro")]
    ExtraMOTPro,
    #[serde(rename = "EXXA")]
    EXXA,
    #[serde(rename = "EY")]
    EY,
    #[serde(rename = "EZ")]
    EZ,
    #[serde(rename = "FA")]
    FA,
    #[serde(rename = "FEX")]
    FEX,
    #[serde(rename = "FF")]
    FF,
    #[serde(rename = "FF ZERTIFIKATE")]
    FFZERTIFIKATE,
    #[serde(rename = "FFE")]
    FFE,
    #[serde(rename = "FH")]
    FH,
    #[serde(rename = "FMX")]
    FMX,
    #[serde(rename = "FNX")]
    FNX,
    #[serde(rename = "FP")]
    FP,
    #[serde(rename = "FPL")]
    FPL,
    #[serde(rename = "FRANKFURT")]
    FRANKFURT,
    #[serde(rename = "FRX")]
    FRX,
    #[serde(rename = "FS")]
    FS,
    #[serde(rename = "FTX")]
    FTX,
    #[serde(rename = "FTXX")]
    FTXX,
    #[serde(rename = "FUKUOKA")]
    FUKUOKA,
    #[serde(rename = "G1")]
    G1,
    #[serde(rename = "G4")]
    G4,
    #[serde(rename = "GA")]
    GA,
    #[serde(rename = "GB")]
    GB,
    #[serde(rename = "GBT")]
    GBT,
    #[serde(rename = "GC")]
    GC,
    #[serde(rename = "GD")]
    GD,
    #[serde(rename = "GE")]
    GE,
    #[serde(rename = "GEMMA")]
    GEMMA,
    #[serde(rename = "GEORGIA")]
    GEORGIA,
    #[serde(rename = "Gettex")]
    Gettex,
    #[serde(rename = "GF")]
    GF,
    #[serde(rename = "GG")]
    GG,
    #[serde(rename = "GH")]
    GH,
    #[serde(rename = "GHANA")]
    GHANA,
    #[serde(rename = "GI")]
    GI,
    #[serde(rename = "Gibraltar")]
    Gibraltar,
    #[serde(rename = "GK")]
    GK,
    #[serde(rename = "GL")]
    GL,
    #[serde(rename = "GM")]
    GM,
    #[serde(rename = "GME")]
    GME,
    #[serde(rename = "GMNI")]
    GMNI,
    #[serde(rename = "gmni")]
    Gmni,
    #[serde(rename = "GN")]
    GN,
    #[serde(rename = "GQ")]
    GQ,
    #[serde(rename = "GR")]
    GR,
    #[serde(rename = "GS")]
    GS,
    #[serde(rename = "GT")]
    GT,
    #[serde(rename = "GU")]
    GU,
    #[serde(rename = "GUATEMALA")]
    GUATEMALA,
    #[serde(rename = "GUAYAQUIL")]
    GUAYAQUIL,
    #[serde(rename = "GW")]
    GW,
    #[serde(rename = "GY")]
    GY,
    #[serde(rename = "GZ")]
    GZ,
    #[serde(rename = "H1")]
    H1,
    #[serde(rename = "H2")]
    H2,
    #[serde(rename = "HAMBURG")]
    HAMBURG,
    #[serde(rename = "HANNOVER")]
    HANNOVER,
    #[serde(rename = "HANOI")]
    HANOI,
    #[serde(rename = "HB")]
    HB,
    #[serde(rename = "HCM CITY EXCH")]
    HCMCITYEXCH,
    #[serde(rename = "HD")]
    HD,
    #[serde(rename = "HE")]
    HE,
    #[serde(rename = "HEX")]
    HEX,
    #[serde(rename = "HI-MTF")]
    HIMTF,
    #[serde(rename = "HITB")]
    HITB,
    #[serde(rename = "hitb")]
    Hitb,
    #[serde(rename = "HK")]
    HK,
    #[serde(rename = "HKG")]
    HKG,
    #[serde(rename = "HKM")]
    HKM,
    #[serde(rename = "HM")]
    HM,
    #[serde(rename = "HNX")]
    HNX,
    #[serde(rename = "HO")]
    HO,
    #[serde(rename = "HONG KONG")]
    HONGKONG,
    #[serde(rename = "HUOB")]
    HUOB,
    #[serde(rename = "huob")]
    Huob,
    #[serde(rename = "HX")]
    HX,
    #[serde(rename = "I2")]
    I2,
    #[serde(rename = "IA")]
    IA,
    #[serde(rename = "IAD")]
    IAD,
    #[serde(rename = "IB")]
    IB,
    #[serde(rename = "IC")]
    IC,
    #[serde(rename = "ICD")]
    ICD,
    #[serde(rename = "ICE")]
    ICE,
    #[serde(rename = "ICE ECX")]
    ICEECX,
    #[serde(rename = "ICF")]
    ICF,
    #[serde(rename = "ID")]
    ID,
    #[serde(rename = "IDEM")]
    IDEM,
    #[serde(rename = "IDR")]
    IDR,
    #[serde(rename = "IDX")]
    IDX,
    #[serde(rename = "IE")]
    IE,
    #[serde(rename = "IEA")]
    IEA,
    #[serde(rename = "IF")]
    IF,
    #[serde(rename = "IFE")]
    IFE,
    #[serde(rename = "IG")]
    IG,
    #[serde(rename = "IH")]
    IH,
    #[serde(rename = "IJ")]
    IJ,
    #[serde(rename = "IM")]
    IM,
    #[serde(rename = "IN")]
    IN,
    #[serde(rename = "INCH")]
    INCH,
    #[serde(rename = "INDIA INX")]
    INDIAINX,
    #[serde(rename = "INDONESIA EXCH")]
    INDONESIAEXCH,
    #[serde(rename = "indr")]
    Indr,
    #[serde(rename = "INE")]
    INE,
    #[serde(rename = "INTERCONTINENTAL")]
    INTERCONTINENTAL,
    #[serde(rename = "INX")]
    INX,
    #[serde(rename = "IO")]
    IO,
    #[serde(rename = "IQ")]
    IQ,
    #[serde(rename = "IR")]
    IR,
    #[serde(rename = "IS")]
    IS,
    #[serde(rename = "ISE")]
    ISE,
    #[serde(rename = "ISF")]
    ISF,
    #[serde(rename = "ISG")]
    ISG,
    #[serde(rename = "ISLAND ECN LTD")]
    ISLANDECNLTD,
    #[serde(rename = "IST")]
    IST,
    #[serde(rename = "IT")]
    IT,
    #[serde(rename = "ITBI")]
    ITBI,
    #[serde(rename = "itbi")]
    Itbi,
    #[serde(rename = "IX")]
    IX,
    #[serde(rename = "IY")]
    IY,
    #[serde(rename = "JA")]
    JA,
    #[serde(rename = "JAMAICA")]
    JAMAICA,
    #[serde(rename = "JASDAQ")]
    JASDAQ,
    #[serde(rename = "JB")]
    JB,
    #[serde(rename = "JC")]
    JC,
    #[serde(rename = "JD")]
    JD,
    #[serde(rename = "JE")]
    JE,
    #[serde(rename = "JF")]
    JF,
    #[serde(rename = "JFX")]
    JFX,
    #[serde(rename = "JG")]
    JG,
    #[serde(rename = "JI")]
    JI,
    #[serde(rename = "JJ")]
    JJ,
    #[serde(rename = "JM")]
    JM,
    #[serde(rename = "JN")]
    JN,
    #[serde(rename = "JO")]
    JO,
    #[serde(rename = "JOHANNESBURG")]
    JOHANNESBURG,
    #[serde(rename = "JP")]
    JP,
    #[serde(rename = "JQ")]
    JQ,
    #[serde(rename = "JR")]
    JR,
    #[serde(rename = "JS")]
    JS,
    #[serde(rename = "JSE Cent Ord Bk")]
    JSECentOrdBk,
    #[serde(rename = "JSE Contrib Prx")]
    JSEContribPrx,
    #[serde(rename = "JT")]
    JT,
    #[serde(rename = "JU")]
    JU,
    #[serde(rename = "JV")]
    JV,
    #[serde(rename = "JW")]
    JW,
    #[serde(rename = "JX")]
    JX,
    #[serde(rename = "JY")]
    JY,
    #[serde(rename = "KA")]
    KA,
    #[serde(rename = "KAS")]
    KAS,
    #[serde(rename = "KAZAKHSTAN")]
    KAZAKHSTAN,
    #[serde(rename = "KB")]
    KB,
    #[serde(rename = "KCB")]
    KCB,
    #[serde(rename = "KCON")]
    KCON,
    #[serde(rename = "kcon")]
    Kcon,
    #[serde(rename = "KE")]
    KE,
    #[serde(rename = "KF")]
    KF,
    #[serde(rename = "KFE")]
    KFE,
    #[serde(rename = "KH")]
    KH,
    #[serde(rename = "KIEV")]
    KIEV,
    #[serde(rename = "KK")]
    KK,
    #[serde(rename = "KL")]
    KL,
    #[serde(rename = "KN")]
    KN,
    #[serde(rename = "korb")]
    Korb,
    #[serde(rename = "KOREA")]
    KOREA,
    #[serde(rename = "KOSDAQ")]
    KOSDAQ,
    #[serde(rename = "KP")]
    KP,
    #[serde(rename = "KQ")]
    KQ,
    #[serde(rename = "KRKN")]
    KRKN,
    #[serde(rename = "krkn")]
    Krkn,
    #[serde(rename = "KS")]
    KS,
    #[serde(rename = "KUWAIT")]
    KUWAIT,
    #[serde(rename = "KX")]
    KX,
    #[serde(rename = "KY")]
    KY,
    #[serde(rename = "KYRGZSTAN")]
    KYRGZSTAN,
    #[serde(rename = "KZ")]
    KZ,
    #[serde(rename = "L1")]
    L1,
    #[serde(rename = "L3")]
    L3,
    #[serde(rename = "LA")]
    LA,
    #[serde(rename = "LA PAZ")]
    LAPAZ,
    #[serde(rename = "LABUAN INTL FIN")]
    LABUANINTLFIN,
    #[serde(rename = "LB")]
    LB,
    #[serde(rename = "LC")]
    LC,
    #[serde(rename = "LCLB")]
    LCLB,
    #[serde(rename = "LD")]
    LD,
    #[serde(rename = "LDX")]
    LDX,
    #[serde(rename = "LE")]
    LE,
    #[serde(rename = "LF")]
    LF,
    #[serde(rename = "LG")]
    LG,
    #[serde(rename = "LH")]
    LH,
    #[serde(rename = "LI")]
    LI,
    #[serde(rename = "LISBON")]
    LISBON,
    #[serde(rename = "LJUBLJANA")]
    LJUBLJANA,
    #[serde(rename = "LMAX")]
    LMAX,
    #[serde(rename = "lmax")]
    Lmax,
    #[serde(rename = "LME")]
    LME,
    #[serde(rename = "LMP")]
    LMP,
    #[serde(rename = "LN")]
    LN,
    #[serde(rename = "LO")]
    LO,
    #[serde(rename = "LONDON")]
    LONDON,
    #[serde(rename = "LONDON INTL")]
    LONDONINTL,
    #[serde(rename = "LR")]
    LR,
    #[serde(rename = "LS")]
    LS,
    #[serde(rename = "LSE")]
    LSE,
    #[serde(rename = "LSE-RETAIL")]
    LSERETAIL,
    #[serde(rename = "LT")]
    LT,
    #[serde(rename = "LU")]
    LU,
    #[serde(rename = "LUSAKA")]
    LUSAKA,
    #[serde(rename = "LUXEMBOURG")]
    LUXEMBOURG,
    #[serde(rename = "LV")]
    LV,
    #[serde(rename = "LX")]
    LX,
    #[serde(rename = "LY")]
    LY,
    #[serde(rename = "LYON")]
    LYON,
    #[serde(rename = "M0")]
    M0,
    #[serde(rename = "MA")]
    MA,
    #[serde(rename = "MACEDONIA")]
    MACEDONIA,
    #[serde(rename = "MADRAS")]
    MADRAS,
    #[serde(rename = "MADRID")]
    MADRID,
    #[serde(rename = "MAE")]
    MAE,
    #[serde(rename = "MALAWI")]
    MALAWI,
    #[serde(rename = "MALTA")]
    MALTA,
    #[serde(rename = "MANAGUA")]
    MANAGUA,
    #[serde(rename = "MARF")]
    MARF,
    #[serde(rename = "MARSEILLE")]
    MARSEILLE,
    #[serde(rename = "MAURITIUS")]
    MAURITIUS,
    #[serde(rename = "MB")]
    MB,
    #[serde(rename = "MBA")]
    MBA,
    #[serde(rename = "MC")]
    MC,
    #[serde(rename = "MCE")]
    MCE,
    #[serde(rename = "MCI")]
    MCI,
    #[serde(rename = "MCT")]
    MCT,
    #[serde(rename = "MCX")]
    MCX,
    #[serde(rename = "MD")]
    MD,
    #[serde(rename = "MDE")]
    MDE,
    #[serde(rename = "MDX")]
    MDX,
    #[serde(rename = "ME")]
    ME,
    #[serde(rename = "MELBOURNE")]
    MELBOURNE,
    #[serde(rename = "MENDOZA")]
    MENDOZA,
    #[serde(rename = "MERJ")]
    MERJ,
    #[serde(rename = "MERVAL")]
    MERVAL,
    #[serde(rename = "MET")]
    MET,
    #[serde(rename = "mexc")]
    Mexc,
    #[serde(rename = "MEXICO")]
    MEXICO,
    #[serde(rename = "MF")]
    MF,
    #[serde(rename = "MFA")]
    MFA,
    #[serde(rename = "MFM")]
    MFM,
    #[serde(rename = "MFP")]
    MFP,
    #[serde(rename = "MGE")]
    MGE,
    #[serde(rename = "MI")]
    MI,
    #[serde(rename = "MICEX")]
    MICEX,
    #[serde(rename = "MICEX A1")]
    MICEXA1,
    #[serde(rename = "MICEX A2")]
    MICEXA2,
    #[serde(rename = "MICEX B")]
    MICEXB,
    #[serde(rename = "MICEX D")]
    MICEXD,
    #[serde(rename = "MICEX Unlisted")]
    MICEXUnlisted,
    #[serde(rename = "MICEX V")]
    MICEXV,
    #[serde(rename = "MIF")]
    MIF,
    #[serde(rename = "MIL")]
    MIL,
    #[serde(rename = "MILAN")]
    MILAN,
    #[serde(rename = "MK")]
    MK,
    #[serde(rename = "MM")]
    MM,
    #[serde(rename = "MN")]
    MN,
    #[serde(rename = "MO")]
    MO,
    #[serde(rename = "MOEX Level 1")]
    MOEXLevel1,
    #[serde(rename = "MOEX Level 2")]
    MOEXLevel2,
    #[serde(rename = "MOEX Level 3")]
    MOEXLevel3,
    #[serde(rename = "MONGOLIA")]
    MONGOLIA,
    #[serde(rename = "MONTENEGRO")]
    MONTENEGRO,
    #[serde(rename = "MONTEVIDEO")]
    MONTEVIDEO,
    #[serde(rename = "MOSCOW")]
    MOSCOW,
    #[serde(rename = "MOT")]
    MOT,
    #[serde(rename = "MOZAMBIQUE")]
    MOZAMBIQUE,
    #[serde(rename = "MP")]
    MP,
    #[serde(rename = "MS")]
    MS,
    #[serde(rename = "MSE")]
    MSE,
    #[serde(rename = "MSX")]
    MSX,
    #[serde(rename = "MT")]
    MT,
    #[serde(rename = "MTS AMSTERDAM")]
    MTSAMSTERDAM,
    #[serde(rename = "MTS Austria")]
    MTSAustria,
    #[serde(rename = "MTS BELGIUM")]
    MTSBELGIUM,
    #[serde(rename = "MTS Finland")]
    MTSFinland,
    #[serde(rename = "MTS FRANCE")]
    MTSFRANCE,
    #[serde(rename = "MTS Germany")]
    MTSGermany,
    #[serde(rename = "MTS GREECE")]
    MTSGREECE,
    #[serde(rename = "MTS IRELAND")]
    MTSIRELAND,
    #[serde(rename = "MTS Israel")]
    MTSIsrael,
    #[serde(rename = "MTS PORTUGAL")]
    MTSPORTUGAL,
    #[serde(rename = "MTS S.p.A")]
    MTSSpA,
    #[serde(rename = "MTS Spain")]
    MTSSpain,
    #[serde(rename = "MU")]
    MU,
    #[serde(rename = "MUMBAI")]
    MUMBAI,
    #[serde(rename = "MUNICH")]
    MUNICH,
    #[serde(rename = "MUSCAT SECS MKT")]
    MUSCATSECSMKT,
    #[serde(rename = "MV")]
    MV,
    #[serde(rename = "MW")]
    MW,
    #[serde(rename = "MX")]
    MX,
    #[serde(rename = "MY")]
    MY,
    #[serde(rename = "MZ")]
    MZ,
    #[serde(rename = "N2X")]
    N2X,
    #[serde(rename = "NA")]
    NA,
    #[serde(rename = "NAGOYA")]
    NAGOYA,
    #[serde(rename = "NAIROBI")]
    NAIROBI,
    #[serde(rename = "NAMIBIA")]
    NAMIBIA,
    #[serde(rename = "NANTES")]
    NANTES,
    #[serde(rename = "NASDAQ")]
    NASDAQ,
    #[serde(rename = "NASDAQ DUBAI")]
    NASDAQDUBAI,
    #[serde(rename = "NASDAQ OMX PHLX")]
    NASDAQOMXPHLX,
    #[serde(rename = "NASDAQ/NCM")]
    NASDAQNCM,
    #[serde(rename = "NASDAQ/NGM")]
    NASDAQNGM,
    #[serde(rename = "NASDAQ/NGS")]
    NASDAQNGS,
    #[serde(rename = "NB")]
    NB,
    #[serde(rename = "NC")]
    NC,
    #[serde(rename = "ND")]
    ND,
    #[serde(rename = "NDM")]
    NDM,
    #[serde(rename = "NDX")]
    NDX,
    #[serde(rename = "NE")]
    NE,
    #[serde(rename = "NEW YORK")]
    NEWYORK,
    #[serde(rename = "NEW ZEALAND")]
    NEWZEALAND,
    #[serde(rename = "NF")]
    NF,
    #[serde(rename = "NFE")]
    NFE,
    #[serde(rename = "NFX")]
    NFX,
    #[serde(rename = "NG")]
    NG,
    #[serde(rename = "NGC")]
    NGC,
    #[serde(rename = "NGM")]
    NGM,
    #[serde(rename = "NI")]
    NI,
    #[serde(rename = "NIGERIA")]
    NIGERIA,
    #[serde(rename = "NJ")]
    NJ,
    #[serde(rename = "NK")]
    NK,
    #[serde(rename = "NL")]
    NL,
    #[serde(rename = "NLX")]
    NLX,
    #[serde(rename = "NM")]
    NM,
    #[serde(rename = "NN")]
    NN,
    #[serde(rename = "NO")]
    NO,
    #[serde(rename = "NOMX 1stNorth C")]
    NOMX1stNorthC,
    #[serde(rename = "NOMX 1stNorth F")]
    NOMX1stNorthF,
    #[serde(rename = "NOMX 1stNorth S")]
    NOMX1stNorthS,
    #[serde(rename = "NOMX COPENHAGEN")]
    NOMXCOPENHAGEN,
    #[serde(rename = "NOMX HELSINKI")]
    NOMXHELSINKI,
    #[serde(rename = "NOMX ICELAND")]
    NOMXICELAND,
    #[serde(rename = "NOMX RIGA")]
    NOMXRIGA,
    #[serde(rename = "NOMX STOCKHOLM")]
    NOMXSTOCKHOLM,
    #[serde(rename = "NOMX TALLINN")]
    NOMXTALLINN,
    #[serde(rename = "NOMX VILNIUS")]
    NOMXVILNIUS,
    #[serde(rename = "NORDIC ABM")]
    NORDICABM,
    #[serde(rename = "NOT LISTED")]
    NOTLISTED,
    #[serde(rename = "NOUVEAU MARCHE")]
    NOUVEAUMARCHE,
    #[serde(rename = "NP")]
    NP,
    #[serde(rename = "NPE")]
    NPE,
    #[serde(rename = "NQ")]
    NQ,
    #[serde(rename = "NQL")]
    NQL,
    #[serde(rename = "NR")]
    NR,
    #[serde(rename = "NS")]
    NS,
    #[serde(rename = "NSE")]
    NSE,
    #[serde(rename = "NSE Australia")]
    NSEAustralia,
    #[serde(rename = "NSE IFSC")]
    NSEIFSC,
    #[serde(rename = "NSE INDIA")]
    NSEINDIA,
    #[serde(rename = "NSEL")]
    NSEL,
    #[serde(rename = "NT")]
    NT,
    #[serde(rename = "NV")]
    NV,
    #[serde(rename = "nvdx")]
    Nvdx,
    #[serde(rename = "NW")]
    NW,
    #[serde(rename = "NX")]
    NX,
    #[serde(rename = "NY")]
    NY,
    #[serde(rename = "NYB")]
    NYB,
    #[serde(rename = "NYF")]
    NYF,
    #[serde(rename = "NYM")]
    NYM,
    #[serde(rename = "NYSE AMERICAN")]
    NYSEAMERICAN,
    #[serde(rename = "NYSE ARCA")]
    NYSEARCA,
    #[serde(rename = "NYSE BONDMATCH")]
    NYSEBONDMATCH,
    #[serde(rename = "NZ")]
    NZ,
    #[serde(rename = "NZX")]
    NZX,
    #[serde(rename = "OBX")]
    OBX,
    #[serde(rename = "OC")]
    OC,
    #[serde(rename = "OCG")]
    OCG,
    #[serde(rename = "ODE")]
    ODE,
    #[serde(rename = "OF")]
    OF,
    #[serde(rename = "OKCN")]
    OKCN,
    #[serde(rename = "okcn")]
    Okcn,
    #[serde(rename = "OKEX")]
    OKEX,
    #[serde(rename = "okex")]
    Okex,
    #[serde(rename = "OM")]
    OM,
    #[serde(rename = "OMEGA CANADA ATS")]
    OMEGACANADAATS,
    #[serde(rename = "OMP")]
    OMP,
    #[serde(rename = "OS")]
    OS,
    #[serde(rename = "OSAKA")]
    OSAKA,
    #[serde(rename = "OSAKA 2")]
    OSAKA2,
    #[serde(rename = "OSE")]
    OSE,
    #[serde(rename = "OSLO")]
    OSLO,
    #[serde(rename = "oslx")]
    Oslx,
    #[serde(rename = "OTC BB")]
    OTCBB,
    #[serde(rename = "OTC US")]
    OTCUS,
    #[serde(rename = "OU")]
    OU,
    #[serde(rename = "P2")]
    P2,
    #[serde(rename = "PA")]
    PA,
    #[serde(rename = "PAKISTAN")]
    PAKISTAN,
    #[serde(rename = "PALESTINE")]
    PALESTINE,
    #[serde(rename = "PANAMA")]
    PANAMA,
    #[serde(rename = "PB")]
    PB,
    #[serde(rename = "PBT")]
    PBT,
    #[serde(rename = "PC")]
    PC,
    #[serde(rename = "PD")]
    PD,
    #[serde(rename = "PDEx")]
    PDEx,
    #[serde(rename = "PE")]
    PE,
    #[serde(rename = "PEX")]
    PEX,
    #[serde(rename = "PF")]
    PF,
    #[serde(rename = "PFTS")]
    PFTS,
    #[serde(rename = "PG")]
    PG,
    #[serde(rename = "PHILIPPINES")]
    PHILIPPINES,
    #[serde(rename = "PHL")]
    PHL,
    #[serde(rename = "PINK SHEETS")]
    PINKSHEETS,
    #[serde(rename = "PK")]
    PK,
    #[serde(rename = "pksp")]
    Pksp,
    #[serde(rename = "PL")]
    PL,
    #[serde(rename = "PLX")]
    PLX,
    #[serde(rename = "PM")]
    PM,
    #[serde(rename = "PMI")]
    PMI,
    #[serde(rename = "PMX")]
    PMX,
    #[serde(rename = "PN")]
    PN,
    #[serde(rename = "PNX")]
    PNX,
    #[serde(rename = "PO")]
    PO,
    #[serde(rename = "POLO")]
    POLO,
    #[serde(rename = "polo")]
    Polo,
    #[serde(rename = "PORT MORESBY")]
    PORTMORESBY,
    #[serde(rename = "PORTAL")]
    PORTAL,
    #[serde(rename = "PP")]
    PP,
    #[serde(rename = "PQ")]
    PQ,
    #[serde(rename = "PRAGUE")]
    PRAGUE,
    #[serde(rename = "PRG")]
    PRG,
    #[serde(rename = "PRO SEC MKT(PSM)")]
    PROSECMKTPSM,
    #[serde(rename = "PS")]
    PS,
    #[serde(rename = "PURE TRADING")]
    PURETRADING,
    #[serde(rename = "PW")]
    PW,
    #[serde(rename = "PX")]
    PX,
    #[serde(rename = "PZ")]
    PZ,
    #[serde(rename = "QATAR")]
    QATAR,
    #[serde(rename = "QD")]
    QD,
    #[serde(rename = "QE")]
    QE,
    #[serde(rename = "QF")]
    QF,
    #[serde(rename = "QG")]
    QG,
    #[serde(rename = "QH")]
    QH,
    #[serde(rename = "QM")]
    QM,
    #[serde(rename = "QN")]
    QN,
    #[serde(rename = "qsp3")]
    Qsp3,
    #[serde(rename = "QT")]
    QT,
    #[serde(rename = "QU")]
    QU,
    #[serde(rename = "QUITO")]
    QUITO,
    #[serde(rename = "QUON")]
    QUON,
    #[serde(rename = "Quotrix")]
    Quotrix,
    #[serde(rename = "QX")]
    QX,
    #[serde(rename = "RASDAQ")]
    RASDAQ,
    #[serde(rename = "RB")]
    RB,
    #[serde(rename = "RC")]
    RC,
    #[serde(rename = "RE")]
    RE,
    #[serde(rename = "RF")]
    RF,
    #[serde(rename = "RFX")]
    RFX,
    #[serde(rename = "RG")]
    RG,
    #[serde(rename = "RIO DE JANEIRO")]
    RIODEJANEIRO,
    #[serde(rename = "RM")]
    RM,
    #[serde(rename = "RN")]
    RN,
    #[serde(rename = "RO")]
    RO,
    #[serde(rename = "ROFEX")]
    ROFEX,
    #[serde(rename = "RP")]
    RP,
    #[serde(rename = "RQ")]
    RQ,
    #[serde(rename = "RR")]
    RR,
    #[serde(rename = "RS")]
    RS,
    #[serde(rename = "RT")]
    RT,
    #[serde(rename = "RTS")]
    RTS,
    #[serde(rename = "RU")]
    RU,
    #[serde(rename = "RUSSIAN TRADING")]
    RUSSIANTRADING,
    #[serde(rename = "RW")]
    RW,
    #[serde(rename = "RWANDA")]
    RWANDA,
    #[serde(rename = "RX")]
    RX,
    #[serde(rename = "RZ")]
    RZ,
    #[serde(rename = "S1")]
    S1,
    #[serde(rename = "S2")]
    S2,
    #[serde(rename = "S3")]
    S3,
    #[serde(rename = "S4")]
    S4,
    #[serde(rename = "SA")]
    SA,
    #[serde(rename = "SAF")]
    SAF,
    #[serde(rename = "SANTIAGO")]
    SANTIAGO,
    #[serde(rename = "SANTO DOMINGO")]
    SANTODOMINGO,
    #[serde(rename = "SAO PAULO")]
    SAOPAULO,
    #[serde(rename = "SARAJEVO")]
    SARAJEVO,
    #[serde(rename = "SAUDI ARABIA")]
    SAUDIARABIA,
    #[serde(rename = "SB")]
    SB,
    #[serde(rename = "SBA")]
    SBA,
    #[serde(rename = "SC")]
    SC,
    #[serde(rename = "SCE")]
    SCE,
    #[serde(rename = "SCIEX")]
    SCIEX,
    #[serde(rename = "SCOACH-FRANKFURT")]
    SCOACHFRANKFURT,
    #[serde(rename = "SD")]
    SD,
    #[serde(rename = "SE")]
    SE,
    #[serde(rename = "SEDEX-Milan")]
    SEDEXMilan,
    #[serde(rename = "SEND")]
    SEND,
    #[serde(rename = "SF")]
    SF,
    #[serde(rename = "SFE")]
    SFE,
    #[serde(rename = "SG")]
    SG,
    #[serde(rename = "SGX")]
    SGX,
    #[serde(rename = "SGX-ST")]
    SGXST,
    #[serde(rename = "SH")]
    SH,
    #[serde(rename = "SHANGHAI")]
    SHANGHAI,
    #[serde(rename = "SHENZHEN")]
    SHENZHEN,
    #[serde(rename = "SHF")]
    SHF,
    #[serde(rename = "SI")]
    SI,
    #[serde(rename = "SIB")]
    SIB,
    #[serde(rename = "SIBE")]
    SIBE,
    #[serde(rename = "SICEX")]
    SICEX,
    #[serde(rename = "SINGAPORE")]
    SINGAPORE,
    #[serde(rename = "SINGAPORE MAINBD")]
    SINGAPOREMAINBD,
    #[serde(rename = "SISBEX")]
    SISBEX,
    #[serde(rename = "SIX")]
    SIX,
    #[serde(rename = "SIX Digital")]
    SIXDigital,
    #[serde(rename = "SIX Europe LTD")]
    SIXEuropeLTD,
    #[serde(rename = "SIX STRUCTURED")]
    SIXSTRUCTURED,
    #[serde(rename = "SIX Swiss (SP)")]
    SIXSwissSP,
    #[serde(rename = "SJ")]
    SJ,
    #[serde(rename = "SK")]
    SK,
    #[serde(rename = "SL")]
    SL,
    #[serde(rename = "SLOVAK")]
    SLOVAK,
    #[serde(rename = "SM")]
    SM,
    #[serde(rename = "SME")]
    SME,
    #[serde(rename = "SN")]
    SN,
    #[serde(rename = "SO")]
    SO,
    #[serde(rename = "SOP")]
    SOP,
    #[serde(rename = "SP")]
    SP,
    #[serde(rename = "SPCEX")]
    SPCEX,
    #[serde(rename = "SPX")]
    SPX,
    #[serde(rename = "SQ")]
    SQ,
    #[serde(rename = "SR")]
    SR,
    #[serde(rename = "SS")]
    SS,
    #[serde(rename = "SSE")]
    SSE,
    #[serde(rename = "ST")]
    ST,
    #[serde(rename = "St. Petersburg")]
    StPetersburg,
    #[serde(rename = "STMP")]
    STMP,
    #[serde(rename = "stmp")]
    Stmp,
    #[serde(rename = "STRASBOURG")]
    STRASBOURG,
    #[serde(rename = "STUTTGART")]
    STUTTGART,
    #[serde(rename = "SU")]
    SU,
    #[serde(rename = "SUSH")]
    SUSH,
    #[serde(rename = "sush")]
    Sush,
    #[serde(rename = "SV")]
    SV,
    #[serde(rename = "SW")]
    SW,
    #[serde(rename = "SX")]
    SX,
    #[serde(rename = "SXHA")]
    SXHA,
    #[serde(rename = "sxha")]
    Sxha,
    #[serde(rename = "SY")]
    SY,
    #[serde(rename = "SZ")]
    SZ,
    #[serde(rename = "T1")]
    T1,
    #[serde(rename = "T2")]
    T2,
    #[serde(rename = "T3")]
    T3,
    #[serde(rename = "TA")]
    TA,
    #[serde(rename = "TAD")]
    TAD,
    #[serde(rename = "Taipei")]
    Taipei,
    #[serde(rename = "TAIWAN")]
    TAIWAN,
    #[serde(rename = "TASHKENT")]
    TASHKENT,
    #[serde(rename = "TAV")]
    TAV,
    #[serde(rename = "TB")]
    TB,
    #[serde(rename = "TBIT")]
    TBIT,
    #[serde(rename = "TBMA")]
    TBMA,
    #[serde(rename = "TBS POLAND")]
    TBSPOLAND,
    #[serde(rename = "TC")]
    TC,
    #[serde(rename = "TCC")]
    TCC,
    #[serde(rename = "TCM")]
    TCM,
    #[serde(rename = "TD")]
    TD,
    #[serde(rename = "TE")]
    TE,
    #[serde(rename = "TEF")]
    TEF,
    #[serde(rename = "TEHERAN")]
    TEHERAN,
    #[serde(rename = "TEL AVIV")]
    TELAVIV,
    #[serde(rename = "TF")]
    TF,
    #[serde(rename = "TFE")]
    TFE,
    #[serde(rename = "TFX")]
    TFX,
    #[serde(rename = "TG")]
    TG,
    #[serde(rename = "TGE")]
    TGE,
    #[serde(rename = "TH")]
    TH,
    #[serde(rename = "THAILAND")]
    THAILAND,
    #[serde(rename = "THIRD MKT CORP")]
    THIRDMKTCORP,
    #[serde(rename = "TI")]
    TI,
    #[serde(rename = "TIDX")]
    TIDX,
    #[serde(rename = "TISE")]
    TISE,
    #[serde(rename = "TJ")]
    TJ,
    #[serde(rename = "TK")]
    TK,
    #[serde(rename = "TL")]
    TL,
    #[serde(rename = "TLX")]
    TLX,
    #[serde(rename = "TN")]
    TN,
    #[serde(rename = "TO")]
    TO,
    #[serde(rename = "TOKYO")]
    TOKYO,
    #[serde(rename = "TOKYO 2")]
    TOKYO2,
    #[serde(rename = "TOM")]
    TOM,
    #[serde(rename = "TORONTO")]
    TORONTO,
    #[serde(rename = "TP")]
    TP,
    #[serde(rename = "TQ")]
    TQ,
    #[serde(rename = "TR")]
    TR,
    #[serde(rename = "TRACE")]
    TRACE,
    #[serde(rename = "TRADEGATE")]
    TRADEGATE,
    #[serde(rename = "TRCK")]
    TRCK,
    #[serde(rename = "TRINIDAD&TOBAGO")]
    TRINIDADTOBAGO,
    #[serde(rename = "TS")]
    TS,
    #[serde(rename = "TSE")]
    TSE,
    #[serde(rename = "TSX VENTURE")]
    TSXVENTURE,
    #[serde(rename = "TT")]
    TT,
    #[serde(rename = "TTC")]
    TTC,
    #[serde(rename = "TU")]
    TU,
    #[serde(rename = "TUNIS")]
    TUNIS,
    #[serde(rename = "TV")]
    TV,
    #[serde(rename = "TW")]
    TW,
    #[serde(rename = "TX")]
    TX,
    #[serde(rename = "TY")]
    TY,
    #[serde(rename = "TZ")]
    TZ,
    #[serde(rename = "UA")]
    UA,
    #[serde(rename = "UB")]
    UB,
    #[serde(rename = "UC")]
    UC,
    #[serde(rename = "UD")]
    UD,
    #[serde(rename = "UE")]
    UE,
    #[serde(rename = "UF")]
    UF,
    #[serde(rename = "UG")]
    UG,
    #[serde(rename = "UGANDA")]
    UGANDA,
    #[serde(rename = "UH")]
    UH,
    #[serde(rename = "UI")]
    UI,
    #[serde(rename = "UJ")]
    UJ,
    #[serde(rename = "UK")]
    UK,
    #[serde(rename = "UKR")]
    UKR,
    #[serde(rename = "UKRAINIAN EXCH")]
    UKRAINIANEXCH,
    #[serde(rename = "UL")]
    UL,
    #[serde(rename = "UM")]
    UM,
    #[serde(rename = "UN")]
    UN,
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
    #[serde(rename = "UO")]
    UO,
    #[serde(rename = "UP")]
    UP,
    #[serde(rename = "UPBT")]
    UPBT,
    #[serde(rename = "upbt")]
    Upbt,
    #[serde(rename = "UQ")]
    UQ,
    #[serde(rename = "UR")]
    UR,
    #[serde(rename = "URCEX")]
    URCEX,
    #[serde(rename = "US")]
    US,
    #[serde(rename = "USE")]
    USE,
    #[serde(rename = "USP2")]
    USP2,
    #[serde(rename = "usp2")]
    Usp2,
    #[serde(rename = "USP3")]
    USP3,
    #[serde(rename = "usp3")]
    Usp3,
    #[serde(rename = "UT")]
    UT,
    #[serde(rename = "UU")]
    UU,
    #[serde(rename = "UV")]
    UV,
    #[serde(rename = "UW")]
    UW,
    #[serde(rename = "UX")]
    UX,
    #[serde(rename = "UY")]
    UY,
    #[serde(rename = "UZ")]
    UZ,
    #[serde(rename = "VA")]
    VA,
    #[serde(rename = "VALENCIA")]
    VALENCIA,
    #[serde(rename = "VARAZDIN")]
    VARAZDIN,
    #[serde(rename = "VB")]
    VB,
    #[serde(rename = "VC")]
    VC,
    #[serde(rename = "VE")]
    VE,
    #[serde(rename = "VF")]
    VF,
    #[serde(rename = "VG")]
    VG,
    #[serde(rename = "VH")]
    VH,
    #[serde(rename = "VI")]
    VI,
    #[serde(rename = "VIENNA")]
    VIENNA,
    #[serde(rename = "VJ")]
    VJ,
    #[serde(rename = "VK")]
    VK,
    #[serde(rename = "VL")]
    VL,
    #[serde(rename = "VM")]
    VM,
    #[serde(rename = "VN")]
    VN,
    #[serde(rename = "Vorvel")]
    Vorvel,
    #[serde(rename = "VP")]
    VP,
    #[serde(rename = "VR")]
    VR,
    #[serde(rename = "VS")]
    VS,
    #[serde(rename = "VU")]
    VU,
    #[serde(rename = "VX")]
    VX,
    #[serde(rename = "VY")]
    VY,
    #[serde(rename = "WARSAW")]
    WARSAW,
    #[serde(rename = "WBA")]
    WBA,
    #[serde(rename = "WCE")]
    WCE,
    #[serde(rename = "WSE")]
    WSE,
    #[serde(rename = "WT")]
    WT,
    #[serde(rename = "WTB")]
    WTB,
    #[serde(rename = "WX")]
    WX,
    #[serde(rename = "X1")]
    X1,
    #[serde(rename = "X2")]
    X2,
    #[serde(rename = "X9")]
    X9,
    #[serde(rename = "XA")]
    XA,
    #[serde(rename = "XB")]
    XB,
    #[serde(rename = "XBTR")]
    XBTR,
    #[serde(rename = "XC")]
    XC,
    #[serde(rename = "XD")]
    XD,
    #[serde(rename = "XE")]
    XE,
    #[serde(rename = "XETRA")]
    XETRA,
    #[serde(rename = "XF")]
    XF,
    #[serde(rename = "XG")]
    XG,
    #[serde(rename = "XH")]
    XH,
    #[serde(rename = "XI")]
    XI,
    #[serde(rename = "XJ")]
    XJ,
    #[serde(rename = "XK")]
    XK,
    #[serde(rename = "XL")]
    XL,
    #[serde(rename = "XM")]
    XM,
    #[serde(rename = "XN")]
    XN,
    #[serde(rename = "XO")]
    XO,
    #[serde(rename = "XP")]
    XP,
    #[serde(rename = "XQ")]
    XQ,
    #[serde(rename = "XR")]
    XR,
    #[serde(rename = "XS")]
    XS,
    #[serde(rename = "XT")]
    XT,
    #[serde(rename = "XU")]
    XU,
    #[serde(rename = "XV")]
    XV,
    #[serde(rename = "XW")]
    XW,
    #[serde(rename = "XX")]
    XX,
    #[serde(rename = "XY")]
    XY,
    #[serde(rename = "XZ")]
    XZ,
    #[serde(rename = "YC")]
    YC,
    #[serde(rename = "YELLOW SHEETS")]
    YELLOWSHEETS,
    #[serde(rename = "YLX")]
    YLX,
    #[serde(rename = "YOBT")]
    YOBT,
    #[serde(rename = "yobt")]
    Yobt,
    #[serde(rename = "YSE")]
    YSE,
    #[serde(rename = "ZA")]
    ZA,
    #[serde(rename = "ZAGREB")]
    ZAGREB,
    #[serde(rename = "ZAIF")]
    ZAIF,
    #[serde(rename = "zaif")]
    Zaif,
    #[serde(rename = "ZB")]
    ZB,
    #[serde(rename = "ZBCN")]
    ZBCN,
    #[serde(rename = "zbcn")]
    Zbcn,
    #[serde(rename = "ZC")]
    ZC,
    #[serde(rename = "ZCE")]
    ZCE,
    #[serde(rename = "ZG")]
    ZG,
    #[serde(rename = "ZH")]
    ZH,
    #[serde(rename = "ZIMBABWE")]
    ZIMBABWE,
    #[serde(rename = "ZL")]
    ZL,
    #[serde(rename = "ZS")]
    ZS,
    #[serde(rename = "ZU")]
    ZU,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_enum_serialization;

    test_enum_serialization!(
        test_serialize_frankfurt,
        ExchCode,
        FRANKFURT,
        "\"FRANKFURT\""
    );
    test_enum_serialization!(test_serialize_bbox, ExchCode, Bbox, "\"bbox\"");
}
