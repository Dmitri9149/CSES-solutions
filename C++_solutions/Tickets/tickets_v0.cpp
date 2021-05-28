/*
There are n concert tickets available, each with a certain price. Then, m customers arrive, one after another.

Each customer announces the maximum price he or she is willing to pay for a ticket, and after this, they will get a ticket with the nearest possible price such that it does not exceed the maximum price.

Input

The first input line contains integers n and m: the number of tickets and the number of customers.

The next line contains n integers h1,h2,…,hn: the price of each ticket.

The last line contains m integers t1,t2,…,tm: the maximum price for each customer.

Output

Print, for each customer, the price that they will pay for their ticket. After this, the ticket cannot be purchased again.

If a customer cannot get any ticket, print −1.
*/
//#include <iostream>
//#include <vector>
//#include <algorithm>
#include <bits/stdc++.h>

using namespace std;

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    int nn;
    int mm;
    int temp;
    cin >> nn;
    cin >> mm;
    vector<int>tickets;
    vector<int>prices;
    vector<int>results;


    while(nn--) {
	cin >> temp;
	tickets.emplace_back(temp);

    }

//    for (int i = 0; i < nn; i++) {
//    
//    	cout << tickets[i];
//    }
//    cout << *tickets.begin() << "begin";
//    cout << *tickets.end() << "end";

    sort(tickets.begin(),tickets.end());


//     for (auto price : tickets)
//     {
//        printf("sorted %i  ", price);
//     }

    while(mm--) {
      cin >> temp;
      auto it = upper_bound(tickets.begin(),tickets.end(), temp);
      if (it == tickets.begin()) {
	cout <<  -1 << "\n";
      } else {
	cout << *(--it) << "\n";
//	results.emplace_back(*it);
//	cout << "it" << *it;
	tickets.erase(it);
      }

    }
    

//    for (int j=0; j < mm; j++) {
//      cout << "results in for loop " << results[j] << "\n";
//    }

//    for (auto price: results) {
//      printf(" Final prices %i ", price);
//    }

//    return 0;
}
