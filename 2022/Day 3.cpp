#include <iostream>
#include <fstream>
#include <string>
#include <cctype>

using namespace std;
int main(void){
    std::ifstream ifile("input3.txt");
    std::string line[3];
    int sum = 0;
    /*
    while(std::getline(ifile, line)){
        bool found = false;
        for(int i = 0; i < line.length()/2; i++){
            for(int j = line.length()/2; j < line.length(); j++){
                if(line[i]==line[j] && !found){
                    if(islower(line[i])){
                        sum+=line[i]-96;
                        cout << line[i] << endl;
                    }
                    else{
                        sum+=line[i]-38;
                        cout << line[i] << endl;
                    }
                    found=true;
                }
            }   
        }
    }
    */
    while(getline(ifile, line[0])&&getline(ifile, line[1])&&getline(ifile, line[2])){
        for(int i = 0; i < line[0].length(); i++){
            if(line[1].find(line[0][i])!=string::npos && line[2].find(line[0][i])!=string::npos){
                if(islower(line[0][i])){
                    sum+=line[0][i]-96;
                    cout << line[0][i] << endl;
                    break;
                }
                else{
                    sum+=line[0][i]-38;
                    cout << line[0][i] << endl;
                    break;
                }
            }
        }
    }
    cout << sum << endl;
}