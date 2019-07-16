(function() {
    console.log('final result', search(10000000000000, 0));
})()

function search(increment, start) {
    console.log('starting increment', increment, 'at', start);
    for (let i = start; i < Number.MAX_VALUE && !Number.isNaN(i); i += increment) {
        let s = i.toString();
        console.log('i: ', s);
        if (s.indexOf('e') > -1) {
            console.log('found first modified integer', i, '->', s);
            return search(increment / 10, i - increment);
        }
    }
}