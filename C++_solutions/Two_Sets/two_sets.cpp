/*Your task is to divide the numbers 1,2,…,n into two sets of equal sum. */

#include <iostream>

using namespace std;

int main() {
    long int nn;
    cin >> nn;
    int m = (nn+1)/2;
    long int t1[m];
    long int t2[m];

    int flag;

    if (nn == 0 || nn == 1) {
      cout << "NO";
      return 0;
    }

    if (nn%4 == 0) {
      flag = 0;
      cout << "YES" << "\n";
      for (int i = 0; i < nn/2; i++) {
	if (i%2==0) {
	  t1[i]=nn-2*i;
	  t2[i]=nn-2*i-1;
	} else {
	  t2[i]=nn-2*i;
	  t1[i]=nn-2*i-1;
	}
      }
    } else if ((nn+1)%4 == 0) {
      flag = 1;
      cout << "YES" << "\n";
      for (int i =0; i <= (nn+1)/2; i++) {
      	if (i%2==0) {
	  t1[i]=nn-2*i;
	  t2[i]=nn-2*i-1;
	} else {
	  t2[i]=nn-2*i;
	  t1[i]=nn-2*i-1;
	}	
      }
    } else {
      cout << "NO";
      return 0;
    }

    if (flag == 0) {
      
      cout << m << "\n";		 	
      for (int i = 0; i < m; i++) {
	  cout << t1[i] << " ";
      }
      cout << "\n";
      cout << m << "\n";
      for (int i = 0; i < m; i++) {
	cout << t2[i] << " ";
      }
    
    } else {
      cout << m-1 << "\n";		 	
      for (int i = 0; i < m; i++) {
	if (t1[i]!=0) {
	  cout << t1[i] << " ";
	}
      }
      cout << "\n";
      cout << m << "\n";
      for (int i = 0; i < m; i++) {
	cout << t2[i] << " ";
      }


    }
}
