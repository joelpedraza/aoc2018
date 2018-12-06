pub fn part1() {
    println!("Problem 2, Part 1: {}", calculate_checksum(INPUT))
}

pub fn part2() {
    println!("Problem 2, Part 2: {}", find_boxes_with_single_transpose_trie(INPUT))
}

type LetterCount = [u8; 32];

fn ascii_alpha_to_usize(c: char) -> Option<usize> {
    match c {
        'a' ... 'z' => Some((c as u8 - 'a' as u8) as usize),
        'A' ... 'Z' => Some((c as u8 - 'A' as u8) as usize),
        _ => None
    }
}

pub fn calculate_checksum(input: &str) -> usize {
    let mut twos = 0;
    let mut threes = 0;

    for line in input.lines() {
        let mut letter_count: LetterCount = [0; 32];
        for c in line.chars() {
            let i = ascii_alpha_to_usize(c).expect("encountered non ascii-alpha char");
            letter_count[i] += 1;
        }

        let mut twos_ = 0;
        let mut threes_ = 0;

        for count in letter_count.iter() {
            if *count == 2 { twos_ = 1 };
            if *count == 3 { threes_ = 1 };
        }

        twos += twos_;
        threes += threes_;
    }

    twos * threes
}

pub fn find_boxes_with_single_transpose_bruteforce(input: &str) -> String {
    let words: Vec<&str> = input.lines().collect();
    let len = words.len();
    
    for i in 0..len {
        for j in i+1..len {
            let a = words[i];
            let b = words[j];

            if has_single_transpose(a, b) {
                let mut res: String = String::new();
                
                for (x, y) in a.chars().zip(b.chars()) {
                    if x == y { res.push(x) }
                }

                return res
            }
        }
    }

    unreachable!("No two words had only single transposition")
}

pub fn find_boxes_with_single_transpose_trie(input: &str) -> String {
    use ::trie::Trie;

    let mut trie = Trie::new();
    input.lines()
        .filter_map(|word| {
            trie.insert(word)
                .map(|i| {
                    let mut s = word[..i].to_owned();
                    s.push_str(&word[i+1..]);
                    s
                })
        })
        .next()
        .unwrap()
}

fn has_single_transpose(word1: &str, word2: &str) -> bool {
    let mut count = 0;
    for (a, b) in word1.chars().zip(word2.chars()) {
        if a != b { count += 1 };
        if count > 1 { return false }
    }
    count == 1
}

// Test it later! These are puzzle solutions, so even more than 'real' projects: Later means never!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_is_correct() {}

    #[test]
    fn trie_is_correct_with_sample_data() {
        let actual = find_boxes_with_single_transpose_bruteforce(SAMPLE_INPUT);
        let expected = find_boxes_with_single_transpose_trie(SAMPLE_INPUT);
        assert_eq!(expected, actual)
    }

    #[test]
    fn trie_is_correct_with_input_data() {
        let actual = find_boxes_with_single_transpose_bruteforce(INPUT);
        let expected = find_boxes_with_single_transpose_trie(INPUT);
        assert_eq!(expected, actual)
    }

    #[test]
    fn part_2_is_correct() {}

    const SAMPLE_INPUT: &str = "\
abcde\n\
fghij\n\
klmno\n\
pqrst\n\
fguij\n\
axcye\n\
wvxyz\
";
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use self::test::Bencher;

    #[bench]
    fn p2_bf(b: &mut Bencher) {
        b.iter(|| find_boxes_with_single_transpose_bruteforce(INPUT));
    }

    #[bench]
    fn p2_trie(b: &mut Bencher) {
        b.iter(|| find_boxes_with_single_transpose_trie(INPUT));
    }
}

// ACTUAL INPUT
// ============

const INPUT: &str = "\
wxlnjevbfozadyiqpuzkrhstkg\n\
wxlsjivbfodamyiqpuzcxhstkg\n\
wxlnjevbfouammnqpuzcrhstkg\n\
wxlnjevbfobwmyiqpuzprhstkg\n\
wxlnjefbfodamyiqpuzcnustkg\n\
wxlnjevbfodakyyupuzcrhstkg\n\
wxlnjejbfodafynqpuzcrhstkg\n\
wxlnjevbfodomyifptzcrhstkg\n\
wxlnjevbfodamyiwcuzcrhstkz\n\
wxlnjevbfofamyiqpuznrhstxg\n\
wxlnjevbfodamyiqbupcrxstkg\n\
wxjnjevbfodxmyeqpuzcrhstkg\n\
xwlnjevbfosamyiqpuzcrhstkg\n\
fxlnjevbfodrmyiqpuzcrbstkg\n\
wxlnjevpfodamyiqquzzrhstkg\n\
wwlnjenbfodawyiqpuzcrhstkg\n\
wxrnjevbfodamyiqpuzlrhstrg\n\
welnjeqbfodkmyiqpuzcrhstkg\n\
walnjfvbfodamyiqpuzcrhwtkg\n\
wdlnjevbfodamriqpuzjrhstkg\n\
wxlnjevbfodmnyizpuzcrhstkg\n\
wxlnjevbfodgmyiqpuxcrjstkg\n\
wxlnjevbfkdamyiqpudcrestkg\n\
wxlyjevbfodamyiqpuzcehstkj\n\
wxlnjevamodamyiqpuzcrhatkg\n\
fxlnqevsfodamyiqpuzcrhstkg\n\
wqlnjevbfodanyiqvuzcrhstkg\n\
wxlnjevbfoiamyzqpuzcrnstkg\n\
wxlnjevbfodamyiqpuacrhsjig\n\
wxlnjuvbfodzmyvqpuzcrhstkg\n\
kxlnjevbfolamviqpuzcrhstkg\n\
wxlnjesbfldamyiqpuycrhstkg\n\
nxltjevbfodgmyiqpuzcrhstkg\n\
ojlnjevbfooamyiqpuzcrhstkg\n\
wxlnjevbfodaffiqpugcrhstkg\n\
cxlnievbfodamyuqpuzcrhstkg\n\
wxlouevbjodamyiqpuzcrhstkg\n\
wafnjevbfhdamyiqpuzcrhstkg\n\
wxlnjevbfxdamrpqpuzcrhstkg\n\
wxlnjepffodamyiqphzcrhstkg\n\
wxlnkevbfohamciqpuzcrhstkg\n\
wzlnmevdfodamyiqpuzcrhstkg\n\
wxlzjtvbfodamyiqpuzcrhstkd\n\
gxlnjehbfojamyiqpuzcrhstkg\n\
wxlnjeoqfodamyiqprzcrhstkg\n\
nxllvevbfodamyiqpuzcrhstkg\n\
wxlnjevtfomamyiqpurcrhstkg\n\
sxlnjevafodamyikpuzcrhstkg\n\
wxlnjevbfodamyfqpuzcyhztkg\n\
wxlnjevbfodamyiqpulnrhstkh\n\
wxlnwevbfodumyiqpuzqrhstkg\n\
wxldjevbfodamyiqpzzcrhstkk\n\
jxlnjevbfodamyiqphzcrnstkg\n\
fxlnjeibfodcmyiqpuzcrhstkg\n\
wxlnjevufodamyiqpnzcrhstkk\n\
wglnjevbfodamyiqpuzcesstkg\n\
wxlvjevbdodamyiqpuzcrhstkc\n\
wxlnjevbfodabyicpuzcrhstkl\n\
wxlnjevbfodamyiqpizcrhstvt\n\
wolnjevbfodawyiqiuzcrhstkg\n\
wxlyjevbfodamyuqpxzcrhstkg\n\
wxlijevbfodamyikpuzyrhstkg\n\
wxennevbfodamyiqpuzcrtstkg\n\
wxlnjevbyodamyuqpwzcrhstkg\n\
wxlnjevbfoiomyiqpuzcrhsteg\n\
wxlnjehbrodamyiqpuicrhstkg\n\
xxlnjevufodamyiqbuzcrhstkg\n\
wxlojevbfodamyiqpezcrhatkg\n\
wxljjevbfolamuiqpuzcrhstkg\n\
wxlnjevbfodamyiqruzcrhstpi\n\
wxlnjevbfomamyiqjnzcrhstkg\n\
wxlnjevbfodahyiqzuzcrhstpg\n\
wxtnjevbfodamyiqpuzcrhsdrg\n\
wxlnjevbfodamynrpuzcrhstkz\n\
wxlqjevefqdamyiqpuzcrhstkg\n\
wxlnjevbfmdamyiqnuzckhstkg\n\
wxlnjevbfodajyiqprzcrjstkg\n\
wxlnjqvbhodamyidpuzcrhstkg\n\
wxlnjhvbfodamriqpuzcchstkg\n\
wglnjevbfodamyiqpupfrhstkg\n\
wulnjevdfodamyiqpuzcrhsteg\n\
vxlojevbfodamyiqpuzcrhstsg\n\
wxlnjvvbfodamiiqpuzcrhttkg\n\
wxlnjevbfodabyiqpuzzrhetkg\n\
wxhnjevbfodamyiqpuwcrsstkg\n\
wslzjbvbfodamyiqpuzcrhstkg\n\
rxlnjevbfodhmyiqpupcrhstkg\n\
wxlnjevbfhdamyiqpuvcrhskkg\n\
wxlrjevbxodamyiqpuzcrhstag\n\
wxlsbevbfodammiqpuzcrhstkg\n\
wxlnjzvbfodemyiqpmzcrhstkg\n\
wxlnoevbfodgmyiqpuzbrhstkg\n\
wxlnjefbfodamyinpuzcrhwtkg\n\
bxlnjevbfwdamyiqpuocrhstkg\n\
cxlnjevbjodamyiqpuzcrhslkg\n\
wflnjevbforemyiqpuzcrhstkg\n\
wxlmjeoboodamyiqpuzcrhstkg\n\
wxlnjevbfadaiyiqpuzcrhutkg\n\
wxlnmevbfodamyyqpuzcrjstkg\n\
wxlnjovbfodamyippjzcrhstkg\n\
wxlnjmvbfodamyiqpszcrhsbkg\n\
wxlnjeebfodamyicpuxcrhstkg\n\
wxlnrehbfodamyiqpuzcrhytkg\n\
wxlnjevbfogamyiqwurcrhstkg\n\
wxlujevbnodamyiqpuzcrhstng\n\
wxlnoenofodamyiqpuzcrhstkg\n\
wxsnjevbfsdamyiqsuzcrhstkg\n\
wxlnjevwfodamyiqpuzxrhqtkg\n\
wxlnjevbnodamyiqxulcrhstkg\n\
wxlijetpfodamyiqpuzcrhstkg\n\
wxlnjzvbfidamyiqpuzcrbstkg\n\
wxlnjevefodavyiqpuzcthstkg\n\
wxlnjevbfozamyiqpurcrbstkg\n\
wxlnjfvpfodamyiqpuzcrhntkg\n\
wxlnjevbfvdamyiqvuzcrhqtkg\n\
wilejevbfodamyilpuzcrhstkg\n\
wxlnhevbfodamtiqpuzcrhstke\n\
wxlwjevbfodahyiqppzcrhstkg\n\
wxlnjevbfodamyuqpuzwrrstkg\n\
xxsnjevbfodamyiqpuzcrhstkl\n\
wglnjevbdodamyaqpuzcrhstkg\n\
wxlnjefbwodamyiqpuzcrhsykg\n\
wxwnjevbfodamyiqpuzcrhpckg\n\
wxlnjuvbfidamyiqpuzczhstkg\n\
wxlnzhybfodamyiqpuzcrhstkg\n\
wxunjevufodamyiqpuzcrhspkg\n\
wxunjevbfodcmyiqpuzcrhstcg\n\
wxlnjevbfodhwyiqpuxcrhstkg\n\
wxlnjevtfodasyiqpuzcrhstkb\n\
wxlvjevbfqdamyiqprzcrhstkg\n\
sxlnjevbfodamyiqplzcrhstkq\n\
wxlnlevbfodamyiqpuzcrpstka\n\
wxlnjevbfodaiyiqovzcrhstkg\n\
wxlntevbfodamyiqpuzcrkntkg\n\
wxlnjevbfodsmyiqpuzcrhstir\n\
wxlnnevbfodaoyiqpuzmrhstkg\n\
xxlnjevbfodamyinpnzcrhstkg\n\
wxlnjedefodamyigpuzcrhstkg\n\
wxlnxeabfodamyiqpnzcrhstkg\n\
wxlnxevbfodpmtiqpuzcrhstkg\n\
wxlnjevnfodamyiqpuzcuhqtkg\n\
wxlnjevbfodakyiqluzcrhstmg\n\
wxlnjevbaodamoiqpyzcrhstkg\n\
wwlnjevbfoaajyiqpuzcrhstkg\n\
wxlnjevbfedamyiqpuzcrhsang\n\
wxlwjevbfodamyiqpuzcrmdtkg\n\
wxlnjevbhodamyiqpmzxrhstkg\n\
wxlnjevbfodamyiqpuzzrhwtkj\n\
wxlnjevbfpdvmyiqpuzzrhstkg\n\
wxlnjegcfodamyiqpxzcrhstkg\n\
fxlnjevbfodamyiqpuzcrhstat\n\
wxlnjevbfodcmybqpuzcrkstkg\n\
wxlnjevbfodamiiqpuzrrhstxg\n\
wxvnjevifodamdiqpuzcrhstkg\n\
wxltjevbfodamyiqpuzmrhktkg\n\
wxlnjevbfobaaygqpuzcrhstkg\n\
wmlnjevbfodamyiqpuycrhsukg\n\
wxlnjevboodamyiqpuzcrhuhkg\n\
wxlnjevgfodaqyiqpuzcghstkg\n\
wxlnjevjnodamyiqpuzcrhstke\n\
wclnjevbfodamyiqpuncchstkg\n\
wxlnjevbfndamyxqpuzcshstkg\n\
rxldjevbfodamyiqpuvcrhstkg\n\
wxlnwevbfodamywqpuzrrhstkg\n\
ixlnjevbqodpmyiqpuzcrhstkg\n\
wxlnjlvbfocamyiqpuzgrhstkg\n\
wxlnjevffodamyiqnuzcrhutkg\n\
wxlajevbfodamyiqpuccrhshkg\n\
vwlnjevbfodamyiqpuzcrhstky\n\
wxlajevbfodamyiqpuzcfhstkl\n\
wxlnjevbfodamniqouzcrhstko\n\
wxlnjevbfodamyiqpuzqrhsqka\n\
wxlnjeybfodamyiqpuzclhsnkg\n\
wxbnjlvbfoyamyiqpuzcrhstkg\n\
wxbnjevbfodemyiqpuzcrhstkj\n\
wxlnbefbfodamyiqpkzcrhstkg\n\
wxlnjvvbyodamyitpuzcrhstkg\n\
jxlnjevbfopamyiqpuzprhstkg\n\
wxlnjevbfodaeyiupxzcrhstkg\n\
wnlnjevbfodamyqqpuzcrhstcg\n\
wxlxzuvbfodamyiqpuzcrhstkg\n\
wxlnjevbcodymyiqpuzcrhstke\n\
wxlnjezbfodamynqpuvcrhstkg\n\
wxlnjevbfodamyxlpuzcyhstkg\n\
wxlnjevbffdaiyiqpuzirhstkg\n\
wxlnjevbfodymyiqwuzcrhstfg\n\
wxlnzevbfodscyiqpuzcrhstkg\n\
hxlnjevbfodamyawpuzcrhstkg\n\
welnjevbfodamciqplzcrhstkg\n\
wxlnjeqbfodawyiqpuzkrhstkg\n\
wxlnjelbfodamviqpuzckhstkg\n\
wxlneevjfodamyiqpuzcrhstkd\n\
wxlnjevbfodamyaqpuytrhstkg\n\
wxlnjpvyfodamyiqpuzcshstkg\n\
wxlnjevbfodmbyiqpuzcrhstkp\n\
wxlnjegbfodamdiqpuzcrhstdg\n\
wmlnjevbfodamyiqpuecrhsukg\n\
wxlnjevbfodamyiqpuocrhwtjg\n\
jxfnwevbfodamyiqpuzcrhstkg\n\
wxlnjevffodamyiqpurcrhstkd\n\
wxlnjevbfofamyiqpuzcrhsmkt\n\
wxlnjevbfodmmyiqpuzcrdsttg\n\
axlnjevbfodgmyiqpuzerhstkg\n\
wxtnjevbfodamyifpuwcrhstkg\n\
wxlgjevbfodamyiqpuzrrhvtkg\n\
wxlnjevbfouamyeqfuzcrhstkg\n\
wxlnjevbfmxamyiqpuzcahstkg\n\
wxlnjevffoxamyiqpuecrhstkg\n\
wxlnyevbfodamyiqttzcrhstkg\n\
bxlnjevbfodzmysqpuzcrhstkg\n\
wxlnjevbfodayyiqpuzcrherkg\n\
yxlnjevbfodayyiqpuzcrwstkg\n\
wllnjevbfodambiqpuzurhstkg\n\
wxlnjevbfsdamyiqpuqcrhstkh\n\
wxlcjevbfodamyiqcuzcxhstkg\n\
wxlnjevbfodayticpuzcrhstkg\n\
wxltzevbfodamyiqpzzcrhstkg\n\
wxlnjevbfodamgiqpuzcphstng\n\
wxlnjevbfqdfmziqpuzcrhstkg\n\
wxlnaevbfodamyiqpuzcfustkg\n\
wxlnjevbfodamyxqxuzcrhstdg\n\
wxlnjevkbodamyiqpufcrhstkg\n\
whlnjevbfodauyiqputcrhstkg\n\
wxlnjevbfodamyiephzcrhsnkg\n\
wxlnjevbfodfmoiqpuzcrhstkf\n\
wxlnjevbfodamyiqxuzaxhstkg\n\
wxlnjevtfotamyiqpuzcrhsttg\n\
wxlgjevbfodamyiqpuhcrostkg\n\
dxlnjtvbfodamyiqpuzcshstkg\n\
wxlfjevbfodumyiqppzcrhstkg\n\
wxlnzevbfodamyiqpuzqrhstkx\n\
wflnjevbfodamyiqpurcrhsthg\n\
wxlnjevbfodzfyiqpuzcrjstkg\n\
wxlnjevbfrdamviqpuzmrhstkg\n\
wnlngevmfodamyiqpuzcrhstkg\n\
walzjevbfodamyiqpuzcrhsjkg\n\
wqlnjevbfodamyiqpuzcshslkg\n\
wxlnjevkfodfmyiepuzcrhstkg\n\
wxgnjehbfodamyhqpuzcrhstkg\n\
wxlnjevbfodamyiqfuacrostkg\n\
wxlnjexbfodamyiwpuzcrqstkg\n\
wxlntevafodamyiqpuzcrhsnkg\n\
wxvnjevbfodamyiqpuzcvistkg\n\
mxlnjeebfodamyiqpuzcrhsgkg\n\
wxlnjevyfodamyiqpuzcrhdtkf\n\
wxlnjcvbfodamyicpuzcrhsckg\n\
wxlnjekbfodlmyiqpuzcthstkg\n\
wxlnjvvbfodamyiopuzcrhstqg\n\
wxlnjevbsodamyiqpuhcrhstwg\n\
wxxnjevufodamyiqruzcrhstkg\
";