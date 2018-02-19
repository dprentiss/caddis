var Capsule = {
    hull: {
        cylinder: {
            material: {name: 'steel'},
            innerRadius: { quantity: 5, unit: 'm' },
            outerRadius: { quantity: 5.02, unit: 'm' },
            length: { quantity: 30, unit: 'm' }
            },
        getVolume: function () {
            let ri = this.cylinder.innerRadius.quantity,
            ro = this.cylinder.outerRadius.quantity,
            l = this.cylinder.length.quantity;
            let v = l*3.1416*(ro*ro-ri*ri)+4/3*3.1416*(ro*ro*ro-ri*ri*ri);
            console.log(v);
        },
         getInternalVolume: function () {
            let ri = this.cylinder.innerRadius.quantity,
            l = this.cylinder.length.quantity;
            let v = l*3.1416*(ri*ri)+4/3*3.1416*(ri*ri*ri);
            console.log(v);
        }
   }
};

var Engine = {
    mass: { quantity: 470, unit: 'kg' }
};

var Cylinder = {
    radius: { quantity: 1, unit 'm' },
    length: { quantity: 1, unit 'm' },
    getVolume: function () {
        let r = this.radius.quantity,
        l = this.length.quantity;
        return l * r * r * Math.PI;
}

var ISQ = {
    [
        {
            baseQuantitiy: 'length',
            dimensionSymbol: 'L',
        }
    ],

    systemsOfUnits: {
        SI: {
        {baseQuantity: 'length', baseUnit: {name: 'metre', symbol: 'm'}},
        {baseQuantity: 'mass', baseUnit: {name: 'kilogram', symbol: 'kg'},
        {baseQuantity: 'time', baseUnit: {name: 'second', symbol: 's'}},
        {baseQuantity: 'electric current', baseUnit: {name: 'ampere', symbol: 'A'}},
        {baseQuantity: 'thermodynamic temperature', baseUnit: {name: 'kelvin', symbol: 'K'}},
        {baseQuantity: 'amount of substance', baseUnit: {name: 'mole', symbol: 'mol'}},
        {baseQuantity: 'luminous intensity', baseUnit: {name: 'candela', symbol: 'cd'}}
        }

    /* ISQ base quantity (1.4, 1.6)
     */
    baseQuantities: [
        'length',
        'mass',
        'time',
        'electric current',
        'thermodynamic temperature',
        'amount of substance',
        'luminous intensity',
        'number of entities'
    ],
    quantityDimensions: [
        {baseQuantity: 'length', dimensionSymbol: 'L'},
        {baseQuantity: 'mass', dimensionSymbol: 'M'},
        {baseQuantity: 'time', dimensionSymbol: 'T'},
        {baseQuantity: 'electric current', dimensionSymbol: 'L'},
        {baseQuantity: 'thermodynamic temperature', dimensionSymbol: 'Θ'},
        {baseQuantity: 'amount of substance', dimensionSymbol: 'N'},
        {baseQuantity: 'luminous intensity', dimensionSymbol: 'J'}
    ],
    QuantityValue: function({number:0, reference:''}) {
    }
};
