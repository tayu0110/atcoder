function main(input) {
    input = input.trim(' ').split('\n')
    let [ q ] = input[0].split(' ').map(e => parseInt(e, 10))
    if(q === 1) console.log('ABC')
    else console.log('chokudai')
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))