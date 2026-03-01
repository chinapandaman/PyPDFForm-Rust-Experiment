from pypdfform_rust import fill_field

def fill(name: str):
    return fill_field(name)

print(fill("foo"))
