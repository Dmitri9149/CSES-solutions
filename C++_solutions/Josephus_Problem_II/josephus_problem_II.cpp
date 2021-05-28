/*Consider a game where there are n children (numbered 1,2,…,n) in a circle. 
During the game, repeatedly k children are skipped and one child is removed from the circle. 
In which order will the children be removed?

Input

The only input line has two integers n and k.

Output

Print n integers: the removal order.

Constraints
1≤n≤2⋅105
0≤k≤109
*/

#include <iostream>
using namespace std;

#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>
using namespace __gnu_pbds;

#define ordered_set tree<int, null_type,less<int>, rb_tree_tag,tree_order_statistics_node_update>

int main(){
  ordered_set set;
  int n,k; cin>>n>>k;
  for (int i = 1; i <= n; i++)
    set.insert(i);
  int pos=0;
  while(set.size()>1)
  {
    pos=(pos+k)%(int)set.size();
    cout<<*(set.find_by_order(pos))<<" ";
    set.erase(*(set.find_by_order(pos)));
  }
  cout<<*(set.find_by_order(0))<<endl;
}


