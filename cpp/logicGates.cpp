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

bool gateInput(int gate)
{
  switch(gate)
  {
    case 0: 
      return false;
      break;

    case 1:
      return true;
      break;

    default:
      return false;
      
  }

}

bool andGate(bool one, bool two)
{
  return one && two ? true : false;
}

bool notGate(bool gate)
{
  return gate ? false : true;
}

bool nandGate(bool one, bool two)
{
  return notGate(andGate(one, two));
}

bool orGate(bool one, bool two)
{
  return nandGate(notGate(one), notGate(two));
}

bool norGate(bool one, bool two)
{
  return orGate(one, two) ? false : true;
} 

bool xorGate(bool one, bool two)
{
  return andGate(orGate(one, two), nandGate(one, two));
}

int main()
{

  bool oneGate, twoGate;
  int gates;


  cout << "INSERT FIRST BOOLEAN (0 or 1): ";

  cin >> gates;

  oneGate = gateInput(gates);

  cout << "INSERT ANOTHER BOOLEAN (0 or 1): ";
  
  cin >> gates;

  twoGate = gateInput(gates);

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