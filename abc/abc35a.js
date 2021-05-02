function main(input) {
    input = input.trim().split('\n')
    let [w, h] = input[0].split(' ').map(e => parseInt(e, 10))
    if(w/4 === h/3) {
        console.log("4:3")
    } else if(w/16 === h/9) {
        console.log("16:9")
    }
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))