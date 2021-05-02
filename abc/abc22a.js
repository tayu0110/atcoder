function main(input) {
    input = input.trim(' ').split('\n')
    let [nst, w, ...a] = input
    let [n, s, t] = nst.split(' ').map(e => parseInt(e, 10))
    w = parseInt(w, 10)
    let ans = (w >= s && w <= t) ? 1 : 0
    a = a.forEach(e => {
        e = parseInt(e, 10)
        w += e
        if(w >= s && w <= t) ans++
    })
    console.log(ans)
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))