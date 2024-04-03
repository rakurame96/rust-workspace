#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main () {

   vector<string> treasure_map  = {"gold", "diamonds", "ruby"};
   vector<string> another_map = treasure_map ;
   another_map[0] = "shiny new sword";

   cout << "treasure_map : " << treasure_map[0]  << endl;
   cout << "another_map : " << another_map[0] << endl;


   return 0;
}
