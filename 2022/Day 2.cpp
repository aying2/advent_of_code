#include <iostream>
#include <fstream>
#include <string>

using namespace std;
int main(void){
    std::ifstream ifile("input2.txt");
    std::string line;
    int sum = 0;
    /*
    while(std::getline(ifile, line)){
        if(line[2]=='X'){
            sum+=1;
            if(line[0]=='A') sum+=3;
            else if(line[0]=='C') sum+=6;
        }
        else if(line[2]=='Y'){
            sum+=2;
            if(line[0]=='B') sum+=3;
            else if(line[0]=='A') sum+=6;
        }
        else if(line[2]=='Z'){
            sum+=3;
            if(line[0]=='C') sum+=3;
            else if(line[0]=='B') sum+=6;
        }

        
    }
    cout << sum << endl;
    */
    while(std::getline(ifile, line)){
        if(line[2]=='X'){
            sum+=0;
            if(line[0]=='A') sum+=3;
            else if(line[0]=='B') sum+=1;
            else if(line[0]=='C') sum+=2;
        }
        else if(line[2]=='Y'){
            sum+=3;
            if(line[0]=='A') sum+=1;
            else if(line[0]=='B') sum+=2;
            else if(line[0]=='C') sum+=3;
        }
        else if(line[2]=='Z'){
            sum+=6;
            if(line[0]=='A') sum+=2;
            else if(line[0]=='B') sum+=3;
            else if(line[0]=='C') sum+=1;
        }

        
    }
    cout << sum << endl;
}