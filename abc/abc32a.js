function main(input) {
    input = input.trim().split('\n')
    let a = parseInt(input[0], 10)
    let b = parseInt(input[1], 10)
    let n = parseInt(input[2], 10)
    while(n%a != 0 || n%b != 0) n++;
    console.log(n)
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))