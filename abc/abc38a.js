function main(input) {
    let s = input.trim()
    let size = s.length
    if(s[size-1] === 'T') {
        console.log('YES')
    } else {
        console.log('NO')
    }
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))