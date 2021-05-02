function main(input) {
    input = input.trim(' ').split('\n');
    let [x] = input[0].split(' ').map(e => parseInt(e, 10))
    console.log(Math.floor(x/10) + x%10)
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))