#include <iostream>
#include <fstream>
#include <string>
#include <cctype>

using namespace std;
int main(int argc, char** argv){
    ifstream ifile(argv[1]);
    string str;
    int range1[2];
    int range2[2];
    int sum = 0;
    char buf;
    while(ifile>>range1[0]&&
        ifile >> buf &&
        ifile>>range1[1]&&
        ifile >> buf &&
        ifile>>range2[0]&&
        ifile >> buf &&
        ifile>>range2[1]){
        /*
        if(range2[0]<=range1[0]&&range1[1]<=range2[1]){
            sum++;
        }
        else if(range1[0]<=range2[0]&&range2[1]<=range1[1]){
            sum++;
        }
        */
        if(!(range1[0]>range2[1]||range1[1]<range2[0]||range2[0]>range1[1]||range2[1]<range1[0])){
            sum++;
        }
    }
    cout << sum << endl;
}