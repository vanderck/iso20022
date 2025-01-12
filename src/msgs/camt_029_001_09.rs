use regex::Regex;
use once_cell::sync::Lazy;
use validator::Validate;
#[path = "date_format.rs"] mod df;

pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:camt.029.001.09".to_string()
}
::lazy_static::lazy_static! { static ref ACTIVE_CURRENCY_CODE_REGEX: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^[A-Z]{3,3}$").unwrap()});}
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
pub struct ActiveCurrencyCode {
    #[validate(regex = "ACTIVE_CURRENCY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CancellationIndividualStatus1Code {
    #[serde(rename = "RJCR")]
    Rjcr,
    #[serde(rename = "ACCR")]
    Accr,
    #[serde(rename = "PDCR")]
    Pdcr,
    #[default]
    Unknown
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
pub struct ChargeIncludedIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ClearingChannel2Code {
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "RTNS")]
    Rtns,
    #[serde(rename = "MPNS")]
    Mpns,
    #[serde(rename = "BOOK")]
    Book,
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
::lazy_static::lazy_static! { static ref EXACT_2_NUMERIC_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^[0-9]{2}$").unwrap()});}
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
pub struct Exact2NumericText {
    #[validate(regex = "EXACT_2_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct ExternalCashClearingSystem1Code {
    #[validate(length(min = 1, max = 3,))]
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
pub struct ExternalChargeType1Code {
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
pub struct ExternalClaimNonReceiptRejection1Code {
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
pub struct ExternalInvestigationExecutionConfirmation1Code {
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
pub struct ExternalMandateSetupReason1Code {
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
pub struct ExternalPaymentCancellationRejection1Code {
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
pub struct ExternalPaymentCompensationReason1Code {
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
pub struct ExternalPaymentModificationRejection1Code {
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Frequency6Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "QURT")]
    Qurt,
    #[serde(rename = "MIAN")]
    Mian,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "ADHO")]
    Adho,
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "FRTN")]
    Frtn,
    #[default]
    Unknown
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GroupCancellationStatus1Code {
    #[serde(rename = "PACR")]
    Pacr,
    #[serde(rename = "RJCR")]
    Rjcr,
    #[serde(rename = "ACCR")]
    Accr,
    #[serde(rename = "PDCR")]
    Pdcr,
    #[default]
    Unknown
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
    #[serde(with = "df::date_format")]
    #[serde(rename = "$text")]
    pub value: ::chrono::DateTime<::chrono::Utc>,
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
pub struct Max1025Text {
    #[validate(length(min = 1, max = 1025,))]
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
pub struct Max105Text {
    #[validate(length(min = 1, max = 105,))]
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
pub enum PaymentMethod4Code {
    #[serde(rename = "CHK")]
    Chk,
    #[serde(rename = "TRF")]
    Trf,
    #[serde(rename = "DD")]
    Dd,
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
pub enum SequenceType3Code {
    #[serde(rename = "FRST")]
    Frst,
    #[serde(rename = "RCUR")]
    Rcur,
    #[serde(rename = "FNAL")]
    Fnal,
    #[serde(rename = "OOFF")]
    Ooff,
    #[serde(rename = "RPRE")]
    Rpre,
    #[default]
    Unknown
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementMethod1Code {
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "INGA")]
    Inga,
    #[serde(rename = "COVE")]
    Cove,
    #[serde(rename = "CLRG")]
    Clrg,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionIndividualStatus1Code {
    #[serde(rename = "ACTC")]
    Actc,
    #[serde(rename = "RJCT")]
    Rjct,
    #[serde(rename = "PDNG")]
    Pdng,
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ACSP")]
    Acsp,
    #[serde(rename = "ACSC")]
    Acsc,
    #[serde(rename = "ACCR")]
    Accr,
    #[serde(rename = "ACWC")]
    Acwc,
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
pub struct YesNoIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
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
pub struct ActiveCurrencyAndAmount {
    #[validate(regex = "ACTIVE_CURRENCY_CODE_REGEX")]
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
pub struct AmendmentInformationDetails13 {
        #[validate]
        #[serde(rename = "OrgnlMndtId", skip_serializing_if = "Option::is_none")]
        pub orgnlmndtid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "OrgnlCdtrSchmeId", skip_serializing_if = "Option::is_none")]
        pub orgnlcdtrschmeid: Option<PartyIdentification135>,
        #[validate]
        #[serde(rename = "OrgnlCdtrAgt", skip_serializing_if = "Option::is_none")]
        pub orgnlcdtragt: Option<BranchAndFinancialInstitutionIdentification6>,
        #[validate]
        #[serde(rename = "OrgnlCdtrAgtAcct", skip_serializing_if = "Option::is_none")]
        pub orgnlcdtragtacct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "OrgnlDbtr", skip_serializing_if = "Option::is_none")]
        pub orgnldbtr: Option<PartyIdentification135>,
        #[validate]
        #[serde(rename = "OrgnlDbtrAcct", skip_serializing_if = "Option::is_none")]
        pub orgnldbtracct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "OrgnlDbtrAgt", skip_serializing_if = "Option::is_none")]
        pub orgnldbtragt: Option<BranchAndFinancialInstitutionIdentification6>,
        #[validate]
        #[serde(rename = "OrgnlDbtrAgtAcct", skip_serializing_if = "Option::is_none")]
        pub orgnldbtragtacct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "OrgnlFnlColltnDt", skip_serializing_if = "Option::is_none")]
        pub orgnlfnlcolltndt: Option<ISODate>,
        #[validate]
        #[serde(rename = "OrgnlFrqcy", skip_serializing_if = "Option::is_none")]
        pub orgnlfrqcy: Option<Frequency36Choice>,
        #[validate]
        #[serde(rename = "OrgnlRsn", skip_serializing_if = "Option::is_none")]
        pub orgnlrsn: Option<MandateSetupReason1Choice>,
        #[validate]
        #[serde(rename = "OrgnlTrckgDays", skip_serializing_if = "Option::is_none")]
        pub orgnltrckgdays: Option<Exact2NumericText>,
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
pub struct CancellationStatusReason3Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalPaymentCancellationRejection1Code>,
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
pub struct CancellationStatusReason4 {
        #[validate]
        #[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
        pub orgtr: Option<PartyIdentification135>,
        #[validate]
        #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
        pub rsn: Option<CancellationStatusReason3Choice>,
        #[validate]
        #[serde(rename = "AddtlInf", default)]
        pub addtlinf: Vec<Max105Text>,
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
pub struct Case5 {
        #[validate]
        #[serde(rename = "Id")]
        pub id: Max35Text,
        #[validate]
        #[serde(rename = "Cretr")]
        pub cretr: Party40Choice,
        #[validate]
        #[serde(rename = "ReopCaseIndctn", skip_serializing_if = "Option::is_none")]
        pub reopcaseindctn: Option<YesNoIndicator>,
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
pub struct CaseAssignment5 {
        #[validate]
        #[serde(rename = "Id")]
        pub id: Max35Text,
        #[validate]
        #[serde(rename = "Assgnr")]
        pub assgnr: Party40Choice,
        #[validate]
        #[serde(rename = "Assgne")]
        pub assgne: Party40Choice,
        #[validate]
        #[serde(rename = "CreDtTm")]
        pub credttm: ISODateTime,
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
pub struct ChargeType3Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalChargeType1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<GenericIdentification3>,
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
pub struct Charges6 {
        #[validate]
        #[serde(rename = "TtlChrgsAndTaxAmt", skip_serializing_if = "Option::is_none")]
        pub ttlchrgsandtaxamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "Rcrd", default)]
        pub rcrd: Vec<ChargesRecord3>,
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
pub struct Charges7 {
        #[validate]
        #[serde(rename = "Amt")]
        pub amt: ActiveOrHistoricCurrencyAndAmount,
        #[validate]
        #[serde(rename = "Agt")]
        pub agt: BranchAndFinancialInstitutionIdentification6,
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
pub struct ChargesRecord3 {
        #[validate]
        #[serde(rename = "Amt")]
        pub amt: ActiveOrHistoricCurrencyAndAmount,
        #[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
        pub cdtdbtind: Option<CreditDebitCode>,
        #[validate]
        #[serde(rename = "ChrgInclInd", skip_serializing_if = "Option::is_none")]
        pub chrginclind: Option<ChargeIncludedIndicator>,
        #[validate]
        #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
        pub tp: Option<ChargeType3Choice>,
        #[validate]
        #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
        pub rate: Option<PercentageRate>,
        #[serde(rename = "Br", skip_serializing_if = "Option::is_none")]
        pub br: Option<ChargeBearerType1Code>,
        #[validate]
        #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
        pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
        #[validate]
        #[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
        pub tax: Option<TaxCharges2>,
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
pub struct ClaimNonReceipt2 {
        #[validate]
        #[serde(rename = "DtPrcd")]
        pub dtprcd: ISODate,
        #[validate]
        #[serde(rename = "OrgnlNxtAgt", skip_serializing_if = "Option::is_none")]
        pub orgnlnxtagt: Option<BranchAndFinancialInstitutionIdentification6>,
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
pub struct ClaimNonReceipt2Choice {
        #[validate]
        #[serde(rename = "Accptd", skip_serializing_if = "Option::is_none")]
        pub accptd: Option<ClaimNonReceipt2>,
        #[validate]
        #[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
        pub rjctd: Option<ClaimNonReceiptRejectReason1Choice>,
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
pub struct ClaimNonReceiptRejectReason1Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalClaimNonReceiptRejection1Code>,
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
pub struct ClearingSystemIdentification3Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalCashClearingSystem1Code>,
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
pub struct Compensation2 {
        #[validate]
        #[serde(rename = "Amt")]
        pub amt: ActiveCurrencyAndAmount,
        #[validate]
        #[serde(rename = "DbtrAgt")]
        pub dbtragt: BranchAndFinancialInstitutionIdentification6,
        #[validate]
        #[serde(rename = "CdtrAgt")]
        pub cdtragt: BranchAndFinancialInstitutionIdentification6,
        #[validate]
        #[serde(rename = "Rsn")]
        pub rsn: CompensationReason1Choice,
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
pub struct CompensationReason1Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalPaymentCompensationReason1Code>,
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
pub struct CorrectiveGroupInformation1 {
        #[validate]
        #[serde(rename = "MsgId")]
        pub msgid: Max35Text,
        #[validate]
        #[serde(rename = "MsgNmId")]
        pub msgnmid: Max35Text,
        #[validate]
        #[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
        pub credttm: Option<ISODateTime>,
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
pub struct CorrectiveInterbankTransaction2 {
        #[validate]
        #[serde(rename = "GrpHdr", skip_serializing_if = "Option::is_none")]
        pub grphdr: Option<CorrectiveGroupInformation1>,
        #[validate]
        #[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
        pub instrid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
        pub endtoendid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
        pub txid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
        pub uetr: Option<UUIDv4Identifier>,
        #[validate]
        #[serde(rename = "IntrBkSttlmAmt")]
        pub intrbksttlmamt: ActiveOrHistoricCurrencyAndAmount,
        #[validate]
        #[serde(rename = "IntrBkSttlmDt")]
        pub intrbksttlmdt: ISODate,
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
pub struct CorrectivePaymentInitiation4 {
        #[validate]
        #[serde(rename = "GrpHdr", skip_serializing_if = "Option::is_none")]
        pub grphdr: Option<CorrectiveGroupInformation1>,
        #[validate]
        #[serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none")]
        pub pmtinfid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
        pub instrid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
        pub endtoendid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
        pub uetr: Option<UUIDv4Identifier>,
        #[validate]
        #[serde(rename = "InstdAmt")]
        pub instdamt: ActiveOrHistoricCurrencyAndAmount,
        #[validate]
        #[serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none")]
        pub reqdexctndt: Option<DateAndDateTime2Choice>,
        #[validate]
        #[serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none")]
        pub reqdcolltndt: Option<ISODate>,
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
pub struct CorrectiveTransaction4Choice {
        #[validate]
        #[serde(rename = "Initn", skip_serializing_if = "Option::is_none")]
        pub initn: Option<CorrectivePaymentInitiation4>,
        #[validate]
        #[serde(rename = "IntrBk", skip_serializing_if = "Option::is_none")]
        pub intrbk: Option<CorrectiveInterbankTransaction2>,
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
        #[serde(rename = "RsltnOfInvstgtn")]
        pub rsltnofinvstgtn: ResolutionOfInvestigationV09,
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
pub struct Frequency36Choice {
        #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
        pub tp: Option<Frequency6Code>,
        #[validate]
        #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
        pub prd: Option<FrequencyPeriod1>,
        #[validate]
        #[serde(rename = "PtInTm", skip_serializing_if = "Option::is_none")]
        pub ptintm: Option<FrequencyAndMoment1>,
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
pub struct FrequencyAndMoment1 {
        #[serde(rename = "Tp")]
        pub tp: Frequency6Code,
        #[validate]
        #[serde(rename = "PtInTm")]
        pub ptintm: Exact2NumericText,
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
pub struct FrequencyPeriod1 {
        #[serde(rename = "Tp")]
        pub tp: Frequency6Code,
        #[validate]
        #[serde(rename = "CntPerPrd")]
        pub cntperprd: DecimalNumber,
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
pub struct GenericIdentification3 {
        #[validate]
        #[serde(rename = "Id")]
        pub id: Max35Text,
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
pub struct InvestigationStatus5Choice {
        #[validate]
        #[serde(rename = "Conf", skip_serializing_if = "Option::is_none")]
        pub conf: Option<ExternalInvestigationExecutionConfirmation1Code>,
        #[validate]
        #[serde(rename = "RjctdMod", skip_serializing_if = "Option::is_none")]
        pub rjctdmod: Option<ModificationStatusReason1Choice>,
        #[validate]
        #[serde(rename = "DplctOf", skip_serializing_if = "Option::is_none")]
        pub dplctof: Option<Case5>,
        #[validate]
        #[serde(rename = "AssgnmtCxlConf", skip_serializing_if = "Option::is_none")]
        pub assgnmtcxlconf: Option<YesNoIndicator>,
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
pub struct MandateRelatedInformation14 {
        #[validate]
        #[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
        pub mndtid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none")]
        pub dtofsgntr: Option<ISODate>,
        #[validate]
        #[serde(rename = "AmdmntInd", skip_serializing_if = "Option::is_none")]
        pub amdmntind: Option<TrueFalseIndicator>,
        #[validate]
        #[serde(rename = "AmdmntInfDtls", skip_serializing_if = "Option::is_none")]
        pub amdmntinfdtls: Option<AmendmentInformationDetails13>,
        #[validate]
        #[serde(rename = "ElctrncSgntr", skip_serializing_if = "Option::is_none")]
        pub elctrncsgntr: Option<Max1025Text>,
        #[validate]
        #[serde(rename = "FrstColltnDt", skip_serializing_if = "Option::is_none")]
        pub frstcolltndt: Option<ISODate>,
        #[validate]
        #[serde(rename = "FnlColltnDt", skip_serializing_if = "Option::is_none")]
        pub fnlcolltndt: Option<ISODate>,
        #[validate]
        #[serde(rename = "Frqcy", skip_serializing_if = "Option::is_none")]
        pub frqcy: Option<Frequency36Choice>,
        #[validate]
        #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
        pub rsn: Option<MandateSetupReason1Choice>,
        #[validate]
        #[serde(rename = "TrckgDays", skip_serializing_if = "Option::is_none")]
        pub trckgdays: Option<Exact2NumericText>,
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
pub struct MandateSetupReason1Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalMandateSetupReason1Code>,
        #[validate]
        #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
        pub prtry: Option<Max70Text>,
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
pub struct ModificationStatusReason1Choice {
        #[validate]
        #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
        pub cd: Option<ExternalPaymentModificationRejection1Code>,
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
pub struct ModificationStatusReason2 {
        #[validate]
        #[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
        pub orgtr: Option<PartyIdentification135>,
        #[validate]
        #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
        pub rsn: Option<ModificationStatusReason1Choice>,
        #[validate]
        #[serde(rename = "AddtlInf", default)]
        pub addtlinf: Vec<Max105Text>,
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
pub struct NumberOfCancellationsPerStatus1 {
        #[validate]
        #[serde(rename = "DtldNbOfTxs")]
        pub dtldnboftxs: Max15NumericText,
        #[serde(rename = "DtldSts")]
        pub dtldsts: CancellationIndividualStatus1Code,
        #[validate]
        #[serde(rename = "DtldCtrlSum", skip_serializing_if = "Option::is_none")]
        pub dtldctrlsum: Option<DecimalNumber>,
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
pub struct NumberOfTransactionsPerStatus1 {
        #[validate]
        #[serde(rename = "DtldNbOfTxs")]
        pub dtldnboftxs: Max15NumericText,
        #[serde(rename = "DtldSts")]
        pub dtldsts: TransactionIndividualStatus1Code,
        #[validate]
        #[serde(rename = "DtldCtrlSum", skip_serializing_if = "Option::is_none")]
        pub dtldctrlsum: Option<DecimalNumber>,
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
pub struct OriginalGroupHeader14 {
        #[validate]
        #[serde(rename = "OrgnlGrpCxlId", skip_serializing_if = "Option::is_none")]
        pub orgnlgrpcxlid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none")]
        pub rslvdcase: Option<Case5>,
        #[validate]
        #[serde(rename = "OrgnlMsgId")]
        pub orgnlmsgid: Max35Text,
        #[validate]
        #[serde(rename = "OrgnlMsgNmId")]
        pub orgnlmsgnmid: Max35Text,
        #[validate]
        #[serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none")]
        pub orgnlcredttm: Option<ISODateTime>,
        #[validate]
        #[serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none")]
        pub orgnlnboftxs: Option<Max15NumericText>,
        #[validate]
        #[serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none")]
        pub orgnlctrlsum: Option<DecimalNumber>,
        #[serde(rename = "GrpCxlSts", skip_serializing_if = "Option::is_none")]
        pub grpcxlsts: Option<GroupCancellationStatus1Code>,
        #[validate]
        #[serde(rename = "CxlStsRsnInf", default)]
        pub cxlstsrsninf: Vec<CancellationStatusReason4>,
        #[validate]
        #[serde(rename = "NbOfTxsPerCxlSts", default)]
        pub nboftxspercxlsts: Vec<NumberOfTransactionsPerStatus1>,
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
pub struct OriginalGroupInformation29 {
        #[validate]
        #[serde(rename = "OrgnlMsgId")]
        pub orgnlmsgid: Max35Text,
        #[validate]
        #[serde(rename = "OrgnlMsgNmId")]
        pub orgnlmsgnmid: Max35Text,
        #[validate]
        #[serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none")]
        pub orgnlcredttm: Option<ISODateTime>,
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
pub struct OriginalPaymentInstruction30 {
        #[validate]
        #[serde(rename = "OrgnlPmtInfCxlId", skip_serializing_if = "Option::is_none")]
        pub orgnlpmtinfcxlid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none")]
        pub rslvdcase: Option<Case5>,
        #[validate]
        #[serde(rename = "OrgnlPmtInfId")]
        pub orgnlpmtinfid: Max35Text,
        #[validate]
        #[serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none")]
        pub orgnlgrpinf: Option<OriginalGroupInformation29>,
        #[validate]
        #[serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none")]
        pub orgnlnboftxs: Option<Max15NumericText>,
        #[validate]
        #[serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none")]
        pub orgnlctrlsum: Option<DecimalNumber>,
        #[serde(rename = "PmtInfCxlSts", skip_serializing_if = "Option::is_none")]
        pub pmtinfcxlsts: Option<GroupCancellationStatus1Code>,
        #[validate]
        #[serde(rename = "CxlStsRsnInf", default)]
        pub cxlstsrsninf: Vec<CancellationStatusReason4>,
        #[validate]
        #[serde(rename = "NbOfTxsPerCxlSts", default)]
        pub nboftxspercxlsts: Vec<NumberOfCancellationsPerStatus1>,
        #[validate]
        #[serde(rename = "TxInfAndSts", default)]
        pub txinfandsts: Vec<PaymentTransaction103>,
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
pub struct OriginalTransactionReference28 {
        #[validate]
        #[serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
        pub intrbksttlmamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
        pub amt: Option<AmountType4Choice>,
        #[validate]
        #[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
        pub intrbksttlmdt: Option<ISODate>,
        #[validate]
        #[serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none")]
        pub reqdcolltndt: Option<ISODate>,
        #[validate]
        #[serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none")]
        pub reqdexctndt: Option<DateAndDateTime2Choice>,
        #[validate]
        #[serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none")]
        pub cdtrschmeid: Option<PartyIdentification135>,
        #[validate]
        #[serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none")]
        pub sttlminf: Option<SettlementInstruction7>,
        #[validate]
        #[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
        pub pmttpinf: Option<PaymentTypeInformation27>,
        #[serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none")]
        pub pmtmtd: Option<PaymentMethod4Code>,
        #[validate]
        #[serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none")]
        pub mndtrltdinf: Option<MandateRelatedInformation14>,
        #[validate]
        #[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
        pub rmtinf: Option<RemittanceInformation16>,
        #[validate]
        #[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
        pub ultmtdbtr: Option<Party40Choice>,
        #[validate]
        #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
        pub dbtr: Option<Party40Choice>,
        #[validate]
        #[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
        pub dbtracct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
        pub dbtragt: Option<BranchAndFinancialInstitutionIdentification6>,
        #[validate]
        #[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
        pub dbtragtacct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
        pub cdtragt: Option<BranchAndFinancialInstitutionIdentification6>,
        #[validate]
        #[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
        pub cdtragtacct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
        pub cdtr: Option<Party40Choice>,
        #[validate]
        #[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
        pub cdtracct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
        pub ultmtcdtr: Option<Party40Choice>,
        #[validate]
        #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
        pub purp: Option<Purpose2Choice>,
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
pub struct Party40Choice {
        #[validate]
        #[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
        pub pty: Option<PartyIdentification135>,
        #[validate]
        #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
        pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
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
pub struct PaymentTransaction102 {
        #[validate]
        #[serde(rename = "CxlStsId", skip_serializing_if = "Option::is_none")]
        pub cxlstsid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none")]
        pub rslvdcase: Option<Case5>,
        #[validate]
        #[serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none")]
        pub orgnlgrpinf: Option<OriginalGroupInformation29>,
        #[validate]
        #[serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none")]
        pub orgnlinstrid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none")]
        pub orgnlendtoendid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none")]
        pub orgnltxid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "OrgnlClrSysRef", skip_serializing_if = "Option::is_none")]
        pub orgnlclrsysref: Option<Max35Text>,
        #[validate]
        #[serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none")]
        pub orgnluetr: Option<UUIDv4Identifier>,
        #[serde(rename = "TxCxlSts", skip_serializing_if = "Option::is_none")]
        pub txcxlsts: Option<CancellationIndividualStatus1Code>,
        #[validate]
        #[serde(rename = "CxlStsRsnInf", default)]
        pub cxlstsrsninf: Vec<CancellationStatusReason4>,
        #[validate]
        #[serde(rename = "RsltnRltdInf", skip_serializing_if = "Option::is_none")]
        pub rsltnrltdinf: Option<ResolutionData1>,
        #[validate]
        #[serde(rename = "OrgnlIntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
        pub orgnlintrbksttlmamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "OrgnlIntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
        pub orgnlintrbksttlmdt: Option<ISODate>,
        #[validate]
        #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
        pub assgnr: Option<Party40Choice>,
        #[validate]
        #[serde(rename = "Assgne", skip_serializing_if = "Option::is_none")]
        pub assgne: Option<Party40Choice>,
        #[validate]
        #[serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none")]
        pub orgnltxref: Option<OriginalTransactionReference28>,
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
pub struct PaymentTransaction103 {
        #[validate]
        #[serde(rename = "CxlStsId", skip_serializing_if = "Option::is_none")]
        pub cxlstsid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none")]
        pub rslvdcase: Option<Case5>,
        #[validate]
        #[serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none")]
        pub orgnlinstrid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none")]
        pub orgnlendtoendid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
        pub uetr: Option<UUIDv4Identifier>,
        #[serde(rename = "TxCxlSts", skip_serializing_if = "Option::is_none")]
        pub txcxlsts: Option<CancellationIndividualStatus1Code>,
        #[validate]
        #[serde(rename = "CxlStsRsnInf", default)]
        pub cxlstsrsninf: Vec<CancellationStatusReason4>,
        #[validate]
        #[serde(rename = "OrgnlInstdAmt", skip_serializing_if = "Option::is_none")]
        pub orgnlinstdamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "OrgnlReqdExctnDt", skip_serializing_if = "Option::is_none")]
        pub orgnlreqdexctndt: Option<DateAndDateTime2Choice>,
        #[validate]
        #[serde(rename = "OrgnlReqdColltnDt", skip_serializing_if = "Option::is_none")]
        pub orgnlreqdcolltndt: Option<ISODate>,
        #[validate]
        #[serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none")]
        pub orgnltxref: Option<OriginalTransactionReference28>,
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
pub struct PaymentTransaction107 {
        #[validate]
        #[serde(rename = "ModStsId", skip_serializing_if = "Option::is_none")]
        pub modstsid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none")]
        pub rslvdcase: Option<Case5>,
        #[validate]
        #[serde(rename = "OrgnlGrpInf")]
        pub orgnlgrpinf: OriginalGroupInformation29,
        #[validate]
        #[serde(rename = "OrgnlPmtInfId", skip_serializing_if = "Option::is_none")]
        pub orgnlpmtinfid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none")]
        pub orgnlinstrid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none")]
        pub orgnlendtoendid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none")]
        pub orgnltxid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "OrgnlClrSysRef", skip_serializing_if = "Option::is_none")]
        pub orgnlclrsysref: Option<Max35Text>,
        #[validate]
        #[serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none")]
        pub orgnluetr: Option<UUIDv4Identifier>,
        #[validate]
        #[serde(rename = "ModStsRsnInf", default)]
        pub modstsrsninf: Vec<ModificationStatusReason2>,
        #[validate]
        #[serde(rename = "RsltnRltdInf", skip_serializing_if = "Option::is_none")]
        pub rsltnrltdinf: Option<ResolutionData1>,
        #[validate]
        #[serde(rename = "OrgnlIntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
        pub orgnlintrbksttlmamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "OrgnlIntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
        pub orgnlintrbksttlmdt: Option<ISODate>,
        #[validate]
        #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
        pub assgnr: Option<Party40Choice>,
        #[validate]
        #[serde(rename = "Assgne", skip_serializing_if = "Option::is_none")]
        pub assgne: Option<Party40Choice>,
        #[validate]
        #[serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none")]
        pub orgnltxref: Option<OriginalTransactionReference28>,
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
pub struct PaymentTypeInformation27 {
        #[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
        pub instrprty: Option<Priority2Code>,
        #[serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none")]
        pub clrchanl: Option<ClearingChannel2Code>,
        #[validate]
        #[serde(rename = "SvcLvl", default)]
        pub svclvl: Vec<ServiceLevel8Choice>,
        #[validate]
        #[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
        pub lclinstrm: Option<LocalInstrument2Choice>,
        #[serde(rename = "SeqTp", skip_serializing_if = "Option::is_none")]
        pub seqtp: Option<SequenceType3Code>,
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
pub struct ResolutionData1 {
        #[validate]
        #[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
        pub endtoendid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
        pub txid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
        pub uetr: Option<UUIDv4Identifier>,
        #[validate]
        #[serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
        pub intrbksttlmamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
        pub intrbksttlmdt: Option<ISODate>,
        #[serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none")]
        pub clrchanl: Option<ClearingChannel2Code>,
        #[validate]
        #[serde(rename = "Compstn", skip_serializing_if = "Option::is_none")]
        pub compstn: Option<Compensation2>,
        #[validate]
        #[serde(rename = "Chrgs", default)]
        pub chrgs: Vec<Charges7>,
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
pub struct ResolutionOfInvestigationV09 {
        #[validate]
        #[serde(rename = "Assgnmt")]
        pub assgnmt: CaseAssignment5,
        #[validate]
        #[serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none")]
        pub rslvdcase: Option<Case5>,
        #[validate]
        #[serde(rename = "Sts")]
        pub sts: InvestigationStatus5Choice,
        #[validate]
        #[serde(rename = "CxlDtls", default)]
        pub cxldtls: Vec<UnderlyingTransaction22>,
        #[validate]
        #[serde(rename = "ModDtls", skip_serializing_if = "Option::is_none")]
        pub moddtls: Option<PaymentTransaction107>,
        #[validate]
        #[serde(rename = "ClmNonRctDtls", skip_serializing_if = "Option::is_none")]
        pub clmnonrctdtls: Option<ClaimNonReceipt2Choice>,
        #[validate]
        #[serde(rename = "StmtDtls", skip_serializing_if = "Option::is_none")]
        pub stmtdtls: Option<StatementResolutionEntry4>,
        #[validate]
        #[serde(rename = "CrrctnTx", skip_serializing_if = "Option::is_none")]
        pub crrctntx: Option<CorrectiveTransaction4Choice>,
        #[validate]
        #[serde(rename = "RsltnRltdInf", skip_serializing_if = "Option::is_none")]
        pub rsltnrltdinf: Option<ResolutionData1>,
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
pub struct SettlementInstruction7 {
        #[serde(rename = "SttlmMtd")]
        pub sttlmmtd: SettlementMethod1Code,
        #[validate]
        #[serde(rename = "SttlmAcct", skip_serializing_if = "Option::is_none")]
        pub sttlmacct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "ClrSys", skip_serializing_if = "Option::is_none")]
        pub clrsys: Option<ClearingSystemIdentification3Choice>,
        #[validate]
        #[serde(rename = "InstgRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
        pub instgrmbrsmntagt: Option<BranchAndFinancialInstitutionIdentification6>,
        #[validate]
        #[serde(rename = "InstgRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none")]
        pub instgrmbrsmntagtacct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "InstdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
        pub instdrmbrsmntagt: Option<BranchAndFinancialInstitutionIdentification6>,
        #[validate]
        #[serde(rename = "InstdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none")]
        pub instdrmbrsmntagtacct: Option<CashAccount38>,
        #[validate]
        #[serde(rename = "ThrdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
        pub thrdrmbrsmntagt: Option<BranchAndFinancialInstitutionIdentification6>,
        #[validate]
        #[serde(rename = "ThrdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none")]
        pub thrdrmbrsmntagtacct: Option<CashAccount38>,
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
pub struct StatementResolutionEntry4 {
        #[validate]
        #[serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none")]
        pub orgnlgrpinf: Option<OriginalGroupInformation29>,
        #[validate]
        #[serde(rename = "OrgnlStmtId", skip_serializing_if = "Option::is_none")]
        pub orgnlstmtid: Option<Max35Text>,
        #[validate]
        #[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
        pub uetr: Option<UUIDv4Identifier>,
        #[validate]
        #[serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none")]
        pub acctsvcrref: Option<Max35Text>,
        #[validate]
        #[serde(rename = "CrrctdAmt", skip_serializing_if = "Option::is_none")]
        pub crrctdamt: Option<ActiveOrHistoricCurrencyAndAmount>,
        #[validate]
        #[serde(rename = "Chrgs", default)]
        pub chrgs: Vec<Charges6>,
        #[validate]
        #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
        pub purp: Option<Purpose2Choice>,
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
pub struct TaxCharges2 {
        #[validate]
        #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
        pub id: Option<Max35Text>,
        #[validate]
        #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
        pub rate: Option<PercentageRate>,
        #[validate]
        #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
        pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
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
pub struct UnderlyingTransaction22 {
        #[validate]
        #[serde(rename = "OrgnlGrpInfAndSts", skip_serializing_if = "Option::is_none")]
        pub orgnlgrpinfandsts: Option<OriginalGroupHeader14>,
        #[validate]
        #[serde(rename = "OrgnlPmtInfAndSts", default)]
        pub orgnlpmtinfandsts: Vec<OriginalPaymentInstruction30>,
        #[validate]
        #[serde(rename = "TxInfAndSts", default)]
        pub txinfandsts: Vec<PaymentTransaction102>,
}
