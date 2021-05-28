/*
Given a string, your task is to generate all different strings that can be created using its characters.

Input

The only input line has a string of length n. Each character is between a–z.

Output

First print an integer k: the number of strings. Then print k lines: the strings in alphabetical order.

Constraints
1≤n≤8
*/
#include <iostream>
#include <set>
#include <vector>
using namespace std;
set<string> se;
string s;
int n;
string permutation = "";
// in accordance with the task constrain;
int n_max = 8;
std::vector<bool> chosen(n_max, false);

void search () {

  if (permutation.size() ==n) {
    se.insert(permutation);
  } else {
    for (int i = 0; i < n; i++) {
      if (chosen[i]) continue;
	chosen[i] = true;
	permutation.push_back(s[i]);
	search();
	chosen[i]= false;
	permutation.pop_back();
    }
  }
}

int main(){
  cin >> s;
  n = s.size();
//  for (int i = 0; i < n; i++) {
//    chosen[i]=false;
//  }
  search();
  cout<<(int)se.size()<<"\n";
  for(string s : se) cout<<s<<"\n";
  return 0;
}


