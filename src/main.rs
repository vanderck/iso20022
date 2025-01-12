use std::env;
use std::fs;

#[path = "customer_to_bank_credit_transfer_information.rs"] mod customer_to_bank_credit_transfer_information;
/* 
#[path = "msgs/camt_029_001_09.rs"] mod camt_029_001_09;
#[path = "msgs/camt_027_001_07.rs"] mod camt_027_001_07;
#[path = "msgs/camt_056_001_08.rs"] mod camt_056_001_08;
#[path = "msgs/pacs_002_001_10.rs"] mod pacs_002_001_10;
#[path = "msgs/pacs_003_001_08.rs"] mod pacs_003_001_08;
#[path = "msgs/pacs_004_001_09.rs"] mod pacs_004_001_09;
#[path = "msgs/pacs_007_001_09.rs"] mod pacs_007_001_09;
#[path = "msgs/pacs_008_001_08.rs"] mod pacs_008_001_08;
#[path = "msgs/pacs_028_001_03.rs"] mod pacs_028_001_03;
#[path = "msgs/pain_007_001_09.rs"] mod pain_007_001_09;
#[path = "msgs/pain_008_001_08.rs"] mod pain_008_001_08;
#[path = "msgs/pain_009_001_06.rs"] mod pain_009_001_06;
#[path = "msgs/pain_010_001_06.rs"] mod pain_010_001_06;
#[path = "msgs/pain_011_001_06.rs"] mod pain_011_001_06;
#[path = "msgs/pain_012_001_06.rs"] mod pain_012_001_06;
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("File path {}", file_path);
    let paths = fs::read_dir(file_path).unwrap();
    for path in paths {
        let file = path.unwrap();
        if !file.file_type().unwrap().is_file() {
            continue
        }
        let filename = file.file_name().into_string().unwrap();
        println!("Found {}", filename);
        if !filename.ends_with(".xml") {
            println!("Not .xml {}", filename);
            continue
        }
        let contents = fs::read_to_string(file.path());
        if contents.is_err() {
            println!("Error read_to_string : {:?}", contents);
            continue
        }
        let xml = contents.unwrap();
        let pos = xml.find("xmlns=\"urn:iso:std:iso:20022:tech:xsd:");
        if pos.is_none() {
            println!("Error finding xmlns : {:?}", xml);
            continue
        }
        let foundat = pos.unwrap() + 38;
        let pos2 = xml[foundat..].find("\"").map(|i| i + foundat);
        if pos2.is_none() {
            println!("Error finding end of xmlns : {:?}", xml);
            continue
        }
        let substring = &xml[foundat..pos2.unwrap()];
        //https://support.axway.com/doc/a9caac64ccfe70d59e67d89e01aa7ec1/Content/Supported_msgs/Payment%20Initiation/sepa.htm
        match substring {
            "pain.001.001.09" => {
                let response = customer_to_bank_credit_transfer_information::process(&xml);
                if response.is_none() {
                    continue;
                }
                println!("Written : {:?}", response);
            },
            /* 
            "pain.002.001.10" => {
            },
            "pain.008.001.08" => {
                let doc: Result<pain_008_001_08::Document, DeError> = quick_xml::de::from_str(&xml);
                if doc.is_err() {
                    println!("Error from_str : {:?}", doc);
                    return;
                }
                let xml = doc.unwrap();
                let val = xml.validate();
                if val.is_err() {
                    println!("Error validate : {:?}", val);
                    return;
                }
                //println!("{:?}", xml);
            },
            "pacs.008.001.08" => {
                let doc: Result<pacs_008_001_08::Document, DeError> = quick_xml::de::from_str(&xml);
                if doc.is_err() {
                    println!("Error from_str : {:?}", doc);
                    return;
                }
                let xml = doc.unwrap();
                let val = xml.validate();
                if val.is_err() {
                    println!("Error validate : {:?}", val);
                    return;
                }
                //println!("{:?}", xml);
            },
            "camt.029.001.09" => {
                let doc: Result<camt_029_001_09::Document, DeError> = quick_xml::de::from_str(&xml);
                if doc.is_err() {
                    println!("Error from_str : {:?}", doc);
                    return;
                }
                let xml = doc.unwrap();
                let val = xml.validate();
                if val.is_err() {
                    println!("Error validate : {:?}", val);
                    return;
                }
                //println!("{:?}", xml);
            },
            */
            _ => {
                println!("Not processing {}", substring);
                continue;
            }
        }
    }
}
