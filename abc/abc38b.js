function main(input) {
    input = input.trim().split('\n')
    let [h1, w1] = input[0].split(' ').map(e => parseInt(e, 10))
    let [h2, w2] = input[1].split(' ').map(e => parseInt(e, 10))
    if(h1 === h2) {
        console.log('YES')
    } else if(w1 === w2) {
        console.log('YES')
    } else if(h1 === w2) {
        console.log('YES')
    } else if(h2 === w1) {
        console.log('YES')
    } else {
        console.log('NO')
    }
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))