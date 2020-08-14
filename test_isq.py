import unittest
import isq

class TestQuantityValue(unittest.TestCase):

    def testQuantityValueInit(self):
        x = isq.QuantityValue(1, 'm', 0.01)
        self.assertEqual(x.number, 1, "Should be 1")
        self.assertEqual(x.reference, 'm', "Should be 'm'")

    def testQuantityValue__add__(self):
        x = isq.QuantityValue(1, 'm', 0.01) \
            + isq.QuantityValue(1, 'm', 0.01)
        self.assertEqual(x.number, 2, "Should be 2")
        self.assertEqual(x.reference, 'm', "Should be 'm'")

if __name__ == "__main__":
    unittest.main()
