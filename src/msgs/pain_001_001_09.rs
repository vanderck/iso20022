use regex::Regex;
use once_cell::sync::Lazy;
use validator::Validate;

pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:pain.001.001.09".to_string()
}
::lazy_static::lazy_static! { static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^[A-Z]{3,3}$").unwrap()});}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyCode {
    #[validate(regex = "ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AddressType2Code {
    #[serde(rename = "ADDR")]
    Addr,
    #[serde(rename = "PBOX")]
    Pbox,
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "BIZZ")]
    Bizz,
    #[serde(rename = "MLTO")]
    Mlto,
    #[serde(rename = "DLVY")]
    Dlvy,
    #[default]
    Unknown
}
::lazy_static::lazy_static! { static ref ANY_BICDEC_2014_IDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}$").unwrap()});}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AnyBICDec2014Identifier {
    #[validate(regex = "ANY_BICDEC_2014_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Authorisation1Code {
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "FDET")]
    Fdet,
    #[serde(rename = "FSUM")]
    Fsum,
    #[serde(rename = "ILEV")]
    Ilev,
    #[default]
    Unknown
}
::lazy_static::lazy_static! { static ref BICFIDEC_2014_IDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}$").unwrap()});}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BICFIDec2014Identifier {
    #[validate(regex = "BICFIDEC_2014_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BaseOneRate {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BatchBookingIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ChargeBearerType1Code {
    #[serde(rename = "DEBT")]
    Debt,
    #[serde(rename = "CRED")]
    Cred,
    #[serde(rename = "SHAR")]
    Shar,
    #[serde(rename = "SLEV")]
    Slev,
    #[default]
    Unknown
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ChequeDelivery1Code {
    #[serde(rename = "MLDB")]
    Mldb,
    #[serde(rename = "MLCD")]
    Mlcd,
    #[serde(rename = "MLFA")]
    Mlfa,
    #[serde(rename = "CRDB")]
    Crdb,
    #[serde(rename = "CRCD")]
    Crcd,
    #[serde(rename = "CRFA")]
    Crfa,
    #[serde(rename = "PUDB")]
    Pudb,
    #[serde(rename = "PUCD")]
    Pucd,
    #[serde(rename = "PUFA")]
    Pufa,
    #[serde(rename = "RGDB")]
    Rgdb,
    #[serde(rename = "RGCD")]
    Rgcd,
    #[serde(rename = "RGFA")]
    Rgfa,
    #[default]
    Unknown
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ChequeType2Code {
    #[serde(rename = "CCHQ")]
    Cchq,
    #[serde(rename = "CCCH")]
    Ccch,
    #[serde(rename = "BCHQ")]
    Bchq,
    #[serde(rename = "DRFT")]
    Drft,
    #[serde(rename = "ELDR")]
    Eldr,
    #[default]
    Unknown
}
::lazy_static::lazy_static! { static ref COUNTRY_CODE_REGEX: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^[A-Z]{2,2}$").unwrap()});}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CountryCode {
    #[validate(regex = "COUNTRY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CreditDebitCode {
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "DBIT")]
    Dbit,
    #[default]
    Unknown
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DecimalNumber {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DocumentType3Code {
    #[serde(rename = "RADM")]
    Radm,
    #[serde(rename = "RPIN")]
    Rpin,
    #[serde(rename = "FXDR")]
    Fxdr,
    #[serde(rename = "DISP")]
    Disp,
    #[serde(rename = "PUOR")]
    Puor,
    #[serde(rename = "SCOR")]
    Scor,
    #[default]
    Unknown
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DocumentType6Code {
    #[serde(rename = "MSIN")]
    Msin,
    #[serde(rename = "CNFA")]
    Cnfa,
    #[serde(rename = "DNFA")]
    Dnfa,
    #[serde(rename = "CINV")]
    Cinv,
    #[serde(rename = "CREN")]
    Cren,
    #[serde(rename = "DEBN")]
    Debn,
    #[serde(rename = "HIRI")]
    Hiri,
    #[serde(rename = "SBIN")]
    Sbin,
    #[serde(rename = "CMCN")]
    Cmcn,
    #[serde(rename = "SOAC")]
    Soac,
    #[serde(rename = "DISP")]
    Disp,
    #[serde(rename = "BOLD")]
    Bold,
    #[serde(rename = "VCHR")]
    Vchr,
    #[serde(rename = "AROI")]
    Aroi,
    #[serde(rename = "TSUT")]
    Tsut,
    #[serde(rename = "PUOR")]
    Puor,
    #[default]
    Unknown
}
::lazy_static::lazy_static! { static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^[a-zA-Z0-9]{4}$").unwrap()});}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact4AlphaNumericText {
    #[validate(regex = "EXACT_4_ALPHA_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExchangeRateType1Code {
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "SALE")]
    Sale,
    #[serde(rename = "AGRD")]
    Agrd,
    #[default]
    Unknown
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalAccountIdentification1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalCashAccountType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalCategoryPurpose1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalClearingSystemIdentification1Code {
    #[validate(length(min = 1, max = 5,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalDiscountAmountType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalDocumentLineType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalFinancialInstitutionIdentification1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalGarnishmentType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalLocalInstrument1Code {
    #[validate(length(min = 1, max = 35,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalOrganisationIdentification1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalPersonIdentification1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalProxyAccountType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalPurpose1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalServiceLevel1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalTaxAmountType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
::lazy_static::lazy_static! { static ref IBAN_2007_IDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}$").unwrap()});}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IBAN2007Identifier {
    #[validate(regex = "IBAN_2007_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ISODate {
    #[serde(rename = "$text")]
    pub value: ::chrono::NaiveDate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ISODateTime {
    #[serde(rename = "$text")]
    pub value: ::chrono::DateTime<::chrono::Utc>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Instruction3Code {
    #[serde(rename = "CHQB")]
    Chqb,
    #[serde(rename = "HOLD")]
    Hold,
    #[serde(rename = "PHOB")]
    Phob,
    #[serde(rename = "TELB")]
    Telb,
    #[default]
    Unknown
}
::lazy_static::lazy_static! { static ref LEIIDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^[A-Z0-9]{18,18}[0-9]{2,2}$").unwrap()});}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LEIIdentifier {
    #[validate(regex = "LEIIDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max10Text {
    #[validate(length(min = 1, max = 10,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max128Text {
    #[validate(length(min = 1, max = 128,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max140Text {
    #[validate(length(min = 1, max = 140,))]
    #[serde(rename = "$text")]
    pub value: String,
}
::lazy_static::lazy_static! { static ref MAX_15_NUMERIC_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^[0-9]{1,15}$").unwrap()});}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max15NumericText {
    #[validate(regex = "MAX_15_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max16Text {
    #[validate(length(min = 1, max = 16,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max2048Text {
    #[validate(length(min = 1, max = 2048,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max34Text {
    #[validate(length(min = 1, max = 34,))]
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max350Text {
    #[validate(length(min = 1, max = 350,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max35Text {
    #[validate(length(min = 1, max = 35,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max4Text {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max70Text {
    #[validate(length(min = 1, max = 70,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NamePrefix2Code {
    #[serde(rename = "DOCT")]
    Doct,
    #[serde(rename = "MADM")]
    Madm,
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "MIST")]
    Mist,
    #[serde(rename = "MIKS")]
    Miks,
    #[default]
    Unknown
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Number {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PaymentMethod3Code {
    #[serde(rename = "CHK")]
    Chk,
    #[serde(rename = "TRF")]
    Trf,
    #[serde(rename = "TRA")]
    Tra,
    #[default]
    Unknown
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PercentageRate {
    #[serde(rename = "$text")]
    pub value: f64,
}
::lazy_static::lazy_static! { static ref PHONE_NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^\+[0-9]{1,3}-[0-9()+\-]{1,30}$").unwrap()});}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PhoneNumber {
    #[validate(regex = "PHONE_NUMBER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PreferredContactMethod1Code {
    #[serde(rename = "LETT")]
    Lett,
    #[serde(rename = "MAIL")]
    Mail,
    #[serde(rename = "PHON")]
    Phon,
    #[serde(rename = "FAXX")]
    Faxx,
    #[serde(rename = "CELL")]
    Cell,
    #[default]
    Unknown
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Priority2Code {
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "NORM")]
    Norm,
    #[default]
    Unknown
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RegulatoryReportingType1Code {
    #[serde(rename = "CRED")]
    Cred,
    #[serde(rename = "DEBT")]
    Debt,
    #[serde(rename = "BOTH")]
    Both,
    #[default]
    Unknown
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RemittanceLocationMethod2Code {
    #[serde(rename = "FAXI")]
    Faxi,
    #[serde(rename = "EDIC")]
    Edic,
    #[serde(rename = "URID")]
    Urid,
    #[serde(rename = "EMAL")]
    Emal,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "SMSM")]
    Smsm,
    #[default]
    Unknown
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxRecordPeriod1Code {
    #[serde(rename = "MM01")]
    Mm01,
    #[serde(rename = "MM02")]
    Mm02,
    #[serde(rename = "MM03")]
    Mm03,
    #[serde(rename = "MM04")]
    Mm04,
    #[serde(rename = "MM05")]
    Mm05,
    #[serde(rename = "MM06")]
    Mm06,
    #[serde(rename = "MM07")]
    Mm07,
    #[serde(rename = "MM08")]
    Mm08,
    #[serde(rename = "MM09")]
    Mm09,
    #[serde(rename = "MM10")]
    Mm10,
    #[serde(rename = "MM11")]
    Mm11,
    #[serde(rename = "MM12")]
    Mm12,
    #[serde(rename = "QTR1")]
    Qtr1,
    #[serde(rename = "QTR2")]
    Qtr2,
    #[serde(rename = "QTR3")]
    Qtr3,
    #[serde(rename = "QTR4")]
    Qtr4,
    #[serde(rename = "HLF1")]
    Hlf1,
    #[serde(rename = "HLF2")]
    Hlf2,
    #[default]
    Unknown
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TrueFalseIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
}
::lazy_static::lazy_static! { static ref UUIDV_4_IDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$").unwrap()});}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UUIDv4Identifier {
    #[validate(regex = "UUIDV_4_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification4Choice {
        #[validate]
        #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
        pub iban: Option<IBAN2007Identifier>,
        #[validate]
        #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
        pub othr: Option<GenericAccountIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountSchemeName1Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalAccountIdentification1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyAndAmount {
        #[validate(regex = "ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX")]
        #[serde(rename = "@Ccy")]
        pub ccy: String,
        #[validate(range(min = 0.0))]
        #[serde(rename = "$value")]
        pub value: f64,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AddressType3Choice {
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<AddressType2Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountType4Choice {
        #[validate]
        #[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
        pub instdamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "EqvtAmt", skip_serializing_if = "Option::is_none")]
        pub eqvtamt: Option<EquivalentAmount2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Authorisation1Choice {
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<Authorisation1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max128Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BranchAndFinancialInstitutionIdentification6 {
        #[validate]
        #[serde(rename = "FinInstnId")]
        pub fininstnid: FinancialInstitutionIdentification18,
        #[validate]
        #[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
        pub brnchid: Option<BranchData3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BranchData3 {
        #[validate]
        #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
        pub id: Option<Max35Text>,
        #[validate]
        #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
        pub lei: Option<LEIIdentifier>,
        #[validate]
        #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
        pub nm: Option<Max140Text>,
        #[validate]
        #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
        pub pstladr: Option<PostalAddress24>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccount38 {
        #[validate]
        #[serde(rename = "Id")]
        pub id: AccountIdentification4Choice,
        #[validate]
        #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
        pub tp: Option<CashAccountType2Choice>,
        #[validate]
        #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
        pub ccy: Option<ActiveOrHistoricCurrencyCode>,
        #[validate]
        #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
        pub nm: Option<Max70Text>,
        #[validate]
        #[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
        pub prxy: Option<ProxyAccountIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccountType2Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalCashAccountType1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CategoryPurpose1Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalCategoryPurpose1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Cheque11 {
        #[serde(rename = "ChqTp", skip_serializing_if = "Option::is_none")]
        pub chqtp: Option<ChequeType2Code>,
        #[validate]
        #[serde(rename = "ChqNb", skip_serializing_if = "Option::is_none")]
        pub chqnb: Option<Max35Text>,
        #[validate]
        #[serde(rename = "ChqFr", skip_serializing_if = "Option::is_none")]
        pub chqfr: Option<NameAndAddress16>,
        #[validate]
        #[serde(rename = "DlvryMtd", skip_serializing_if = "Option::is_none")]
        pub dlvrymtd: Option<ChequeDeliveryMethod1Choice>,
        #[validate]
        #[serde(rename = "DlvrTo", skip_serializing_if = "Option::is_none")]
        pub dlvrto: Option<NameAndAddress16>,
        #[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
        pub instrprty: Option<Priority2Code>,
        #[validate]
        #[serde(rename = "ChqMtrtyDt", skip_serializing_if = "Option::is_none")]
        pub chqmtrtydt: Option<ISODate>,
        #[validate]
        #[serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none")]
        pub frmscd: Option<Max35Text>,
        #[validate]
        #[validate(length(min = 0,max = 2))]
        #[serde(rename = "MemoFld", default)]
        pub memofld: Vec<Max35Text>,
        #[validate]
        #[serde(rename = "RgnlClrZone", skip_serializing_if = "Option::is_none")]
        pub rgnlclrzone: Option<Max35Text>,
        #[validate]
        #[serde(rename = "PrtLctn", skip_serializing_if = "Option::is_none")]
        pub prtlctn: Option<Max35Text>,
        #[validate]
        #[validate(length(min = 0,max = 5))]
        #[serde(rename = "Sgntr", default)]
        pub sgntr: Vec<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChequeDeliveryMethod1Choice {
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ChequeDelivery1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClearingSystemIdentification2Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalClearingSystemIdentification1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClearingSystemMemberIdentification2 {
        #[validate]
        #[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
        pub clrsysid: Option<ClearingSystemIdentification2Choice>,
        #[validate]
        #[serde(rename = "MmbId")]
        pub mmbid: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Contact4 {
        #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
        pub nmprfx: Option<NamePrefix2Code>,
        #[validate]
        #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
        pub nm: Option<Max140Text>,
        #[validate]
        #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
        pub phnenb: Option<PhoneNumber>,
        #[validate]
        #[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
        pub mobnb: Option<PhoneNumber>,
        #[validate]
        #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
        pub faxnb: Option<PhoneNumber>,
        #[validate]
        #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
        pub emailadr: Option<Max2048Text>,
        #[validate]
        #[serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none")]
        pub emailpurp: Option<Max35Text>,
        #[validate]
        #[serde(rename = "JobTitl", skip_serializing_if = "Option::is_none")]
        pub jobtitl: Option<Max35Text>,
        #[validate]
        #[serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none")]
        pub rspnsblty: Option<Max35Text>,
        #[validate]
        #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
        pub dept: Option<Max70Text>,
        #[validate]
        #[serde(rename = "Othr", default)]
        pub othr: Vec<OtherContact1>,
        #[serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none")]
        pub prefrdmtd: Option<PreferredContactMethod1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CreditTransferTransaction34 {
        #[validate]
        #[serde(rename = "PmtId")]
        pub pmtid: PaymentIdentification6,
        #[validate]
        #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
        pub pmttpinf: Option<PaymentTypeInformation26>,
        #[validate]
        #[serde(rename = "Amt")]
        pub amt: AmountType4Choice,
        #[validate]
        #[serde(rename = "XchgRateInf", skip_serializing_if = "Option::is_none")]
        pub xchgrateinf: Option<ExchangeRate1>,
        #[serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none")]
        pub chrgbr: Option<ChargeBearerType1Code>,
        #[validate]
        #[serde(rename = "ChqInstr", skip_serializing_if = "Option::is_none")]
        pub chqinstr: Option<Cheque11>,
        #[validate]
        #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
        pub ultmtdbtr: Option<PartyIdentification135>,
        #[validate]
        #[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
        pub intrmyagt1: Option<BranchAndFinancialInstitutionIdentification6>,
        #[validate]
        #[serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none")]
        pub intrmyagt1acct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
        pub intrmyagt2: Option<BranchAndFinancialInstitutionIdentification6>,
        #[validate]
        #[serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none")]
        pub intrmyagt2acct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
        pub intrmyagt3: Option<BranchAndFinancialInstitutionIdentification6>,
        #[validate]
        #[serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none")]
        pub intrmyagt3acct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
        pub cdtragt: Option<BranchAndFinancialInstitutionIdentification6>,
        #[validate]
        #[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
        pub cdtragtacct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
        pub cdtr: Option<PartyIdentification135>,
        #[validate]
        #[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
        pub cdtracct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
        pub ultmtcdtr: Option<PartyIdentification135>,
        #[validate]
        #[serde(rename = "InstrForCdtrAgt", default)]
        pub instrforcdtragt: Vec<InstructionForCreditorAgent1>,
        #[validate]
        #[serde(rename = "InstrForDbtrAgt", skip_serializing_if = "Option::is_none")]
        pub instrfordbtragt: Option<Max140Text>,
        #[validate]
        #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
        pub purp: Option<Purpose2Choice>,
        #[validate]
        #[validate(length(min = 0,max = 10))]
        #[serde(rename = "RgltryRptg", default)]
        pub rgltryrptg: Vec<RegulatoryReporting3>,
        #[validate]
        #[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
        pub tax: Option<TaxInformation8>,
        #[validate]
        #[validate(length(min = 0,max = 10))]
        #[serde(rename = "RltdRmtInf", default)]
        pub rltdrmtinf: Vec<RemittanceLocation7>,
        #[validate]
        #[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
        pub rmtinf: Option<RemittanceInformation16>,
        #[validate]
        #[serde(rename = "SplmtryData", default)]
        pub splmtrydata: Vec<SupplementaryData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CreditorReferenceInformation2 {
        #[validate]
        #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
        pub tp: Option<CreditorReferenceType2>,
        #[validate]
        #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
        pub rref: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CreditorReferenceType1Choice {
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<DocumentType3Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CreditorReferenceType2 {
        #[validate]
        #[serde(rename = "CdOrPrtry")]
        pub cdorprtry: CreditorReferenceType1Choice,
        #[validate]
        #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
        pub issr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CustomerCreditTransferInitiationV09 {
        #[validate]
        #[serde(rename = "GrpHdr")]
        pub grphdr: GroupHeader85,
        #[validate]
        #[validate(length(min = 1))]
        #[serde(rename = "PmtInf", default)]
        pub pmtinf: Vec<PaymentInstruction30>,
        #[validate]
        #[serde(rename = "SplmtryData", default)]
        pub splmtrydata: Vec<SupplementaryData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndDateTime2Choice {
        #[validate]
        #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
        pub dt: Option<ISODate>,
        #[validate]
        #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
        pub dttm: Option<ISODateTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndPlaceOfBirth1 {
        #[validate]
        #[serde(rename = "BirthDt")]
        pub birthdt: ISODate,
        #[validate]
        #[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
        pub prvcofbirth: Option<Max35Text>,
        #[validate]
        #[serde(rename = "CityOfBirth")]
        pub cityofbirth: Max35Text,
        #[validate]
        #[serde(rename = "CtryOfBirth")]
        pub ctryofbirth: CountryCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DatePeriod2 {
        #[validate]
        #[serde(rename = "FrDt")]
        pub frdt: ISODate,
        #[validate]
        #[serde(rename = "ToDt")]
        pub todt: ISODate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DiscountAmountAndType1 {
        #[validate]
        #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
        pub tp: Option<DiscountAmountType1Choice>,
        #[validate]
        #[serde(rename = "Amt")]
        pub amt: ActiveOrHistoricCurrencyAndAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DiscountAmountType1Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalDiscountAmountType1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Document {
        #[serde(rename = "@xmlns", default = "namespace")]
        pub xmlns: String,
        #[validate]
        #[serde(rename = "CstmrCdtTrfInitn")]
        pub cstmrcdttrfinitn: CustomerCreditTransferInitiationV09,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentAdjustment1 {
        #[validate]
        #[serde(rename = "Amt")]
        pub amt: ActiveOrHistoricCurrencyAndAmount,
        #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
        pub cdtdbtind: Option<CreditDebitCode>,
        #[validate]
        #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
        pub rsn: Option<Max4Text>,
        #[validate]
        #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
        pub addtlinf: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentLineIdentification1 {
        #[validate]
        #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
        pub tp: Option<DocumentLineType1>,
        #[validate]
        #[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
        pub nb: Option<Max35Text>,
        #[validate]
        #[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
        pub rltddt: Option<ISODate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentLineInformation1 {
        #[validate]
        #[validate(length(min = 1))]
        #[serde(rename = "Id", default)]
        pub id: Vec<DocumentLineIdentification1>,
        #[validate]
        #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
        pub desc: Option<Max2048Text>,
        #[validate]
        #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
        pub amt: Option<RemittanceAmount3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentLineType1 {
        #[validate]
        #[serde(rename = "CdOrPrtry")]
        pub cdorprtry: DocumentLineType1Choice,
        #[validate]
        #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
        pub issr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentLineType1Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalDocumentLineType1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EquivalentAmount2 {
        #[validate]
        #[serde(rename = "Amt")]
        pub amt: ActiveOrHistoricCurrencyAndAmount,
        #[validate]
        #[serde(rename = "CcyOfTrf")]
        pub ccyoftrf: ActiveOrHistoricCurrencyCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExchangeRate1 {
        #[validate]
        #[serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none")]
        pub unitccy: Option<ActiveOrHistoricCurrencyCode>,
        #[validate]
        #[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
        pub xchgrate: Option<BaseOneRate>,
        #[serde(rename = "RateTp", skip_serializing_if = "Option::is_none")]
        pub ratetp: Option<ExchangeRateType1Code>,
        #[validate]
        #[serde(rename = "CtrctId", skip_serializing_if = "Option::is_none")]
        pub ctrctid: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialIdentificationSchemeName1Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalFinancialInstitutionIdentification1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstitutionIdentification18 {
        #[validate]
        #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
        pub bicfi: Option<BICFIDec2014Identifier>,
        #[validate]
        #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
        pub clrsysmmbid: Option<ClearingSystemMemberIdentification2>,
        #[validate]
        #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
        pub lei: Option<LEIIdentifier>,
        #[validate]
        #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
        pub nm: Option<Max140Text>,
        #[validate]
        #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
        pub pstladr: Option<PostalAddress24>,
        #[validate]
        #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
        pub othr: Option<GenericFinancialIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Garnishment3 {
        #[validate]
        #[serde(rename = "Tp")]
        pub tp: GarnishmentType1,
        #[validate]
        #[serde(rename = "Grnshee", skip_serializing_if = "Option::is_none")]
        pub grnshee: Option<PartyIdentification135>,
        #[validate]
        #[serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none")]
        pub grnshmtadmstr: Option<PartyIdentification135>,
        #[validate]
        #[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
        pub refnb: Option<Max140Text>,
        #[validate]
        #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
        pub dt: Option<ISODate>,
        #[validate]
        #[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
        pub rmtdamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "FmlyMdclInsrncInd", skip_serializing_if = "Option::is_none")]
        pub fmlymdclinsrncind: Option<TrueFalseIndicator>,
        #[validate]
        #[serde(rename = "MplyeeTermntnInd", skip_serializing_if = "Option::is_none")]
        pub mplyeetermntnind: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GarnishmentType1 {
        #[validate]
        #[serde(rename = "CdOrPrtry")]
        pub cdorprtry: GarnishmentType1Choice,
        #[validate]
        #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
        pub issr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GarnishmentType1Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalGarnishmentType1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericAccountIdentification1 {
        #[validate]
        #[serde(rename = "Id")]
        pub id: Max34Text,
        #[validate]
        #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
        pub schmenm: Option<AccountSchemeName1Choice>,
        #[validate]
        #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
        pub issr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericFinancialIdentification1 {
        #[validate]
        #[serde(rename = "Id")]
        pub id: Max35Text,
        #[validate]
        #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
        pub schmenm: Option<FinancialIdentificationSchemeName1Choice>,
        #[validate]
        #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
        pub issr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification30 {
        #[validate]
        #[serde(rename = "Id")]
        pub id: Exact4AlphaNumericText,
        #[validate]
        #[serde(rename = "Issr")]
        pub issr: Max35Text,
        #[validate]
        #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
        pub schmenm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericOrganisationIdentification1 {
        #[validate]
        #[serde(rename = "Id")]
        pub id: Max35Text,
        #[validate]
        #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
        pub schmenm: Option<OrganisationIdentificationSchemeName1Choice>,
        #[validate]
        #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
        pub issr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericPersonIdentification1 {
        #[validate]
        #[serde(rename = "Id")]
        pub id: Max35Text,
        #[validate]
        #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
        pub schmenm: Option<PersonIdentificationSchemeName1Choice>,
        #[validate]
        #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
        pub issr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GroupHeader85 {
        #[validate]
        #[serde(rename = "MsgId")]
        pub msgid: Max35Text,
        #[validate]
        #[serde(rename = "CreDtTm")]
        pub credttm: ISODateTime,
        #[validate]
        #[validate(length(min = 0,max = 2))]
        #[serde(rename = "Authstn", default)]
        pub authstn: Vec<Authorisation1Choice>,
        #[validate]
        #[serde(rename = "NbOfTxs")]
        pub nboftxs: Max15NumericText,
        #[validate]
        #[serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none")]
        pub ctrlsum: Option<DecimalNumber>,
        #[validate]
        #[serde(rename = "InitgPty")]
        pub initgpty: PartyIdentification135,
        #[validate]
        #[serde(rename = "FwdgAgt", skip_serializing_if = "Option::is_none")]
        pub fwdgagt: Option<BranchAndFinancialInstitutionIdentification6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InstructionForCreditorAgent1 {
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<Instruction3Code>,
        #[validate]
        #[serde(rename = "InstrInf", skip_serializing_if = "Option::is_none")]
        pub instrinf: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LocalInstrument2Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalLocalInstrument1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NameAndAddress16 {
        #[validate]
        #[serde(rename = "Nm")]
        pub nm: Max140Text,
        #[validate]
        #[serde(rename = "Adr")]
        pub adr: PostalAddress24,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentification29 {
        #[validate]
        #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
        pub anybic: Option<AnyBICDec2014Identifier>,
        #[validate]
        #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
        pub lei: Option<LEIIdentifier>,
        #[validate]
        #[serde(rename = "Othr", default)]
        pub othr: Vec<GenericOrganisationIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentificationSchemeName1Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalOrganisationIdentification1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherContact1 {
        #[validate]
        #[serde(rename = "ChanlTp")]
        pub chanltp: Max4Text,
        #[validate]
        #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
        pub id: Option<Max128Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party38Choice {
        #[validate]
        #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
        pub orgid: Option<OrganisationIdentification29>,
        #[validate]
        #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
        pub prvtid: Option<PersonIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification135 {
        #[validate]
        #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
        pub nm: Option<Max140Text>,
        #[validate]
        #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
        pub pstladr: Option<PostalAddress24>,
        #[validate]
        #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
        pub id: Option<Party38Choice>,
        #[validate]
        #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
        pub ctryofres: Option<CountryCode>,
        #[validate]
        #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
        pub ctctdtls: Option<Contact4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentIdentification6 {
        #[validate]
        #[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
        pub instrid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "EndToEndId")]
        pub endtoendid: Max35Text,
        #[validate]
        #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
        pub uetr: Option<UUIDv4Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentInstruction30 {
        #[validate]
        #[serde(rename = "PmtInfId")]
        pub pmtinfid: Max35Text,
        #[serde(rename = "PmtMtd")]
        pub pmtmtd: PaymentMethod3Code,
        #[validate]
        #[serde(rename = "BtchBookg", skip_serializing_if = "Option::is_none")]
        pub btchbookg: Option<BatchBookingIndicator>,
        #[validate]
        #[serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none")]
        pub nboftxs: Option<Max15NumericText>,
        #[validate]
        #[serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none")]
        pub ctrlsum: Option<DecimalNumber>,
        #[validate]
        #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
        pub pmttpinf: Option<PaymentTypeInformation26>,
        #[validate]
        #[serde(rename = "ReqdExctnDt")]
        pub reqdexctndt: DateAndDateTime2Choice,
        #[validate]
        #[serde(rename = "PoolgAdjstmntDt", skip_serializing_if = "Option::is_none")]
        pub poolgadjstmntdt: Option<ISODate>,
        #[validate]
        #[serde(rename = "Dbtr")]
        pub dbtr: PartyIdentification135,
        #[validate]
        #[serde(rename = "DbtrAcct")]
        pub dbtracct: CashAccount38,
        #[validate]
        #[serde(rename = "DbtrAgt")]
        pub dbtragt: BranchAndFinancialInstitutionIdentification6,
        #[validate]
        #[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
        pub dbtragtacct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "InstrForDbtrAgt", skip_serializing_if = "Option::is_none")]
        pub instrfordbtragt: Option<Max140Text>,
        #[validate]
        #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
        pub ultmtdbtr: Option<PartyIdentification135>,
        #[serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none")]
        pub chrgbr: Option<ChargeBearerType1Code>,
        #[validate]
        #[serde(rename = "ChrgsAcct", skip_serializing_if = "Option::is_none")]
        pub chrgsacct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "ChrgsAcctAgt", skip_serializing_if = "Option::is_none")]
        pub chrgsacctagt: Option<BranchAndFinancialInstitutionIdentification6>,
        #[validate]
        #[validate(length(min = 1))]
        #[serde(rename = "CdtTrfTxInf", default)]
        pub cdttrftxinf: Vec<CreditTransferTransaction34>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentTypeInformation26 {
        #[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
        pub instrprty: Option<Priority2Code>,
        #[validate]
        #[serde(rename = "SvcLvl", default)]
        pub svclvl: Vec<ServiceLevel8Choice>,
        #[validate]
        #[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
        pub lclinstrm: Option<LocalInstrument2Choice>,
        #[validate]
        #[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
        pub ctgypurp: Option<CategoryPurpose1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonIdentification13 {
        #[validate]
        #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
        pub dtandplcofbirth: Option<DateAndPlaceOfBirth1>,
        #[validate]
        #[serde(rename = "Othr", default)]
        pub othr: Vec<GenericPersonIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonIdentificationSchemeName1Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalPersonIdentification1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PostalAddress24 {
        #[validate]
        #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
        pub adrtp: Option<AddressType3Choice>,
        #[validate]
        #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
        pub dept: Option<Max70Text>,
        #[validate]
        #[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
        pub subdept: Option<Max70Text>,
        #[validate]
        #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
        pub strtnm: Option<Max70Text>,
        #[validate]
        #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
        pub bldgnb: Option<Max16Text>,
        #[validate]
        #[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
        pub bldgnm: Option<Max35Text>,
        #[validate]
        #[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
        pub flr: Option<Max70Text>,
        #[validate]
        #[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
        pub pstbx: Option<Max16Text>,
        #[validate]
        #[serde(rename = "Room", skip_serializing_if = "Option::is_none")]
        pub room: Option<Max70Text>,
        #[validate]
        #[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
        pub pstcd: Option<Max16Text>,
        #[validate]
        #[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
        pub twnnm: Option<Max35Text>,
        #[validate]
        #[serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none")]
        pub twnlctnnm: Option<Max35Text>,
        #[validate]
        #[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
        pub dstrctnm: Option<Max35Text>,
        #[validate]
        #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
        pub ctrysubdvsn: Option<Max35Text>,
        #[validate]
        #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
        pub ctry: Option<CountryCode>,
        #[validate]
        #[validate(length(min = 0,max = 7))]
        #[serde(rename = "AdrLine", default)]
        pub adrline: Vec<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProxyAccountIdentification1 {
        #[validate]
        #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
        pub tp: Option<ProxyAccountType1Choice>,
        #[validate]
        #[serde(rename = "Id")]
        pub id: Max2048Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProxyAccountType1Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalProxyAccountType1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Purpose2Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalPurpose1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReferredDocumentInformation7 {
        #[validate]
        #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
        pub tp: Option<ReferredDocumentType4>,
        #[validate]
        #[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
        pub nb: Option<Max35Text>,
        #[validate]
        #[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
        pub rltddt: Option<ISODate>,
        #[validate]
        #[serde(rename = "LineDtls", default)]
        pub linedtls: Vec<DocumentLineInformation1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReferredDocumentType3Choice {
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<DocumentType6Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReferredDocumentType4 {
        #[validate]
        #[serde(rename = "CdOrPrtry")]
        pub cdorprtry: ReferredDocumentType3Choice,
        #[validate]
        #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
        pub issr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RegulatoryAuthority2 {
        #[validate]
        #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
        pub nm: Option<Max140Text>,
        #[validate]
        #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
        pub ctry: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RegulatoryReporting3 {
        #[serde(rename = "DbtCdtRptgInd", skip_serializing_if = "Option::is_none")]
        pub dbtcdtrptgind: Option<RegulatoryReportingType1Code>,
        #[validate]
        #[serde(rename = "Authrty", skip_serializing_if = "Option::is_none")]
        pub authrty: Option<RegulatoryAuthority2>,
        #[validate]
        #[serde(rename = "Dtls", default)]
        pub dtls: Vec<StructuredRegulatoryReporting3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RemittanceAmount2 {
        #[validate]
        #[serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none")]
        pub duepyblamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "DscntApldAmt", default)]
        pub dscntapldamt: Vec<DiscountAmountAndType1>,
        #[validate]
        #[serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none")]
        pub cdtnoteamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "TaxAmt", default)]
        pub taxamt: Vec<TaxAmountAndType1>,
        #[validate]
        #[serde(rename = "AdjstmntAmtAndRsn", default)]
        pub adjstmntamtandrsn: Vec<DocumentAdjustment1>,
        #[validate]
        #[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
        pub rmtdamt: Option<ActiveOrHistoricCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RemittanceAmount3 {
        #[validate]
        #[serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none")]
        pub duepyblamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "DscntApldAmt", default)]
        pub dscntapldamt: Vec<DiscountAmountAndType1>,
        #[validate]
        #[serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none")]
        pub cdtnoteamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "TaxAmt", default)]
        pub taxamt: Vec<TaxAmountAndType1>,
        #[validate]
        #[serde(rename = "AdjstmntAmtAndRsn", default)]
        pub adjstmntamtandrsn: Vec<DocumentAdjustment1>,
        #[validate]
        #[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
        pub rmtdamt: Option<ActiveOrHistoricCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RemittanceInformation16 {
        #[validate]
        #[serde(rename = "Ustrd", default)]
        pub ustrd: Vec<Max140Text>,
        #[validate]
        #[serde(rename = "Strd", default)]
        pub strd: Vec<StructuredRemittanceInformation16>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RemittanceLocation7 {
        #[validate]
        #[serde(rename = "RmtId", skip_serializing_if = "Option::is_none")]
        pub rmtid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "RmtLctnDtls", default)]
        pub rmtlctndtls: Vec<RemittanceLocationData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RemittanceLocationData1 {
        #[serde(rename = "Mtd")]
        pub mtd: RemittanceLocationMethod2Code,
        #[validate]
        #[serde(rename = "ElctrncAdr", skip_serializing_if = "Option::is_none")]
        pub elctrncadr: Option<Max2048Text>,
        #[validate]
        #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
        pub pstladr: Option<NameAndAddress16>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ServiceLevel8Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalServiceLevel1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct StructuredRegulatoryReporting3 {
        #[validate]
        #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
        pub tp: Option<Max35Text>,
        #[validate]
        #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
        pub dt: Option<ISODate>,
        #[validate]
        #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
        pub ctry: Option<CountryCode>,
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<Max10Text>,
        #[validate]
        #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
        pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "Inf", default)]
        pub inf: Vec<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct StructuredRemittanceInformation16 {
        #[validate]
        #[serde(rename = "RfrdDocInf", default)]
        pub rfrddocinf: Vec<ReferredDocumentInformation7>,
        #[validate]
        #[serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none")]
        pub rfrddocamt: Option<RemittanceAmount2>,
        #[validate]
        #[serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none")]
        pub cdtrrefinf: Option<CreditorReferenceInformation2>,
        #[validate]
        #[serde(rename = "Invcr", skip_serializing_if = "Option::is_none")]
        pub invcr: Option<PartyIdentification135>,
        #[validate]
        #[serde(rename = "Invcee", skip_serializing_if = "Option::is_none")]
        pub invcee: Option<PartyIdentification135>,
        #[validate]
        #[serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none")]
        pub taxrmt: Option<TaxInformation7>,
        #[validate]
        #[serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none")]
        pub grnshmtrmt: Option<Garnishment3>,
        #[validate]
        #[validate(length(min = 0,max = 3))]
        #[serde(rename = "AddtlRmtInf", default)]
        pub addtlrmtinf: Vec<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SupplementaryData1 {
        #[validate]
        #[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
        pub plcandnm: Option<Max350Text>,
        #[validate]
        #[serde(rename = "Envlp")]
        pub envlp: SupplementaryDataEnvelope1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SupplementaryDataEnvelope1 {
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxAmount2 {
        #[validate]
        #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
        pub rate: Option<PercentageRate>,
        #[validate]
        #[serde(rename = "TaxblBaseAmt", skip_serializing_if = "Option::is_none")]
        pub taxblbaseamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
        pub ttlamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "Dtls", default)]
        pub dtls: Vec<TaxRecordDetails2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxAmountAndType1 {
        #[validate]
        #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
        pub tp: Option<TaxAmountType1Choice>,
        #[validate]
        #[serde(rename = "Amt")]
        pub amt: ActiveOrHistoricCurrencyAndAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxAmountType1Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalTaxAmountType1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxAuthorisation1 {
        #[validate]
        #[serde(rename = "Titl", skip_serializing_if = "Option::is_none")]
        pub titl: Option<Max35Text>,
        #[validate]
        #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
        pub nm: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxInformation7 {
        #[validate]
        #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
        pub cdtr: Option<TaxParty1>,
        #[validate]
        #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
        pub dbtr: Option<TaxParty2>,
        #[validate]
        #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
        pub ultmtdbtr: Option<TaxParty2>,
        #[validate]
        #[serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none")]
        pub admstnzone: Option<Max35Text>,
        #[validate]
        #[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
        pub refnb: Option<Max140Text>,
        #[validate]
        #[serde(rename = "Mtd", skip_serializing_if = "Option::is_none")]
        pub mtd: Option<Max35Text>,
        #[validate]
        #[serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none")]
        pub ttltaxblbaseamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none")]
        pub ttltaxamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
        pub dt: Option<ISODate>,
        #[validate]
        #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
        pub seqnb: Option<Number>,
        #[validate]
        #[serde(rename = "Rcrd", default)]
        pub rcrd: Vec<TaxRecord2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxInformation8 {
        #[validate]
        #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
        pub cdtr: Option<TaxParty1>,
        #[validate]
        #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
        pub dbtr: Option<TaxParty2>,
        #[validate]
        #[serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none")]
        pub admstnzone: Option<Max35Text>,
        #[validate]
        #[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
        pub refnb: Option<Max140Text>,
        #[validate]
        #[serde(rename = "Mtd", skip_serializing_if = "Option::is_none")]
        pub mtd: Option<Max35Text>,
        #[validate]
        #[serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none")]
        pub ttltaxblbaseamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none")]
        pub ttltaxamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
        pub dt: Option<ISODate>,
        #[validate]
        #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
        pub seqnb: Option<Number>,
        #[validate]
        #[serde(rename = "Rcrd", default)]
        pub rcrd: Vec<TaxRecord2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxParty1 {
        #[validate]
        #[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
        pub taxid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
        pub regnid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
        pub taxtp: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxParty2 {
        #[validate]
        #[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
        pub taxid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
        pub regnid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
        pub taxtp: Option<Max35Text>,
        #[validate]
        #[serde(rename = "Authstn", skip_serializing_if = "Option::is_none")]
        pub authstn: Option<TaxAuthorisation1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxPeriod2 {
        #[validate]
        #[serde(rename = "Yr", skip_serializing_if = "Option::is_none")]
        pub yr: Option<ISODate>,
        #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
        pub tp: Option<TaxRecordPeriod1Code>,
        #[validate]
        #[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
        pub frtodt: Option<DatePeriod2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxRecord2 {
        #[validate]
        #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
        pub tp: Option<Max35Text>,
        #[validate]
        #[serde(rename = "Ctgy", skip_serializing_if = "Option::is_none")]
        pub ctgy: Option<Max35Text>,
        #[validate]
        #[serde(rename = "CtgyDtls", skip_serializing_if = "Option::is_none")]
        pub ctgydtls: Option<Max35Text>,
        #[validate]
        #[serde(rename = "DbtrSts", skip_serializing_if = "Option::is_none")]
        pub dbtrsts: Option<Max35Text>,
        #[validate]
        #[serde(rename = "CertId", skip_serializing_if = "Option::is_none")]
        pub certid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none")]
        pub frmscd: Option<Max35Text>,
        #[validate]
        #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
        pub prd: Option<TaxPeriod2>,
        #[validate]
        #[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
        pub taxamt: Option<TaxAmount2>,
        #[validate]
        #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
        pub addtlinf: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxRecordDetails2 {
        #[validate]
        #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
        pub prd: Option<TaxPeriod2>,
        #[validate]
        #[serde(rename = "Amt")]
        pub amt: ActiveOrHistoricCurrencyAndAmount,
}
