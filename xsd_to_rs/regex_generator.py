from jinja2 import Environment, FileSystemLoader

def generate_regex_file(regex_constants):
    if not regex_constants:
        return
    env = Environment(loader=FileSystemLoader("templates"))
    template = env.get_template("regex.rs.j2")
    with open("regex.rs", "w") as f:
        f.write(template.render(constants=regex_constants))
