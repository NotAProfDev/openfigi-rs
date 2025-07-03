//! Enum for all supported market identifiers codes in [OpenFIGI](https://www.openfigi.com/) mapping requests.
//!
//! This list is based on the [OpenFIGI documentation](https://www.openfigi.com/api/documentation) and may be incomplete or subject to change.
//!
//! Example usage:
//! ```rust
//! use openfigi_rs::model::enums::MicCode;
//! let mic = MicCode::XCME; // Chicago Mercantile Exchange (CME)
//! ```
//!
//! For the full list of values, see: <https://api.openfigi.com/v3/mapping/values/micCode>

use serde::{Deserialize, Serialize};

/// Enum for all supported market identifiers codes.
#[expect(missing_docs)]
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MicCode {
    #[serde(rename = "A2XX")]
    A2XX,
    #[serde(rename = "ACEX")]
    ACEX,
    #[serde(rename = "ADRK")]
    ADRK,
    #[serde(rename = "AFET")]
    AFET,
    #[serde(rename = "AIXK")]
    AIXK,
    #[serde(rename = "AMTS")]
    AMTS,
    #[serde(rename = "AMXO")]
    AMXO,
    #[serde(rename = "APEX")]
    APEX,
    #[serde(rename = "APXL")]
    APXL,
    #[serde(rename = "AQEU")]
    AQEU,
    #[serde(rename = "AQSE")]
    AQSE,
    #[serde(rename = "AQXE")]
    AQXE,
    #[serde(rename = "ARCO")]
    ARCO,
    #[serde(rename = "ARCX")]
    ARCX,
    #[serde(rename = "ARTX")]
    ARTX,
    #[serde(rename = "ASXP")]
    ASXP,
    #[serde(rename = "BATE")]
    BATE,
    #[serde(rename = "BATO")]
    BATO,
    #[serde(rename = "BATS")]
    BATS,
    #[serde(rename = "BATY")]
    BATY,
    #[serde(rename = "BCSE")]
    BCSE,
    #[serde(rename = "BEUE")]
    BEUE,
    #[serde(rename = "BIVA")]
    BIVA,
    #[serde(rename = "BJSE")]
    BJSE,
    #[serde(rename = "BLOX")]
    BLOX,
    #[serde(rename = "BMFM")]
    BMFM,
    #[serde(rename = "BMTF")]
    BMTF,
    #[serde(rename = "BMTS")]
    BMTS,
    #[serde(rename = "BOAT")]
    BOAT,
    #[serde(rename = "BOTC")]
    BOTC,
    #[serde(rename = "BSEX")]
    BSEX,
    #[serde(rename = "BTFE")]
    BTFE,
    #[serde(rename = "BURM")]
    BURM,
    #[serde(rename = "BVCA")]
    BVCA,
    #[serde(rename = "BVMF")]
    BVMF,
    #[serde(rename = "C2OX")]
    C2OX,
    #[serde(rename = "CAPA")]
    CAPA,
    #[serde(rename = "CCFX")]
    CCFX,
    #[serde(rename = "CEDX")]
    CEDX,
    #[serde(rename = "CEUX")]
    CEUX,
    #[serde(rename = "CHIA")]
    CHIA,
    #[serde(rename = "CHIC")]
    CHIC,
    #[serde(rename = "CHIJ")]
    CHIJ,
    #[serde(rename = "CHIX")]
    CHIX,
    #[serde(rename = "CMED")]
    CMED,
    #[serde(rename = "CSE2")]
    CSE2,
    #[serde(rename = "DGCX")]
    DGCX,
    #[serde(rename = "DIFX")]
    DIFX,
    #[serde(rename = "DKED")]
    DKED,
    #[serde(rename = "DKTC")]
    DKTC,
    #[serde(rename = "DSMD")]
    DSMD,
    #[serde(rename = "DUMX")]
    DUMX,
    #[serde(rename = "EBMX")]
    EBMX,
    #[serde(rename = "ECEU")]
    ECEU,
    #[serde(rename = "EDGA")]
    EDGA,
    #[serde(rename = "EDGO")]
    EDGO,
    #[serde(rename = "EDGX")]
    EDGX,
    #[serde(rename = "EMLD")]
    EMLD,
    #[serde(rename = "EMTF")]
    EMTF,
    #[serde(rename = "EMTS")]
    EMTS,
    #[serde(rename = "ENAX")]
    ENAX,
    #[serde(rename = "EPRL")]
    EPRL,
    #[serde(rename = "ERIS")]
    ERIS,
    #[serde(rename = "ETLX")]
    ETLX,
    #[serde(rename = "EUCH")]
    EUCH,
    #[serde(rename = "EUWX")]
    EUWX,
    #[serde(rename = "EXGM")]
    EXGM,
    #[serde(rename = "FISH")]
    FISH,
    #[serde(rename = "FMTS")]
    FMTS,
    #[serde(rename = "FNDK")]
    FNDK,
    #[serde(rename = "FNFI")]
    FNFI,
    #[serde(rename = "FNFT")]
    FNFT,
    #[serde(rename = "FNIS")]
    FNIS,
    #[serde(rename = "FNSE")]
    FNSE,
    #[serde(rename = "FRAB")]
    FRAB,
    #[serde(rename = "FREX")]
    FREX,
    #[serde(rename = "GBOT")]
    GBOT,
    #[serde(rename = "GEMX")]
    GEMX,
    #[serde(rename = "GMEG")]
    GMEG,
    #[serde(rename = "GMNI")]
    GMNI,
    #[serde(rename = "GSXL")]
    GSXL,
    #[serde(rename = "HKME")]
    HKME,
    #[serde(rename = "HMTF")]
    HMTF,
    #[serde(rename = "HOTC")]
    HOTC,
    #[serde(rename = "HSTC")]
    HSTC,
    #[serde(rename = "ICDX")]
    ICDX,
    #[serde(rename = "ICEL")]
    ICEL,
    #[serde(rename = "ICXL")]
    ICXL,
    #[serde(rename = "IEPA")]
    IEPA,
    #[serde(rename = "IEXG")]
    IEXG,
    #[serde(rename = "IFAD")]
    IFAD,
    #[serde(rename = "IFCA")]
    IFCA,
    #[serde(rename = "IFED")]
    IFED,
    #[serde(rename = "IFEU")]
    IFEU,
    #[serde(rename = "IFLL")]
    IFLL,
    #[serde(rename = "IFLO")]
    IFLO,
    #[serde(rename = "IFLX")]
    IFLX,
    #[serde(rename = "IFSG")]
    IFSG,
    #[serde(rename = "IFUS")]
    IFUS,
    #[serde(rename = "IINX")]
    IINX,
    #[serde(rename = "IMTS")]
    IMTS,
    #[serde(rename = "INSE")]
    INSE,
    #[serde(rename = "LEUE")]
    LEUE,
    #[serde(rename = "LICA")]
    LICA,
    #[serde(rename = "LIQU")]
    LIQU,
    #[serde(rename = "LNEQ")]
    LNEQ,
    #[serde(rename = "LSSI")]
    LSSI,
    #[serde(rename = "LTSE")]
    LTSE,
    #[serde(rename = "LYNX")]
    LYNX,
    #[serde(rename = "MALX")]
    MALX,
    #[serde(rename = "MARF")]
    MARF,
    #[serde(rename = "MATN")]
    MATN,
    #[serde(rename = "MCAD")]
    MCAD,
    #[serde(rename = "MCRY")]
    MCRY,
    #[serde(rename = "MCXX")]
    MCXX,
    #[serde(rename = "MEMX")]
    MEMX,
    #[serde(rename = "MFOX")]
    MFOX,
    #[serde(rename = "MISX")]
    MISX,
    #[serde(rename = "MOTX")]
    MOTX,
    #[serde(rename = "MPRL")]
    MPRL,
    #[serde(rename = "MSAX")]
    MSAX,
    #[serde(rename = "MTAA")]
    MTAA,
    #[serde(rename = "MTAH")]
    MTAH,
    #[serde(rename = "MTCH")]
    MTCH,
    #[serde(rename = "MTSC")]
    MTSC,
    #[serde(rename = "MTSD")]
    MTSD,
    #[serde(rename = "MTSF")]
    MTSF,
    #[serde(rename = "MUND")]
    MUND,
    #[serde(rename = "MXOP")]
    MXOP,
    #[serde(rename = "N2EX")]
    N2EX,
    #[serde(rename = "NASX")]
    NASX,
    #[serde(rename = "NCEL")]
    NCEL,
    #[serde(rename = "NDEX")]
    NDEX,
    #[serde(rename = "NEOE")]
    NEOE,
    #[serde(rename = "NEXX")]
    NEXX,
    #[serde(rename = "NILX")]
    NILX,
    #[serde(rename = "NORX")]
    NORX,
    #[serde(rename = "NOTC")]
    NOTC,
    #[serde(rename = "NZFX")]
    NZFX,
    #[serde(rename = "ODXE")]
    ODXE,
    #[serde(rename = "OMGA")]
    OMGA,
    #[serde(rename = "OMIP")]
    OMIP,
    #[serde(rename = "OOTC")]
    OOTC,
    #[serde(rename = "OPEX")]
    OPEX,
    #[serde(rename = "OTCM")]
    OTCM,
    #[serde(rename = "OTXB")]
    OTXB,
    #[serde(rename = "PDEX")]
    PDEX,
    #[serde(rename = "PFTQ")]
    PFTQ,
    #[serde(rename = "PFTS")]
    PFTS,
    #[serde(rename = "PLPD")]
    PLPD,
    #[serde(rename = "PLUS")]
    PLUS,
    #[serde(rename = "PURE")]
    PURE,
    #[serde(rename = "ROCO")]
    ROCO,
    #[serde(rename = "ROFX")]
    ROFX,
    #[serde(rename = "ROTC")]
    ROTC,
    #[serde(rename = "RTSX")]
    RTSX,
    #[serde(rename = "RUSX")]
    RUSX,
    #[serde(rename = "SBIJ")]
    SBIJ,
    #[serde(rename = "SBIU")]
    SBIU,
    #[serde(rename = "SBMF")]
    SBMF,
    #[serde(rename = "SEDX")]
    SEDX,
    #[serde(rename = "SEND")]
    SEND,
    #[serde(rename = "SGMU")]
    SGMU,
    #[serde(rename = "SGMX")]
    SGMX,
    #[serde(rename = "SHAR")]
    SHAR,
    #[serde(rename = "SHSC")]
    SHSC,
    #[serde(rename = "SIMV")]
    SIMV,
    #[serde(rename = "SMEX")]
    SMEX,
    #[serde(rename = "SPHR")]
    SPHR,
    #[serde(rename = "SPIM")]
    SPIM,
    #[serde(rename = "SZSC")]
    SZSC,
    #[serde(rename = "TBSP")]
    TBSP,
    #[serde(rename = "TFEX")]
    TFEX,
    #[serde(rename = "TOMX")]
    TOMX,
    #[serde(rename = "TQEX")]
    TQEX,
    #[serde(rename = "TREA")]
    TREA,
    #[serde(rename = "TREU")]
    TREU,
    #[serde(rename = "TRNL")]
    TRNL,
    #[serde(rename = "TRPX")]
    TRPX,
    #[serde(rename = "TRQX")]
    TRQX,
    #[serde(rename = "TWEA")]
    TWEA,
    #[serde(rename = "TWEM")]
    TWEM,
    #[serde(rename = "UKEX")]
    UKEX,
    #[serde(rename = "WDER")]
    WDER,
    #[serde(rename = "WMTF")]
    WMTF,
    #[serde(rename = "XADE")]
    XADE,
    #[serde(rename = "XADF")]
    XADF,
    #[serde(rename = "XADS")]
    XADS,
    #[serde(rename = "XAIM")]
    XAIM,
    #[serde(rename = "XALG")]
    XALG,
    #[serde(rename = "XAMM")]
    XAMM,
    #[serde(rename = "XAMS")]
    XAMS,
    #[serde(rename = "XAPA")]
    XAPA,
    #[serde(rename = "XARM")]
    XARM,
    #[serde(rename = "XASE")]
    XASE,
    #[serde(rename = "XASX")]
    XASX,
    #[serde(rename = "XATH")]
    XATH,
    #[serde(rename = "XATS")]
    XATS,
    #[serde(rename = "XATX")]
    XATX,
    #[serde(rename = "XBAA")]
    XBAA,
    #[serde(rename = "XBAB")]
    XBAB,
    #[serde(rename = "XBAH")]
    XBAH,
    #[serde(rename = "XBAN")]
    XBAN,
    #[serde(rename = "XBAR")]
    XBAR,
    #[serde(rename = "XBBJ")]
    XBBJ,
    #[serde(rename = "XBCL")]
    XBCL,
    #[serde(rename = "XBCM")]
    XBCM,
    #[serde(rename = "XBCV")]
    XBCV,
    #[serde(rename = "XBCX")]
    XBCX,
    #[serde(rename = "XBDA")]
    XBDA,
    #[serde(rename = "XBDV")]
    XBDV,
    #[serde(rename = "XBEL")]
    XBEL,
    #[serde(rename = "XBER")]
    XBER,
    #[serde(rename = "XBES")]
    XBES,
    #[serde(rename = "XBEY")]
    XBEY,
    #[serde(rename = "XBIL")]
    XBIL,
    #[serde(rename = "XBKK")]
    XBKK,
    #[serde(rename = "XBLB")]
    XBLB,
    #[serde(rename = "XBLN")]
    XBLN,
    #[serde(rename = "XBNV")]
    XBNV,
    #[serde(rename = "XBOG")]
    XBOG,
    #[serde(rename = "XBOL")]
    XBOL,
    #[serde(rename = "XBOM")]
    XBOM,
    #[serde(rename = "XBOS")]
    XBOS,
    #[serde(rename = "XBOT")]
    XBOT,
    #[serde(rename = "XBOX")]
    XBOX,
    #[serde(rename = "XBRA")]
    XBRA,
    #[serde(rename = "XBRD")]
    XBRD,
    #[serde(rename = "XBRN")]
    XBRN,
    #[serde(rename = "XBRU")]
    XBRU,
    #[serde(rename = "XBRV")]
    XBRV,
    #[serde(rename = "XBSD")]
    XBSD,
    #[serde(rename = "XBSE")]
    XBSE,
    #[serde(rename = "XBTR")]
    XBTR,
    #[serde(rename = "XBUD")]
    XBUD,
    #[serde(rename = "XBUE")]
    XBUE,
    #[serde(rename = "XBUL")]
    XBUL,
    #[serde(rename = "XBVC")]
    XBVC,
    #[serde(rename = "XBVM")]
    XBVM,
    #[serde(rename = "XBVR")]
    XBVR,
    #[serde(rename = "XBXO")]
    XBXO,
    #[serde(rename = "XCAI")]
    XCAI,
    #[serde(rename = "XCAS")]
    XCAS,
    #[serde(rename = "XCAY")]
    XCAY,
    #[serde(rename = "XCBF")]
    XCBF,
    #[serde(rename = "XCBO")]
    XCBO,
    #[serde(rename = "XCBT")]
    XCBT,
    #[serde(rename = "XCCX")]
    XCCX,
    #[serde(rename = "XCEC")]
    XCEC,
    #[serde(rename = "XCEG")]
    XCEG,
    #[serde(rename = "XCFE")]
    XCFE,
    #[serde(rename = "XCHG")]
    XCHG,
    #[serde(rename = "XCHI")]
    XCHI,
    #[serde(rename = "XCIE")]
    XCIE,
    #[serde(rename = "XCIS")]
    XCIS,
    #[serde(rename = "XCME")]
    XCME,
    #[serde(rename = "XCNQ")]
    XCNQ,
    #[serde(rename = "XCOL")]
    XCOL,
    #[serde(rename = "XCSE")]
    XCSE,
    #[serde(rename = "XCSX")]
    XCSX,
    #[serde(rename = "XCUE")]
    XCUE,
    #[serde(rename = "XCX2")]
    XCX2,
    #[serde(rename = "XCXD")]
    XCXD,
    #[serde(rename = "XCYS")]
    XCYS,
    #[serde(rename = "XDAR")]
    XDAR,
    #[serde(rename = "XDCE")]
    XDCE,
    #[serde(rename = "XDES")]
    XDES,
    #[serde(rename = "XDFM")]
    XDFM,
    #[serde(rename = "XDHA")]
    XDHA,
    #[serde(rename = "XDMI")]
    XDMI,
    #[serde(rename = "XDPA")]
    XDPA,
    #[serde(rename = "XDRF")]
    XDRF,
    #[serde(rename = "XDSE")]
    XDSE,
    #[serde(rename = "XDSX")]
    XDSX,
    #[serde(rename = "XDUB")]
    XDUB,
    #[serde(rename = "XDUS")]
    XDUS,
    #[serde(rename = "XECM")]
    XECM,
    #[serde(rename = "XECS")]
    XECS,
    #[serde(rename = "XEEE")]
    XEEE,
    #[serde(rename = "XELX")]
    XELX,
    #[serde(rename = "XEMD")]
    XEMD,
    #[serde(rename = "XEQT")]
    XEQT,
    #[serde(rename = "XETR")]
    XETR,
    #[serde(rename = "XEUE")]
    XEUE,
    #[serde(rename = "XEUR")]
    XEUR,
    #[serde(rename = "XFEX")]
    XFEX,
    #[serde(rename = "XFKA")]
    XFKA,
    #[serde(rename = "XFM")]
    XFM,
    #[serde(rename = "XFRA")]
    XFRA,
    #[serde(rename = "XGAT")]
    XGAT,
    #[serde(rename = "XGHA")]
    XGHA,
    #[serde(rename = "XGME")]
    XGME,
    #[serde(rename = "XGSE")]
    XGSE,
    #[serde(rename = "XGTG")]
    XGTG,
    #[serde(rename = "XGUA")]
    XGUA,
    #[serde(rename = "XHAM")]
    XHAM,
    #[serde(rename = "XHAN")]
    XHAN,
    #[serde(rename = "XHEL")]
    XHEL,
    #[serde(rename = "XHFT")]
    XHFT,
    #[serde(rename = "XHKF")]
    XHKF,
    #[serde(rename = "XHKG")]
    XHKG,
    #[serde(rename = "XHNF")]
    XHNF,
    #[serde(rename = "XHNX")]
    XHNX,
    #[serde(rename = "XICE")]
    XICE,
    #[serde(rename = "XICX")]
    XICX,
    #[serde(rename = "XIDX")]
    XIDX,
    #[serde(rename = "XIMC")]
    XIMC,
    #[serde(rename = "XINE")]
    XINE,
    #[serde(rename = "XIQS")]
    XIQS,
    #[serde(rename = "XISA")]
    XISA,
    #[serde(rename = "XIST")]
    XIST,
    #[serde(rename = "XISX")]
    XISX,
    #[serde(rename = "XJAM")]
    XJAM,
    #[serde(rename = "XJAS")]
    XJAS,
    #[serde(rename = "XJSE")]
    XJSE,
    #[serde(rename = "XKAC")]
    XKAC,
    #[serde(rename = "XKAR")]
    XKAR,
    #[serde(rename = "XKAZ")]
    XKAZ,
    #[serde(rename = "XKBT")]
    XKBT,
    #[serde(rename = "XKEM")]
    XKEM,
    #[serde(rename = "XKFB")]
    XKFB,
    #[serde(rename = "XKFE")]
    XKFE,
    #[serde(rename = "XKHA")]
    XKHA,
    #[serde(rename = "XKIS")]
    XKIS,
    #[serde(rename = "XKLS")]
    XKLS,
    #[serde(rename = "XKON")]
    XKON,
    #[serde(rename = "XKOS")]
    XKOS,
    #[serde(rename = "XKRX")]
    XKRX,
    #[serde(rename = "XKSE")]
    XKSE,
    #[serde(rename = "XKUW")]
    XKUW,
    #[serde(rename = "XLAO")]
    XLAO,
    #[serde(rename = "XLDN")]
    XLDN,
    #[serde(rename = "XLFX")]
    XLFX,
    #[serde(rename = "XLIM")]
    XLIM,
    #[serde(rename = "XLIS")]
    XLIS,
    #[serde(rename = "XLIT")]
    XLIT,
    #[serde(rename = "XLJU")]
    XLJU,
    #[serde(rename = "XLME")]
    XLME,
    #[serde(rename = "XLOD")]
    XLOD,
    #[serde(rename = "XLON")]
    XLON,
    #[serde(rename = "XLUS")]
    XLUS,
    #[serde(rename = "XLUX")]
    XLUX,
    #[serde(rename = "XMAB")]
    XMAB,
    #[serde(rename = "XMAD")]
    XMAD,
    #[serde(rename = "XMAE")]
    XMAE,
    #[serde(rename = "XMAL")]
    XMAL,
    #[serde(rename = "XMAN")]
    XMAN,
    #[serde(rename = "XMAT")]
    XMAT,
    #[serde(rename = "XMAU")]
    XMAU,
    #[serde(rename = "XMCE")]
    XMCE,
    #[serde(rename = "XMDS")]
    XMDS,
    #[serde(rename = "XMEV")]
    XMEV,
    #[serde(rename = "XMEX")]
    XMEX,
    #[serde(rename = "XMGE")]
    XMGE,
    #[serde(rename = "XMIO")]
    XMIO,
    #[serde(rename = "XMNT")]
    XMNT,
    #[serde(rename = "XMNX")]
    XMNX,
    #[serde(rename = "XMOC")]
    XMOC,
    #[serde(rename = "XMOD")]
    XMOD,
    #[serde(rename = "XMOL")]
    XMOL,
    #[serde(rename = "XMON")]
    XMON,
    #[serde(rename = "XMOS")]
    XMOS,
    #[serde(rename = "XMOT")]
    XMOT,
    #[serde(rename = "XMPW")]
    XMPW,
    #[serde(rename = "XMRV")]
    XMRV,
    #[serde(rename = "XMSW")]
    XMSW,
    #[serde(rename = "XMTB")]
    XMTB,
    #[serde(rename = "XMUN")]
    XMUN,
    #[serde(rename = "XMUS")]
    XMUS,
    #[serde(rename = "XNAI")]
    XNAI,
    #[serde(rename = "XNAM")]
    XNAM,
    #[serde(rename = "XNAS")]
    XNAS,
    #[serde(rename = "XNCD")]
    XNCD,
    #[serde(rename = "XNCM")]
    XNCM,
    #[serde(rename = "XNDQ")]
    XNDQ,
    #[serde(rename = "XNDX")]
    XNDX,
    #[serde(rename = "XNEC")]
    XNEC,
    #[serde(rename = "XNEP")]
    XNEP,
    #[serde(rename = "XNGM")]
    XNGM,
    #[serde(rename = "XNGO")]
    XNGO,
    #[serde(rename = "XNGS")]
    XNGS,
    #[serde(rename = "XNIM")]
    XNIM,
    #[serde(rename = "XNKS")]
    XNKS,
    #[serde(rename = "XNLX")]
    XNLX,
    #[serde(rename = "XNMS")]
    XNMS,
    #[serde(rename = "XNSA")]
    XNSA,
    #[serde(rename = "XNSE")]
    XNSE,
    #[serde(rename = "XNYM")]
    XNYM,
    #[serde(rename = "XNYS")]
    XNYS,
    #[serde(rename = "XNZE")]
    XNZE,
    #[serde(rename = "XOAM")]
    XOAM,
    #[serde(rename = "XOCH")]
    XOCH,
    #[serde(rename = "XOPV")]
    XOPV,
    #[serde(rename = "XOSE")]
    XOSE,
    #[serde(rename = "XOSL")]
    XOSL,
    #[serde(rename = "XOTC")]
    XOTC,
    #[serde(rename = "XPAE")]
    XPAE,
    #[serde(rename = "XPAR")]
    XPAR,
    #[serde(rename = "XPBT")]
    XPBT,
    #[serde(rename = "XPHL")]
    XPHL,
    #[serde(rename = "XPHS")]
    XPHS,
    #[serde(rename = "XPIC")]
    XPIC,
    #[serde(rename = "XPOM")]
    XPOM,
    #[serde(rename = "XPOR")]
    XPOR,
    #[serde(rename = "XPOS")]
    XPOS,
    #[serde(rename = "XPOW")]
    XPOW,
    #[serde(rename = "XPRA")]
    XPRA,
    #[serde(rename = "XPSX")]
    XPSX,
    #[serde(rename = "XPTY")]
    XPTY,
    #[serde(rename = "XQMH")]
    XQMH,
    #[serde(rename = "XQTX")]
    XQTX,
    #[serde(rename = "XQUI")]
    XQUI,
    #[serde(rename = "XRAS")]
    XRAS,
    #[serde(rename = "XRBM")]
    XRBM,
    #[serde(rename = "XRIS")]
    XRIS,
    #[serde(rename = "XRMZ")]
    XRMZ,
    #[serde(rename = "XROS")]
    XROS,
    #[serde(rename = "XSAF")]
    XSAF,
    #[serde(rename = "XSAM")]
    XSAM,
    #[serde(rename = "XSAP")]
    XSAP,
    #[serde(rename = "XSAT")]
    XSAT,
    #[serde(rename = "XSAU")]
    XSAU,
    #[serde(rename = "XSBI")]
    XSBI,
    #[serde(rename = "XSCE")]
    XSCE,
    #[serde(rename = "XSDX")]
    XSDX,
    #[serde(rename = "XSEC")]
    XSEC,
    #[serde(rename = "XSES")]
    XSES,
    #[serde(rename = "XSFE")]
    XSFE,
    #[serde(rename = "XSGE")]
    XSGE,
    #[serde(rename = "XSGO")]
    XSGO,
    #[serde(rename = "XSHE")]
    XSHE,
    #[serde(rename = "XSHG")]
    XSHG,
    #[serde(rename = "XSIM")]
    XSIM,
    #[serde(rename = "XSMP")]
    XSMP,
    #[serde(rename = "XSPS")]
    XSPS,
    #[serde(rename = "XSRM")]
    XSRM,
    #[serde(rename = "XSSC")]
    XSSC,
    #[serde(rename = "XSSE")]
    XSSE,
    #[serde(rename = "XSTC")]
    XSTC,
    #[serde(rename = "XSTE")]
    XSTE,
    #[serde(rename = "XSTO")]
    XSTO,
    #[serde(rename = "XSTU")]
    XSTU,
    #[serde(rename = "XSVA")]
    XSVA,
    #[serde(rename = "XSWA")]
    XSWA,
    #[serde(rename = "XSWX")]
    XSWX,
    #[serde(rename = "XTAE")]
    XTAE,
    #[serde(rename = "XTAF")]
    XTAF,
    #[serde(rename = "XTAI")]
    XTAI,
    #[serde(rename = "XTAL")]
    XTAL,
    #[serde(rename = "XTEH")]
    XTEH,
    #[serde(rename = "XTFF")]
    XTFF,
    #[serde(rename = "XTKO")]
    XTKO,
    #[serde(rename = "XTKS")]
    XTKS,
    #[serde(rename = "XTKT")]
    XTKT,
    #[serde(rename = "XTRN")]
    XTRN,
    #[serde(rename = "XTSE")]
    XTSE,
    #[serde(rename = "XTSX")]
    XTSX,
    #[serde(rename = "XTUN")]
    XTUN,
    #[serde(rename = "XUBS")]
    XUBS,
    #[serde(rename = "XUGA")]
    XUGA,
    #[serde(rename = "XULA")]
    XULA,
    #[serde(rename = "XUSE")]
    XUSE,
    #[serde(rename = "XVAL")]
    XVAL,
    #[serde(rename = "XVPA")]
    XVPA,
    #[serde(rename = "XVTX")]
    XVTX,
    #[serde(rename = "XWAR")]
    XWAR,
    #[serde(rename = "XWBO")]
    XWBO,
    #[serde(rename = "XZAG")]
    XZAG,
    #[serde(rename = "XZCE")]
    XZCE,
    #[serde(rename = "XZIM")]
    XZIM,
    #[serde(rename = "YLDX")]
    YLDX,
    #[serde(rename = "YYYY")]
    YYYY,
    #[serde(rename = "ZFXM")]
    ZFXM,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_enum_serialization;

    test_enum_serialization!(test_serialize_xcme, MicCode, XCME, "\"XCME\"");
    test_enum_serialization!(test_serialize_yldx, MicCode, YLDX, "\"YLDX\"");
}
