"use strict";  // Use strict mode

// Declare variables
let a = 1;
let b = 2;

// Declare constants
const PI = 3.14;
const MY_NAME = "John";

// Declare functions with different syntax
function add(x, y) {
  return x + y;
}

const subtract = function(x, y) {
  return x - y;
};

const multiply = (x, y) => {
  return x * y;
};

const divide = (x, y) => x / y;

// Declare an object with key-value pairs
const person = {
  firstName: "John",
  lastName: "Doe",
  age: 30,
  hobbies: ["reading", "gaming", "traveling"],
  address: {
    street: "123 Main St",
    city: "New York",
    state: "NY",
    zip: "10001"
  },
  fullName: function() {
    return this.firstName + " " + this.lastName;
  }
};

// Use conditional statements
if (a < b) {
  console.log("a is less than b");
} else if (a === b) {
  console.log("a is equal to b");
} else {
  console.log("a is greater than b");
}

// Use loops
for (let i = 0; i < 5; i++) {
  console.log(i);
}

let i = 0;
while (i < 5) {
  console.log(i);
  i++;
}

// Use switch statement
const day = "Monday";
switch (day) {
  case "Monday":
    console.log("Today is Monday");
    break;
  case "Tuesday":
    console.log("Today is Tuesday");
    break;
  default:
    console.log("Today is not Monday or Tuesday");
}

// Use try-catch block
try {
  throw new Error("Something went wrong");
} catch (error) {
  console.error(error);
} finally {
  console.log("This will always execute");
}

// Use other keywords
let c = a + b;
let d = a * b;
let e = a % b;
let f = a ** b;
let g = a < b;
let h = a || b;

