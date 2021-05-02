function main(input) {
    input = input.trim(' ').split('\n')
    input = input[0].split(' ').map(e => parseInt(e, 10))
    input.sort((l, r) => l - r)
    console.log(input[1])
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))