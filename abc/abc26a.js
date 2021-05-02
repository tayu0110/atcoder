function main(input) {
    input = input.trim(' ').split('\n')
    input = parseInt(input, 10)
    if(input%2 === 0) console.log(input/2 * input/2)
    else {
        let s = Math.round(input/2)
        let l = input - s
        console.log(l * s)
    }
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))