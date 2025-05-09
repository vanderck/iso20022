from dataclasses import dataclass
from typing import List, Optional, Union, Tuple

@dataclass
class RustField:
    name: str
    type_: str
    serde_rename: Optional[str] = None
    validate: Optional[str] = None
    is_optional: bool = False
    is_list: bool = False

@dataclass
class RustStruct:
    name: str
    fields: List[RustField]
    derives: List[str] = None

@dataclass
class RustEnum:
    name: str
    variants: List[str]
    derives: List[str] = None

def to_regex_name(xsd_name: str) -> str:
    result = ""
    prev = ""
    for ch in xsd_name:
        if prev and ch.isupper() and (prev.islower() or prev.isdigit()):
            result += "_"
        result += ch.upper()
        prev = ch
    return result + "_REGEX"

def transform_schema(soup) -> Tuple[List[Union[RustStruct, RustEnum]], List[Tuple[str, str]]]:
    results = []
    regex_constants = []

    simple_types = soup.find_all("xs:simpleType")
    for stype in simple_types:
        restriction = stype.find("xs:restriction")
        if restriction:
            base = restriction['base']
            name = stype['name']

            pattern_tag = restriction.find("xs:pattern")
            if pattern_tag:
                regex_name = to_regex_name(name)
                regex_constants.append((regex_name, pattern_tag['value']))

            if base == "xs:string":
                enums = restriction.find_all("xs:enumeration")
                if enums:
                    results.append(RustEnum(
                        name=name,
                        variants=[e['value'] for e in enums],
                        derives=["serde::Serialize", "serde::Deserialize"]
                    ))
                else:
                    validate = f"regex = \"{to_regex_name(name)}\"" if pattern_tag else None
                    results.append(RustStruct(
                        name=name,
                        fields=[RustField(name="value", type_="String", validate=validate)],
                        derives=["serde::Serialize", "serde::Deserialize", "validator::Validate"]
                    ))
            elif base == "xs:decimal":
                min_inclusive = restriction.find("xs:minInclusive")
                validate = None
                if min_inclusive:
                    validate = f"range(min = {min_inclusive['value']})"
                results.append(RustStruct(
                    name=name,
                    fields=[RustField(name="value", type_="f64", validate=validate)],
                    derives=["serde::Serialize", "serde::Deserialize", "validator::Validate"]
                ))
            elif base == "xs:boolean":
                results.append(RustStruct(
                    name=name,
                    fields=[RustField(name="value", type_="bool")],
                    derives=["serde::Serialize", "serde::Deserialize"]
                ))
            elif base == "xs:date":
                results.append(RustStruct(
                    name=name,
                    fields=[RustField(name="value", type_="::chrono::NaiveDate")],
                    derives=["serde::Serialize", "serde::Deserialize"]
                ))
            elif base == "xs:dateTime":
                results.append(RustStruct(
                    name=name,
                    fields=[RustField(name="value", type_="::chrono::DateTime<::chrono::Utc>")],
                    derives=["serde::Serialize", "serde::Deserialize"]
                ))
            elif base == "xs:base64Binary":
                results.append(RustStruct(
                    name=name,
                    fields=[RustField(name="value", type_="String", validate="length(min = 1, max = 10240)")],
                    derives=["serde::Serialize", "serde::Deserialize", "validator::Validate"]
                ))

    complex_types = soup.find_all("xs:complexType")
    for ctype in complex_types:
        name = ctype.get('name')
        fields = []
        sequence = ctype.find("xs:sequence")
        if sequence:
            for element in sequence.find_all("xs:element"):
                fname = element['name']
                ftype = element.get('type', 'String')
                min_occurs = int(element.get('minOccurs', '1'))
                max_occurs = element.get('maxOccurs', '1')

                is_optional = min_occurs == 0
                is_list = max_occurs == 'unbounded' or int(max_occurs) > 1

                fields.append(RustField(
                    name=fname.lower(),
                    type_=ftype,
                    serde_rename=fname,
                    is_optional=is_optional,
                    is_list=is_list
                ))

        if fields:
            results.append(RustStruct(
                name=name,
                fields=fields,
                derives=["serde::Serialize", "serde::Deserialize", "validator::Validate"]
            ))

    return results, regex_constants

