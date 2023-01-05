#include <iostream>
using namespace std;


void jumpLine(int numberLines)
{
  int n = 0;
  while(n < numberLines)
  {
    cout << "" << endl;
    n++;
  }
}

int andGate(int one, int two)
{
  if(one == 1 && two == 1)
  {
    return 1;
  } else return 0;
}

int notGate(int number)
{
  if(number == 1)
  {
    return 0;
  } else return 1;
}

int nandGate(int one, int two)
{
  return notGate(andGate(one, two));
}

int orGate(int one, int two)
{
  return nandGate(notGate(one), notGate(two));
}

int norGate(int one, int two)
{
  if(orGate(one, two) == 1)
  {
    return 0;
  } else return 1;
} 

int xorGate(int one, int two)
{
  return andGate(orGate(one, two), nandGate(one, two));
}

int main()
{

  int oneGate, twoGate;

  cout << "INSERT ONE BOOLEAN (1 or 0): ";

  cin >> oneGate;

  if(!isdigit(oneGate))
  {
    if(oneGate > 1 || oneGate < 0)
    {
      cout << "OneGate must be 0 or 1" << endl;
      jumpLine(3);
      exit(1);
    }
  } else 
  {
    cout << "OneGate must be a number" << endl;
    jumpLine(3);
    exit(1);
  }

  cout << "INSERT ANOTHER BOOLEAN (1 or 0): ";
  
  cin >> twoGate;

  if(!isdigit(twoGate))
  {
    if(twoGate > 1 || twoGate < 0)
    {
      cout << "TwoGate must be 0 or 1" << endl;
      jumpLine(3);
      exit(1);
    }
  } else 
  {
    cout << "TwoGate must be a number" << endl;
    jumpLine(3);
    exit(1);
  }

  jumpLine(1);
  
  cout << "AND GATE: " << andGate(oneGate, twoGate) << endl;
  cout << "NOT OneGate: " << notGate(oneGate) << endl;
  cout << "NOT TwoGate: " << notGate(twoGate) << endl;
  cout << "NAND GATE: " << nandGate(oneGate, twoGate) << endl;
  cout << "OR GATE: " << orGate(oneGate, twoGate) << endl;
  cout << "NOR GATE: " << norGate(oneGate, twoGate) << endl;
  cout << "XOR GATE: " << xorGate(oneGate, twoGate) << endl;


  jumpLine(3);
  
  return 0;

}


