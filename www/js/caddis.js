document.addEventListener("DOMContentLoaded", init, {once: true});

function init(event) {
    //console.log('"DOMContentLoaded"', document.URL, event);
    tablist = document.getElementById('tab-list');
    addTab('Blog');
    addTab('Plot');
    //console.log(tablist, 'childElementCount', tablist.localName);
    draw();
}

function addTab(text) {
    let tab = document.createElement("button");
    tab.setAttribute("class", "tab");
    tab.setAttribute("type", "button");
    tab.setAttribute("role", "tab");
    tab.setAttribute("aria-selected", "false");
    tab.setAttribute("tabindex", "-1");
    //button.setAttribute("aria-controls", "tab-header");
    tab.addEventListener("click", handleTabSelect);
    tab.append(text);
    tablist.append(tab);
}

function handleTabSelect(event) {
    activateTab(event.target);
}

function activateTab(tab) {
    deactivateTabs();
    tab.setAttribute("aria-selected", "true");
    tab.setAttribute("tabindex", "0");
    console.log(tab);
    //button.addEventListener("click", function() {console.log('Clicked', text);});
}

function deactivateTabs() {
    tabs = document.getElementsByClassName('tab');
    for (let i = 0; i < tabs.length; i++) {
        tabs[i].setAttribute("tabindex", "-1");
        tabs[i].setAttribute("aria-selected", "false");
    };
}

function draw() {
    var canvas = document.getElementById('canvas');
    var xMin = -5;
    var xMax = 5;
    var xScale = 800 / (xMax - xMin) / 1;
    var yScale = -xScale;
    if (canvas.getContext) {
        var ctx = canvas.getContext('2d');
        ctx.beginPath();
        ctx.lineWidth = 0.5;
        ctx.save();
        ctx.setTransform(xScale, 0, 0, yScale, 400, 400);
        ctx.moveTo(0, xMax);
        ctx.lineTo(0, xMin);
        ctx.moveTo(xMax, 0);
        ctx.lineTo(xMin, 0);
        ctx.restore();
        ctx.stroke();

        ctx.beginPath();
        ctx.lineWidth = 2.5;
        ctx.save();
        ctx.setTransform(xScale, 0, 0, yScale, 400, 400);
        plot(tan, range(-5, 5, 500), ctx);
        ctx.restore();
        ctx.stroke();
    }
}

function sampleFunction(func, domainBuffer) {
    return domainBuffer.map(func);
}

var sqr = function(x) {
    return x * x;
};

var tan = function(x) {
    return Math.tan(x);
}

var range = function(min, max, n) {
    let step = (max - min) / (n);
    array = [];
    for (let i = 0; i < n + 1; i++) {
        array.push( min + i * step);
    }
    return new Float32Array(array);
};

var plot = function(func, domainBuffer, ctx) {
    rangeBuffer = sampleFunction(func, domainBuffer);
    xMin = domainBuffer[0];
    xMax = domainBuffer[domainBuffer.length - 1];
    ctx.moveTo(domainBuffer[0], rangeBuffer[0]);
    for (let i = 1; i < domainBuffer.length; i++) {
        if (Math.abs(rangeBuffer[i]) < (xMax - xMin)) {
            //console.log("<:", plotPoints[i][1]);
            ctx.lineTo(domainBuffer[i], rangeBuffer[i]);
        } else {
            //console.log(">:", plotPoints[i][1]);
            ctx.moveTo(domainBuffer[i], rangeBuffer[i]);
        }
    }
};
