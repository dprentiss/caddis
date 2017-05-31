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

