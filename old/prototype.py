from bs4 import BeautifulSoup
import string
import logging
import sys
print (sys.argv)
firstParameter = sys.argv[1]
content = []
# Read the XML file
with open(firstParameter, "r") as file:
    content = file.readlines()
content = "".join(content)
value = firstParameter.split("/")
prefix = value[-1]
schema = prefix.replace('.xsd', '')
prefix = schema.replace('.', '_')
print ('full path', prefix)
logging.basicConfig(filename=prefix + '.rs', format='')
logging.getLogger().setLevel(logging.DEBUG)
def regex_name(string_name):
  regexname = ""
  previouselement = ""
  for element in string_name:
    if previouselement == "":
       regexname = regexname + element
    elif list(string.ascii_uppercase).__contains__(element):
       if list(string.ascii_uppercase).__contains__(previouselement):
            regexname = regexname + element
       elif list(string.ascii_lowercase).__contains__(element):
            regexname = regexname + "_" + element.upper() 
       else:
            regexname = regexname + "_" + element 
    elif list(string.digits).__contains__(element):
       if list(string.digits).__contains__(previouselement):
            regexname = regexname + element 
       else:
            regexname = regexname + "_" + element.upper() 
    elif list(string.ascii_lowercase).__contains__(element):
        regexname = regexname + element.upper() 
    previouselement = element
  return regexname + "_REGEX"

header = '''#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::derive_builder::Builder, ::validator::Validate)]
pub struct '''
enumheader = '''#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum '''

soup = BeautifulSoup(content, 'xml')
logging.debug('''use regex::Regex;
use once_cell::sync::Lazy;
use validator::Validate;
#[path = "date_format.rs"] mod df;
''')

#::lazy_static::lazy_static! {
#    static ref MAX_10_K_BINARY_REGEX: Lazy<Regex> = Lazy::new(|| {Regex::new(r"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?").unwrap()});
#}
#::lazy_static::lazy_static! {
#    static ref ISO_YEAR_REGEX: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^-?\d{4}([+-]\d{2}:\d{2}|Z)?$").unwrap()});
#}
#::lazy_static::lazy_static! {
#    static ref ISO_YEAR_MONTH_REGEX: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^-?\d{4}-(0[1-9]|1[0-2])([+-]\d{2}:\d{2}|Z)?$").unwrap()});
#}
txt1 = '''
pub fn namespace() -> String {{
    "urn:iso:std:iso:20022:tech:xsd:{0}".to_string()
}}'''.format(schema)
logging.debug(txt1)
all_xml_element_tags = soup.find_all("xs:simpleType") # You can also use this too: all_xml_element_tags = soup.find_all("xs:element", {"minOccurs":"0"})
for tag in all_xml_element_tags:
    pattern = tag.find("xs:pattern")
    if pattern is not None:
        txt1 = '''::lazy_static::lazy_static! {'''
        txt1 += ''' static ref {0}: Lazy<Regex> = Lazy::new(|| {{Regex::new(r"^{1}$").unwrap()}});'''.format(regex_name(tag['name']), pattern['value'])
        txt1 += '''}'''
        logging.debug(txt1)
    type = tag.find("xs:restriction")
    if type['base'] == "xs:decimal":
        min = type.find('xs:minInclusive')
        minstr = ''
        if min is not None:
            minstr = '''
    #[validate(range(min = 0.0))]'''
        txt1 = '''{0}{1} {{{2}
    #[serde(rename = "$text")]
    pub value: f64,
}}'''.format(header, tag['name'], minstr)
        logging.debug(txt1)
    elif type['base'] == "xs:string":
          pattern = tag.find_all("xs:enumeration")
          if len(pattern) > 0:
              txt1 = '''{0}{1} {{
'''.format(enumheader, tag['name'])
              for enum in pattern:
                txt1 += '''    #[serde(rename = "{0}")]
    {1},
'''.format(enum['value'], enum['value'].lower().capitalize())
              txt1 += '''    #[default]
    Unknown
}'''
              logging.debug(txt1)
          pattern = tag.find("xs:pattern")
          if pattern is not None:
            txt1 = '''{0}{1} {{
    #[validate(regex = "{2}")]
    #[serde(rename = "$text")]
    pub value: String,
}}'''.format(header, tag['name'], regex_name(tag['name']))
            logging.debug(txt1)
          pattern = tag.find("xs:maxLength")
          pattern2 = tag.find("xs:minLength")
          if pattern is not None:
            txt1 = '''{0}{1} {{
    #[validate(length(min = {2}, max = {3},))]
    #[serde(rename = "$text")]
    pub value: String,
}}'''.format(header, tag['name'], pattern2['value'], pattern['value'])
            logging.debug(txt1)
    elif type['base'] == "xs:boolean":
        txt1 = '''{0}{1} {{
    #[serde(rename = "$text")]
    pub value: bool,
}}'''.format(header, tag['name'])
        logging.debug(txt1)
    elif type['base'] == "xs:base64Binary":
        txt1 = '''{0}{1} {{
    #[validate(length(min = 1, max = 10240,), regex = "MAX_10_K_BINARY_REGEX")]
    pub value: String,
}}'''.format(header, tag['name'])
        logging.debug(txt1)
    elif type['base'] == "xs:date":
        txt1 = '''{0}{1} {{
    #[serde(rename = "$text")]
    pub value: ::chrono::NaiveDate,
}}'''.format(header, tag['name'])
        logging.debug(txt1)
    elif type['base'] == "xs:dateTime":
        txt1 = '''{0}{1} {{
    #[serde(with = "df::date_format")]
    #[serde(rename = "$text")]
    pub value: ::chrono::DateTime<::chrono::Utc>,
}}'''.format(header, tag['name'])
        logging.debug(txt1)
    elif type['base'] == "xs:time":
        txt1 = '''{0}{1} {{
    #[serde(rename = "$value")]
    pub value: ::chrono::NaiveTime,
}}'''.format(header, tag['name'])
        logging.debug(txt1)
    elif type['base'] == "xs:gYear":
        txt1 = '''{0}{1} {{
    #[validate(regex = "ISO_YEAR_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}}'''.format(header, tag['name'])
        logging.debug(txt1)
    elif type['base'] == "xs:gYearMonth":
        txt1 = '''{0}{1} {{
    #[validate(regex = "ISO_YEAR_MONTH_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}}'''.format(header, tag['name'])
        logging.debug(txt1)
    else:
        print(type['base'])

all_xml_element_tags = soup.find_all("xs:complexType")
for tag in all_xml_element_tags:
    pattern = tag.find("xs:choice")
    if pattern is not None:
        txt1 = '''{0}{1} {{'''.format(header, tag['name']) 
        pattern = tag.find_all("xs:element")
        for enum in pattern:
            txt1 += '''
        #[validate]
        #[serde(rename = "{0}", skip_serializing_if = "Option::is_none")]
        pub {1}: Option<{2}>,'''.format(enum['name'], enum['name'].lower(), enum['type'])
        txt1 += '''
}'''
#{0}{1} {{
#    pub value: {2}
#}}'''.format(header, tag['name'], tag['name'] + 'Enum') #wrong
        logging.debug(txt1)
        continue
    pattern = tag.find("xs:sequence")
    if pattern is not None:        
        txt1 = '''{0}{1} {{'''.format(header, tag['name'])
        if tag['name'] == "Document":
            txt1 += '''
        #[serde(rename = "@xmlns", default = "namespace")]
        pub xmlns: String,'''
        pattern = tag.find_all("xs:element")
        for enum in pattern:
            if enum.has_attr('minOccurs'):
                min = enum['minOccurs']
                max = enum['maxOccurs']
                if max == "1":
                    txt1 += '''
        #[validate]
        #[serde(rename = "{0}", skip_serializing_if = "Option::is_none")]
        pub {1}: Option<{2}>,'''.format(enum['name'], enum['name'].lower(), enum['type'])
                elif max == 'unbounded' and min == '0':
                    txt1 += '''
        #[validate]
        #[serde(rename = "{1}", default)]
        pub {2}: Vec<{3}>,'''.format(min, enum['name'], enum['name'].lower(), enum['type'])
                elif max == 'unbounded':
                    txt1 += '''
        #[validate]
        #[validate(length(min = {0}))]
        #[serde(rename = "{1}", default)]
        pub {2}: Vec<{3}>,'''.format(min, enum['name'], enum['name'].lower(), enum['type'])
                elif len(max) > 0 and len(min) > 0:
                    txt1 += '''
        #[validate]
        #[validate(length(min = {0},max = {1}))]
        #[serde(rename = "{2}", default)]
        pub {3}: Vec<{4}>,'''.format(min, max, enum['name'], enum['name'].lower(), enum['type'])
                else:
                    print("err")
            else: 
                txt1 += '''
        #[validate]
        #[serde(rename = "{0}")]
        pub {1}: {2},'''.format(enum['name'], enum['name'].lower(), enum['type'])
        txt1 += '''
}'''
        logging.debug(txt1)
        continue
    pattern = tag.find("xs:simpleContent")
    if pattern is not None:
        extension = tag.find("xs:extension")
        attribute = tag.find("xs:attribute")
        txt1 = '''{0}{1} {{
        #[validate]
        #[serde(rename = "{2}")]
        pub value: {3},
        #[validate]
        #[serde(rename = "@{4}")] 
        pub {5}: {6},
}}'''.format(header, tag['name'],tag['name'], extension['base'], attribute['name'], attribute['name'].lower(), attribute['type'])
        logging.debug(txt1)

#remove validate on enums
#_simpletype needs to be duplicated to the extension
