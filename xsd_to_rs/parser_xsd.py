from bs4 import BeautifulSoup

def parse_xsd(filepath):
    with open(filepath, 'r') as file:
        content = file.read()
    return BeautifulSoup(content, 'xml')
