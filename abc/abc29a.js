function main(input) {
    input = input.trim(' ').split('\n');
    let [w] = input
    w += 's'
    console.log(w)
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))