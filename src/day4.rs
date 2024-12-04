use ndarray::{Array2, ArrayBase, Dim, OwnedRepr};

struct ProcessedInput {
    array: ArrayBase<OwnedRepr<char>, Dim<[usize; 2]>>,
    line_count: usize,
    line_length: usize,
}
impl ProcessedInput {
    fn process(input: &str) -> Self {
        let chars = input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let line_count = chars.len();
        let line_length = chars.first().unwrap().len();

        let array = Array2::from_shape_vec(
            (line_length, line_count),
            chars.into_iter().flatten().collect(),
        )
        .unwrap();

        Self {
            array,
            line_count,
            line_length,
        }
    }
}

fn part_1(input: &str) -> usize {
    const POSSIBLE_COORDINATES: [[(isize, isize); 3]; 8] = [
        [(-1, 0), (-2, 0), (-3, 0)],
        [(-1, -1), (-2, -2), (-3, -3)],
        [(0, -1), (0, -2), (0, -3)],
        [(1, -1), (2, -2), (3, -3)],
        [(1, 0), (2, 0), (3, 0)],
        [(1, 1), (2, 2), (3, 3)],
        [(0, 1), (0, 2), (0, 3)],
        [(-1, 1), (-2, 2), (-3, 3)],
    ];

    let ProcessedInput {
        array,
        line_count,
        line_length,
    } = ProcessedInput::process(input);

    let mut count = 0;

    for y in 0..=line_count {
        'x: for x in 0..line_length {
            if array.get((x, y)).is_none_or(|&char| char != 'X') {
                continue 'x;
            }

            'c: for coordinate in POSSIBLE_COORDINATES {
                let mut temp = String::from("X");

                for (relative_x, relative_y) in coordinate {
                    #[allow(clippy::cast_sign_loss)]
                    #[allow(clippy::cast_possible_wrap)]
                    let Some(char) = array.get((
                        (x as isize + relative_x) as usize,
                        (y as isize + relative_y) as usize,
                    )) else {
                        continue 'c;
                    };
                    temp.push(*char);
                }

                if temp == "XMAS" {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part_2(input: &str) -> usize {
    let ProcessedInput {
        array,
        line_count,
        line_length,
    } = ProcessedInput::process(input);

    let mut count = 0;

    for y in 1..line_count - 1 {
        for x in 1..line_length - 1 {
            if array.get((x, y)).is_none_or(|&char| char != 'A') {
                continue;
            }

            match (
                array.get(((x - 1), (y - 1))),
                array.get(((x + 1), (y - 1))),
                array.get(((x + 1), (y + 1))),
                array.get(((x - 1), (y + 1))),
            ) {
                (Some('M'), Some('M'), Some('S'), Some('S'))
                | (Some('S'), Some('S'), Some('M'), Some('M'))
                | (Some('M'), Some('S'), Some('S'), Some('M'))
                | (Some('S'), Some('M'), Some('M'), Some('S')) => count += 1,
                _ => (),
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_example() {
        const INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part_1(INPUT), 18)
    }

    #[test]
    fn p1_puzzle() {
        assert_eq!(part_1(INPUT), 2536)
    }

    #[test]
    fn p2_example() {
        const INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part_2(INPUT), 9)
    }

    #[test]
    fn p2_puzzle() {
        assert_eq!(part_2(INPUT), 1875)
    }
}

const INPUT: &str = r"XMXXMSSSMSXSXMMXSAMMXXSXMASMSSXXMAMXAMXSXMXSMAMMASXXASMMXMASXMSSXMMMXMXSXXSXMXXSAMXSXSXSAMXMSAMXMAXXXMXMAMSASXMSSXMSXSXXMAXXSSSMXMXMXMMAASXM
MSSMMAAXAMMSAAAXXAMSXMAMXMSMAMMXSAXMXMASAMASMAMMASXMAMAXXXMASAAAXMSMSMAMAMMAMAMMMMXMASMMXMXAMASXMAMMMMASMMSMMMMASAAXAAMMMMSMXAAMSAMSASMMMMAX
AAAXMMXMSMASXMMSMXMMASMAAMXMASAMMAXSAMASAMASXXSMASASMSMMMMASMMMMMMAAXMXSAMSAMASMASAMAMASMMMMSAMXMAXAASASAAXAAXMASMMMSMXAAXAAMSMMMAXMAXMAASMM
SSMMXSAAXMMSAXMAXAAXAMXXMSSSMSMAMAMXAMXXXMXSXMMMMSAMXAXXAMXAXXMAXMASMXASAXXMSAXXASAMAXAMXXAXMXXMSASXMMASMMSSMSMMSASAMXSSSSMSAAAXMMMMXMMSXSXA
XAAAAMMSMSXXMXAASXSMSMSMXXMAMXMASXSSMMXMAMAMMMAAMMAMMMXSSMSAMXSMSASMXXMSXMAAMAMMMSXMAMMSXSASASAMXASXAMAXMAAAAXXMSXMXMXMXMAXAMXSAXAAXAXAXAMXS
SXMMSXAXASXAMMMMSXMXMASXMMXMMSMAXXMAMAMXAMASASMSMXAMXMMAMASAMXSXMMXAXSXSAMXMMAMAAMXMASMAXAAMAMSMMXMMMMMSMMMSAMXAXMMMSXSASMMMSMMMSSSMMMMMAMSM
AAXAXMSMAMSMMAAMSMSAMMMAMAASASMASMSAMAMMASASASXMASMSMSAAMAMAMAMASXMMMAASXMAXSASMMXAXAMMMMMSMXMAXSASASAMXXSMMMSMMMAXAAASAXSAXXAAAMAMXSASMXMAS
SSMMSMAMSMSXMSMMSASXSXSAMSMMASASMMSASASMXMMXMMXMASAAAXAMMXSAMASMMMAXXMMMXSAMXXSAMSMMSSXXAAAMMSSMSASXMASMXMASAXAMSSMMMMMMMMXSSSMXSAMXSASXMSMS
XMAMAXMXXAMAMAMAMXMXXASAXAXMXMXASASAMASMSAMXXSSMMMMMMMAMXMSASXMMASMMSSMMMSMMMMMMMASMXAMSMSMSAMXAMAMAXAMMMMAMMMSAAXSASASASXMAMAAXMASXMMMMXSXS
SMASXSXASASAMAMXSSSSMASXMASMSMSAMXMAMXMASAMSXMAASAMSXXMMXASAMXMSAMAAXXAAAMAMMAAMXAXXMAXMAAXMAXMXMSMSMMMMAXMMSAAMXSMXSASAAMAMSXMMSXXAXAAXMSAS
XMAXMXAXSAMXXXMAMXAAXMMMXAAAAAXASMSSMXMXMAMXASAMMASMXSXMXMMAMAAMASMMSMSMXSAMXMMSMSSXSXMMSMSSSMXXAMAXMAXSAXMAMXSAMXXXMMMMMASXSMSXXMMMSMMMAMMM
SMSSSXSAMXMMSMSMXMSMMAAXXSXSMMSAMAMAXXXXSAMSAMASMXMMASAMXXMAMSSSMMAXAXAXXMMXSAMSAMAMXAXXMXAAXXAXMMSMMSAMXSMASAMAASAMSAMXXSMAXAMMXAAMAMXSMSMS
SAXAMAXAMAAASAAAAMAXMMXMMXMXAMXAMAMMMMMMSSXMAXAMAXAMMMMSAMSMXXMAMXMSXSMSMSAAXAMMAMMMSXMMXMMSMAMSMAMAAXMSAAXAMXSMMAMAAASXMMMMMSMMMSXSASMSAAAA
MMMAMMMSXSSMMSMSXSAMMSMMASXMSMSMSASXAXAXMXXSXMSSXSMXSXXXAXAMMMXAMAAXMSXAMMMSSMMSAMAMAAMSAMXAMAMXMAXMMSAMXSMMSAXASXXXSXMXXAAAAMAMAMXSAMAMSMMM
SSSSMMXMXMXAAMXMASAMSAMMAXXAAAAAXASXMSXSAMXMAXXAXXMASMSMMSMSAMMSSMMXAMSXSAMXAMAXXXASXMMAMMXXSXMAXMXSAMXMAMMXMASMMMSXMAMAXXSSMSAMSXMMSMMMMMMM
SAAXMMAMAXSSMSAXAMAMMAXMMSMSMMMSMSMMXAMMXAASMMMXMAMASAAAAAASMSAXAAAMMMMMSASMMMMSSMMXAXSMMSAMSAMAMSAMAMAXSXMAMAAXAAAMSXMASXAAMMMMAAMAMXXXAAAA
XMSMSMASMMMMASMMSSSMSSMSAAXXASAMXMAMMASMMSXSXAMXSXMAMXMSSXXMAMXSMMXSAAAASXMASXSAMMSSSMXAAMAMSAMSSMASXMSAMXSSSSSMXSAMXXSASMMMMAAMSSMAXASMSSSS
MXMAASMMMAASXMXXAAXMAAAMSSMSXMASAMAMSSMAAXAMMMXXMXMAXAXMAMXMXMXXXMASXSMXMMXAMXMASAXMMAMMMSMMMAMXAXAMMMMAMAMAAXXAMMMXXMMXMXSXSSMMAMMSMXMAMAMM
XAMMMSXAXSXSMMXMMMMMSMMMAMASXMAXXXXMXAMMXXASAXXAXMSAMSSMSAMXAMXMMMXSAMXAASMSSXSAMMSAXSMSAAAASAMXSMXXAXXXMAMMMMSXMAAMMMXAAXXAMXXMASAAMXMAMAMX
SASMAMMMMMASAMXMMXSAAXXMXMXMASXSMSMMSAMSSSSMAASMXMASMAAAMASMAMAAXMAMXSSMMMAXAAMXMXMMMXAMSSSMSAMXAAMSSSMMSXSXAXMAMXMMAXXMSMMMMAXSAAXASXSMMAXX
MASMMMAXAXAXXMXSXXMAMSAMMSXSAMAAAAAAXAMAAMMMXMAAAXAAXAAMMAMMAXASMXXMSMASMMMMMMMAXAASAMXMAMAXMXSMMXMAAAAAAASMMMMAMMSSXMXMAMAAXSMMXXXSAMXASMMS
AASAMMSXMAMSSMASMMMAXMASAAXMASMMSMMMMSMMSMSSSXMSMMMSAMMSXASXMSMXMAAMASAMXAXSMXSMMSXXMXSAMXXMSSMMXMMMSMMMMAMAMASXMAXMASMSMSSSXMAMAMSAXSAMXSAA
MASAMAXXXMAAAMAXAMXXXSAMMMMMMMXXXXXSAXAXMAAAMXMAXXXMAMAMMAMXAAAAMSSMAMMXMMMMAAXMAMMMMAMAXXMASXAXMXAAXMAMXASAXMAAASMMMMMAXAXXMXAMASAAMXXXAMMM
SASAMASASAMSSMASXMMSAMMMMASXMASMXMMMASAMXMMSMXSAXSMSAMASMMSMSMMMMAAMAMXSAMSSMMSMASAAMAMAMXASMMMMMSMMSASAMASASMMSMXAAAMMMMXMSMSSMXSMAAMMMXSAX
AAXAMXSAMMAAAMASAAAMXXXASASAMMAMAMAMMMMMMSMMMMMMSMASXSASAAAAAXAMMSSMSAAMAMAAXXXMAXMMSXSAMXSAXSAAXAAXMAXAAMMMMAAMASMMMSXXXAAXMAAXXMXMXAAMAMMS
MXSSSMMMMXMSXMASMMMSMMSXSASXMMMSXSXXXAXAAAAAMAAXAMMMAMAMMMMMMMMSAMMAMMXXMMSSMMAMSSMXAMMXSMMSXMASMMSMMAMXXSASXMMMAMMXAAMSMMXXMSSMXMASMMMSXSAS
XMMAAASAMXXAMMXXAMAXAAMXMAMAASXMAXMASMSMMXSMSXSXSSMMXMSMMSXXAAXMAXMXMMMSMAAAASMXAAMMAMSASXMXMSAMXAXXMSMMASAMXMAXSMSXMSAMXSAMMAAAASASMMMAAMAS
MMMMMSXMSASMSXSSSMAMSXSAMAMMMSAMXMXASAAMMMMASAMXMAMMMAXAAAMSSSSSMMAAAASAMXSSXMMXSMMSAMMMSASAMXMMMSMMMXAXAMXMSSXSAAXAMXXSAMASMSSSMXAMAASMMMAM
MASAMXMXSAMXAAXAMMXXMASASXSSSSMMMXMXSXXSAAMAMAMMAMASMMXMAMXAMXMASMXXSMSASMMMMASAMAXMAMAXXMMASMMAAAAAASAMXSAXAAMSMMSXMAXMASAMAXAAAMSMSMSAAMAS
SASMSAAAMASMMMMAMXMASAMXMAAXASXASAAMMXASMSMAMXMAMSAXSMAMXSMMMSMMMMAXXXSAMAAAXXMASMMMASMSSXSAMASMSSSMXMAMAXXSMXMXXASMMMXSAMXMMMMMXMMAXASMMSAS
MASXSMSXMSMMAXMXMASMMMSSMMMMMMMMSMXAAXMSXAMSMSXXAMXXMMMSMMASAMMXSMMMSAXMASXSSMMXXMASXAMXAMMMSXMMMAXMASMMSSMMXMXXMXXAXAAMASXAMAXMXMMSMMMXAMAS
MSMAMAMMXXASMSMXSMMXASAMXMXXAASXMMSMMMXMXMXMAMMSMMSMSAMAMMAMASXAXAAAMMMSXMAMMXSXMXMMXSASAMAXMAMSMMMSAMAAAAAMXSMMSASMMMMSMMMMSMSMXSAMXAXMMMAS
SAMXMXMMAMAMXXMAMAASMMMMSXMSSSSXAASAMSXMXXAMAMAAXMMASXXXSMXXAMMSSSMSSXXMASMXAASMXAAAMXMSASXXSAMXAAAMAMMMMSSMMMASXMAAXAAAXMAXAMAMXMASMMSSSMSS
AMMASXAMMSMSMSMSSSMXMASAMMMAMAMMMMSAAMAMSMMSSMSMSAMXMSMAMXSMXMMAXXMAMAMXMMXMMXSASMSSXSXSAMAASAMXSMSSMSSSMXXMAXSMAXSMMMSSSSSSSSSSXSAMAXAAAMAS
MXMAXMAXXAASAMAAAMMMSAMASAMASASXMASMMXAMXAXAXAMXMMMAAAMAMAXAAMMMSMMMSSXXAXAAMMMMMAAAASAMSMMMSASAAXXAAAXAXSSSSSMSSMXXAMMAMXAXXAMAASMSSMMSMMAS
XSMAXMSSMSMMAMMXSMAMMXXXSAMASXSXMXXASMXMMMMMXSSMAAXASMSSMMSSMSAMXXSMAMXMASMXSASXMMMMMMAMAXXXXAMMSMXMMMSMXMAAMXAAMAASXSMMMMMMMMMMMMAAXAXMAMMM
AMMAMSAAXMASXMXAMMASMMSMMMSXMAXAXAXAMMMSSMAXSMAMSMSAMXXMAXAAXSASMAMMAMASXSAASXSAXXXXSSSMMSSSMMMXMAMXMXAXMMMMMMMMXSXSASAMASAAXAAAAXMASMMMSSSM
AMAXMMASMXXMASAXMASAAASXAAMASMSXMMMMSXAAASMXXMXMAMMAMMSSMMSSMSAMMSMSXSMSAMMMSSSXMASXXAMXXAAAAASMXAAAMMXSAAAAAXAXXMSMASXMASMSMASXSSMMXAXXAAAS
SSMSMMMMMXSSXMASMMMSMMMMXMSAMXMAAXAAAMMSMMMXXSMMXMSAMXAASAMXMMSMXXAAAXAMAMAMMASXMMXMXMMMMMSMSMSASMSXSAXMMSSSSSMSAXXMAMAMXMXXAMXXAAXAXMMMMSMM
XAASASAAXSAMXMASAAXAXMASAXMXSASXMMMSSSXMASMSMAASAAXMASMAMSMXXAAXAMXMMMSMASASMAMMMAXSMXAXAAMMMXMAMAXAXMMXXMAXAAASMMSMXSMMASMMSMSMSMMMMSASAMXS
MMMSAMMSSMMMXMASMMXMMSAMXMASMAXXAXXAAMXSMAAAMXMMAXXXMMXSSMSMMSSMMAAXMAASASAMMAXAMSSMASASMMSASMMAMXMMMSAMSMSMSMMMMSMXAMAMASMAAXAMMXAAAAAMXSAS
SMXMAMAMXXXMAMASAMXSAMMSSMMMMAMMSMMXMAMMSMSXSMSSMMSSSMAXAAAXAAAMSSXSASMMMMMSSSSSSMAMXMAAXAXXXAXXXAASAMASXAXAXAAAMASMXSAMMSMSMSMAXASMSSMSXMAS
SAXSAMXSASMSASASAMXMASAAXAAXMAMAAXMAMSMAXXXAMXAASXXAMMSSMSMSSMSMMAAAMXXAMSMMAAXMAMAMXMXMMMSSSSMSSSXMASXMMSMSMSXMSAMMXSXSXXAAAMXMSMXAMMMXAMAM
MXMMMAAMASASASMSAMSMSSMMSSMSMAMSSMMMXMMSMMMMAMSSMXMXMAXXAXXAASAAMMMMXMMXXAAMMMMMAMXSSMAXAXMAAAXAAXXSMMMMAXAXAAXXMXSMMMMMMMSMSMAMMXMSMSXSXMAS
MAMXSMMMAMMMAMASXMAXAXMXMAMAMAXMAMXAMMAMAXASAAMAMMMSMMSMXMMXMMSXMXXMXMASXSMMXSAMXXSAXSASXSSMMMMMSMMXASAMXMMMMMSXAMXMAAAMAMXMMMXSMAXXAMXMAMAX
SMSAXAASMSSMMMAMASMMMSSSSSMSSMSMASXSXMAMMSMSAXSAMAAAAMXXSXAMSAMXMASXSMMSAMAMAMAXASAMXMMXAXAAMXMXMAMSXMAXXAXASMMMSMAMSXMSSSMMAMXMMMSMAMASXMSS
XAMMSSMSAAMXXMSSMXXXXAAMAXAAAXAMMMMXAMASAMXXMAXAMMXXSMASXMXSAASAMAMMAAMMXMSMMSSMMSSXMSSSMSSMMMSMSAMMXSAMSMMMXAAAASMXMXMMXAMMMMAMAMXMAMMSAAXA
MMMMAMMMMMSSSXAAXXMMMMSMAMMMMXXMAAAXXMAMMSSMMSSSMSXMMMMMAAXXXMMAMASXSXMAXXXAXAMAXXASAMXAXAAAMAAXSMSAAMAMAASMSSMSXMXAMASXSXMASMMSXSASASXSMMMM
XXAMSSMXAXAASMSSMMMAAAAMMMXXXMSXSMXSAMXSMAXAXMAXAAMSASXSSMSMSSSXMXSXMAMMSMSMMXSXMAMMXXXMMSSMMMXXXAMMMSXSMSMAAMMXXMSXSAXAMASASAXAAXASAXMXSAAX
SSMSAAMSSMMMMMAAXAXSXXSSSMSSMMSAAAASAMASMSMMMMAMMMXSASAAMAAASAMXMASMSAMSAAAASMMMSAXSSSSSMXMAASXSMMMSXMXMMMMMMXXAMMMAMMMSMAMASMMMSMXMXMMAMXMS
AAXXMMMAMXAMXMXMMSMXAAMAMAMXAAMSMMMXAMASMAASAMXSASAMMMMMMXASMXSASMXAXASMMMMXMAAAXAXXAAAMMAMXMAAXAXASAMMMASXSMMMMSAMXMXAMMXSXSXMAXAMXXXMASAXM
SSMSXXMAXSAMXMXXAXAMMMSAMXMMMMXMXXASXMASMSMXASASASMMAMAXMMMMMMSASXSSMXSMAAMASMMSSSMMMMMMSSSSSMMSSMASAMAMAMXAAAAXMMSMMMMSAMXMMMMAXXMXMMSAMASX
MAAXMXMMMMSMXAAMXMMXMASAMASXASAMXXMAXSXSAXMSMMXSAMXSASMSMSAAAAMAMAXAAXMMSSMASAAAAAMSMXSAAXXAXXXXMMMMAMSMASXSSMMSXAAAXAXMAXSAASMSSSMASXMXMSAX
SMXMAAXSAXAXXMXXXAXSMMMAXAMMAMXSXSXMXSAMAAASAMAMAMASMSAAASMXSSMSMSMMXSAAAAMSMMSMSSMXSAMMMSMMMSAMXAMMSMXMASAMXXMAMSSSSSMSAMXSXSAAAASXMAXAXAAX
SAASXSXSXSMSMMASMMMASASMMSMSSSMMMSAMXMAMSAMXAMXSAMXSXXAMMMXMAMXAAASXASMMSSMXAXXAAMXMMMXAAAAAAXMASASAAXAMXSAMAMMMMAMXXXXMASAMMMMMSMMMSSMMXMSM
MSMSAXXMMXXAXAAMAAAAMMSMAXAAAAXMASXMASAMMSXSXMAMXSXMXXXXSAMMASXMSMSMMSAAXAASXMMMMSAMASMMXSMMMMAMSAMXSSXSASAMXMAMMAMMMMXXAMAXAXAAAAAXAMXSAAXM
MAXMMMXSASXMSMSMSAMXSASMXMMMSMMMMSASXSXSAXXMASXMAMAMMMMMMASMASAXAAXXMMMMSAMXMXAMASXXAXAXAXMAMXSAMMXSAMAAAXXMAMASMMMAASAMSSMMSMMMSSMSSMASMSMS
MSMAMAAMXMAAAAAMXSXXMASMMMAMXMASMSAMAMAMMSMSMSAMXXAMAAAAMXMMASMMMSMSSXMXSXSAMMSSXMMASXMMXMASMMXAMAMMAMMMSSSSSMMMAASMSMMMAAAAXASAMXAAXMAXXXAX
MAMAMMSXMXMMMMMSAMXMMXMAXXAMXAMXMMXMAMAMXAAAASAMASXSSSSSSXXMASAXXXXAAXSASMSASAAMMXAXXAXXAMAMMAMAMAMMAMXXAAAXAASMSMMXXAAMMSMAXAMXXMMMMMXMAMSM
SASASXAMSSSMXAAMAXXAMXSAMMXSASXSXAXMASXSMMMMMMAMMSAAAMAAMAXMXMAMXXMMMMMASAMAMMSMASMMSSMSMSAXMASXSXSMSMMAMMMMMMMAMSMMSSMSXMASMMMSSSMSAASMSMAS
SASASMASAAAMSMSMSSSSMAMASXMAXMAXMMXSASAMASAXAXXMXSMMMMMMMMXSSMSSXSAXXSMAMMMSMMMAXAAXAAAAAMMSMMSAXAAXMAMXMMSSSXMXMAXAAXAXASAXAMAAMAASMSMAAAAS
MAMXMAMMMSMMMAXMAAAXMAXAMXXMSMSMSAAMAMAMAMASXSAMAMSXXSMMXSAMAAXAASMMAXMMSMMMASXSSSSMSSSMSMAAXSMXMMMASAMXMMAAAXSXSSMSSMMSAMASAMMSSMXMXAMAMMXS
MXMMMSXXAAAXMAMMMMMMSMSMSMMXSAAAMMSMSSMMSSXAMSAMSSSXMMASAMXSMMMMMMMMXXAXMAMSAMXMAAMMMAAXXMMSXSASXSXAXAMAXMMXMMXAAAAXXAMMXMASAMSAMXAMXMSMXSAM
SAMXAXMMSSSSMSSSMASXAMAAXAMAMXMSMXXXMAXAMXMMXMAMMXMXMSAMMSMMMXAXXMASMXMSSSMMAMXMMMMMMSMMXSMMAMAMASMXSASASMSSXSXSMMXMASXAXMXSXMMMSSXSAAAAAMAS
SASMMMAAXAAAAAAASASXMSAXMSMMSMMMXMSXSAMMSAAXMMSMMASAXMAMAAAAMSASMSASAMXAAAMXAMXXAXXAAMASASAMAMXMAMAASAMMSAAXAXAXAXSSSMMMSMAMAMAXAAASMSMXXMAM
SAMASMSMMMSMMMSMMXSAMAMAAXXAMAAXAMAAMAMMSASAXAMASASASMSMSXSMXAMXMMXSXAMMSMMMSMSMSSSMSMMMMMASXMAMXMXMMXMAXMMMMMXMXAMMAAXXAMAXAMXMMAMXMXMASMMS
MAMSMMAAAXMAMXAMXAMAMASXSSMMSSMSSMMSMSAAMXMMMXMAMASAXAXAXAMMSSMAXSAMXMAMXXSXMASAMAAXMASASXXMMSAMAMAXASMSMXXASMMAXMSSSMMSXSMSSSXXSASAMXMAMAMX
SSMMASMMMMASXMAMMAMAMXMMAAAAAAAAMMXXAMMXSAMAXMASMMMXSMMAMXMAXASXSMASXMSMMMAAMAMXMSMMSASASMAMXSASMSASMSAAMXSXXMASXMAXAMXAXMXAAAAMMAXMXAMAMSMM
XMAMAMXAAXMAASMMMSSMSSSMSMMMMSMMXXXMAMXASASMSMAXAXAASXSXSMMMMXMMXMAMMAAAASMMMXMXMAMXXXMAMMAMAMXMXAMSAMMMSASAASMMAMMSSMMMSSMMSMXMMAMMSMSMXXAS
ASXMSSSMMXMSMMMAAAAMAAXAAMASXXAXXMXSSMMMSAMXXMMSMMMXSASAMAAXMMXXMASAMSMSMSAMSASXSASXXXAASMMSXSXXXSSMMMXXMAXAXMAMMMXAAAAAAAAAXAXXMAMXAXXMASXM
XSXAXAMXSXMMSAXMMSSMMSMSMSASASXMASAXAXMMMMMMAMXAMXMAMXMAMMMXSAXAASXSMMXMAMXMSAAASAMMMSSXSAMXXMMXAXMAXAXSMSMASMXMSSMSSMMSSSMMSXMXMASXMMSMASAM
XMMMMAMAMAMASMMSMMAXAXAMXMAXMAMXXMMSMMMAMXAAAXMSSSMXMASAMXAXXSXMXAAAXSAMXMSASAMXMAAMMAMMSAMXAAMMMMSMMASXAMXAMMMMAAXMAMAAAAAASAMXMASAAAMMASAM
MMAMSAMAXAMXSMAMAMAMMSXMXSMSXMSMAAAAXASASMMSMSAAAXSAMXMMXMASMMMSMMMMXMXXAXMAMMXXMSSMAASASMMSMSXAAMASASMMSMMMSAAMSSMMSMMMSMMASMMMMXSAMMSMAMXM
XSAMSXSSSXMXXMAMAMXSXSAXAXAMXMAMSMSSMMAMXMXAAXMMMMSMSAAXAMXXAAAAAXXXASMSMSMSMSMSMAAMSMMMXAMXAMMSXSAMSAXAAMAMMMMMMAAAXASXXXMAXMASXMMMSXXMSSSM
MSXMMAMMAMSMSMMSSMXMASAMMMXXASAMMAAXXAXMAMSMSMXXSASMSAMMASAMSMSSSMXMAXXAXXXAAXAAMXMXXXXXXMMMXMAXXMAMXMMSSSSSSMSASXMMXMMAMXMXMMAMMAAAMXAXAAXA
AXMAMAMXAAAXMAMAXSXMAMXXXAMSXSXSMMMSMMASAMAAXAXXMASAMASXMMMMMAXAMMMSSMXSMSXMSMSMSMXMXMMSMSASAMXMMMMMXXAAXXXAAAMXMMSSSXMXMAXASMMSSSMSMSAMMSMM
AAAAXAMMXSXSSSMASXSMASAAMAXMASAMXAAMMAAXMSMSMMSASMMAMAMAXAXAMMMMXSAAAMAMMSAXMAMXAAASMMMAASAMXMXMAMMMSMMSSSMMMMSMMXXMAASXMXSXMASMMXAXMXAXMMMM
SMSSSMMSAMAMAXMSMAXMMMMXSMSXMXSXSMSMSMSSMXAAAASASMSXMASMSXSXSXXXAMMSMMMSASMMMAMSMSASAAMMXMXMASAMSXAAAAMXAXAXMXAAMSMMSMMAXMAXSMMAAXMMMSMMSAMX
XMMAAAAMAMSMSMXAMXMMSMMAAAXAMSMXSAXAAXMAMSXMSMMXMAAXXAXXMXSAMMMMMSAMXSXMMMMASAXXXMASMMXSAMMSASASAXMXSSMXMSMMXSMSMAAXAAXMMSAXXSSSMMSAXAMMXASA
XAMSMMMSSMXAAASMMMAXAAMSSMXSMAMAMAMXMXMAMXMXXXAXMAMAMXSAAAMAMAXAAMASAMAMAMSASMSMXMMMXXMMASAMXSXMMXSAMXXAXAASAMXAXXMSASMSAMXSAAMAAAMXMMXXMASX
SMMMAMAAMAMXXXMXXASXMSMAAMAMSMSSMAMSMSMASXAXSMSMSAXMAXSAMXSSMSSSSSXMMMAMMMMXSAAXAXMAMMSSXMMSAMAMXAMASAMMSSMMASXXMMAMAMAXMMXMAMSXMMSSMSAMSAMX
SXMSAMMSXXMSMMMSXMXAAMMXXMAMAXMAAAXAAXSAMMSMMAAASXSASXXAAAMAAAXXAXAXXXAMXASAMXMSXSMMSAAAXXMMASASMMXXMASAAAASAMMAMSASAMMMSSMSSXMXMXXXAAAMMASM
MAASXSSXXMMAXAAAAXXMSXAMSMMSAXSMMMXMXMMMXAAASXMXMMMMMXSAMXSMMMSMMMSMSSSSSXMMSAMXXXMAMMSMMSMSMSASXSXMMAMMSMMMAMXAXSAMXMSAMXAXMASXMMMSAMXMSSMM
SSMXMMAMXSSMSMSSMMXXAMAMMAXAMXSAAXASMSASXSSXMMSXXAAAAXXXSASMAASAAAAMXAAXMAMASMMMSSMSSMAMXXAAAMXMASMXMASXAXMMAMSMXMSMMXMSSMMMXXMAASASXXMMSAMX
AXMASMAXXAAMAAAAAXXMMMXMSSMSMASMMSAMAMXXAAMXMASXSSSMXSXMMASAMAXXMASMMMMMSAMXSMSMAMAAAAASAMXMMMXMMMAASXSMSSSMMMASAAAMMSMAMAMSXSMSMMASMMAXXAMX
MAMXXMASMMSMSSSXMMMMSSSXAMAXMAMXMAMMAMMSMMMASMXAMXMXAMXMMMXMASXXXMAXAAAXSMSXSASMSMSMSSMSASXSMMMSMMSMSMSAMAMXXSASMSMSMAAASAMXAXAAAMAMASMMSAMX
XSSXSAMXAXMAXXMASXSAAAMMMSSMMMSMSAMXXMAAAASXSXMMMAAMMSAXAMAAAXMASXAXSMSMSXSXMAMXXAXAMMXXAMASAMASAAXAXAMAMAMXXMXMAMAAXSSMSASMMMSMSMMSXMAXSMMX
XAAAMASMSMMXMMMXXAMMXXMAXMAMXMAAMASXMMMSXMMAMXAAMAXSXSXXXMXMMMMMMMMMMMMMMASXMAMMMMMMMSAMXMAMAMXSMMSMMXSMMAMSMMAMMMSMXMAMXXMAMAMAXMASXMXMSAMX
MMAMMAMAAXMAMMMSSSSXSAMXXAMMAMMSMAMAXXXXXSMSMAXSXSASMMASMMSMSASASMXAMAAAMAMXSAXAAXSSMXSMAMMSXMXXAAAXMXSXSAXAAMSMMAAXAAAMMMSXMASXMASXMSXXXAMX
AMAXMAXXMASAXAMXAAAASAMSSSXSSSMMMSSXMXXXAMAMXMSAAXASMSMMAAAASASASAMSSSMSMXMXSXXSMMMAMAMMAXMAMXASMXXSXAMAXXXSMMAAMSSSXSMSXAXAMASASXXAXXMMSSMM
SASMSMSMXAMMMSXMAMMMMAMAAAAAAAAMAMAMXMMMXMXSAMMMSMASAAXXXMMMMMMMMAMMAXAXAASXMXXAASXSMAXMASMMMMMMMSMMMXSMSSMMXSSXMXAXMXAXMMSXSAMAMASMMMMAAXSA
ASMAMXAXSXXSAXXSAMAXSXMMSMMMSSMMXSAMAAAASMMMMSMAXMAMXMSASMXXXAAAXXMAMMMMMMSAASMMMMAXSXSMMMMAAAXAAAAASAMXAXXAXMASMMMMMMXMAMAXAMMAMXMMAAMSSMAS
SXMAMSSMSAAMASMMASXMMAMXMXSMXMMAMSASMSMXSAAMXAMXSMMSMMAXMASMSSSSSSXAXAXAAMMMMMXSAMXMXMSAAASMSMSMSXSAMAXMAXSMMXAAXAMAASXMSAXAMSSSSXASXMXAXAMX
AXMSXXAAMMMMXSASAMXXSAMASXSAAXMAXXMMXAMMMXMMXMSMMAMMMXSAMXSXMXAMAMSSSXSSSXMSAXMAMXMMMASXMMSXMASAMXXMMMSXSAXMASMSSXXMSSMAMMMMXAAAMXXMAXMXSXXM
MASXMSMMMXXXAMMMXMMMMASAMXMMSMSMSXSSSSMAMMSSSXSMSAMAAXMMSMMAMMXMAMAXMAXXXAASMSXMASAAAXMSSMSAMXSASMSMSAAAAMMMAXSAXXMXMMMAAAAMMMSMAMXMAMSASAMX
XMXAXAMASXMMMSSMAMXAXXMMSSMAMASASXAAAXMAMXAAMASASMSMSXSASASAMAMSSSXMSMSAMMMMASMXXASXMSAMXAMMMAXAMAAAMMSMMXXMSXMXMASAAXSSSMXXAAAXAMMXSXMAXXAA
MXSMSMSASAAXAAAMAXMSMMAAAAMAXAMAMMMMMMSASMMMMAMXMASXMAMAMAMMMMMMASXAXAMAMXMMXMAMXMMSMMXMMMMSMSSSMSMMMXMAXXAMXAMXMAXMSMAAMMMSSSSSSXXAMAMAMSSM
AMAMAMMMXMMMMSSSMSAMASMMSXMXSSMMMMXSXMXAMXASMMSMMMMASXMSMSMXSMAMAMMXMMMMMAMSAMAMXMASXAXXXXAXXAAAMXMSXXSXMSXAXAMMMMMXAXSMXAMXXAXAAXMASAMAAAAM
SSMSASMMXMAMXAXAMMMSMMAXMMSAMXAXMAASXMMSMMMSAAMAAMSXMSAAAAASAMXMXXMAMAAAMXXSAXMSXMMSMMSMMMMSMMSMMAMXMAMMXMMXSAMAAMASAMXSSMSXMXMXMMSASXSMMSSM
XAASASAAASXSMMXXMAMXASAMMAMAMSXMMMXMAMAMASASMMSSMMAAXASMMAMMMXAMAMXMMSSSMXASAMSXXSAMXMAXAAMSMXAASXSASXSMAXMXSASMMXAMASAAMXMXASXMXAMXXMXMAAAX
SMMMAMMMMSMAAAASMSSSXMAMMAMAMXMMSMSSSMASXMAXXXAMXAXXMMXMXMSMXXMMASAAXMAXAMXMXMAAAMASXSASMXXXSSSXMAXASAAXMXMAXXMASMSMAMXMSASMSAXAXSMXXSAMMXSM
SAXMXMMXMXMMMMXSAAAXMASXSSSMSAMMSAAAXXAMAMXMSMMSSXMMSMASAAAMAMXSASMSMMAMXMAMMSMSMSAMAMMAMMASAMXMASMAMMMMSAMSSXMAMAXMXMSXSAXSMXMSMAMAXMAMSAXX
MMSMMMSAMMSMXMSMMMMMMAXXAMAMSMSAMMMMMMSAMXAAAASAMASAAMASMXMXMAMMASAAAMMMXXMXAXAAAMMMXMMAAXXMASMXAXMXSAMASXSAAXSSMSMMMMMMMMMMMMAAMAMXSSMMMASA
MAAXAMSASAAMAMXMAXXAMXSAXSAMXMMMSSXXXAXAXSMSSSMASAMSSMXMXAXXSXXMAMXXSAAAXSSSSMSMSMAXAXSXMXXXMAMMSSXAXAMASXMAMMXMAAXAAAAMASAAAMSMSMSSMASXMMMM
MSSSXXXXMXMMMXASMSSSSXMMMXXSXXSXAMMMMMMMMXAAMAMAMXMAXXAASXSAAASMMSXMMMMSSMAAXMXMMXMSMMMAXSSMSSSMAXMMSMMMXMASXASMSMSSSSXMAXXSMXAAAMXAMAMXAAMX
MMAAXSSSMSXMXSASAAAMAAMXSXMMMMMMAXAAAASMMMMMXSMXMXMSSSMXMAXMSMMAMAMASAMXAMMMSSXMAXXXMAMXMAAAAAMMMMSAMAAXXAMXMSMAAXXMAXAMMSAMXSMSMMMXMASXSMMS
XMMMMAAAASMMAAMMMMMMSSMAMAAAAAXSAMSMSMMAASMSXAXXXAMMAMXAMMMXMXSXMAMXSAXXAMXMMMMMSSMXSMXAXMMMMMMSXAMASXSSSMXXMAMXMSSXMMXMAAAMXMXXAASXMSXMAAXA
MXSAMXSMMMAMMSXSXXXXAXMAMASXMXMMMMMAMASMMMAAXMMASASMAMSMSAAAMXMXSSSMSXXMSMXMAMXAMAMXMASMXXAMXXAMMASXMAXAASAMXMSAMAMMSMMMMSMMAMMMSMSAXMASXMMA
AMXXMAAAXSMMXAMAXMAMMMSSXMXMASXSXAMAMAMAAMXMXSAMAMXMAMXXMXXASAMMXAAASXMSAAMSSMMXSAMSMAAMXSSSSMSSMMSAMMMXMMMSAXSAMMSAAASXMAASAMMAMXSAMSXMAMMX
SSSSMMMSMMSMMMAMSAXXSMAXSAMXAAASMMXAMSSSMMMSAMXXASXMXSXMSSSXSXSSMSMMMAXSMSMXMASAMAMXXSMSAAAAXMAAXASAMSMSXXAMMMSAMAXXSXMAMXXSASMMXAMXMXMSXMXS
AAAMASMAXMAXAXSXSXMASMAMMMMSAMXSASMXXAAAXXXMASASASAMASAMAAXAMMMAAAAMXMMXMXMASAMASASXAMAMMSMMMMMSMMSAMAASAMMMSASXMXSMMASMMMMMAMAMSAMMMAXXAMMS
MMMSAMSAMSMSSXXXMAMMXMASXAAAXXXSAMASMMMMMMXSXMMSASMMASAMMMMXMASMMMMMSXMXMAXMMSAAMASMMMXMXMXMASAAAMMMMSSMAMAMMASXSXSASAAMSASMMMXMAMSASXMXAMAM
XXXMXMMMXAMAXMXSMXMMAMAMMMSMMMAMSMAMAXMSMSASAAAMAMAMASMMSAMXMXMAXXMASAMXSMSMMMMMMXMXXMAMXMAMAXSMMMAAAXAMXMXSMAMASAMXMMMASASXMSSSMASASAMXAMAS
XSAMXSAMAAMAMMASAXASASAMSAMAAXXXAMXSSMSAAMASXMMSASXMAXMAMASMMSSMMMMAXAMMMMAAAXAXXMASMXAMAMAMSMMMSXSSMSAMSMXMMAMXMAMXXXXXMAMXXAAAXMMMMAMSXSAS
MMAAAXAMSMMSSMAXMSXSXXAAMAMSAMXMMXXAMXMMSMXMMXXMXXAMMMMXSAMAAMAMSSMMXAMAAXSSMMSMMMXXAXSSMSASXAMAMXAMXSAMMAAMMXXXAMSMMMSMSMMMMMSMMXSAMAMAXMAS
ASXMSMXMMMAMAMXMXMAMAMSMMMMMSMXSXMXMASMXMASXMASMSSMMSAMXMASMMMAMMMASMMSSSMAXAAMMSASMSMAAASXSMMMASAXXAXAMXSMSMMXMAMAAAAAAAMAAAMXXXAXXXSMSMSAS
MMMAXMMSMMASXMMAAMXMAXAAXSAAMXAAAXMMMMMXXAMASAXXAAXASMXAMAMASMMSXMXMAAAAXMAMMMMAXAMAAMMMMMMSAMMMSXMMSSSMXXMXASMMMSSSMSMSMSSSSSSMMAMSMXAXAMAS
MAMAMMASASXSAAMSSMXSASMSXXMSSMSSXMAAAMXMMMMXMSSMSMMXXMMMMAXXMAAMAMXSMMMSMMXSXAMSMSMSMMXXAXAMAMSXMAMXXAMMSAMSXMASAAXMAMAAAXAAAAAXASAAAMAMMMMM
SAMXSMASXMAMMSMAAMXMXXMAMMAAXAAMAMMSMMAXAMMXAXXAAXXMXSAMSSMASXMMAMASAMXMAMAMMXMXAXAXASMMSSMMAMXASAMMMMSASAMAXXAMMSMMSMMMSMMMMSMSAXSMMMSMMAAX
SASAAMMMAMXMAMMSSMAXMXMAXMAMMMMSSMAXMSSSXSASXXMXMSSMAMMMAASAMXASXSXSXMASAMASMMSMSMMMMMXAMAMXSMMMSASXAMMASMMMSSXXSXMAAAMAMAXXAAAMMXAXSXMASXSS
SAMXMAXSXMXMAXAXAMSSMAMSXSASXMAXMMMSAAMAAMAXAASXXAAMXMXMSMMXSSXMASAMMSAMAXAXAAAAAXMAMXMXMASAMAAXXAMMXSMAMAAXMAXMSAMSSSMASMMSAMXMSSMMXAMMMAXM
MXMMXSXMASXXMMXSAMASMAMXASAXAMSXSAMXMMSMMMMMSAMAMMSAMXSAMAAXXXMXAXASXSMSSMMSMSMSMSXSXMAXSAMASXMSMSXSMSMSMSMSAMXXSAMXMAMASAMXXMASXAXSMSMAMMMS
SMSXAMMXAMXMSSMSAMAXSASMMMSMSMAASMMXAXXAXAXAMAMMXMAMMASAXMASMMSMMMXMAMXAXSXMAAAMXMXXAMSAMXSMMAAXAMAMXXAMXXXSAXXAMAMXSMMMXAMXXXXSMMMSAAXSAMXA
MAMMMMAMSSXXAAXSXMSXMXSAXAXMASMMMSSMMMMMMMMXXAMXMMMXSASAMXSXAAAASASMSMXMASMMSMSMAXAXAMXMSASMSMMMMMMSAMMMMXASXMMMSSMMXMAXMMMMSSMSAAAMXMMMAMAM
MAMAMMSMMAMMXSMXMMMAMMSXMAMSMSXSAMXAAAASAMMXSASXXAAXMAMXMMAMMSMSXASAASAMAMAMAAAMMMMXMMMMMAMXMAMASAAXMXMAMMMMMSXMAAAXXMAMXASXSAASMMXMMMASAMXA
SAMMSAMASAMSAXMSXSSSMASMSMASMMMMAMSXMSMSASAAXAMXSMSSMMMASXAXAXXMMMMXMXXXXMXMMSMMSMSAMAASMSMXSAMXSMSXXAXASAMMASAMSXMMSMAXSAMSSMMMASXSAMASAMMS
SMSAMXSASAMXASASAAAAMAMAAMAMAAXXMMSAMXMSAMXSMAMXXMAAXAMAXSMMMMMSAAAXXSSMSMSXMAAXAASASMMSAXAAXXMMXMMMSXSASASMAMAMXMSASMAMMAMXMXMASMAAMMASAMAA
XAAMSMMXSMSMAMAMMMMMXXMMMMASMMSXMASAMMMMXSAMXSMSAMXSSMMSMMXXSAAXSMSXMXAAAASMSSSMMXMMXMAMXMMMSSSMSAMXAASXSXMMMXMMMAMASMMXSMMXSXXMASXMXSASMMXS
MMMAMXMMSASXXMSMSXXXAMSMMSMSMASXMAMAMAAAXMASXAMAMXAAAXAXAXMASMSXMXAXSSMMMXMAXXAMXMSSXMXMASXMAXAAMAMMMXMAMXSXMASAMAMAMASMMAAAXXMMAMXXXMMMASMA
XMMAMAAAMAMMMMXAXSSMMMAAXAMXMAMMMMSSXSMSSMSSMMMAMMMMMMAXSXMAMXMASMMAXAMXSSXSMSSMSAAASXMSASAMSXMMSAMAMSMSMASASAMAMXMXSAMASMMXSASMSSSMSSMSAMXS
MAMAMXMMMAMAAAMMMXMASXMSSXMAMMMXAAAAXXXMXMAMAXSXSASASMXXMXMASASMMAXMSAMASMAMAAAAAXSMMXXMASXMMAXMMASXSXXAMASAMXSXMSXXMAMXMAAXSAMAXAAAAAMMSMXM
ASXSSXSASMSSMXSAMXMSMMAAAASMSMSSSSSMXMAMSMMMSMMASAMAXMAMMAXMSASMSSMXSAMXSMAMMMMSMAXXSXSMMSAMSXMSSMMMMXMMMAMXMASAAXAMSSXMMMMXMMMXMSMMMSMAXMMX
SXAAAXSAMAAAAMSXMMSXAMMMXMXAAAAAXAMXAMAMXASAXXMAMAMSMSAASAMXMMMAAAMASAMXSMMSXXXAMASAMXXAAMAMMSMAAASXSAMXMMSMMASMMMXMAMAMSAMXXMMSXMASXMMSMMSM
MMMMMMMMMMMMMMMAMSMMSMASASMSMMMMMMMSMSAXSAMXSMMMMMMXASXXMMSAAASMSSMASAMXMAMXMXMAAXMMMSSMMSXMAAMSSMXASXXAXAXAMAMAXASMSXSMSMSMSSXMASMMMMAMAASA
AAXAXXAXXXSASASAMAMMMMMXAAAXMASAAAMXAAMAAAMAXMASAAAMXMSXAAMMXMXXMAMASAMMSAMMMASXMASXAXAMXAAMXSMMAASMMMSAXSSMMSSMMMSXMAXAMMAAAMASMMXASMAMMMMS
SSSMSSMSMMSASMXAXXSAXXSMSMSMSASMSMSMMMMXMAXXXSASXSMXMAMMMMSSXMMMSXMMMAMASMSXSASAXASMSSXMMSXMAMASMMSAAXMSMAAAAMAMXMMAMAMAMSMMASXXAXSAMMSSMXAX
XAAXAAMAAMMMMXMXSAMXSXMAAAAMMMSAXASAMXXMXXAMXSXMAXAAMAMAAXAMMSAAXXMXSAMXSAAAMXSAMXSAAAAXXMAMASAMMASMMMAMMSMMSSSMSASXMSSXMXXMAMMSSMMXXAXAAMMX
MSMMSSMMMSAAAAXMAXMAMMMSMMMSAAMMMAMAXXSMSMSMAMASXMSXSASMSMMSASMSMSAASMMMMMMSMXSASXMMMSXMASAMAMMMMAXASXXSAAAAAAMASAAXAXAAMXSMSSMAMAXMASMMMSSS
AAXAAAAXXSMMSSSXAMSAMXAAAAXSASAMMAMSAMXAAAAMASAMAAXMAXSAAASMMSAXAMMXMAMSAMXAXMSAMMAAAAXAXSXMMSXXMASAMMAMXSSMMSMAMSMAMMSMMSAMXAMASMMASXAAAAAA
SSSSSSMMXMAMMMXMAMSMMMSSSMXXMMMXSXSMSXMAMSMSXMXSMMMXMASXXSMSMMMMXMAMXAXSXMSAMXMXMSSMSXMSAMXASMXMAMAMXMMMMMAMXXMXXAXXXAXSXXXMSAMAXXASMSSMMMSM";
