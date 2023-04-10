#include <iostream>
#include <string>

struct Bank {
    int money = 0;
    std::string accountName;
};

void printInfo(Bank Wells) {
    std::cout << "Dear: " << Wells.accountName << " you have $" << Wells.money << " in your account" << std::endl;
    return;
}

int main (int argc, char **argv) {
    int foo = 10;
    std::string fooB = "This is a test";
    Bank Wells; 

    printInfo(Wells);

    auto amex = new Bank;
    
    Wells.money = 1000;
    Wells.accountName = fooB;

    printInfo(Wells);

    return 0;
} // this is a comment