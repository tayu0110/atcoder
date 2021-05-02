function main(input) {
    input = input.trim().split('\n')
    let n = input[0].split(' ').map(e => parseInt(e, 10))
    if(n % 1111 === 0) console.log('SAME')
    else console.log('DIFFERENT')
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))