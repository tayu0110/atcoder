function main(input) {
    let s = input
    let t = 'BWBWBW'
    let scale = ['F#', 'So', 'S#', 'La', 'L#', 'Si', 'Do', 'D#', 'Re', 'R#', 'Mi', 'Fa']
    let scaleNum = scale.length

    let ans;
    for(let i = 0; i < s.length; i++) {
        let u = s.substr(i, 6)
        if(u === t) {
            if(i % scaleNum === 0) i = scaleNum
            ans = scale[scaleNum - i]
            break
        }
    }

    console.log(ans)
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))