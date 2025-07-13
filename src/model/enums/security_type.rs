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
    /// Asset Backed Security (backed by AUTO loans)
    #[serde(rename = "ABS Auto")]
    ABSAuto,
    /// Asset Backed Security (backed by Other less common loans and receivables) i.e. Charge accounts, Reference Notes, etc.
    #[serde(rename = "ABS Card")]
    ABSCard,
    /// Asset Backed Security (backed by Other less common loans and receivables) i.e. Charge accounts, Reference Notes, etc.
    #[serde(rename = "ABS Home")]
    ABSHome,
    /// Asset Backed Security (backed by Home Equity loans)
    #[serde(rename = "ABS Other")]
    ABSOther,
    /// Mexico Bankers Acceptances
    #[serde(rename = "ACCEPT BANCARIA")]
    ACCEPTBANCARIA,
    /// These are bonds that had been adjustable, but were remarketed as Fixed interest rate bonds, and the coupon rate will no longer be subject to change.
    #[serde(rename = "ADJ CONV. TO FIXED")]
    ADJCONVTOFIXED,
    /// These are bonds that had been adjustable, but were remarketed as Fixed interest rate bonds, and the coupon rate will no longer be subject to change. The security is an Original Issue Discount.
    #[serde(rename = "ADJ CONV. TO FIXED, OID")]
    ADJCONVTOFIXEDOID,
    /// The rate changes throughout the life of the bond. This includes variables and step-coupons.
    #[serde(rename = "ADJUSTABLE")]
    ADJUSTABLE,
    /// The rate changes throughout the life of the bond. The security is an Original Issue Discount.
    #[serde(rename = "ADJUSTABLE, OID")]
    ADJUSTABLEOID,
    /// A negotiable certificate issued by a U.S. bank representing a specified number of shares (or one share) in a foreign stock that is traded on a U.S. exchange. ADRs are denominated in U.S. dollars, with the underlying security held by a U.S. financial institution overseas. ADRs help to reduce administration and duty costs that would otherwise be levied on each transaction.
    #[serde(rename = "ADR")]
    ADR,
    /// Asset Backed Loans issued by an Agency such as SBA
    #[serde(rename = "Agncy ABS Home")]
    AgncyABSHome,
    /// Asset Backed Loans issued by an Agency such as SBA
    #[serde(rename = "Agncy ABS Other")]
    AgncyABSOther,
    /// Agency Commercial Mortgage security (ie GNR ticker)
    #[serde(rename = "Agncy CMBS")]
    AgncyCMBS,
    /// Agency Floater
    #[serde(rename = "Agncy CMO FLT")]
    AgncyCMOFLT,
    /// Agency CMO Inverse Floating Rate security, Fannie Mae, Freddie Mac, Ginnie Mae
    #[serde(rename = "Agncy CMO INV")]
    AgncyCMOINV,
    /// Collateralized Mortgage Obligation (CMO). A security which pools together mortgages and separates them into short-, medium-, and long-term positions (called tranches). Tranches are set up to pay different rates of interest depending upon their maturity.
    #[serde(rename = "Agncy CMO IO")]
    AgncyCMOIO,
    /// Agency CMO Other(Less common type of security such as PAC or Sequential) Fannie Mae, Freddie Mac, Ginnie Mae
    #[serde(rename = "Agncy CMO Other")]
    AgncyCMOOther,
    /// Agency collateralized Mortgage Obligations - Principal only
    #[serde(rename = "Agncy CMO PO")]
    AgncyCMOPO,
    /// Agency CMO Z (Accrual Bond), Fannie Mae, Freddie Mac, Ginnie Mae
    #[serde(rename = "Agncy CMO Z")]
    AgncyCMOZ,
    /// A financial security backed by a loan, lease or receivables against assets other than real estate and mortgage-backed securities.
    #[serde(rename = "Asset-Based")]
    AssetBased,
    /// Event-driven loan intended to provide finance for a short period of time until longer-term financing can be obtained. Fully secured by a specific pool of assets, with availability determined as a percentage of the current value of the collateral assets.
    #[serde(rename = "ASSET-BASED BRIDGE")]
    ASSETBASEDBRIDGE,
    /// Event-driven loan intended to provide finance for a short period of time until longer-term financing can be obtained. Fully secured by a specific pool of assets, with availability determined as a percentage of the current value of the collateral assets. Once borrowed funds are repaid they can be borrowed again.
    #[serde(rename = "ASSET-BASED BRIDGE REV")]
    ASSETBASEDBRIDGEREV,
    #[serde(rename = "ASSET-BASED BRIDGE TERM")]
    ASSETBASEDBRIDGETERM,
    #[serde(rename = "ASSET-BASED DELAY-DRAW TERM")]
    ASSETBASEDDELAYDRAWTERM,
    /// Fully secured by a specific pool of assets, with availability determined as a percentage of the current value of the collateral assets. Issued when a company is in bankruptcy
    #[serde(rename = "ASSET-BASED DIP")]
    ASSETBASEDDIP,
    #[serde(rename = "ASSET-BASED DIP DELAY-DRAW")]
    ASSETBASEDDIPDELAYDRAW,
    /// Fully secured by a specific pool of assets, with availability determined as a percentage of the current value of the collateral assets. Issued when a company is in bankruptcy. Once borrowed funds are repaid they can be borrowed again.
    #[serde(rename = "ASSET-BASED DIP REV")]
    ASSETBASEDDIPREV,
    /// Fully secured by a specific pool of assets, with availability determined as a percentage of the current value of the collateral assets. Issued when a company is in bankruptcy. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "ASSET-BASED DIP TERM")]
    ASSETBASEDDIPTERM,
    /// Fully secured by a specific pool of assets, with availability determined as a percentage of the current value of the collateral assets. Guaranteed by the lender(s) and used for purchase of goods, completion of construction projects, and/or general purposes.
    #[serde(rename = "ASSET-BASED LOC")]
    ASSETBASEDLOC,
    #[serde(rename = "ASSET-BASED PIK REV")]
    ASSETBASEDPIKREV,
    #[serde(rename = "ASSET-BASED PIK TERM")]
    ASSETBASEDPIKTERM,
    /// Fully secured by a specific pool of assets, with availability determined as a percentage of the current value of the collateral assets. Once borrowed funds are repaid they can be borrowed again.
    #[serde(rename = "ASSET-BASED REV")]
    ASSETBASEDREV,
    /// Fully secured by a specific pool of assets, with availability determined as a percentage of the current value of the collateral assets. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "ASSET-BASED TERM")]
    ASSETBASEDTERM,
    /// Market issue type where the issue is an Australian issuer issuing in AUD into the Australian domestic mkt.
    #[serde(rename = "AUSTRALIAN")]
    AUSTRALIAN,
    /// Certificate of Deposit (CD). A debt instrument issued by a bank that will pay interest, periodically or at maturity (set by competitive forces in the marketplace), and principal when it reaches maturity. Maturities range from a few weeks to several years. It may have a fixed or floating rate of interest. Certificate of Deposit Rollover. An investment strategy used to defer taxes on income from a Certificate of Deposit. An individual who has purchased a certificate of deposit on margin expected to mature in the following year, can deduct the interest on the loan in the current year and move the income from the certificate to the up-coming year.
    #[serde(rename = "AUSTRALIAN CD")]
    AUSTRALIANCD,
    /// Commercial Paper. An unsecured short-term promise to repay a fixed amount on a certain future date. Commercial paper usually matures from 2 to 270 days and is traded on a discount basis. This debt instrument, issued by banks, companies and other borrowers, use only their credit ratings to back the security. Commercial Paper House. A dealer who buys commercial paper at one price, then tries to sell it at another.
    #[serde(rename = "AUSTRALIAN CP")]
    AUSTRALIANCP,
    /// Austrian Certificate: Depositary Receipt issued in Austria,
    #[serde(rename = "Austrian Crt")]
    AustrianCrt,
    /// Banker's Acceptance. A draft drawn and accepted by banks that is based upon funds that will pay its face value at maturity. The security is traded at a discount. Because the accepting institution is obligated to pay for the bill, a banker's acceptance is considered less risky than commercial paper. Bloomberg abbreviates bankers acceptance as BA.
    #[serde(rename = "BANK ACCEPT BILL")]
    BANKACCEPTBILL,
    /// A bank bill is a short-term money market investment of which the acceptor and/or endorser is a bank.
    #[serde(rename = "BANK BILL")]
    BANKBILL,
    /// Banker's Acceptance. A draft drawn and accepted by banks that is based upon funds that will pay its face value at maturity. The security is traded at a discount. Because the accepting institution is obligated to pay for the bill, a banker's acceptance is considered less risky than commercial paper. Bloomberg abbreviates bankers acceptance as BA.
    #[serde(rename = "BANK NOTE")]
    BANKNOTE,
    /// Bankers Acceptances - a draft or bill of exchange drawn on a bank and accepted by it
    #[serde(rename = "BANKERS ACCEPT")]
    BANKERSACCEPT,
    /// A draft drawn and accepted by banks that is based upon funds that will pay its face value at maturity. The security is traded at a discount.
    #[serde(rename = "BANKERS ACCEPTANCE")]
    BANKERSACCEPTANCE,
    /// Exchange of different floating indices in the same currency
    #[serde(rename = "BASIS SWAP")]
    BASISSWAP,
    #[serde(rename = "BASIS TRADE ON CLOSE")]
    BASISTRADEONCLOSE,
    /// A warrant over a group of securities, often within a sector.
    #[serde(rename = "Basket WRT")]
    BasketWRT,
    /// Brazilian Depositary Receipt: These are certificates that represent the stocks from a foreign company and that are issued in Brazil by a local financial institution based on the amount of shares the institution holds.
    #[serde(rename = "BDR")]
    BDR,
    /// Bearer Security. A negotiable security owned by the holder who is not registered upon the issuers books. Title to bearer securities is transferred by delivery.
    #[serde(rename = "BEARER DEP NOTE")]
    BEARERDEPNOTE,
    /// Belgium Certificate: Depositary Receipts issued in Belgium.
    #[serde(rename = "Belgium Cert")]
    BelgiumCert,
    /// Belgium Domestic Commercial Paper - An unsecured short-term promise to repay a fixed amount on a certain future date. Commercial paper usually matures from 2 to 270 days and is traded on a discount basis. This debt instrument, issued by banks, companies and other borrowers, use only their credit ratings to back the security. Commercial Paper House. A dealer who buys commercial paper at one price, then tries to sell it at another
    #[serde(rename = "BELGIUM CP")]
    BELGIUMCP,
    /// Bill of Exchange. A short-term debt, the collateral for which is either a commodity or another product in the midst of transit. Papers that require the addressee to pay the bearer or anotherr person on demand.
    #[serde(rename = "BILL OF EXCHANGE")]
    BILLOFEXCHANGE,
    #[serde(rename = "BILLET A ORDRE")]
    BILLETAORDRE,
    /// Bond. A long-term debt security, issued by a corporation or government, with a stated interest rate and fixed due dates when interest and principal must be paid. Specific features are written into each bond's indenture, including whether the interest and principal will be paid to the person in whose name the security is registered, or if it will be payable to anyone presenting its coupons, in which case it is considered a bearer bond. Bondholders have a promise of repayment from the issuer but hold no ownership privileges as stockholders do.
    #[serde(rename = "Bond")]
    Bond,
    #[serde(rename = "BRAZIL GENERIC")]
    BRAZILGENERIC,
    /// Brazilian Mmkt instrument, interbank deposits or certificado de deposito interbanco (CDI)
    #[serde(rename = "BRAZILIAN CDI")]
    BRAZILIANCDI,
    /// A loan, often event-driven, intended to provide finance for a short period of time until longer-term, permanent financing can be obtained. Bridge loans often contain significant fee and/or interest rate step up provisions which kick in after a short period (i.e. 12 months or less)
    #[serde(rename = "BRIDGE")]
    BRIDGE,
    /// Event-driven loan intended to provide finance for a short period of time until longer-term financing can be obtained. Borrower is allowed to draw down on the loan in one or more drawings within a specified time period after the signing date.
    #[serde(rename = "BRIDGE DELAY-DRAW")]
    BRIDGEDELAYDRAW,
    /// Event-driven loan intended to provide finance for a short period of time until longer-term financing can be obtained. Borrower is allowed to draw down on the loan in one or more drawings within a specified time period after the signing date.
    #[serde(rename = "BRIDGE DELAY-DRAW TERM")]
    BRIDGEDELAYDRAWTERM,
    #[serde(rename = "BRIDGE DIP TERM")]
    BRIDGEDIPTERM,
    /// Short-term financing issued by a bank to ensure/guarantee that an underlying project is completed.
    #[serde(rename = "BRIDGE GUARANTEE FAC")]
    BRIDGEGUARANTEEFAC,
    /// Event-driven loan intended to provide finance for a short period of time until longer-term financing can be obtained. Interest cannot be charged, complying with Islamic/sharpie law. Instead, the loan is structured using discounts, sale or lease, profit participation, or repurchase agreements.
    #[serde(rename = "BRIDGE ISLAMIC")]
    BRIDGEISLAMIC,
    /// Event-driven loan intended to provide finance for a short period of time until longer-term financing can be obtained. Interest cannot be charged, complying with Islamic/sharpie law. Instead, the loan is structured using discounts, sale or lease, profit participation, or repurchase agreements. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "BRIDGE ISLAMIC TERM")]
    BRIDGEISLAMICTERM,
    /// Event-driven loan intended to provide finance for a short period of time until longer-term financing can be obtained. Interest may be capitalized and added to the principal amount of the loan, which is then compounded and due to lenders at maturity.
    #[serde(rename = "BRIDGE PIK")]
    BRIDGEPIK,
    /// Event-driven loan intended to provide finance for a short period of time until longer-term financing can be obtained. Interest may be capitalized and added to the principal amount of the loan, which is then compounded and due to lenders at maturity. Once borrowed funds are repaid they can be borrowed again.
    #[serde(rename = "BRIDGE PIK REV")]
    BRIDGEPIKREV,
    /// Event-driven loan intended to provide finance for a short period of time until longer-term financing can be obtained. Interest may be capitalized and added to the principal amount of the loan, which is then compounded and due to lenders at maturity. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "BRIDGE PIK TERM")]
    BRIDGEPIKTERM,
    /// Event-driven loan intended to provide finance for a short period of time until longer-term financing can be obtained. Once borrowed funds are repaid they can be borrowed again.
    #[serde(rename = "BRIDGE REV")]
    BRIDGEREV,
    /// Short-term financing issued by a bank to ensure/guarantee that an underlying project is completed. Once funds are borrowed, they can be borrowed again.
    #[serde(rename = "BRIDGE REV GUARANTEE FAC")]
    BRIDGEREVGUARANTEEFAC,
    #[serde(rename = "BRIDGE STANDBY TERM")]
    BRIDGESTANDBYTERM,
    /// Event-driven loan intended to provide finance for a short period of time until longer-term financing can be obtained. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "BRIDGE TERM")]
    BRIDGETERM,
    #[serde(rename = "BRIDGE TERM GUARANTEE FAC")]
    BRIDGETERMGUARANTEEFAC,
    #[serde(rename = "BRIDGE TERM VAT-TRNCH")]
    BRIDGETERMVATTRNCH,
    /// Event-driven loan intended to provide finance for a short period of time until longer-term financing can be obtained. Given as temporary reimbursement for a VAT credit it has with the state.
    #[serde(rename = "BRIDGE VAT-TRNCH")]
    BRIDGEVATTRNCH,
    /// Bulldog Securities. Foreign securities traded in the U.K. with face values and interest payments quoted in pounds sterling. As with Gilts, bulldogs follow an Actual/Actual day count convention.
    #[serde(rename = "BULLDOG")]
    BULLDOG,
    /// Quoted in basis points, butterfly swaps take the spread between 3 different interest rate swaps.
    #[serde(rename = "BUTTERFLY SWAP")]
    BUTTERFLYSWAP,
    /// Canadian Interest Bearing Commercial Paper
    #[serde(rename = "CAD INT BEAR CP")]
    CADINTBEARCP,
    #[serde(rename = "CALC_INSTRUMENT")]
    CALCINSTRUMENT,
    /// Option on a Calendar Spreads which consists of two futures contracts within the same instrument group and with different maturity months.
    #[serde(rename = "Calendar Spread Option")]
    CalendarSpreadOption,
    /// Call Loan. A loan usually made to a broker or specialist by a commercial bank with securities as collateral. Brokers may then loan the money to customers to finance margin activities. Most such loans can be terminated by either party on 24-hours notice and hence are said to involve call money. Also called Brokers'Loan.
    #[serde(rename = "CALL LOANS")]
    CALLLOANS,
    /// An unsecured short-term promise to repay a fixed amount on a certain future date. Commercial paper usually matures from 2 to 270 days and is traded on a discount basis. This debt instrument, issued by banks, companies and other borrowers, use only their credit ratings to back the security.
    #[serde(rename = "CALLABLE CP")]
    CALLABLECP,
    /// Market issue type where the issue is a Canadian issuer issuing in CAD into the Canadian domestic market.
    #[serde(rename = "CANADIAN")]
    CANADIAN,
    /// Market issue type where the issue is a Canadian issuer issuing in CAD into the Canadian domestic market.
    #[serde(rename = "Canadian")]
    Canadian,
    /// Canadian Certificate of Deposit - A debt instrument issued by a bank that will pay interest, periodically or at maturity (set by competitive forces in the marketplace), and principal when it reaches maturity. Maturities range from a few weeks to several years. It may have a fixed or floating rate of interest. Certificate of Deposit Rollover. An investment strategy used to defer taxes on income from a Certificate of Deposit. An individual who has purchased a certificate of deposit on margin expected to mature in the following year, can deduct the interest on the loan in the current year and move the income from the certificate to the up-coming year.
    #[serde(rename = "CANADIAN CD")]
    CANADIANCD,
    /// Canadian Commercial Paper
    #[serde(rename = "CANADIAN CP")]
    CANADIANCP,
    #[serde(rename = "Canadian DR")]
    CanadianDR,
    #[serde(rename = "CAPS & FLOORS")]
    CAPSANDFLOORS,
    /// Carry Forward. shares that show the accumulated and undivided profits of a corporation after provision has been made for dividends and reserves
    #[serde(rename = "Car Forward")]
    CarForward,
    /// Legal tender or coins that can be used in exchange goods, debt, or services. Sometimes also including the value of assets that can be converted into cash immediately, as reported by a company.
    #[serde(rename = "CASH")]
    CASH,
    #[serde(rename = "CASH FLOW")]
    CASHFLOW,
    #[serde(rename = "CASH FLOW, OID")]
    CASHFLOWOID,
    #[serde(rename = "CASH RATE")]
    CASHRATE,
    /// A money market instrument that represents an obligation between a borrower and a lender as to the terms and conditions of the loan. Collateralized borrowing and lending obligations (CBLOs) are used by those who have been phased out of or heavily restricted in the interbank call money market.
    #[serde(rename = "CBLO")]
    CBLO,
    /// Certificate of Deposit (CD). A debt instrument issued by a bank that will pay interest, periodically or at maturity (set by competitive forces in the marketplace), and principal when it reaches maturity. Maturities range from a few weeks to several years. It may have a fixed or floating rate of interest. Certificate of Deposit Rollover. An investment strategy used to defer taxes on income from a Certificate of Deposit. An individual who has purchased a certificate of deposit on margin expected to mature in the following year, can deduct the interest on the loan in the current year and move the income from the certificate to the up-coming year.
    #[serde(rename = "CD")]
    CD,
    /// Chess Depositary Interest
    #[serde(rename = "CDI")]
    CDI,
    /// Continental Depositary Receipt: Certificates issued by the Amsterdam Depository Company NV representing a specific number of shares of a security traded on an exchange of another country.
    #[serde(rename = "CDR")]
    CDR,
    #[serde(rename = "CEDEAR")]
    CEDEAR,
    /// Cash Flow (Only for CDO,CLO,CBO securities)
    #[serde(rename = "CF")]
    CF,
    #[serde(rename = "CHILEAN CD")]
    CHILEANCD,
    #[serde(rename = "CHILEAN DN")]
    CHILEANDN,
    /// A closed-end fund is a collective investment scheme with a limited number of shares. New shares are rarely issued after the fund is launched; shares are not normally redeemable for cash or securities until the fund liquidates. Typically an investor can acquire shares in a closed-end fund by buying shares on a secondary market from a broker, market maker, or other investor as opposed to an Open-end fund where all transactions eventually involve the fund company creating new shares on the fly (in exchange for either cash or securities) or redeeming shares (for cash or securities).
    #[serde(rename = "Closed-End Fund")]
    ClosedEndFund,
    /// Commercial Mortgage Backed Security: a mortgage backed by commercial property.
    #[serde(rename = "CMBS")]
    CMBS,
    /// Commodity Future Warrant - Indicates that the underlying is a commodity future.
    #[serde(rename = "Cmdt Fut WRT")]
    CmdtFutWRT,
    /// Commodity Index Warrant - Indicates that the underlying is a commodity index
    #[serde(rename = "Cmdt Idx WRT")]
    CmdtIdxWRT,
    /// Collateralized Call Notes
    #[serde(rename = "COLLAT CALL NOTE")]
    COLLATCALLNOTE,
    #[serde(rename = "COLOMBIAN CD")]
    COLOMBIANCD,
    /// Commercial Notes
    #[serde(rename = "COMMERCIAL NOTE")]
    COMMERCIALNOTE,
    /// An unsecured short-term promise to repay a fixed amount on a certain future date. Commercial paper usually matures from 2 to 270 days and is traded on a discount basis. This debt instrument, issued by banks, companies and other borrowers, use only their credit ratings to back the security.
    #[serde(rename = "COMMERCIAL PAPER")]
    COMMERCIALPAPER,
    #[serde(rename = "Commodity Index")]
    CommodityIndex,
    /// Common Stock. A unit of ownership in a public company for which the holder can vote on corporate matters and receive dividends fromthe company's growth. If the company is liquidated the claims of secured and unsecured creditors, bondholders and preferred stockholders take precedence over common stock holders. However, common stock has more potential for appreciation.
    #[serde(rename = "Common Stock")]
    CommonStock,
    /// Contract For Difference. An arrangement made in a futures contract whereby differences in settlement are made through cash payments, rather than the delivery of physical goods or securities.
    #[serde(rename = "CONTRACT FOR DIFFERENCE")]
    CONTRACTFORDIFFERENCE,
    /// IMM FRAs, FRAs where the payment dates align with Eurodollar future contracts' expiration date
    #[serde(rename = "CONTRACT FRA")]
    CONTRACTFRA,
    /// Convertible Bond. A bond containing a provision that permits conversion to the issuer's common stock at some fixed exchange ratio.
    #[serde(rename = "Conv Bond")]
    ConvBond,
    /// Convertible Preferred Stock. Preferred stock that includes an option for the holder to convert the preferred shares into a fixed number of common shares, usually anytime after a predetermined date.
    #[serde(rename = "Conv Prfd")]
    ConvPrfd,
    /// Corporate Bond Warrant - Indicates that the underlying is a corporate bond
    #[serde(rename = "Corp Bnd WRT")]
    CorpBndWRT,
    #[serde(rename = "Cover Pool")]
    CoverPool,
    /// Extendible Notes issues off of Commercial Paper program prospectus
    #[serde(rename = "CP-LIKE EXT NOTE")]
    CPLIKEEXTNOTE,
    /// A special type of floating rate bond that is linked to the Consumer Price Index (CPI)
    #[serde(rename = "CPI LINKED")]
    CPILINKED,
    /// Cross-rate: Currency against any other currency except US Dollar, eg EURCAD
    #[serde(rename = "CROSS")]
    CROSS,
    #[serde(rename = "Crypto")]
    Crypto,
    /// Currency Futures. Futures contracts on major currencies (e.g., the British pound, the German mark and the U.S. dollar) that are held by companies doing worldwide business in an effort to reduce their risks by hedging the value of their home country's currency.
    #[serde(rename = "Currency future.")]
    Currencyfuture,
    /// Option. A contract that provides the right, but not the obligation, to buy or sell a specific amount of a specific security within a predetermined time period. A call option provides the right to buy 100 shares of the underlying security. A put option provides the right to sell 100 shares of the underlying security. The seller of the contracts is called the writer. Bloomberg abbreviates option as OPT.
    #[serde(rename = "Currency option.")]
    Currencyoption,
    /// A currency transaction where one leg is the USD
    #[serde(rename = "Currency spot.")]
    Currencyspot,
    /// Currency Warrant - Indicates that the underlying is a currency
    #[serde(rename = "Currency WRT")]
    CurrencyWRT,
    #[serde(rename = "CURVE_ROLL")]
    CURVEROLL,
    /// Term loan where borrower is allowed to draw down on the loan in one or more drawings within a specified time period after the signing date. While the loan is technically effective from the signing date, availability is often subject to the completion of specific construction landmarks and/or the closing of a specific acquisition (note that this should not be confused with the Signed but not Effective status).
    #[serde(rename = "DELAY-DRAW")]
    DELAYDRAW,
    /// Borrower is allowed to draw down on the loan in one or more drawings within a specified time period after the signing date. Interest cannot be charged, complying with Islamic/sharpie law.
    #[serde(rename = "DELAY-DRAW ISLAMIC")]
    DELAYDRAWISLAMIC,
    /// Borrower is allowed to draw down on the loan in one or more drawings within a specified time period after the signing date. Interest cannot be charged, complying with Islamic/sharpie law. Guaranteed by the lender(s) and used for purchase of goods, completion of construction projects, and/or general purposes.
    #[serde(rename = "DELAY-DRAW ISLAMIC LOC")]
    DELAYDRAWISLAMICLOC,
    /// Borrower is allowed to draw down on the loan in one or more drawings within a specified time period after the signing date. Interest cannot be charged, complying with Islamic/sharpie law. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "DELAY-DRAW ISLAMIC TERM")]
    DELAYDRAWISLAMICTERM,
    #[serde(rename = "DELAY-DRAW LOC")]
    DELAYDRAWLOC,
    /// Borrower is allowed to draw down on the loan in one or more drawings within a specified time period after the signing date. Interest may be capitalized and added to the principal amount of the loan, which is then compounded and due to lenders at maturity. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "DELAY-DRAW PIK TERM")]
    DELAYDRAWPIKTERM,
    /// Borrower is allowed to draw down on the loan in one or more drawings within a specified time period after the signing date. Issued on a stand-by basis and borrowed only if and when needed. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "DELAY-DRAW STANDBY TERM")]
    DELAYDRAWSTANDBYTERM,
    /// Borrower is allowed to draw down on the loan in one or more drawings within a specified time period after the signing date. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "DELAY-DRAW TERM")]
    DELAYDRAWTERM,
    #[serde(rename = "DELAY-DRAW TERM GUARANTEE F")]
    DELAYDRAWTERMGUARANTEEF,
    /// Borrower is allowed to draw down on the loan in one or more drawings within a specified time period after the signing date. Once borrowed, cannot be re-borrowed. Given as temporary reimbursement for a VAT credit it has with the state.
    #[serde(rename = "DELAY-DRAW TERM VAT-TRNCH")]
    DELAYDRAWTERMVATTRNCH,
    /// The interest rate paid by financial institutions to deposit account holders
    #[serde(rename = "DEPOSIT")]
    DEPOSIT,
    /// Deposit Note. Deposit Notes are securities issued by a financial institution using the deposits of the institution as collateral. These securities carry the FDIC insurance.
    #[serde(rename = "DEPOSIT NOTE")]
    DEPOSITNOTE,
    /// Event-driven loan intended to provide finance for a short period of time until longer-term financing can be obtained. Denominated in Renminbi (RMB) currency, the lending happens outside Mainland China. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "DIM SUM BRIDGE TERM")]
    DIMSUMBRIDGETERM,
    /// Denominated in Renminbi (RMB) currency, the lending happens outside Mainland China. Borrower is allowed to draw down on the loan in one or more drawings within a specified time period after the signing date. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "DIM SUM DELAY-DRAW TERM")]
    DIMSUMDELAYDRAWTERM,
    /// Event-driven loan intended to provide finance for a short period of time until longer-term financing can be obtained. Denominated in Renminbi (RMB) currency, the lending happens outside Mainland China. Once borrowed, can be borrowed again.
    #[serde(rename = "DIM SUM REV")]
    DIMSUMREV,
    /// Denominated in Renminbi (RMB) currency, the lending happens outside Mainland China. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "DIM SUM TERM")]
    DIMSUMTERM,
    /// Loans issued when a company is in bankruptcy. These have a ranking of Super Priority Secured unless stated otherwise.
    #[serde(rename = "DIP")]
    DIP,
    /// Issued when a company is in bankruptcy. Once borrowed, cannot be re-borrowed. Interest cannot be charged, complying with Islamic/sharpie law.
    #[serde(rename = "DIP DELAY-DRAW ISLAMIC TERM")]
    DIPDELAYDRAWISLAMICTERM,
    /// Issued when a company is in bankruptcy. Interest may be capitalized and added to the principal amount of the loan, which is then compounded and due to lenders at maturity. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "DIP DELAY-DRAW PIK TERM")]
    DIPDELAYDRAWPIKTERM,
    /// Issued when a company is in bankruptcy. Borrower is allowed to draw down on the loan in one or more drawings within a specified time period after the signing date. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "DIP DELAY-DRAW TERM")]
    DIPDELAYDRAWTERM,
    /// Issued when a company is in bankruptcy. Guaranteed by the lender(s) and used for purchase of goods, completion of construction projects, and/or general purposes.
    #[serde(rename = "DIP LOC")]
    DIPLOC,
    /// Issued when a company is in bankruptcy. Interest may be capitalized and added to the principal amount of the loan, which is then compounded and due to lenders at maturity. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "DIP PIK TERM")]
    DIPPIKTERM,
    /// Issued when a company is in bankruptcy. Once borrowed funds are repaid they can be borrowed again.
    #[serde(rename = "DIP REV")]
    DIPREV,
    #[serde(rename = "DIP STANDBY LOC")]
    DIPSTANDBYLOC,
    /// Issued when a company is in bankruptcy. A letter of credit facility which is fully cash collateralized with proceeds of a term loan which has been issued by the same borrower, usually with the same amount and terms.
    #[serde(rename = "DIP SYNTH LOC")]
    DIPSYNTHLOC,
    /// Issued when a company is in bankruptcy. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "DIP TERM")]
    DIPTERM,
    /// Bank of Intl Settlement Discount Notes / Discount Rate
    #[serde(rename = "DISCOUNT FIXBIS")]
    DISCOUNTFIXBIS,
    /// Discount Notes - Short-term issues with no fixed offering schedule which are sold in $100,000 denominations with maturities of 30 to 270 days. Such notes are sold without government guarantees.
    #[serde(rename = "DISCOUNT NOTES")]
    DISCOUNTNOTES,
    /// Dividend Neutral Stock Futures (DNSF) are derivative instruments that give investors exposure to price movements of an underlying financial instrument (shares). A DNSF is a basket of two futures contracts, constructed from a Single Stock Futures Contract and a Dividend Futures Contract.
    #[serde(rename = "DIVIDEND NEUTRAL STOCK FUTURE")]
    DIVIDENDNEUTRALSTOCKFUTURE,
    /// Time Deposit. A deposit in an interest-paying account that requires the money to remain on account for a specific length of time. While withdrawals can generally be made from a passbook account at any time, other time deposits, such as certificates of deposit, are penalized for early withdrawal.
    #[serde(rename = "DOMESTC TIME DEP")]
    DOMESTCTIMEDEP,
    /// A security issued into a country's domestic market
    #[serde(rename = "DOMESTIC")]
    DOMESTIC,
    /// A medium term note issued into a country's domestic market.
    #[serde(rename = "DOMESTIC MTN")]
    DOMESTICMTN,
    /// Certificaten Van Aandelen. Netherland securities in the form of bearer certificates, issued by an administration office against the underlying original, mostly registered shares. The office acts as a trustee for the holders of the certificate and exercises voting rights on the shares.
    #[serde(rename = "Dutch Cert")]
    DutchCert,
    /// Commercial Paper. An unsecured short-term promise to repay a fixed amount on a certain future date. Commercial paper usually matures from 2 to 270 days and is traded on a discount basis. This debt instrument, issued by banks, companies and other borrowers, use only their credit ratings to back the security. Commercial Paper House. A dealer who buys commercial paper at one price, then tries to sell it at another. - DUTCH GUILDER
    #[serde(rename = "DUTCH CP")]
    DUTCHCP,
    /// European Depository Receipt. A document issued in place of stock shares that represents ownership of the shares, thus making it easier to deal in foreign securities because the actual stock certificates do not have to be physically transferred.
    #[serde(rename = "EDR")]
    EDR,
    /// Statistical movement of the stock market as represented by a basket of stocks.
    #[serde(rename = "Equity Index")]
    EquityIndex,
    /// Option. A contract that provides the right, but not the obligation, to buy or sell a specific amount of a specific security within a predetermined time period. A call option provides the right to buy 100 shares of the underlying security.A put option provides the right to sell 100 shares of theunderlying security. The seller of the contracts is called the writer. Bloomberg abbreviates option as OPT.
    #[serde(rename = "Equity Option")]
    EquityOption,
    /// Indicates that the underlying is an equity
    #[serde(rename = "Equity WRT")]
    EquityWRT,
    /// Exchange Traded Product is a derivatively-priced security which trades intra-day on a national stock exchange. ETPs are typically benchmarked to indices, stocks, commodities, or may be actively managed.
    #[serde(rename = "ETP")]
    ETP,
    /// Eurodollar Certificate of Deposit. A certificate of deposit issued by a bank outside the United States, with interest and principal paid in dollars.
    #[serde(rename = "EURO CD")]
    EUROCD,
    /// Euro Commercial Paper
    #[serde(rename = "EURO CP")]
    EUROCP,
    /// Euro Medium Term Note (EMTN). Continuously offered notes with maturities ranging from nine months to 40 years. Typically unsecured debt obligations, each euro medium term note issue (also known as a tranche) is a drawdown from a program (prospectus) level. EMTNs are flexible in structure and market timing because of this type of issuance (modeled from the commercial paper market). EMTNs are traded in the Euro market.
    #[serde(rename = "EURO MTN")]
    EUROMTN,
    /// Euro Non-Dollar. Any security issued by a U.S. company in a foreign country in a currency other than U.S. dollars.
    #[serde(rename = "EURO NON-DOLLAR")]
    EURONONDOLLAR,
    /// Euro Structured Loan Note
    #[serde(rename = "EURO STRUCTRD LN")]
    EUROSTRUCTRDLN,
    /// Time Deposit / Euro Market
    #[serde(rename = "EURO TIME DEPST")]
    EUROTIMEDEPST,
    /// Eurodollar Bonds. Bonds paying interest and principal in Eurodollars. Such bonds are not regulated by the Securities and Exchange Commission and can therefore be sold at interest rates that are lower than U.S. rates.
    #[serde(rename = "EURO-DOLLAR")]
    EURODOLLAR,
    /// Bonds issued by the countries that redenominated their debt from their legacy country to the Euro. Currency of the bond is in Euro.
    #[serde(rename = "EURO-ZONE")]
    EUROZONE,
    /// Extendible Commercial Notes
    #[serde(rename = "EXTEND COMM NOTE")]
    EXTENDCOMMNOTE,
    /// Extendible Note issued off of MTN type prospectus
    #[serde(rename = "EXTEND. NOTE MTN")]
    EXTENDNOTEMTN,
    /// Federal Deposit Insurance Corporation. A federal agency that insures deposits in member banks and thrifts up to $100,000.
    #[serde(rename = "FDIC")]
    FDIC,
    /// The Fed Funds type is used by banks to track the overnight borrowing they partake in to meet bank reserve requirements. Banks needs to have certain amount of cash on hand at close of every day, if they are short they will borrow from another bank for the night. This program type was established to help the banks manage this process.
    #[serde(rename = "FED FUNDS")]
    FEDFUNDS,
    /// This stands for Fundo De Investimento Em Direitos Creditorios in Portuguese
    #[serde(rename = "FIDC")]
    FIDC,
    /// Commodity Future. A contract to buy or sell a specific commodity at a specified price at a certain future date.
    #[serde(rename = "Financial commodity future.")]
    Financialcommodityfuture,
    /// Cash Settled (Financial) Generics on Commodity Futures. Generics are used to combine multiple futures contracts to build a generic history stream.
    #[serde(rename = "Financial commodity generic.")]
    Financialcommoditygeneric,
    /// Cash Settled (Financial) Options on Commodity Futures.
    #[serde(rename = "Financial commodity option.")]
    Financialcommodityoption,
    /// Cash Settled (Financial) Spot on Commodity Futures. The commodity will be delivered and settled immediately or in which a futures contract will expire in one month or less.
    #[serde(rename = "Financial commodity spot.")]
    Financialcommodityspot,
    /// Futures contracts that use indices as their base and theoretically settle by delivery of the underlying securities or commodities that make up the indices.
    #[serde(rename = "Financial index future.")]
    Financialindexfuture,
    /// Valid return for generic tickers.
    #[serde(rename = "Financial index generic.")]
    Financialindexgeneric,
    /// An option that is based on a stock index (performance measure of a group of stocks). Such options enable investors to trade in a particular market or industry group without having to buy individual stocks. Index options are issued by the Options Clearing Corporation.
    #[serde(rename = "Financial index option.")]
    Financialindexoption,
    /// Finland Domestic Certificates of Deposit
    #[serde(rename = "FINNISH CD")]
    FINNISHCD,
    /// Finland Domestic Commercial Paper
    #[serde(rename = "FINNISH CP")]
    FINNISHCP,
    /// The coupon stays constant through maturity.
    #[serde(rename = "FIXED")]
    FIXED,
    #[serde(rename = "Fixed Income Index")]
    FixedIncomeIndex,
    /// The coupon stays constant through maturity. The security is an Original Issue Discount.
    #[serde(rename = "FIXED, OID")]
    FIXEDOID,
    /// The rate set at a specific time
    #[serde(rename = "FIXING RATE")]
    FIXINGRATE,
    /// The coupon is based off an index or benchmark, and changes in value as the index changes.
    #[serde(rename = "FLOATING")]
    FLOATING,
    /// Floating Rate Commercial Paper
    #[serde(rename = "FLOATING CP")]
    FLOATINGCP,
    /// The coupon is based off an index or benchmark, and changes in value as the index changes. The security is an Original Issue Discount.
    #[serde(rename = "FLOATING, OID")]
    FLOATINGOID,
    #[serde(rename = "FNMA FHAVA")]
    FNMAFHAVA,
    /// Foreign Share is a common stock, but it is the foreign line of the security and it trades on the foreign board. The foreign board was set up as an alternative board for foreign investors seeking direct ownership of securities
    #[serde(rename = "Foreign Sh.")]
    ForeignSh,
    /// Forward. A financial instrument an investor sells for future delivery. This act is in direct violation of federal securities laws.
    #[serde(rename = "FORWARD")]
    FORWARD,
    /// Same as a forward but traded currencies do not follow spot conventions.
    #[serde(rename = "FORWARD CROSS")]
    FORWARDCROSS,
    /// The forward rates over different time periods
    #[serde(rename = "FORWARD CURVE")]
    FORWARDCURVE,
    /// Forward rate agreements are common OTC financial derivatives in which the buyer or seller will be compensated based upon the difference between an agreed upon future interest rate level and the realized interest rate. The contract will determine the rates to be used along with the termination date and notional value. On this type of agreement, it is only the differential that is paid on the notional amount of the contract.
    #[serde(rename = "FRA")]
    FRA,
    /// French Domestic Certificates of Deposit
    #[serde(rename = "FRENCH CD")]
    FRENCHCD,
    /// French Certificate - Depositary Receipt issued in France
    #[serde(rename = "French Cert")]
    FrenchCert,
    /// French Domestic Commercial Paper
    #[serde(rename = "FRENCH CP")]
    FRENCHCP,
    /// A mutual fund that invests in other mutual funds.
    #[serde(rename = "Fund of Funds")]
    FundofFunds,
    #[serde(rename = "Futures Monthly Ticker")]
    FuturesMonthlyTicker,
    /// Forward starting swap, swap starting in a future date
    #[serde(rename = "FWD SWAP")]
    FWDSWAP,
    #[serde(rename = "FX Curve")]
    FXCurve,
    /// Setup for Fixed Rate Agency Discount Notes
    #[serde(rename = "FX DISCOUNT NOTE")]
    FXDISCOUNTNOTE,
    /// Global Depository Receipts. (GDR) A negotiable certificate held in the bank of one country representing a specific number of shares of a stock traded on an exchange of another country.
    #[serde(rename = "GDR")]
    GDR,
    /// Futures contracts on major currencies
    #[serde(rename = "Generic currency future.")]
    Genericcurrencyfuture,
    /// Generics on Index Futures. Generics are used to combine multiple futures contracts to build a generic history stream.
    #[serde(rename = "Generic index future.")]
    Genericindexfuture,
    /// Also known as Deutsches Zertifikat (German certificate). The DTZ identifies deposit receipts issued by German banks evidencing the ownership of foreign securities.
    #[serde(rename = "German Cert")]
    GermanCert,
    /// German Domestic Commercial Paper
    #[serde(rename = "GERMAN CP")]
    GERMANCP,
    /// Global Bond. A certificate representing the total debt of an issue. Such bonds are created to control the primary market distribution of an issue in compliance with selling restrictions in certain jurisdictions or because definitive bond certificates are not available. Also known as global certificate.
    #[serde(rename = "GLOBAL")]
    GLOBAL,
    /// Also known as a performance bond/guarantee, is a contingent liability which is issued by a bank to ensure/guarantee that an underlying project is completed. Similar to a letter of credit.
    #[serde(rename = "GUARANTEE FAC")]
    GUARANTEEFAC,
    /// Hybrid (Only for CDO,CLO,CBO securities)
    #[serde(rename = "HB")]
    HB,
    /// Hong Kong Depository Receipt
    #[serde(rename = "HDR")]
    HDR,
    /// Hong Kong Certificates of Deposit
    #[serde(rename = "HONG KONG CD")]
    HONGKONGCD,
    /// Indicates that the underlying is an interest rate future
    #[serde(rename = "I.R. Fut WRT")]
    IRFutWRT,
    /// Indicates that the underlying is an interest rate swap
    #[serde(rename = "I.R. Swp WRT")]
    IRSwpWRT,
    /// International Depository Receipt. A receipt given for a foreign corporation's share certificates.
    #[serde(rename = "IDR")]
    IDR,
    /// Represents an FX forward contract with a maturity on an IMM date
    #[serde(rename = "IMM FORWARD")]
    IMMFORWARD,
    /// Forward swap rates with payment dates that align with Eurodollar future contracts' expiration dates.
    #[serde(rename = "IMM SWAP")]
    IMMSWAP,
    /// Index. A compilation of statistical data that tracks changes in the economy or in financial markets.
    #[serde(rename = "Index")]
    Index,
    /// Option on Index Futures. Investors trading index options are essentially betting on the overall movement of the stock market as represented by a basket of stocks. Options on the S and P 500 are some of the most actively traded options in the world.
    #[serde(rename = "Index Option")]
    IndexOption,
    /// Index Warrant - Indicates that the underlying is an index
    #[serde(rename = "Index WRT")]
    IndexWRT,
    /// Indian Certificates of Deposit
    #[serde(rename = "INDIAN CD")]
    INDIANCD,
    /// Indian Commercial Paper
    #[serde(rename = "INDIAN CP")]
    INDIANCP,
    /// Indonesian Commercial Paper
    #[serde(rename = "INDONESIAN CP")]
    INDONESIANCP,
    /// Indicates that the underlying is an index future
    #[serde(rename = "Indx Fut WRT")]
    IndxFutWRT,
    /// A derivative used to transfer inflation risk from one party to another through an exchange of cash flows.
    #[serde(rename = "INFLATION SWAP")]
    INFLATIONSWAP,
    /// Bank of Intl Settlement Discount Notes / Interest Bearing Coupon Rate
    #[serde(rename = "INT BEAR FIXBIS")]
    INTBEARFIXBIS,
    /// Interest Rate Warrant - Indicates that the underlying is an interest rate
    #[serde(rename = "Int. Rt. WRT")]
    IntRtWRT,
    /// Also called Convertible Capital Appreciate Bonds. The bond starts as a Zero Coupon, then converts at some predetermined date to a fixed rate or current interest bond.
    #[serde(rename = "INTER. APPRECIATION")]
    INTERAPPRECIATION,
    /// Also called Convertible Capital Appreciate Bonds. The bond starts as a Zero Coupon, then converts at some predetermined date to a fixed rate or current interest bond.
    #[serde(rename = "INTER. APPRECIATION, OID")]
    INTERAPPRECIATIONOID,
    /// A type of loan that interest cannot be charged on, complying with Islamic/sharpie law. Instead, the loan is structured using discounts, sale or lease, profit participation, or repurchase agreements. Lenders are generally referred to as depositors. Islamic loans are further classified into sub-categories such as ljara, Murabaha, Mudarabah, etc.
    #[serde(rename = "ISLAMIC")]
    ISLAMIC,
    /// Islamic Bankers Acceptances
    #[serde(rename = "ISLAMIC BA")]
    ISLAMICBA,
    /// Islamic Commercial Paper
    #[serde(rename = "ISLAMIC CP")]
    ISLAMICCP,
    #[serde(rename = "ISLAMIC GUARANTEE FAC")]
    ISLAMICGUARANTEEFAC,
    /// Interest cannot be charged, complying with Islamic/sharpie law. Guaranteed by the lender(s) and used for purchase of goods, completion of construction projects, and/or general purposes.
    #[serde(rename = "ISLAMIC LOC")]
    ISLAMICLOC,
    /// Interest cannot be charged, complying with Islamic/sharpie law. Instead, the loan is structured using discounts, sale or lease, profit participation, or repurchase agreements. Once borrowed funds are repaid they can be borrowed again.
    #[serde(rename = "ISLAMIC REV")]
    ISLAMICREV,
    /// Interest cannot be charged, complying with Islamic/sharpie law. Instead, the loan is structured using discounts, sale or lease, profit participation, or repurchase agreements. Issued on a stand-by basis and borrowed only if and when needed.
    #[serde(rename = "ISLAMIC STANDBY")]
    ISLAMICSTANDBY,
    /// Interest cannot be charged, complying with Islamic/sharpie law. Instead, the loan is structured using discounts, sale or lease, profit participation, or repurchase agreements. Issued on a stand-by basis and borrowed only if and when needed. Once borrowed, cannot be borrowed again.
    #[serde(rename = "ISLAMIC STANDBY REV")]
    ISLAMICSTANDBYREV,
    #[serde(rename = "ISLAMIC STANDBY TERM")]
    ISLAMICSTANDBYTERM,
    /// Interest cannot be charged, complying with Islamic/sharpie law. Instead, the loan is structured using discounts, sale or lease, profit participation, or repurchase agreements. Once borrowed, cannot be borrowed again.
    #[serde(rename = "ISLAMIC TERM")]
    ISLAMICTERM,
    #[serde(rename = "ISLAMIC TERM GUARANTEE FAC")]
    ISLAMICTERMGUARANTEEFAC,
    #[serde(rename = "ISLAMIC TERM VAT-TRNCH")]
    ISLAMICTERMVATTRNCH,
    /// Jumbo Certificate of Deposit. A certificate of deposit with a denomination of at least $100,000, usually purchased by institutional investors.
    #[serde(rename = "JUMBO CD")]
    JUMBOCD,
    /// Korean Certificates of Deposit
    #[serde(rename = "KOREAN CD")]
    KOREANCD,
    /// Korean Commercial Paper
    #[serde(rename = "KOREAN CP")]
    KOREANCP,
    /// Lebanese Commercial Paper
    #[serde(rename = "LEBANESE CP")]
    LEBANESECP,
    /// Liquidity Note - asset backed, cp-like extendible instruments
    #[serde(rename = "LIQUIDITY NOTE")]
    LIQUIDITYNOTE,
    /// A financial obligation of the borrower which has been guaranteed by the lender(s). There are different types of Letters of Credit (L/Cs) used for various purposes. Trade and Commercial L/Cs are used for purchases of goods. Standby and Documentary L/Cs are used for the completion of construction projects and/or general purposes.
    #[serde(rename = "LOC")]
    LOC,
    /// A financial obligation of the borrower which has been guaranteed by the lender(s) to ensure a project is completed.
    #[serde(rename = "LOC GUARANTEE FAC")]
    LOCGUARANTEEFAC,
    #[serde(rename = "LOC TERM")]
    LOCTERM,
    /// Limited Partnership. A partnership with at least one of the partners holding only a limited liability.
    #[serde(rename = "Ltd Part")]
    LtdPart,
    /// Malaysian Commercial Paper
    #[serde(rename = "MALAYSIAN CP")]
    MALAYSIANCP,
    /// An investment account that is owned by anindividual investorand looked after by a hired professional money manager. In contrast to mutual funds (which are professionally managed on behalf of many mutual-fund holders), managed accounts are personalizedinvestment portfolios tailored to the specific needs of the account holder.
    #[serde(rename = "Managed Account")]
    ManagedAccount,
    /// Margin Term Deposits
    #[serde(rename = "MARGIN TERM DEP")]
    MARGINTERMDEP,
    /// MMKT instrument issued by large, creditworthy companies to banks.
    #[serde(rename = "MASTER NOTES")]
    MASTERNOTES,
    /// Mortgage-Backed Security (MBS). A debt instrument with a pool of real estate loans as the underlying collateral. The mortgage payments of the individual real estate assets are used to pay interest and principal on the bonds. - 10 years
    #[serde(rename = "MBS 10yr")]
    MBS10yr,
    /// Mortgage-Backed Security (MBS). A debt instrument with a pool of real estate loans as the underlying collateral. The mortgage payments of the individual real estate assets are used to pay interest and principal on the bonds. - 15 years
    #[serde(rename = "MBS 15yr")]
    MBS15yr,
    /// Mortgage-Backed Security (MBS). A debt instrument with a pool of real estate loans as the underlying collateral. The mortgage payments of the individual real estate assets are used to pay interest and principal on the bonds. - 20 years
    #[serde(rename = "MBS 20yr")]
    MBS20yr,
    /// Mortgage-Backed Security (MBS). A debt instrument with a pool of real estate loans as the underlying collateral. The mortgage payments of the individual real estate assets are used to pay interest and principal on the bonds. - 30 years
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
    /// Mortgage-Backed Security (MBS). A debt instrument with a pool of real estate loans as the underlying collateral. The mortgage payments of the individual real estate assets are used to pay interest and principal on the bonds.
    #[serde(rename = "MBS ARM")]
    MBSARM,
    /// Balloon. The final payment on a loan or mortgage that is much larger than previous payments. Some debts are structured with balloon payments to account for events such as increased interest rates during the life of the loan.
    #[serde(rename = "MBS balloon")]
    MBSballoon,
    /// Description for Agency Pools (Fannie, Freddie, Ginnie) that are unable to be mapped to a generic security.
    #[serde(rename = "MBS Other")]
    MBSOther,
    /// Medium Term Note (MTN). Continuously offered notes with maturities ranging from nine months to 40 years. Typically unsecured debt obligations, each medium term note issue (also known as a tranche) is a drawdown from a program (prospectus) level. MTNs are flexible in structure and market timing because of this type of issuance (modeled from the commercial paper market).
    #[serde(rename = "MED TERM NOTE")]
    MEDTERMNOTE,
    /// Medium Term Certificates of Deposit - medium term maturities
    #[serde(rename = "MEDIUM TERM CD")]
    MEDIUMTERMCD,
    /// Medium Term Euro Certificates of Deposit - medium term maturities
    #[serde(rename = "MEDIUM TERM ECD")]
    MEDIUMTERMECD,
    /// Mexican Commercial Paper
    #[serde(rename = "MEXICAN CP")]
    MEXICANCP,
    /// Mexican Short Term MMKT instrument
    #[serde(rename = "MEXICAN PAGARE")]
    MEXICANPAGARE,
    #[serde(rename = "Misc.")]
    Misc,
    /// Master Limited Partnership: A type of limited partnership that is publicly traded. There are two types of partners in this type of partnership: The limited partner is the person or group that provides the capital to the MLP and receives periodic income distributions from the MLP's cash flow, whereas the general partner is the party responsible for managing the MLP's affairs and receives compensation that is linked to the performance of the venture.
    #[serde(rename = "MLP")]
    MLP,
    #[serde(rename = "MONETARY BILLS")]
    MONETARYBILLS,
    /// A short-term money market, which allows for large financial institutions, such as banks, mutual funds and corporations to borrow and lend money at interbank rates
    #[serde(rename = "MONEY MARKET CALL")]
    MONEYMARKETCALL,
    /// Commercial Paper. An unsecured short-term promise to repay a fixed amount on a certain future date. Commercial paper usually matures from 2 to 270 days and is traded on a discount basis. This debt instrument, issued by banks, companies and other borrowers, use only their credit ratings to back the security. Commercial Paper House. A dealer who buys commercial paper at one price, then tries to sell it at another.
    #[serde(rename = "MUNI CP")]
    MUNICP,
    /// Municipal Commercial Paper / Interest Bearing
    #[serde(rename = "MUNI INT BEAR CP")]
    MUNIINTBEARCP,
    /// A Municipal Swap is a swap that exchanges a municipal bond index with a funding leg
    #[serde(rename = "MUNI SWAP")]
    MUNISWAP,
    /// An Islamic financing structure, where an intermediary buys a property with free and clear title to it. The intermediary and prospective buyer then agree upon a sale price (including an agreed upon profit for the intermediary) that can be made through a series of installments, or as a lump sum payment.
    #[serde(rename = "MURABAHA")]
    MURABAHA,
    /// Mutual Fund. An investment company that pools individual, institutional and other investors' money and invests it in a variety of securities or markets. Shares can be redeemed at Net Asset Value (NAV). The funds offer investors diversification and professional management for a yearly management fee.
    #[serde(rename = "Mutual Fund")]
    MutualFund,
    /// Market Value (Only for CDO,CLO,CBO securities)
    #[serde(rename = "MV")]
    MV,
    /// Short term Mexican Mmkt instrument
    #[serde(rename = "MX CERT BURSATIL")]
    MXCERTBURSATIL,
    /// A swap that is similar to a non-deliverable forward, with the only difference being that settlement for both parties is done through a major currency. Non-deliverable swaps are used when the swap includes a major currency, such as the U.S. dollar, and a restricted currency, such as the Philippine peso or South Korean won.
    #[serde(rename = "NDF SWAP")]
    NDFSWAP,
    #[serde(rename = "NEG EURO CP")]
    NEGEUROCP,
    /// Negotiable Institutional Deposit
    #[serde(rename = "NEG INST DEPOSIT")]
    NEGINSTDEPOSIT,
    /// Negotiable Certificate of Deposit. A negotiable money market instrument that trades on the open market with high returns and low risks. These instruments are typically large-dollar denominated and are held by large institutional investors. Alsoknown as jumbo CD's
    #[serde(rename = "NEGOTIABLE CD")]
    NEGOTIABLECD,
    #[serde(rename = "NEW ZEALAND CD")]
    NEWZEALANDCD,
    /// New Zealand Commercial Paper
    #[serde(rename = "NEW ZEALAND CP")]
    NEWZEALANDCP,
    /// Non-deliverable forwards are a type of cash settled financial derivative. In the foreign exchange market, traders and investors will enter into an outright forward or futures contract where counterparties agree to settle based upon the difference between a strike price and the prevailing spot rate for a specified notional amount on a defined fixing date in the future.
    #[serde(rename = "NON-DELIVERABLE FORWARD")]
    NONDELIVERABLEFORWARD,
    /// Denominated in a currency on a non-deliverable basis, but settled in another currency that is deliverable
    #[serde(rename = "NON-DELIVERABLE IRS SWAP")]
    NONDELIVERABLEIRSSWAP,
    /// Thailand Non Voting Depositary Receipt.
    #[serde(rename = "NVDR")]
    NVDR,
    /// New York Registered Shares. A foreign company's shares that are registered through their home office in New York.
    #[serde(rename = "NY Reg Shrs")]
    NYRegShrs,
    /// Original Issue Discount (OID). A bond with its par value discounted at the time it is issued. The difference between the purchase price and the adjusted price is considered income in addition to any interest that may be paid. If held to maturity, no capital gains tax will be paid since the gain is considered interest.
    #[serde(rename = "OID")]
    OID,
    /// Onshore forward foreign exchange transactions occur in controlled domestic markets and involve the purchase of a specified amount of one currency and selling of another on an agreed future date. Onshore forward rates are not necessarily determined by an arbitrage free relationship between the interest rates of the two currencies and the current spot rate.
    #[serde(rename = "ONSHORE FORWARD")]
    ONSHOREFORWARD,
    /// The Swap can only be traded in the local currency in the local market (ex: KRW onshore swaps)
    #[serde(rename = "ONSHORE SWAP")]
    ONSHORESWAP,
    /// A fund that allows investors to buy and sell stock in it on an on-going basis since it has unlimited shares. It continuously issues and redeems shares so the number of shares issued and outstanding varies from day to day.
    #[serde(rename = "Open-End Fund")]
    OpenEndFund,
    /// A contract that provides the right, but not the obligation, to buy or sell a specific amount of a specific security within a predetermined time period.
    #[serde(rename = "OPTION")]
    OPTION,
    #[serde(rename = "Option on Equity Future")]
    OptiononEquityFuture,
    /// A measure of the variation of the price of the underlying asset
    #[serde(rename = "OPTION VOLATILITY")]
    OPTIONVOLATILITY,
    #[serde(rename = "OTHER")]
    OTHER,
    /// Used for ticketing Overnight transactions
    #[serde(rename = "OVER/NIGHT")]
    OVERNIGHT,
    #[serde(rename = "OVERDRAFT")]
    OVERDRAFT,
    /// An overnight index swap uses an overnight rate index, such as the Federal Funds Rate, as the underlying for its floating leg.
    #[serde(rename = "OVERNIGHT INDEXED SWAP")]
    OVERNIGHTINDEXEDSWAP,
    #[serde(rename = "PANAMANIAN CP")]
    PANAMANIANCP,
    #[serde(rename = "Participate Cert")]
    ParticipateCert,
    /// Philippino Commercial Paper
    #[serde(rename = "PHILIPPINE CP")]
    PHILIPPINECP,
    /// The amount of a specified underlying asset of a contract that is physically delivered by the seller of the contract to the exchange, and by the exchange to the buyers of the contract.
    #[serde(rename = "Physical commodity forward.")]
    Physicalcommodityforward,
    /// Commodity Future. A contract to buy or sell a specific commodity a ta specified price at a certain future date.
    #[serde(rename = "Physical commodity future.")]
    Physicalcommodityfuture,
    /// Physically Deliverable (Physical) Generics on Commodity Futures. Generics are used to combine multiple futures contracts to build a generic history stream.
    #[serde(rename = "Physical commodity generic.")]
    Physicalcommoditygeneric,
    /// Physical Commodity. The actual, tangible commodity, such as corn or soybeans, which a seller delivers to the buyer of a commodities contract.
    #[serde(rename = "Physical commodity option.")]
    Physicalcommodityoption,
    /// Physically Deliverable (Physical) Spot on Commodity Futures. Spot is the actual physical product on which a futures contract is based.
    #[serde(rename = "Physical commodity spot.")]
    Physicalcommodityspot,
    /// Stock Index Future. A futures contract that uses a stock index as its base, and which is settled by delivery of the underlying stocks.
    #[serde(rename = "Physical index future.")]
    Physicalindexfuture,
    /// Index Options. An option that is based on a stock index (performance measure of a group of stocks). Such options enable investors to trade in a particular market or industry group without having to buy individual stocks. Index options are issued by the Options Clearing Corporation.
    #[serde(rename = "Physical index option.")]
    Physicalindexoption,
    /// A loan whereby interest may be capitalized and added to the principal amount of the loan, which is then compounded and the aggregated amount of which will be due to lenders at maturity. PIK loans are usually in the form of a term loan.
    #[serde(rename = "PIK")]
    PIK,
    #[serde(rename = "PIK LOC")]
    PIKLOC,
    /// A loan whereby interest may be capitalized and added to the principal amount of the loan, which is then compounded and the aggregated amount of which will be due to lenders at maturity. Once borrowed, can be borrowed again.
    #[serde(rename = "PIK REV")]
    PIKREV,
    /// A loan whereby interest may be capitalized and added to the principal amount of the loan, which is then compounded and the aggregated amount of which will be due to lenders at maturity. Fully cash collateralized with proceeds of a term loan which has been issued by the same borrower, usually with the same amount and terms.
    #[serde(rename = "PIK SYNTH LOC")]
    PIKSYNTHLOC,
    /// A loan whereby interest may be capitalized and added to the principal amount of the loan, which is then compounded and the aggregated amount of which will be due to lenders at maturity. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "PIK TERM")]
    PIKTERM,
    /// Argentinian MMkt instrument
    #[serde(rename = "PLAZOS FIJOS")]
    PLAZOSFIJOS,
    /// Portugal Domestic Commercial Paper
    #[serde(rename = "PORTUGUESE CP")]
    PORTUGUESECP,
    /// Preference Stock. A special type of common stock that shares in the earnings of the company, has limited voting rights, and may have a dividend preference. Preference shares may also have liquidation preference.
    #[serde(rename = "Preference")]
    Preference,
    /// Preferred Stock. Stock shares that represent a portion of ownership in a company, with the shares normally carrying fixed dividends. Sometimes the shares have voting rights, but not generally.
    #[serde(rename = "Preferred")]
    Preferred,
    /// Not used
    #[serde(rename = "PRES")]
    PRES,
    /// Indicates that the underlying is a preferred security
    #[serde(rename = "Prfd WRT")]
    PrfdWRT,
    /// Private Placement. A large block of securities offered for sale to an institutional investor or a small number of investors through private negotiations, as opposed to the securities being acquired in a public offering. Such placements are not registered with the SEC.
    #[serde(rename = "PRIV PLACEMENT")]
    PRIVPLACEMENT,
    /// a preferred security issued as a private placement (not registered with SEC). This also includes 144A securities that typically get exchanged into public securities once the company has the documentation ready for the SEC. 144A securities allow issuing entities to obtain their money be selling the securities and then typically have 90-180 days to file the correct documentation with the SEC.
    #[serde(rename = "PRIVATE")]
    PRIVATE,
    /// Private Company: A company whose ownership is private. As a result, it does not need to meet the strict Securities and Exchange Commission filing requirements of public companies. Private companies may issue stock and have shareholders. However, their shares do not trade on public exchanges and are not issued through an initial public offering. In general, the shares of these businesses are less liquid and the values are difficult to determine.
    #[serde(rename = "Private Comp")]
    PrivateComp,
    #[serde(rename = "Private-equity backed")]
    Privateequitybacked,
    /// Promissory Note. A written commitment to pay another party a specific amount of money by a specific date. A note, for example, is a promissory note.
    #[serde(rename = "PROMISSORY NOTE")]
    PROMISSORYNOTE,
    /// Commercial Paper issued by Canadian Provinces
    #[serde(rename = "PROV T-BILL")]
    PROVTBILL,
    #[serde(rename = "Prvt CMBS")]
    PrvtCMBS,
    /// Privately Issued CMO Floating Rate Security
    #[serde(rename = "Prvt CMO FLT")]
    PrvtCMOFLT,
    /// Privately Issued CMO Inverse Floating Rate Security
    #[serde(rename = "Prvt CMO INV")]
    PrvtCMOINV,
    /// Privately Issued CMO Interest Only Security
    #[serde(rename = "Prvt CMO IO")]
    PrvtCMOIO,
    /// Privately Issued CMO Other type (Less common type of security such as PAC or Sequential)
    #[serde(rename = "Prvt CMO Other")]
    PrvtCMOOther,
    /// Privately Issued CMO Principal Only Security
    #[serde(rename = "Prvt CMO PO")]
    PrvtCMOPO,
    /// Privately Issued CMO Z (Accrual Bond)
    #[serde(rename = "Prvt CMO Z")]
    PrvtCMOZ,
    /// non private issue
    #[serde(rename = "PUBLIC")]
    PUBLIC,
    /// A private equity fund is a collective investment scheme used for making investments in various equity (and to a lesser extent debt) securities according to one of the investment strategies associated with private equity. Private equity funds are typically limited partnerships with a fixed term of 10 years (often with annual extensions). At inception, institutional investors make an unfunded commitment to the limited partnership, which is then drawn over the term of the fund. From investors point of view funds can be traditional where all the investors invest with equal terms or asymmetric where different investors have different terms.
    #[serde(rename = "Pvt Eqty Fund")]
    PvtEqtyFund,
    /// Russian Depositary Certificate. The Russian Depositary Certificates represent fractional undivided interests in Sub-Trusts of the Russian Depositary.
    #[serde(rename = "RDC")]
    RDC,
    /// Certificates issued by a bank, representing shares held by the bank, usually by a branch or correspondent in the country of issue.
    #[serde(rename = "Receipt")]
    Receipt,
    /// Real Estate Investment Trust (REIT). A corporation or business trust which owns, manages and/or leases commercial real estate properties, and/or invests in Real Estate related securities, such as mortgaged-backed securities or whole loans and when meeting certain tax code requirements, is exempt at the entity level from corporate income tax. A creation of Congress, REIT legislation was initially passed in 1960 and was amended in 1986 to enhance and increase ownership and management focus on the underlying asset operations. REITs are exempt from federal (and usually state) corporate income taxation, subject to meeting certain IRS requirements for real estate investment and ownership, real estate income and dividend levels, i.e., dividend payout requirements for REITs require that 95% of taxable income be paid as a dividend.
    #[serde(rename = "REIT")]
    REIT,
    /// A form of short-term borrowing for dealers in government securities. The dealer sells the government securities to investors, usually on an overnight basis, and buys them back the following day.
    #[serde(rename = "REPO")]
    REPO,
    #[serde(rename = "RESERVE-BASED DIP REV")]
    RESERVEBASEDDIPREV,
    #[serde(rename = "RESERVE-BASED REV")]
    RESERVEBASEDREV,
    #[serde(rename = "RESERVE-BASED TERM")]
    RESERVEBASEDTERM,
    /// Bonds that came out of the Brady plan in the 1980's that addressed the LDC debt crisis.
    #[serde(rename = "RESTRUCTURD DEBT")]
    RESTRUCTURDDEBT,
    /// Certificates of Deposit sold to Retail customers
    #[serde(rename = "RETAIL CD")]
    RETAILCD,
    /// Not applicable to currencies
    #[serde(rename = "RETURN IDX")]
    RETURNIDX,
    /// Funds may be borrowed, repaid, and then borrowed again throughout the life of the loan. (Resembles a credit card).
    #[serde(rename = "REV")]
    REV,
    /// Funds may be borrowed, repaid, and then borrowed again throughout the life of the loan. Contingent liability which is issued by a bank to ensure/guarantee that an underlying project is completed.
    #[serde(rename = "REV GUARANTEE FAC")]
    REVGUARANTEEFAC,
    /// Funds may be borrowed, repaid, and then borrowed again throughout the life of the loan. Given as temporary reimbursement for a VAT credit it has with the state.
    #[serde(rename = "REV VAT-TRNCH")]
    REVVATTRNCH,
    #[serde(rename = "Revolver")]
    Revolver,
    /// A security giving stockholders entitlement to purchase new shares issued by the corporation at a predetermined price (normally less than the current market price) in proportion to the number of shares already owned. Rights are issued only for a short period of time, after which they expire.
    #[serde(rename = "Right")]
    Right,
    /// Royalty Trust. An oil or gas company spins off property to its shareholders, which means it will not be taxed at the corporate level and will offer high returns to stockholders.
    #[serde(rename = "Royalty Trst")]
    RoyaltyTrst,
    /// Short Term Loan Notes
    #[serde(rename = "S.TERM LOAN NOTE")]
    STERMLOANNOTE,
    /// Samurai Bond. A debt instrument denominated in Japanese yen, but not issued by a Japanese agency or company.
    #[serde(rename = "SAMURAI")]
    SAMURAI,
    /// Savings Plan is a general category which groups variable annuity, collective investment trusts and separately managed accounts. Securities categorized under collective investment trusts and separately managed accounts fund types are for Bloombergs internal use.
    #[serde(rename = "Savings Plan")]
    SavingsPlan,
    #[serde(rename = "Savings Share")]
    SavingsShare,
    /// Agency Pool issued by Small Business Association
    #[serde(rename = "SBA Pool")]
    SBAPool,
    /// Swedish Depository Receipt
    #[serde(rename = "SDR")]
    SDR,
    /// Security Lending. aka stock lending. Refers to the lending of securities by one party to another. The terms of the loan will be governed by a Securities Lending Agreement, which, under U.S. law, requires that the borrower provides the lender with collateral, in the form of cash, government securities, or a Letter of Credit of value equal to or greater than the loaned securities. As payment for the loan, the parties negotiate a fee, quoted as an annualized percentage of the value of the loaned securities. If the agreed form of collateral is cash, then the fee may be quoted as a rebate, meaning that the lender will earn all of the interest which accrues on the cash collateral, and will rebate an agreed rate of interest to the borrower.
    #[serde(rename = "Sec Lending")]
    SecLending,
    /// Shogun Security. A foreign corporation-issued, non-yen denominated security distributed in Japan. Also known as a geisha bond.
    #[serde(rename = "SHOGUN")]
    SHOGUN,
    /// Short Term Bank Notes
    #[serde(rename = "SHORT TERM BN")]
    SHORTTERMBN,
    /// Short Term Deposit Notes- Short-term deposit notes issued directly by the bank. These deposit notes are issued for the bank's own general funding purposes
    #[serde(rename = "SHORT TERM DN")]
    SHORTTERMDN,
    /// Singapore Commercial Paper
    #[serde(rename = "SINGAPORE CP")]
    SINGAPORECP,
    #[serde(rename = "Singapore DR")]
    SingaporeDR,
    /// Single Stock Dividend Future. A dividend future is a derivative contract that allows investors to take positions on future dividend payments.
    #[serde(rename = "SINGLE STOCK DIVIDEND FUTURE")]
    SINGLESTOCKDIVIDENDFUTURE,
    /// Single Stock Forward. A forward contract is a non-standardized contract between two parties where the price is agreed at the initial trade date. These contracts are not exchange listed.
    #[serde(rename = "SINGLE STOCK FORWARD")]
    SINGLESTOCKFORWARD,
    /// All standard equity futures are single stock futures or SSFs. Other possible returns are SSDF, Single Stock Dividend Future, DNSF, Dividend Neutral Stock Future, and CFD, Contract For Difference. These returns only apply to the equity button.
    #[serde(rename = "SINGLE STOCK FUTURE")]
    SINGLESTOCKFUTURE,
    #[serde(rename = "SINGLE STOCK FUTURE SPREAD")]
    SINGLESTOCKFUTURESPREAD,
    /// Structured Note
    #[serde(rename = "SN")]
    SN,
    #[serde(rename = "SPANISH CP")]
    SPANISHCP,
    /// Auxiliary program type setup for linking securities to mmkt ticker
    #[serde(rename = "SPECIAL LMMK PGM")]
    SPECIALLMMKPGM,
    /// Spot. Available for immediate delivery
    #[serde(rename = "SPOT")]
    SPOT,
    /// Statistical movement of the underlying security as represented by a basket of spot traded securities.
    #[serde(rename = "Spot index.")]
    Spotindex,
    /// A financial commitment issued on a stand-by basis and borrowed only if and when needed. It is not used alone as a loan type.
    #[serde(rename = "STANDBY")]
    STANDBY,
    /// A financial commitment issued on a stand-by basis and borrowed only if and when needed. Guaranteed by the lender(s) and used for purchase of goods, completion of construction projects, and/or general purposes.
    #[serde(rename = "STANDBY LOC")]
    STANDBYLOC,
    #[serde(rename = "STANDBY LOC GUARANTEE FAC")]
    STANDBYLOCGUARANTEEFAC,
    /// A financial commitment issued on a stand-by basis and borrowed only if and when needed. Once borrowed, can be borrowed again.
    #[serde(rename = "STANDBY REV")]
    STANDBYREV,
    /// A financial commitment issued on a stand-by basis and borrowed only if and when needed. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "STANDBY TERM")]
    STANDBYTERM,
    #[serde(rename = "Stapled Security")]
    StapledSecurity,
    /// Certificates of Deposit issued in British Sterling Market
    #[serde(rename = "STERLING CD")]
    STERLINGCD,
    /// Certificate of Deposit (CD). A debt instrument issued by a bank that will pay interest, periodically or at maturity (set by competitive forces in the marketplace), and principal when it reaches maturity. Maturities range from a few weeks to several years. It may have a fixed or floating rate of interest. Certificate of Deposit Rollover. An investment strategy used to defer taxes on income from a Certificate of Deposit. An individual who has purchased a certificate of deposit on margin expected to mature in the following year, can deduct the interest on the loan in the current year and move the income from the certificate to the up-coming year. - STERLING
    #[serde(rename = "STERLING CP")]
    STERLINGCP,
    #[serde(rename = "Strategy Trade.")]
    StrategyTrade,
    /// Swap Rates. Interest rate swaps are highly customized agreements that possess a wide range of configurations and complexities. However, most are of the generic, or plain vanilla sort. A plain vanilla swap is an agreement of two parties to exchange interest payments with each other during an agreed-upon period of time. In a plain vanilla swap, one of the interest payment streams is fixed, while the other is floating, and no exchange of principle takes place.
    #[serde(rename = "SWAP")]
    SWAP,
    /// Primarily quoted in basis points, swap spreads take the difference between two different interest rate swaps.
    #[serde(rename = "SWAP SPREAD")]
    SWAPSPREAD,
    /// Volatility of the underlying swap rate
    #[serde(rename = "SWAPTION VOLATILITY")]
    SWAPTIONVOLATILITY,
    /// Swedish Commercial Paper
    #[serde(rename = "SWEDISH CP")]
    SWEDISHCP,
    /// Short term loan that allows the borrower to draw funds on short notice, usually the same day. It is not used alone as a loan type.
    #[serde(rename = "SWINGLINE")]
    SWINGLINE,
    /// Swiss Certificate. Original share certificate of a foreign company, mainly originating in the US, the UK or Canada, which is quoted on a Swiss Stock Exchange and is registered in the nominee name of a specified Swiss nominee endorsed in blank. For dividend payments or in corporate actions it must be stamped by the nominee company.
    #[serde(rename = "Swiss Cert")]
    SwissCert,
    /// A letter of credit facility which is fully cash collateralized with proceeds of a term loan which has been issued by the same borrower, usually with the same amount and terms.
    #[serde(rename = "SYNTH LOC")]
    SYNTHLOC,
    /// A letter of credit facility which is fully cash collateralized with proceeds of a term loan which has been issued by the same borrower, usually with the same amount and terms. Once borrowed, can be borrowed again.
    #[serde(rename = "SYNTH REV")]
    SYNTHREV,
    /// A letter of credit facility which is fully cash collateralized with proceeds of a term loan which has been issued by the same borrower, usually with the same amount and terms. Once borrowed, cannot be re-borrowed.
    #[serde(rename = "SYNTH TERM")]
    SYNTHTERM,
    #[serde(rename = "Synthetic Term")]
    SyntheticTerm,
    /// Taiwan Commercial Paper
    #[serde(rename = "TAIWAN CP")]
    TAIWANCP,
    /// Taiwan Guaranteed Commercial Paper
    #[serde(rename = "TAIWAN CP GUAR")]
    TAIWANCPGUAR,
    /// Taiwan Negotiable Certificates of Deposit
    #[serde(rename = "TAIWAN NEGO CD")]
    TAIWANNEGOCD,
    /// Taiwan Time Deposits
    #[serde(rename = "TAIWAN TIME DEPO")]
    TAIWANTIMEDEPO,
    /// The bond pays little or no interest, but gives a tax credit to bondholders.
    #[serde(rename = "TAX CREDIT")]
    TAXCREDIT,
    /// The bond pays little or no interest, but gives a tax credit to bondholders. The security is an Original Issue Discount.
    #[serde(rename = "TAX CREDIT, OID")]
    TAXCREDITOID,
    /// Taiwan Depository Receipt
    #[serde(rename = "TDR")]
    TDR,
    /// A loan with a fixed availability period and often includes some minimum repayment schedule. Once borrowings have been repaid, they cannot be re-borrowed.
    #[serde(rename = "TERM")]
    TERM,
    /// A loan with a fixed availability period and often includes some minimum repayment schedule. Once borrowings have been repaid, they cannot be re-borrowed.
    #[serde(rename = "Term")]
    Term,
    /// Term Deposits
    #[serde(rename = "TERM DEPOSITS")]
    TERMDEPOSITS,
    #[serde(rename = "TERM GUARANTEE FAC")]
    TERMGUARANTEEFAC,
    #[serde(rename = "TERM REV")]
    TERMREV,
    /// A loan with a fixed availability period and often includes some minimum repayment schedule. Once borrowings have been repaid, they cannot be re-borrowed. Given as temporary reimbursement for a VAT credit it has with the state.
    #[serde(rename = "TERM VAT-TRNCH")]
    TERMVATTRNCH,
    /// Thailand Commercial Paper
    #[serde(rename = "THAILAND CP")]
    THAILANDCP,
    #[serde(rename = "TLTRO TERM")]
    TLTROTERM,
    /// Tracking Stock. Shares issued by a company which pay a dividend determined by the performance of a specific portion of the whole company. Tracking stocks do not represent or require any change in business structure. Holders of tracking stock are considered to hold equity in the parent company and not the specific entity represented by the tracking stock. Tracking stock is often set up by companies that have several diverse divisions, both so that investors can take a share in a division of their interest, and so that the performance of these divisions can be tracked in terms of shareholder interest. A company will sometimes issue a tracking stock when it has a very successful division that it feels is underappreciated by the market and not fully reflected in the company's stock price.
    #[serde(rename = "Tracking Stk")]
    TrackingStk,
    /// Represents the interest rate of short term bonds issued by governments.
    #[serde(rename = "TREASURY BILL")]
    TREASURYBILL,
    /// US Certificates of Deposit
    #[serde(rename = "U.S. CD")]
    USCD,
    /// US Commercial Paper
    #[serde(rename = "U.S. CP")]
    USCP,
    /// Interest-Bearing Note. A note on which the issuer has agreed to pay the face value plus interest.
    #[serde(rename = "U.S. INT BEAR CP")]
    USINTBEARCP,
    /// Unit Investment Trust. A portfolio made up of a variety of different income securities that are pooled together and sold to investors as units. Each unit represents a fractional, undivided interest in the portfolio's principal and income.
    #[serde(rename = "UIT")]
    UIT,
    /// Gilt. A British word used to connote a debt obligation.
    #[serde(rename = "UK GILT STOCK")]
    UKGILTSTOCK,
    #[serde(rename = "UMBS MBS Other")]
    UMBSMBSOther,
    /// A unit is an exchange traded equity-type investment that is similar to common stock. By owning securities or assets of an underlying business (or businesses), an income trust is structured to distribute cash flows from those businesses to unit holders in a tax efficient manner.
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
    /// US issuer issuing in US dollar in the US market.
    #[serde(rename = "US DOMESTIC")]
    USDOMESTIC,
    /// US Government issued security
    #[serde(rename = "US GOVERNMENT")]
    USGOVERNMENT,
    /// A US issuer issuing into the US domestic mkt in a currency other than US Dollars
    #[serde(rename = "US NON-DOLLAR")]
    USNONDOLLAR,
    /// Variable Rate Demand Obligation, Municipal instrument with varying resets frequencies
    #[serde(rename = "VAR RATE DEM OBL")]
    VARRATEDEMOBL,
    /// A facility a company is given as temporary reimbursement for a VAT credit it has with the state. The banks anticipate a credit that should be covered by the state.
    #[serde(rename = "VAT-TRNCH")]
    VATTRNCH,
    /// Venezuelan Commercial Paper
    #[serde(rename = "VENEZUELAN CP")]
    VENEZUELANCP,
    #[serde(rename = "VIETNAMESE CD")]
    VIETNAMESECD,
    /// Volatility derivatives are a class of derivative securities.
    #[serde(rename = "VOLATILITY DERIVATIVE")]
    VOLATILITYDERIVATIVE,
    /// Warrant. A company-issued certificate that represents an option to buy a certain number of stock shares at a specific price before a predetermined date. A warrant, because it has a value of its own, can be traded on the open market. Bloomberg abbreviates warrants as WRNTS or WT.
    #[serde(rename = "Warrant")]
    Warrant,
    /// Yankee Bond. A foreign bond denominated in U.S. dollars and registered with the Securities and Exchange Commission for sale in the United States.
    #[serde(rename = "YANKEE")]
    YANKEE,
    /// Yankee Certificates of Deposit, issued by Foreign Banks via US Branch
    #[serde(rename = "YANKEE CD")]
    YANKEECD,
    /// Japanese Yen Certificates of Deposit
    #[serde(rename = "YEN CD")]
    YENCD,
    /// Japanese Yen Commercial Paper
    #[serde(rename = "YEN CP")]
    YENCP,
    /// A chart consisting of the yields of bonds of the same quality but different maturities. This can be used as a gauge to evaluate the future of the interest rates. An upward trend with short-term rates lower than long-term rates is called a positive yield curve, while a down trend is a negative or inverted yield curve.
    #[serde(rename = "Yield Curve")]
    YieldCurve,
    /// Zero Coupon Bond. A bond sold at deep discount that pays no interest until maturity, at which time the holder receives the bond's face value plus all accrued interest. While holders receive no interest on the bond until maturity, they are responsible for paying taxes each year on the unpaid interest.
    #[serde(rename = "ZERO COUPON")]
    ZEROCOUPON,
    /// Also called a Capital Appreciation Bond (CAB). The bond is sold at a deep discount, but on interest payments are made. The bondholder receives the full value (par amount) of the bond at maturity. The security is an Original Issue Discount
    #[serde(rename = "ZERO COUPON, OID")]
    ZEROCOUPONOID,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_enum_serialization;

    test_enum_serialization!(
        test_serialize_abs_card,
        SecurityType,
        ABSCard,
        "\"ABS Card\""
    );
    test_enum_serialization!(
        test_serialize_adjustable,
        SecurityType,
        ZEROCOUPONOID,
        "\"ZERO COUPON, OID\""
    );
}
