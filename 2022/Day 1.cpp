#include <iostream>
#include <fstream>
#include <string>
#include <map>

int main(void){
    std::ifstream ifile("input.txt");
    std::string line;
    int sum = 0;
    int count = 0;
    std::map<int, int> mymap;
    while(std::getline(ifile, line)){
        if(line==""){
            mymap[sum] = count;
            sum = 0;
            count++;
        }
        else {
            sum+=std::stoi(line);
        }
    }

    std::cout << (--mymap.end())->first << ": " << (--mymap.end())->second << std::endl;

    int three = 0;
    std::map<int, int>::const_iterator it = --mymap.end();
    for(int i = 0; i < 3 ; ++i){
        std::cout << it->first << ": " << it->second << std::endl;
        three+=it->first;
        --it;
    }
    std::cout << three << std::endl;
}