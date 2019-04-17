import unittest
import isq

class TestQuantityValue(unittest.TestCase):

    def testQuantityValueInit(self):
        x = isq.QuantityValue(1, 'm', 0.01)
        self.assertEqual(x.number, 1, "Should be 1")

if __name__ == "__main__":
    unittest.main()
