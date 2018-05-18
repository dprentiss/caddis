from re import split
from math import floor, exp, sqrt, log10
from fractions import Fraction

class QuantityDimension:
    def __init__(self, baseQuantityExponents = None):
        self.baseQuantityExponents = {
                'length': 0,
                'mass': 0,
                'time': 0,
                'electric current': 0,
                'thermodynamic temperature': 0,
                'amount of substance': 0,
                'luminous intensity':0}
        if baseQuantityExponents != None:
            for key, value in baseQuantityExponents.items():
                isPresentAndValid = key in self.baseQuantityExponents and \
                        isinstance(value, (Fraction, int))
                if isPresentAndValid:
                    self.baseQuantityExponents.update({key: value})
                else:
                    raise TypeError('\'' + key + '\' is not an ISQ base quantity')

class QuantityValue:
    def __init__(self, number, reference,
            standardUncertainty = 0, uncertainty = None):

        self.number = number
        self.reference = reference
        self.uncertainty = uncertainty
        if self.uncertainty == None:
            self.standardUncertainty = abs(standardUncertainty)
            self.relativeUncertainty = self.standardUncertainty / abs(self.number)
        
        self.referenceList = [split('\^', unit) for unit in
                split('\s+', self.reference)]

        self.dimList = filter(lambda i: (i[0] != '1' and i[0] != ''), [[dim[0], int(dim[1]) if len(dim) == 2 else 1] for dim in self.referenceList])
        self.dimDict = {}
        self.updateDims()

    def parseReference(self):
        pass
        
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
        if engExponent != 0:
            exponentStr = 'e' + repr(engExponent)
        else:
            exponentStr = ""
        return '{:.{}f}'.format(engMantissa, numDigits) + uncertaintyStr + exponentStr + ' ' + self.reference
    
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
                             sqrt(self.number) * 0.5 * self.relativeUncertainty)
        result.dimDict = {key: int(value / 2) for key, value in self.dimDict.items()}
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
    
    def __sub__(self,other):
        if isinstance(other, self.__class__) and self.dimDict == other.dimDict:
            return QuantityValue(self.number - other.number,
                                 self.reference,
                                 sqrt(self.standardUncertainty**2 + other.standardUncertainty**2))
        else:
            raise TypeError("cannot subtract values with different quantity dimensions")
                               
    def __mul__(self, other):
        if isinstance(other, self.__class__):
            return QuantityValue(self.number * other.number,
                                 self.reference + ' ' + other.reference,
                                 abs(self.number * other.number) 
                                 * sqrt(self.relativeUncertainty**2 + other.relativeUncertainty**2))
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
            return QuantityValue(self.number / other.number,
                                 self.reference + ' ' + other.invReference(),
                                 abs(self.number / other.number) 
                                 * sqrt(self.relativeUncertainty**2 + other.relativeUncertainty**2))
        elif isinstance(other, (int, float)):
            return QuantityValue(self.number / other,
                             self.reference,
                             abs(1 / other) * self.standardUncertainty)
    
    def __rtruediv__(self, other):
        if isinstance(other, (int, float)):
            return QuantityValue(other / self.number,
                             self.reference,
                             abs(other) * sqrt(self.relativeUncertainty**2))
        else:
            raise TypeError("types not supported for division")
    
    def __neg__(self):
        return QuantityValue(-1 * self.number,
                             self.reference,
                             self.standardUncertainty)

m = QuantityValue(1, 'm', 0)
km = QuantityValue(1e3, 'km', 0)
s = QuantityValue(1, 's', 0)
N = QuantityValue(1, 'kg m s^-2', 0)
