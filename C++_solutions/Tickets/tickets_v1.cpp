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

Constraints
1≤n,m≤2⋅105
1≤hi,ti≤109
*/

#include <iostream>
#include <set>

using namespace std;

int n,m,h,t; multiset<int> tickets;
 
void solve() {
	cin >> n >> m;
	for (int i=0;i<n;++i){
		cin >> h; tickets.insert(h);
	}
	for (int i=0;i<m;++i){
		cin >> t;
		auto it = tickets.upper_bound(t);
		if (it==tickets.begin()){
			cout << -1 << "\n";
		}
		else{
			cout << *(--it) << "\n";
			tickets.erase(it);
		}
	}   
}

int main(){
	solve();
	return 0;
}
