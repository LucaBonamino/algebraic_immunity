from algebraic_immunity import AlgebraicImmunity, BooleanFunction as RBooleanFunction
import unittest
from sage.all import *
from sage.crypto.boolean_function import BooleanFunction
import random

# result = AlgebraicImmunity.algebraic_immunity([0,1,1,0,1,0,0,1], 3)
# print(result)

class TestAlgebraicImmunity(unittest.TestCase):

    def test_ai(self):
        t = [1,0,1,0,1,1,0,1]
        b = BooleanFunction(t)
        self.assertEqual(b.algebraic_immunity(), AlgebraicImmunity.ai(t, 8))

    def test_random(self):
        for _ in range(100):
            for n in range(1, 10):
                t = [random.randint(0,1) for _ in range(2**n)]
                try:
                    bf = BooleanFunction(t).algebraic_immunity()
                except:
                    continue
                r = AlgebraicImmunity.ai(t, n)
                self.assertEqual(r, bf)

    
    def test_ramdom_from_boolean_function(self):
        for _ in range(100):
            for n in range(1, 10):
                t = [random.randint(0,1) for _ in range(2**n)]
                try:
                    bf = BooleanFunction(t).algebraic_immunity()
                except:
                    continue
                rbf = RBooleanFunction(t)
                r = rbf.algebraic_immunity()
                self.assertEqual(r, bf)

