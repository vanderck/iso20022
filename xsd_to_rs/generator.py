from jinja2 import Environment, FileSystemLoader
import os
from transformer import RustStruct, RustEnum

env = Environment(loader=FileSystemLoader("templates"))

def generate_rust_code(rust_structures, original_filepath):
    base_name = os.path.basename(original_filepath).replace(".xsd", ".rs")
    with open(base_name, "w") as f:
        f.write("mod regex;\n\n")  # Include regex module at the top
        for item in rust_structures:
            if isinstance(item, RustStruct):
                template = env.get_template("struct.rs.j2")
            elif isinstance(item, RustEnum):
                template = env.get_template("enum.rs.j2")
            else:
                continue
            f.write(template.render(item=item) + "\n")

def generate_regex_file(regex_constants):
    if not regex_constants:
        return
    template = env.get_template("regex.rs.j2")
    with open("regex.rs", "w") as f:
        f.write(template.render(constants=regex_constants))