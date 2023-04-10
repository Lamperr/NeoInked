#include <stdio.h>

int main() { // this is a comment
   int auto_var = 1;
   double double_var = 3.14;
   char char_var = 'a';
   const int const_var = 42;
   volatile int volatile_var = 1;
   signed int signed_var = -10;
   unsigned int unsigned_var = 10;
   short int short_var = 10;
   long int long_var = 100;
   float float_var = 1.23;
   enum {EASY=1, MEDIUM=2, HARD=3} difficulty;
   struct person {
      char name[50];
      int age;
      float weight;
   } person1;
   union car {
      int year;
      char model[20];
   } car1;
   typedef int INT;
   INT typedef_var = 10;
   if (auto_var < 2) {
      printf("auto\n");
   }
   else if (double_var < 3.14) {
      printf("double\n");
   }
   else {
      printf("else\n");
   }
   switch (difficulty) {
      case EASY:
         printf("Easy\n");
         break;
      case MEDIUM:
         printf("Medium\n");
         break;
      case HARD:
         printf("Hard\n");
         break;
      default:
         printf("Default\n");
   }
   while (signed_var < 0) {
      printf("while\n");
      signed_var++;
   }
   do {
      printf("do while\n");
      unsigned_var--;
   } while (unsigned_var > 0);
   for (int i = 0; i < 10; i++) {
      printf("for\n");
   }
   goto label;
   printf("This will not be printed.\n");
   label: printf("goto\n");
   return 0;
}