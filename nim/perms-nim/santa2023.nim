import permgroup, minkwitz
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

proc factorizeWord(gens: string; base: seq[int]; perm: Perm; N: static[int]): seq[int] =
    let gens = N.parseGensFromArrayFormat(gens).normalize
    let base = base.mapIt(it.Point)
    let tt = buildShortWordsSGS(gens, base, n=500, s=120, w=150)
    let fact = factorizeM(gens, base, tt, perm)
    let generated_perm = composeSeq(gens, fact)
    echo "Factored: ", fact, " | Length: ", fact.len
    assert generated_perm == perm

proc factorizeWordUsingKalka(gens: string, perm: Perm, N: static[int]): seq[int] =
  let gens = N.parseGens(gens).normalize
  let list = gens.factorizeK(perm)
  echo gens.composeSeq(list) == perm 
  return list



# let str_elm2 = "[228, 22, 258, 49, 4, 235, 229, 25, 42, 52, 47, 39, 0, 31, 206, 211, 62, 40, 213, 51, 215, 221, 223, 253, 65, 251, 260, 219, 227, 262, 37, 202, 11, 35, 231, 225, 16, 220, 58, 20, 239, 209, 57, 204, 12, 45, 241, 252, 60, 232, 18, 217, 55, 14, 43, 9, 8, 246, 254, 212, 234, 224, 29, 38, 244, 30, 128, 116, 148, 187, 74, 138, 180, 110, 90, 150, 190, 89, 162, 136, 154, 120, 98, 70, 77, 104, 133, 175, 143, 84, 99, 79, 129, 100, 166, 195, 182, 103, 76, 149, 183, 158, 71, 83, 197, 119, 176, 117, 96, 151, 168, 160, 171, 167, 186, 75, 163, 164, 132, 121, 192, 181, 155, 152, 191, 140, 194, 82, 153, 91, 173, 130, 127, 114, 81, 161, 145, 141, 139, 169, 122, 105, 144, 109, 178, 112, 95, 101, 85, 157, 126, 68, 137, 111, 86, 179, 123, 80, 124, 146, 94, 73, 184, 147, 170, 67, 177, 108, 97, 193, 165, 135, 196, 185, 93, 106, 78, 88, 87, 156, 131, 115, 188, 159, 92, 118, 142, 174, 69, 72, 113, 107, 189, 125, 172, 66, 134, 102, 3, 250, 21, 199, 36, 259, 61, 216, 27, 237, 5, 63, 32, 59, 207, 247, 6, 53, 218, 210, 54, 1, 10, 233, 226, 64, 44, 261, 243, 2, 56, 205, 198, 46, 13, 222, 19, 34, 24, 214, 28, 50, 263, 240, 257, 48, 33, 7, 236, 238, 242, 41, 203, 17, 200, 255, 208, 248, 26, 201, 23, 249, 15, 230, 245, 256]"
let str_elm = "[4, 5, 30, 25, 0, 2, 1, 3, 18, 14, 20, 15, 17, 9, 8, 13, 19, 22, 10, 16, 12, 21, 11, 23, 26, 7, 6, 27, 31, 24, 28, 29]"

let group_path = "./groups/G_globe_3_4"
let size = readPermutationSize(open(group_path, fmRead))
let data2 = readGeneratorString(open(group_path, fmRead))
let myPerm = parsePermFromArrayFormat(32, str_elm)
var base : seq[int] = newSeq[int](size)
for i in 0..size-1:
  base[i] = i

const length = 32
let result = factorizeWord(data2, base, myPerm, length)
echo result


