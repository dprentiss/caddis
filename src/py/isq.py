from re import split
from math import floor, exp, sqrt, log10
from fractions import Fraction

class QuantityDimension:
    dimensionSymbols = (
            ('length', 'L'),
            ('mass', 'M'),
            ('time', 'T'),
            ('electric current', 'I'),
            ('thermodynamic temperature', 'Θ'),
            ('amount of substance', 'N'),
            ('luminous intensity', 'J')
    )
    def __init__(self, baseQuantityExponents = None):
        exponents = {
            'length': 0,
            'mass': 0,
            'time': 0,
            'electric current': 0,
            'thermodynamic temperature': 0,
            'amount of substance': 0,
            'luminous intensity': 0
        }
        if baseQuantityExponents != None:
            for key, value in baseQuantityExponents.items():
                isPresentAndValid = key in exponents and \
                                    isinstance(value, (Fraction, int))
                if isPresentAndValid:
                    exponents.update({key: value})
                else:
                    raise TypeError('\'' + key + '\' is not an ISQ base quantity')
        self.baseQuantityExponents = frozenset(exponents.items())

    def dim(self, key=None):
        if key == None:
            return dict(self.baseQuantityExponents)
        else:
            return dict(self.baseQuantityExponents)[key]
   
    def __str__(self):
        s = ''
        symbols = dict(self.dimensionSymbols)
        for key in symbols:
            if self.dim(key) != 0:
                s += symbols[key]
                if self.dim(key) != 1:
                    s += '^(' + str(self.dim(key)) + ')'
        return s
    
    def __repr__(self):
        return self.__str__()
    
    def __eq__(self, other):
        return self.baseQuantityExponents == other.baseQuantityExponents
    
    def __ne__(self, other):
        return self.baseQuantityExponents != other.baseQuantityExponents
    
    def __mul__(self, other):
        if isinstance(other, self.__class__):
            exponents = {key: self.dim(key) + other.dim(key) for key in self.dim()}
            return QuantityDimension(exponents)
        else:
            raise TypeError("something wrong with baseQuantityExponents")
            
    def __truediv__(self, other):
        if isinstance(other, self.__class__):
            return self * other.inv()
        else:
            raise TypeError("something wrong with baseQuantityExponents")
    
    def inv(self):
        exponents = {key: self.dim(key) * -1 for key in self.dim()}
        return QuantityDimension(exponents)
            
    def sqrt(self):
        exponents = {key: Fraction(self.dim(key),2) for key in self.dim()}
        return QuantityDimension(exponents)
            
class QuantityValue:
    def __init__(self, number, reference,
                 standardUncertainty = 0, uncertainty = None,
                 quantityDimension = QuantityDimension()):

        self.number = number
        self.reference = reference
        self.uncertainty = uncertainty
        if self.uncertainty == None:
            self.standardUncertainty = abs(standardUncertainty)
            if self.number != 0:
                self.relativeUncertainty = self.standardUncertainty / abs(self.number)
            else:
                self.relativeUncertainty = 0
        self.quantityDimension = quantityDimension

        self.referenceList = [split('\^', unit) for unit in
                              split('\s+', self.reference)]

        self.dimList = filter(lambda i: (i[0] != '1' and i[0] != ''), [[dim[0], Fraction(dim[1]) if len(dim) == 2 else 1] for dim in self.referenceList])
        self.dimDict = {}
        self.updateDims()

    def dim(self, key=None):
        return self.quantityDimension(key)

    def updateDims(self):
        for i in self.dimList:
            self.dimDict[i[0]] = (self.dimDict[i[0]] + i[1]) if i[0] in self.dimDict else i[1]
        self.reference = ''
        for key, exp in list(self.dimDict.items()):
            if exp == 0:
                del self.dimDict[key]
                continue
            self.reference += '{}{}{}'.format(
                '' if self.reference == '' else ' ',
                key if exp !=0 else '',
                '^' + str(exp) if (exp != 0 and exp != 1) else '')

    def __str__(self):
        exponent = floor(log10(abs(self.number)))
        engExponent = floor(exponent / 3) * 3
        engMantissa = self.number / 10**engExponent
        if self.standardUncertainty != 0:
            uncertaintyExponent = floor(log10(self.standardUncertainty))
            uncertaintyMantissa = self.standardUncertainty / 10**(uncertaintyExponent-1)
            numDigits = engExponent - uncertaintyExponent + 1
            uncertaintyStr = '(' + '{:.0f}'.format(uncertaintyMantissa) + ')'
        else:
            numDigits = '4'
            uncertaintyStr = '(...)'
        if engExponent < 0:
            exponentStr = ' ' + factorPrefixSymbols[engExponent]
            #exponentStr = 'e' + repr(engExponent) + ' ' + factorPrefixSymbols[engExponent]
        elif engExponent > 0:
            exponentStr = ' ' + factorPrefixSymbols[engExponent]
            #exponentStr = 'e' + repr(engExponent) + ' ' + factorPrefixSymbols[engExponent]
        else:
            exponentStr = ' '
        return '{:.{}f}'.format(engMantissa, numDigits) + uncertaintyStr + exponentStr + self.reference

    def __repr__(self):
        return self.__str__()

    def invReference(self):
        referenceStr = ''
        for key in self.dimDict:
            exp = -1 * self.dimDict[key]
            referenceStr += '{}{}{}'.format(
                '' if referenceStr == '' else ' ',
                key if exp != 0 else '',
                '^' + str(exp) if (exp != 0 and exp != 1) else ''
            )
        return referenceStr

    def exp(self):
        if self.dimDict == {}:
            return QuantityValue(exp(self.number),
                                 '',
                                 exp(self.number) * self.standardUncertainty)
        else:
            raise TypeError("exp function only applies to quantities of dimension 1")

    def sqrt(self):
        result = QuantityValue(sqrt(self.number),
                               self.reference,
                               sqrt(self.number) * 0.5 * self.relativeUncertainty,
                               quantityDimension = self.quantityDimension.sqrt())
        result.dimDict = {key: Fraction(value / 2) for key, value in self.dimDict.items()}
        result.updateDims()
        return result

    def __add__(self, other):
        if isinstance(other, self.__class__) and self.dimDict == other.dimDict:
            return QuantityValue(self.number + other.number,
                                 self.reference,
                                 sqrt(self.standardUncertainty**2 + other.standardUncertainty**2))
        elif isinstance(other, (int, float)) and self.dimDict == {}:
            return QuantityValue(other + self.number,
                                 self.reference,
                                 abs(other) * self.standardUncertainty)
        else:
            raise TypeError("cannot add values with different quantity dimensions",
                            self.__str__(), self.__class__, other.__str__(), other.__class__)

    def __radd__(self, other):
        return self.__add__(other)

    def __sub__(self, other):
        if isinstance(other, self.__class__) and self.dimDict == other.dimDict:
            return QuantityValue(self.number - other.number,
                                 self.reference,
                                 sqrt(self.standardUncertainty**2 + other.standardUncertainty**2))
        elif isinstance(other, (int, float)) and self.dimDict == {}:
            return QuantityValue(self.number - other,
                                 self.reference,
                                 abs(other) * self.standardUncertainty)
        else:
            raise TypeError("cannot subtract values with different quantity dimensions")

    def __rsub__(self, other):
        return self.__sub__(other)

    def __mul__(self, other):
        if isinstance(other, self.__class__):
            return QuantityValue(self.number * other.number,
                                 self.reference + ' ' + other.reference,
                                 abs(self.number * other.number)
                                 * sqrt(self.relativeUncertainty**2 + other.relativeUncertainty**2),
                                 quantityDimension = self.quantityDimension * other.quantityDimension)
        elif isinstance(other, (int, float)):
            return QuantityValue(other * self.number,
                                 self.reference,
                                 abs(other) * self.standardUncertainty)
        else:
            raise TypeError("types not supported for multiplication:,", self.__str__(), self.__class__, other.__str__(), other.__class__)

    def __rmul__(self, other):
        return self.__mul__(other)

    def __truediv__(self, other):
        if isinstance(other, self.__class__):
            return self * other.inv()
        elif isinstance(other, (int, float)):
            return QuantityValue(self.number / other,
                                 self.reference,
                                 abs(1 / other) * self.standardUncertainty,
                                 quantityDimension = self.quantityDimension)

    def __rtruediv__(self, other):
        if isinstance(other, (int, float)):
            return QuantityValue(other / self.number,
                                 self.invReference(),
                                 abs(other / self.number) * sqrt(self.relativeUncertainty**2),
                                 quantityDimension = self.quantityDimension.inv())
        else:
            raise TypeError("types not supported for division")

    def __neg__(self):
        return QuantityValue(-1 * self.number,
                             self.reference,
                             self.standardUncertainty,
                             quantityDimension = self.quantityDimension)
    
    def inv(self):
        return QuantityValue(1/self.number, self.invReference(),
                             abs(1/self.number) * self.relativeUncertainty,
                             quantityDimension = self.quantityDimension.inv())

length = QuantityDimension({'length':1})
mass = QuantityDimension({'mass':1})
time = QuantityDimension({'time':1})    
current = QuantityDimension({'electric current':1})
temp = QuantityDimension({'thermodynamic temperature':1})
mol = QuantityDimension({'amount of substance':1})
intensity = QuantityDimension({'luminous intensity':1})


factorPrefixSymbols = {-12:'p',-9:'n',-6:'μ',-3:'m',3:'k',6:'M',9:'G',12:'T'}
baseUnitSymbols = {'length':'m','mass':'kg','time':'s','electric current':'A','thermodynamic temperature':'K','amount of substance':'mol','luminous intensity':'cd'}

m = QuantityValue(1, 'm', quantityDimension=length)
kg = QuantityValue(1, 'kg', quantityDimension=mass)
s = QuantityValue(1, 's', quantityDimension=time)
A = QuantityValue(1, 'A', quantityDimension=current)
K = QuantityValue(1, 'K', quantityDimension=temp)
mol = QuantityValue(1, 'mol', quantityDimension=mol)
cd = QuantityValue(1, 'cd', quantityDimension=intensity)

N = kg * m / s / s
Pa = kg / m / s / s
V = kg * m * m / s / s / s / A
W = kg * m * m / s / s / s
J = kg * m * m / s / s
Hz = 1 / s

derivedQuantities = {}
derivedQuantities[N.quantityDimension.baseQuantityExponents] = 'N'
derivedQuantities[Pa.quantityDimension.baseQuantityExponents] = 'Pa'
derivedQuantities[V.quantityDimension.baseQuantityExponents] = 'V'
derivedQuantities[W.quantityDimension.baseQuantityExponents] = 'W'
derivedQuantities[J.quantityDimension.baseQuantityExponents] = 'J'
derivedQuantities[Hz.quantityDimension.baseQuantityExponents] = 'Hz'
