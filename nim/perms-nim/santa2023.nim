import permgroup, minkwitz
import kalka
import schreier
from sequtils import mapIt
from strutils import `format`, join, splitLines, `%`
import options


proc readPermutationSize(f: File): int =
    return strutils.parseInt(readLine(f))

proc readGeneratorString(f: File): string =
    # first line: size of generator arrays
    let size = strutils.parseInt(readLine(f))
    # second line: num of generators following this line
    let next_lines = strutils.parseInt(readLine(f))
    # next lines until end: generators in array format
    var lines = newSeq[string](size)
    for i in 0 ..< next_lines:
        lines[i] = readLine(f)
    return join(lines, "\n")
#
proc factorizeWord(gens: string; base: seq[int]; perm: Perm; N: static[int]): seq[int] =
    var gens = N.parseGensFromArrayFormat(gens).normalize
    let base = base.mapIt(it.Point)
    let tt = buildShortWordsSGS(gens, base, n=500, s=120, w=150)
    echo "sgs: ", tt
    let fact = factorizeM(gens, base, tt, perm)
    let generated_perm = composeSeq(gens, fact)
    let str_repr = get_str_repr(gens, fact)
    echo "Solution String in Kaggle Format", str_repr
    echo "Factored: ", fact, " | Length: ", fact.len
    assert generated_perm == perm

proc factorizeWordUsingKalka(gens: string, perm: Perm, N: static[int]): seq[int] =
  var gens = N.parseGensFromArrayFormat(gens).normalize
  let list = gens.factorizeK(perm)
  let str_repr = get_str_repr(gens, list)
  echo "Solution String in Kaggle Format", str_repr
  echo "Length: ", list.len
  assert composeSeq(gens, list) == perm
  return list

let str_elm = "[24, 6, 7, 2, 29, 5, 27, 26, 18, 22, 23, 10, 21, 15, 17, 13, 8, 20, 9, 16, 14, 11, 19, 12, 3, 4, 31, 25, 28, 30, 0, 1]"
# let short_perm = parsePermFromArrayFormat(4, "[2, 0, 1, 3]")
# echo "signature", short_perm.signature
const 
  W8 = 8 
let data = "A: (1, 5, 7)(2, 6, 8)\nB: (1, 5)(3, 4, 8, 2)\n" # S_4, standard
let gens = W8.parseGens(data).normalize
let base : GroupBase = @[0, 1, 2, 3, 4, 5, 6, 7].mapIt(it.Point)
let tt = buildShortWordsSGS(gens, base, n=10000, s=3*3, w=100)
for i in 0..tt.len-1:
  for j in 0..tt[i].len-1:
    if tt[i][j].isSome:
      echo tt[i][j].get
      echo (i, j)

# let gens = 3.parseGens("A: (1 2)\nB: (2 3)").normalize
# let perm = 3.parsePerm("(1 2 4 3)")
# echo perm
# echo perm.signature
# let list = gens.factorizeWord(perm)
# echo "Solution String in Kaggle Format: ", get_str_repr(gens, list)

# let group_path = "./groups/G_globe_3_4"
# let size = readPermutationSize(open(group_path, fmRead))
# let gens = readGeneratorString(open(group_path, fmRead))
# let myPerm = parsePermFromArrayFormat(32, str_elm)
# var base : seq[int] = newSeq[int](size)
# for i in 0..size-1:
#   base[i] = i
#
# const length = 32
# let result = factorizeWord(gens, base, myPerm, length)
# let result = factorizeWordUsingKalka(gens, perm, length)
