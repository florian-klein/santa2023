import permgroup, minkwitz
import schreier
from sequtils import mapIt
from strutils import `format`, join, splitLines, `%`
import options

# 24
# f0: [0, 1, 19, 17, 6, 4, 7, 5, 2, 9, 3, 11, 12, 13, 14, 15, 16, 20, 18, 21, 10, 8, 22, 23]
# -f0: [0, 1, 8, 10, 5, 7, 4, 6, 21, 9, 20, 11, 12, 13, 14, 15, 16, 3, 18, 2, 17, 19, 22, 23]
# f1: [18, 16, 2, 3, 4, 5, 6, 7, 8, 0, 10, 1, 13, 15, 12, 14, 22, 17, 23, 19, 20, 21, 11, 9]
# -f1: [9, 11, 2, 3, 4, 5, 6, 7, 8, 23, 10, 22, 14, 12, 15, 13, 1, 17, 0, 19, 20, 21, 16, 18]
# r0: [0, 5, 2, 7, 4, 21, 6, 23, 10, 8, 11, 9, 3, 13, 1, 15, 16, 17, 18, 19, 20, 14, 22, 12]
# -r0: [0, 14, 2, 12, 4, 1, 6, 3, 9, 11, 8, 10, 23, 13, 21, 15, 16, 17, 18, 19, 20, 5, 22, 7]
# r1: [4, 1, 6, 3, 20, 5, 22, 7, 8, 9, 10, 11, 12, 2, 14, 0, 17, 19, 16, 18, 15, 21, 13, 23]
# -r1: [15, 1, 13, 3, 0, 5, 2, 7, 8, 9, 10, 11, 12, 22, 14, 20, 18, 16, 19, 17, 4, 21, 6, 23]
# d0: [0, 1, 2, 3, 4, 5, 18, 19, 8, 9, 6, 7, 12, 13, 10, 11, 16, 17, 14, 15, 22, 20, 23, 21]
# -d0: [0, 1, 2, 3, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13, 18, 19, 16, 17, 6, 7, 21, 23, 20, 22]
# d1: [1, 3, 0, 2, 16, 17, 6, 7, 4, 5, 10, 11, 8, 9, 14, 15, 12, 13, 18, 19, 20, 21, 22, 23]
# -d1: [2, 0, 3, 1, 8, 9, 6, 7, 12, 13, 10, 11, 16, 17, 14, 15, 4, 5, 18, 19, 20, 21, 22, 23]

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
    let tt = buildShortWordsSGS(gens, base, n=100, s=200, w=100)
    let fact = factorizeM(gens, base, tt, perm)
    let generated_perm = composeSeq(gens, fact)
    echo "Factored: ", fact, " | Length: ", fact.len
    assert generated_perm == perm

# print size of transtable in mb

# let str_elm2 = "[228, 22, 258, 49, 4, 235, 229, 25, 42, 52, 47, 39, 0, 31, 206, 211, 62, 40, 213, 51, 215, 221, 223, 253, 65, 251, 260, 219, 227, 262, 37, 202, 11, 35, 231, 225, 16, 220, 58, 20, 239, 209, 57, 204, 12, 45, 241, 252, 60, 232, 18, 217, 55, 14, 43, 9, 8, 246, 254, 212, 234, 224, 29, 38, 244, 30, 128, 116, 148, 187, 74, 138, 180, 110, 90, 150, 190, 89, 162, 136, 154, 120, 98, 70, 77, 104, 133, 175, 143, 84, 99, 79, 129, 100, 166, 195, 182, 103, 76, 149, 183, 158, 71, 83, 197, 119, 176, 117, 96, 151, 168, 160, 171, 167, 186, 75, 163, 164, 132, 121, 192, 181, 155, 152, 191, 140, 194, 82, 153, 91, 173, 130, 127, 114, 81, 161, 145, 141, 139, 169, 122, 105, 144, 109, 178, 112, 95, 101, 85, 157, 126, 68, 137, 111, 86, 179, 123, 80, 124, 146, 94, 73, 184, 147, 170, 67, 177, 108, 97, 193, 165, 135, 196, 185, 93, 106, 78, 88, 87, 156, 131, 115, 188, 159, 92, 118, 142, 174, 69, 72, 113, 107, 189, 125, 172, 66, 134, 102, 3, 250, 21, 199, 36, 259, 61, 216, 27, 237, 5, 63, 32, 59, 207, 247, 6, 53, 218, 210, 54, 1, 10, 233, 226, 64, 44, 261, 243, 2, 56, 205, 198, 46, 13, 222, 19, 34, 24, 214, 28, 50, 263, 240, 257, 48, 33, 7, 236, 238, 242, 41, 203, 17, 200, 255, 208, 248, 26, 201, 23, 249, 15, 230, 245, 256]"
let str_elm = "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95]"

let group_path = "./groups/G_cube_4_4_4"
let size = readPermutationSize(open(group_path, fmRead))
let data2 = readGeneratorString(open(group_path, fmRead))
# let myPerm = parsePermFromArrayFormat(96, str_elm)
let myPerm = parsePermFromArrayFormat(96, str_elm)
# create array of length 6534 with elements from 0 to 6533
var base : seq[int] = newSeq[int](size)
for i in 0..size-1:
  base[i] = i

const length = 96
let result = factorizeWord(data2, base, myPerm, length)
echo result


