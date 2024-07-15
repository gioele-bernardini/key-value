#ifndef _NEURON_HPP_
#define _NEURON_HPP_

#include <iostream>

using namespace std;

class Neuron {
  public:
    Neuron(double val);

    // Fast Sigmoid Function
    // f(x) = x / (1 + |x|)
    void activate();

    // Derivative for Sigmoid is
    // f'(x) = f(x) * (1 - f(x))

    // Getter functions
    double getVal() {
      return this->val;
    }
    double getActivatedVal() {
      return this->activatedVal;
    }
    double getDerivedVal() {
      return this->derivedVal;
    }

  private:
    // Original raw value not 'squished'
    double val;
    // 0-1 for a Sigmoid
    double activatedVal;

    // The approximated value of the activatedVal
    double derivedVal;
};

#endif

