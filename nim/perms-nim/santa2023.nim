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

# let str_elm = "[4, 5, 30, 25, 0, 2, 1, 3, 18, 14, 20, 15, 17, 9, 8, 13, 19, 22, 10, 16, 12, 21, 11, 23, 26, 7, 6, 27, 31, 24, 28, 29]"
# let short_perm = parsePermFromArrayFormat(4, "[2, 0, 1, 3]")
# echo "signature", short_perm.signature

let gens = 3.parseGens("A: (1 2)\nB: (2 3)").normalize
let perm = 3.parsePerm("(1 2 4 3)")
echo perm
echo perm.signature
# let list = gens.factorizeK(perm)
# echo "Solution String in Kaggle Format: ", get_str_repr(gens, list)

# let group_path = "./groups/G_globe_3_4"
# let size = readPermutationSize(open(group_path, fmRead))
# let gens = readGeneratorString(open(group_path, fmRead))
# let myPerm = parsePermFromArrayFormat(32, str_elm)
# var base : seq[int] = newSeq[int](size)
# for i in 0..size-1:
#   base[i] = i
#
const length = 32
# # let result = factorizeWord(gens, base, myPerm, length)
let result = factorizeWordUsingKalka(gens, perm, length)
