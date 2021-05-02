function main(input) {
    input = input.trim(' ').split('\n')
    let n = input[0].split(' ').map(e => parseInt(e, 10))
    if(n < 60) console.log('Bad')
    else if(n < 90) console.log('Good')
    else if(n < 100) console.log('Great')
    else console.log('Perfect')
    return;
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))