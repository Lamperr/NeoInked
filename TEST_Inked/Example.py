import math  # Import a module

# Declare variables
a = 1
b = 2

# Declare constants
PI = 3.14
MY_NAME = "John"

# Declare functions with different syntax
def add(x, y):
    return x + y

subtract = lambda x, y: x - y

def multiply(x, y=2):
    return x * y

def divide(x, y=1):
    return x / y

# Declare a class with methods
class Person:
    def __init__(self, first_name, last_name, age):
        self.first_name = first_name
        self.last_name = last_name
        self.age = age

    def full_name(self):
        return self.first_name + " " + self.last_name

# Declare a list and a dictionary
fruits = ["apple", "banana", "orange"]
person = {"name": "John", "age": 30, "city": "New York"}

# Use conditional statements
if a < b:
    print("a is less than b")
elif a == b:
    print("a is equal to b")
else:
    print("a is greater than b")

# Use loops
for i in range(5):
    print(i)

i = 0
while i < 5:
    print(i)
    i += 1

# Use try-except block
try:
    result = a / 0
    print(result)
except ZeroDivisionError as error:
    print(error)
finally:
    print("This will always execute")

# Use other keywords
c = a + b
d = a * b
e = a % b
f = a ** b
g = a < b
h = a or b
i = a and b
j = not a
# k = del person["age"]
l = type(a)
m = None
n = [1, 2, 3]
o = isinstance(n, list)
p = issubclass(Person, object)
q = self
r = super()
# s = global a
# t = nonlocal b
