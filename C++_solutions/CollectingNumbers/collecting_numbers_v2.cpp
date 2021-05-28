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
