import sys
from parser import parse_xsd
from transformer import transform_schema
from generator import generate_rust_code
from regex_generator import generate_regex_file


def main():
    if len(sys.argv) < 2:
        print("Usage: python main.py <schema.xsd>")
        return

    filepath = sys.argv[1]
    soup = parse_xsd(filepath)
    rust_structures, regex_constants = transform_schema(soup)
    generate_rust_code(rust_structures, filepath)
    generate_regex_file(regex_constants)


if __name__ == "__main__":
    main()
