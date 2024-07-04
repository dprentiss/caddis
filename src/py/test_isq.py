import unittest
import isq

class TestQuantityDimension(unittest.TestCase):
    
    def test__init__(self):
        q = isq.length
        self.assertEqual(str(q), 'L', "Should be 'L'")
        q = isq.mass
        self.assertEqual(str(q), 'M', "Should be 'M'")
        q = isq.time
        self.assertEqual(str(q), 'T', "Should be 'T'")
        q = isq.current
        self.assertEqual(str(q), 'I', "Should be 'I'")
        q = isq.temp
        self.assertEqual(str(q), 'Θ', "Should be 'Θ'")
        q = isq.amount 
        self.assertEqual(str(q), 'N', "Should be 'N'")
        q = isq.intensity
        self.assertEqual(str(q), 'J', "Should be 'J'")

class TestQuantityValue(unittest.TestCase):

    def test__init__(self):
        x = isq.QuantityValue(1, 'm', 0.01)
        self.assertEqual(x.number, 1, "Should be 1")
        self.assertEqual(x.reference, 'm', "Should be 'm'")

    def test__add__(self):
        x = isq.QuantityValue(1, 'm', 0.01) \
            + isq.QuantityValue(1, 'm', 0.01)
        self.assertEqual(x.number, 2, "Should be 2")
        self.assertEqual(x.reference, 'm', "Should be 'm'")

if __name__ == "__main__":
    unittest.main()
