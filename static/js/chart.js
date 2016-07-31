var CSS_COLOR_NAMES = ["Red","Blue","Green","Yellow","Purple","Orange"];
var CIRCLE_SIZE = 5;

function drawCircle(x, y, size) {
    var plot = d3.select('svg');
    plot.append("circle")
        .style('fill', 'black')
        .attr('cx', x)
        .attr('cy', y)
        .attr('r', size);
}

function drawAllCircles(data) {
    var plot = d3.select('svg');

    for (var i = 0; i < data.length; i++) {
        var point = data[i]
        plot.append("circle")
            .style('fill', 'black')
            .attr('cx', point[0])
            .attr('cy', point[1])
            .attr('r', CIRCLE_SIZE)
    }
}

function colourClusters(clusters) {
    var plot = d3.select('svg');
    var i = 0;
    plot.selectAll("circle").selectAll(function() {
        var color = clusters[i] >= 0 ? CSS_COLOR_NAMES[clusters[i]] : 'black';
        d3.select(this).style('fill', color);
        i += 1;
    });
}

function getCoords() {
    var plot = d3.select('svg');
    let coords = [];
    plot.selectAll("circle").selectAll(function() {
        coords.push([
            parseFloat(d3.select(this).attr('cx')),
            parseFloat(d3.select(this).attr('cy'))
        ]);
    });

    return coords;
}

function deleteAllPoints() {
    var plot = d3.select('svg');
    plot.selectAll("circle").remove();
}

function clusterData(model, params) {
    var coords = getCoords();
    var body = {};
    if (params == null) {
        body = JSON.stringify({ data: coords });
    } else {
        params['data'] = coords;
        body = JSON.stringify(params);
    }
    

    d3.request('/models/' + model)
        .post(body, (err, resp) => {
            if (err) {
                console.log(err);
            } else {
                var json_resp = JSON.parse(resp['responseText']);
                colourClusters(json_resp);
            }
        });
}

function createScatter() {
    var plot = d3.select('svg');
    
    plot.on('click', function() {
        var coords = d3.mouse(this);
        drawCircle(coords[0], coords[1], CIRCLE_SIZE);
    });    
}
