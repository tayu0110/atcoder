function main(input) {
    input = input.trim(' ').split('\n')
    let [l1, l2, l3] = input[0].split(' ').map(e => parseInt(e, 10))
    if(l1 === l2) {
        console.log(l3)
    } else if(l1 === l3) {
        console.log(l2)
    } else {
        console.log(l1)
    }
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))