function main(input) {
    input = input.trim(' ').split('\n')
    let [a, b, c, k] = input[0].split(' ').map(e => parseInt(e, 10))
    let [s, t] = input[1].split(' ').map(e => parseInt(e, 10))
    let cost = s*a + t*b
    if(s+t >= k) {
        console.log(cost - (s+t)*c)
    } else {
        console.log(cost)
    }
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))