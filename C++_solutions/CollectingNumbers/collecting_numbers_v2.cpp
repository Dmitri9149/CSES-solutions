/*
You are given an array that contains each number between 1…n exactly once. Your task is to collect the numbers from 1 to n in increasing order.

On each round, you go through the array from left to right and collect as many numbers as possible. What will be the total number of rounds?

Input

The first line has an integer n: the array size.

The next line has n integers x1,x2,…,xn: the numbers in the array.

Output

Print one integer: the number of rounds.

Constraints
1≤n≤2⋅105
*/

#include <iostream>

using namespace std;

void  solve() {
    int n,h;
    int l = 1;
    cin >> n;
    int numbers[n+1]={0};
    for (int i=1;i<=n;++i){
            cin >> h; numbers[h]=i;
    }

    int clock = 1;
    for (int i = 1; i <=n; i++) {
      if (l > numbers[i]) clock++;
      l = numbers[i];
    }

    cout << clock;
}
int main(){
    solve();
    return 0;
}
