#[path = "msgs/pain_001_001_09.rs"]
pub mod pain_001_001_09;
#[path = "msgs/pain_002_001_10.rs"]
pub mod pain_002_001_10;

use std::{fs, io::Write};

use chrono::Utc;
use quick_xml::DeError;
use uuid::Uuid;
use validator::{HasLen, Validate};

pub fn process(xml: &String) -> Option<String> {
    let doc: Result<pain_001_001_09::Document, DeError> = quick_xml::de::from_str(xml);
    if doc.is_err() {
        println!("Error from_str : {:?}", doc);
        return None;
    }
    let xml = doc.unwrap();
    let my_uuid = Uuid::new_v4().to_string();
    let mut response: pain_002_001_10::DocumentBuilder =
        pain_002_001_10::DocumentBuilder::default();
    response.xmlns(pain_002_001_10::namespace());
    let val = xml.validate();
    if val.is_err() {
        println!("Error validate : {:?}", val);
    }
    if xml.cstmrcdttrfinitn.grphdr.nboftxs.value != xml.cstmrcdttrfinitn.pmtinf.length().to_string()
    {
    }
    let pmtinf = xml.cstmrcdttrfinitn.pmtinf;
    response.cstmrpmtstsrpt(pain_002_001_10::CustomerPaymentStatusReportV10 {
        grphdr: pain_002_001_10::GroupHeader86 {
            msgid: pain_002_001_10::Max35Text { value: my_uuid.clone().into() },
            credttm: pain_002_001_10::ISODateTime { value: Utc::now() },
            initgpty: Some(pain_002_001_10::PartyIdentification135 {
                id: Some(pain_002_001_10::Party38Choice {
                    orgid: Some(pain_002_001_10::OrganisationIdentification29 { 
                        anybic: Some(pain_002_001_10::AnyBICDec2014Identifier { value: "test".to_string() }),
                        ..Default::default()
                     }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
        orgnlgrpinfandsts: pain_002_001_10::OriginalGroupHeader17 {
            orgnlmsgid: pain_002_001_10::Max35Text { value: xml.cstmrcdttrfinitn.grphdr.msgid.value },
            orgnlmsgnmid: pain_002_001_10::Max35Text { value: "pain.001.001.09".into() },
            orgnlcredttm: Some(pain_002_001_10::ISODateTime { value: xml.cstmrcdttrfinitn.grphdr.credttm.value }),
            //orgnlnboftxs: Some(pain_002_001_10::Max15NumericText { value: "".into() }),
            //orgnlctrlsum: Some(pain_002_001_10::DecimalNumber { value: 0.0 }),
            grpsts: Some(pain_002_001_10::ExternalPaymentGroupStatus1Code { value: "RJCT".into() }),
            //stsrsninf: vec![],
            //nboftxspersts: vec![],
            ..Default::default()
        },
        orgnlpmtinfandsts: vec![],
        splmtrydata: vec![],
        ..Default::default()
    });







    
    let response = response.build();
    if response.is_err() {
        println!("Error : {:?}", response);
        return None;
    }
    let response = response.unwrap();
    let val = response.validate();
    if val.is_err() {
        println!("Error : {:?}", val);
        return None;
    }
    let xml = quick_xml::se::to_string(&response).expect("Didn't serialize");

    let file_path = my_uuid + ".xml";
    let file = fs::File::create(&file_path);
    if file.is_err() {
        println!("Error : {:?}", val);
        return None;
    }
    let mut f = file.unwrap();
    f.write_all(xml.as_bytes());
    return Some(file_path);
}

/*
let mut doc: pain_001_001_09::DocumentBuilder= pain_001_001_09::DocumentBuilder::default();
doc.xmlns("urn:iso:std:iso:20022:tech:xsd:pain.001.001.09".into());
doc.cstmrcdttrfinitn(pain_001_001_09::CustomerCreditTransferInitiationV09 {
    grphdr: pain_001_001_09::GroupHeader85 {
        msgid: pain_001_001_09::Max35Text {
            value: "PAYINVOICES-APRIL23".into(),
        },
        credttm: pain_001_001_09::ISODateTime { value: Utc::now() },
        nboftxs: pain_001_001_09::Max15NumericText { value: "2".into() },
        ctrlsum: Some(pain_001_001_09::DecimalNumber { value: 100.0 }),
        initgpty: pain_001_001_09::PartyIdentification135 {
            nm: Some(pain_001_001_09::Max140Text {
                value: "EUROBANK DEBTOR LTD".into(),
            }),
            id: Some(pain_001_001_09::Party38Choice {
                //value: pain_001_001_09::Party38ChoiceEnum {
                    orgid: Some(pain_001_001_09::OrganisationIdentification29 {
                        anybic: Some(
                            pain_001_001_09::AnyBICDec2014Identifier {
                                value: "ERBKCY2N".into(),
                            }
                        ),
                        othr: vec![pain_001_001_09::GenericOrganisationIdentification1 {
                            id: pain_001_001_09::Max35Text {
                                value: "28733".into(),
                            },
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            //}),
            ),
            ..Default::default()
        },
        authstn: vec![pain_001_001_09::Authorisation1Choice {
                cd: Some(pain_001_001_09::Authorisation1Code::Auth),
                ..Default::default()
        }],
        ..Default::default()
    },
    pmtinf: vec![pain_001_001_09::PaymentInstruction30 {
        pmtinfid: pain_001_001_09::Max35Text {
            value: "PAYINVOICES-APRIL23".into(),
        },
        pmtmtd: pain_001_001_09::PaymentMethod3Code::Trf,
        btchbookg: Some(pain_001_001_09::BatchBookingIndicator{value: false}),
        nboftxs: Some(pain_001_001_09::Max15NumericText { value: "2".into() }),
        ctrlsum: Some(pain_001_001_09::DecimalNumber { value: 100.0 }),
        pmttpinf: Some(pain_001_001_09::PaymentTypeInformation26 {
            instrprty: Some(pain_001_001_09::Priority2Code::Norm),
            ..Default::default()
        }),
        reqdexctndt: pain_001_001_09::DateAndDateTime2Choice {
                dt: Some(pain_001_001_09::ISODate {
                    value: chrono::NaiveDate::from_ymd_opt(2023, 6, 5).unwrap(),
                }),
                ..Default::default()
        },
        dbtr: pain_001_001_09::PartyIdentification135 {
            nm: Some(pain_001_001_09::Max140Text { value: "EUROBANK DEBTOR LTD".into()}),
            id: Some(pain_001_001_09::Party38Choice {
                //value: pain_001_001_09::Party38ChoiceEnum {
                    orgid: Some(pain_001_001_09::OrganisationIdentification29 {
                        othr: vec![pain_001_001_09::GenericOrganisationIdentification1 {
                            id: pain_001_001_09::Max35Text {
                                value: "28733".into(),
                            },
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            //}),
            ),
            /*
            pstladr: Some(pain_001_001_09::PostalAddress24 {
                strtnm: Some(pain_001_001_09::Max70Text {
                    value: "Alt Karlo".into(),
                }),
                    ctry: Some(pain_001_001_09::CountryCode { value: "DE".into() }),
                bldgnb: Some(pain_001_001_09::Max16Text { value: "31a".into() }),
                pstcd: Some(pain_001_001_09::Max16Text { value: "3 OG".into()}),
                twnnm: Some(pain_001_001_09::Max35Text { value: "Bayern".into()}),
                ..Default::default()
            }),
            */
            ..Default::default()
        },
        dbtracct: pain_001_001_09::CashAccount38 {
            id: pain_001_001_09::AccountIdentification4Choice {
                    iban: Some(pain_001_001_09::IBAN2007Identifier {
                        value: "CY43018000010000700000007641".into()
                    }),
                    ..Default::default()
            },
            ccy: Some(pain_001_001_09::ActiveOrHistoricCurrencyCode {
                value: "EUR".into()
            }),
            ..Default::default()
        },
        dbtragt: pain_001_001_09::BranchAndFinancialInstitutionIdentification6{
            fininstnid: pain_001_001_09::FinancialInstitutionIdentification18 {
                bicfi: Some(pain_001_001_09::BICFIDec2014Identifier{
                    value: "ERBKCY2N".into()
                }),
                ..Default::default()
            },
            ..Default::default()
        },
        cdttrftxinf: vec![
            pain_001_001_09::CreditTransferTransaction34 {
                pmtid: pain_001_001_09::PaymentIdentification6 {
                    instrid: Some(pain_001_001_09::Max35Text { value: "PAY001".into() }),
                    endtoendid: pain_001_001_09::Max35Text { value: "001".into() },
                ..Default::default()
                },
                amt: pain_001_001_09::AmountType4Choice {
                    instdamt: Some(
                        pain_001_001_09::ActiveOrHistoricCurrencyAndAmount {
                             value: 50.0 ,
                             ccy: "EUR".into()
                            }),
                    ..Default::default()
                 },
                 chrgbr: Some(pain_001_001_09::ChargeBearerType1Code::Debt),
                 ultmtdbtr: Some(pain_001_001_09::PartyIdentification135 {
                    nm: Some(pain_001_001_09::Max140Text{ value: "EUROBANK DEBTOR LTD".into() }),
                    id: Some(pain_001_001_09::Party38Choice{
                        orgid: Some(pain_001_001_09::OrganisationIdentification29 {
                            anybic: Some(
                                pain_001_001_09::AnyBICDec2014Identifier {
                                    value: "ERBKCY2N".into(),
                                }
                            ),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                 }),
                 cdtragt: Some(pain_001_001_09::BranchAndFinancialInstitutionIdentification6 {
                    fininstnid: pain_001_001_09::FinancialInstitutionIdentification18 {
                        bicfi: Some(pain_001_001_09::BICFIDec2014Identifier { value: "REVOLT21".into() }),
                        ..Default::default()
                    }, ..Default::default()}),
                 cdtr: Some(pain_001_001_09::PartyIdentification135 {
                    nm: Some(pain_001_001_09::Max140Text { value: "BENEFICIARY 1".into() }),
                    ..Default::default()
                }),
                 cdtracct: Some(pain_001_001_09::CashAccount38 {
                    id: pain_001_001_09::AccountIdentification4Choice {
                        iban: Some(pain_001_001_09::IBAN2007Identifier { value: "LT493250004172871093".into() }),
                        ..Default::default()
                    },
                    ..Default::default()
                 }),
                 rmtinf: Some(pain_001_001_09::RemittanceInformation16 { ustrd: vec![
                    pain_001_001_09::Max140Text {
                        value: "INVOICE 1".into(),
                    }],
                    ..Default::default()
                 }),
                 ..Default::default()
            },

            pain_001_001_09::CreditTransferTransaction34 {
                pmtid: pain_001_001_09::PaymentIdentification6 {
                    instrid: Some(pain_001_001_09::Max35Text { value: "PAID002".into() }),
                    endtoendid: pain_001_001_09::Max35Text { value: "002".into() },
                ..Default::default()
                },
                amt: pain_001_001_09::AmountType4Choice {
                    instdamt: Some(
                        pain_001_001_09::ActiveOrHistoricCurrencyAndAmount {
                            value: 50.0 ,
                             ccy: "EUR".into()
                            }),
                    ..Default::default()
                 },
                 chrgbr: Some(pain_001_001_09::ChargeBearerType1Code::Shar),
                 ultmtdbtr: Some(pain_001_001_09::PartyIdentification135 {
                    nm: Some(pain_001_001_09::Max140Text{ value: "EUROBANK DEBTOR LTD".into() }),
                    id: Some(pain_001_001_09::Party38Choice{
                        orgid: Some(pain_001_001_09::OrganisationIdentification29 {
                            anybic: Some(
                                pain_001_001_09::AnyBICDec2014Identifier {
                                    value: "ERBKCY2N".into(),
                                }
                            ),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                 }),
                 cdtragt: Some(pain_001_001_09::BranchAndFinancialInstitutionIdentification6 {
                    fininstnid: pain_001_001_09::FinancialInstitutionIdentification18 {
                        bicfi: Some(pain_001_001_09::BICFIDec2014Identifier { value: "BCYPCY2N".into() }),
                        ..Default::default()
                    }, ..Default::default()}),
                 cdtr: Some(pain_001_001_09::PartyIdentification135 {
                    nm: Some(pain_001_001_09::Max140Text { value: "BENEFICIARY 2".into() }),
                    ..Default::default()
                }),
                 cdtracct: Some(pain_001_001_09::CashAccount38 {
                    id: pain_001_001_09::AccountIdentification4Choice {
                        iban: Some(pain_001_001_09::IBAN2007Identifier { value: "CY33002001950000357017062617".into() }),
                        ..Default::default()
                    },
                    ..Default::default()
                 }),
                 rmtinf: Some(pain_001_001_09::RemittanceInformation16 { ustrd: vec![
                    pain_001_001_09::Max140Text {
                        value: "INVOICE 2".into(),
                    }],
                    ..Default::default()
                 }),
                 ..Default::default()
            },
        ],
        ..Default::default()
    }],
    splmtrydata: vec![]
});
let doc =  doc.build();
if doc.is_err() {
    println!("Error : {:?}", doc);
    return;
}
let doc = doc.unwrap();
let val = doc.validate();
if val.is_err() {
    println!("Error : {:?}", val);
    return;
}
let xml = quick_xml::se::to_string(&doc).expect("Didn't serialize");
println!("{:?}", xml);
let doc: Result<pain_001_001_09::Document, DeError> = quick_xml::de::from_str(&xml);
if doc.is_err() {
    println!("Error from_str : {:?}", doc);
    return;
}
let doc = doc.unwrap();
let val = doc.validate();
if val.is_err() {
    println!("Error validate : {:?}", val);
    return;
}
*/
