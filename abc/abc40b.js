function main(input) {
    input = input.trim().split('/n')
    let n = input[0].split(' ').map(e => parseInt(e, 10))
    let ans = 10000000;
    for(let i = 1; i*i <= n; i++) {
        let j = Math.floor(n / i)
        let k = n - i * j
        ans = Math.min(ans, Math.abs(i-j) + k)
    }
    console.log(ans)
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))