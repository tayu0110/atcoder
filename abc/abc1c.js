function main(input) {
  input = input.trim(' ').split('\n')
  let [deg, dis] = input[0].split(' ').map(e => parseInt(e, 10))
  deg *= 10
  let dir
  if(1125 <= deg && deg < 3375) dir = 'NNE'
  else if(3375 <= deg && deg < 5625) dir = 'NE'
  else if(5625 <= deg && deg < 7875) dir = 'ENE'
  else if(7875 <= deg && deg < 10125) dir = 'E'
  else if(10125 <= deg && deg < 12375) dir = 'ESE'
  else if(12375 <= deg && deg < 14625) dir = 'SE'
  else if(14625 <= deg && deg < 16875) dir = 'SSE'
  else if(16875 <= deg && deg < 19125) dir = 'S'
  else if(19125 <= deg && deg < 21375) dir = 'SSW'
  else if(21375 <= deg && deg < 23625) dir = 'SW'
  else if(23625 <= deg && deg < 25875) dir = 'WSW'
  else if(25875 <= deg && deg < 28125) dir = 'W'
  else if(28125 <= deg && deg < 30375) dir = 'WNW'
  else if(30375 <= deg && deg < 32625) dir = 'NW'
  else if(32625 <= deg && deg < 34875) dir = 'NNW'
  else dir = 'N'
  dis = Math.round(dis/60*10)
  let w
  if(0 <= dis && dis <= 2) w = 0
  else if(3 <= dis && dis <= 15) w = 1
  else if(16 <= dis && dis <= 33) w = 2
  else if(34 <= dis && dis <= 54) w = 3
  else if(55 <= dis && dis <= 79) w = 4
  else if(80 <= dis && dis <= 107) w = 5
  else if(108 <= dis && dis <= 138) w = 6
  else if(139 <= dis && dis <= 171) w = 7
  else if(172 <= dis && dis <= 207) w = 8
  else if(208 <= dis && dis <= 244) w = 9
  else if(245 <= dis && dis <= 284) w = 10
  else if(285 <= dis && dis <= 326) w = 11
  else if(327 <= dis) w = 12
  if(w === 0) dir = 'C'
  console.log(`${dir} ${w}`)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))