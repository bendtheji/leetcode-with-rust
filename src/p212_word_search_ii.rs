#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

struct TrieNode {
    val: char,
    is_word: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new(val: char, is_word: bool) -> Self {
        Self {
            val,
            is_word,
            children: HashMap::new(),
        }
    }
}

fn build_trie(words: Vec<String>) -> HashMap<char, TrieNode> {
    let mut trie = HashMap::new();

    for word in words {
        let word: Vec<char> = word.chars().collect();
        let mut cur_node = trie.entry(word[0]).or_insert(TrieNode::new(word[0], false));

        for i in 1..word.len() {
            cur_node = cur_node.children.entry(word[i]).or_insert(TrieNode::new(word[i], false));
        }
        cur_node.is_word = true;
    }

    trie
}

pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let words = build_trie(words);
    let mut result = HashSet::new();
    let mut visited = vec![vec![false; board[0].len()]; board.len()];
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            let mut cur_string = String::new();
            backtrack(&board, &words, row as i32, col as i32, &mut cur_string, &mut result, &mut visited);
        }
    }
    result.into_iter().collect()
}

fn backtrack(board: &Vec<Vec<char>>, words: &HashMap<char, TrieNode>, row: i32, col: i32,
             cur_string: &mut String, result: &mut HashSet<String>, visited: &mut Vec<Vec<bool>>) {
    if row < 0 || row > (board.len() - 1) as i32 || col < 0 || col > (board[0].len() - 1) as i32 {
        return;
    }

    if visited[row as usize][col as usize] {
        return;
    }

    visited[row as usize][col as usize] = true;

    if let Some(node) = words.get(&board[row as usize][col as usize]) {
        cur_string.push(node.val);
        if node.is_word && !result.contains(cur_string) {
            result.insert(cur_string.clone());
        }
        let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        for direction in directions {
            let row = row + direction.0;
            let col = col + direction.1;
            backtrack(board, &node.children, row, col, cur_string, result, visited);
        }
        cur_string.pop();
    }
    visited[row as usize][col as usize] = false;
}

#[cfg(test)]
mod tests {
    use crate::p212_word_search_ii::find_words;

    fn build_words(words: &[&str]) -> Vec<String> {
        words.into_iter().map(|str| str.to_string()).collect()
    }

    #[test]
    fn test_one() {
        let board = vec![vec!['o', 'a', 'a', 'n'], vec!['e', 't', 'a', 'e'], vec!['i', 'h', 'k', 'r'], vec!['i', 'f', 'l', 'v']];
        let words = build_words(&["oath", "pea", "eat", "rain"]);
        let output = find_words(board, words).sort_unstable();
        let expected = build_words(&["eat", "oath"]).sort_unstable();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_two() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let words = build_words(&["abcb"]);
        let output = find_words(board, words).sort_unstable();
        let expected = build_words(&[]).sort_unstable();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_three() {
        let board = vec![vec!['o', 'a', 'b', 'n'], vec!['o', 't', 'a', 'e'], vec!['a', 'h', 'k', 'r'], vec!['a', 'f', 'l', 'v']];
        let words = build_words(&["oa", "oaa"]);
        let output = find_words(board, words).sort_unstable();
        let expected = build_words(&["oa", "oaa"]).sort_unstable();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_four() {
        let board = vec![vec!['a', 'a']];
        let words = build_words(&["aaa"]);
        let output = find_words(board, words).sort_unstable();
        let expected = build_words(&[]).sort_unstable();
        assert_eq!(output, expected);
    }


    #[test]
    #[ignore]
    /// very long to run
    fn test_five() {
        let board = vec![vec!['m', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'],
                         vec!['n', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                         vec!['o', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                         vec!['p', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                         vec!['q', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                         vec!['r', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                         vec!['s', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                         vec!['t', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                         vec!['u', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                         vec!['v', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                         vec!['w', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                         vec!['x', 'y', 'z', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a']];
        let words = build_words(&["aaaaaaaaaa", "baaaaaaaaa", "caaaaaaaaa", "daaaaaaaaa", "eaaaaaaaaa", "faaaaaaaaa", "gaaaaaaaaa",
            "haaaaaaaaa", "iaaaaaaaaa", "jaaaaaaaaa", "kaaaaaaaaa", "laaaaaaaaa", "maaaaaaaaa", "naaaaaaaaa", "oaaaaaaaaa",
            "paaaaaaaaa", "qaaaaaaaaa", "raaaaaaaaa", "saaaaaaaaa", "taaaaaaaaa", "uaaaaaaaaa", "vaaaaaaaaa", "waaaaaaaaa",
            "xaaaaaaaaa", "yaaaaaaaaa", "zaaaaaaaaa", "abaaaaaaaa", "bbaaaaaaaa", "cbaaaaaaaa", "dbaaaaaaaa", "ebaaaaaaaa",
            "fbaaaaaaaa", "gbaaaaaaaa", "hbaaaaaaaa", "ibaaaaaaaa", "jbaaaaaaaa", "kbaaaaaaaa", "lbaaaaaaaa", "mbaaaaaaaa",
            "nbaaaaaaaa", "obaaaaaaaa", "pbaaaaaaaa", "qbaaaaaaaa", "rbaaaaaaaa", "sbaaaaaaaa", "tbaaaaaaaa", "ubaaaaaaaa",
            "vbaaaaaaaa", "wbaaaaaaaa", "xbaaaaaaaa", "ybaaaaaaaa", "zbaaaaaaaa", "acaaaaaaaa", "bcaaaaaaaa", "ccaaaaaaaa",
            "dcaaaaaaaa", "ecaaaaaaaa", "fcaaaaaaaa", "gcaaaaaaaa", "hcaaaaaaaa", "icaaaaaaaa", "jcaaaaaaaa", "kcaaaaaaaa",
            "lcaaaaaaaa", "mcaaaaaaaa", "ncaaaaaaaa", "ocaaaaaaaa", "pcaaaaaaaa", "qcaaaaaaaa", "rcaaaaaaaa", "scaaaaaaaa",
            "tcaaaaaaaa", "ucaaaaaaaa", "vcaaaaaaaa", "wcaaaaaaaa", "xcaaaaaaaa", "ycaaaaaaaa", "zcaaaaaaaa", "adaaaaaaaa",
            "bdaaaaaaaa", "cdaaaaaaaa", "ddaaaaaaaa", "edaaaaaaaa", "fdaaaaaaaa", "gdaaaaaaaa", "hdaaaaaaaa", "idaaaaaaaa",
            "jdaaaaaaaa", "kdaaaaaaaa", "ldaaaaaaaa", "mdaaaaaaaa", "ndaaaaaaaa", "odaaaaaaaa", "pdaaaaaaaa", "qdaaaaaaaa",
            "rdaaaaaaaa", "sdaaaaaaaa", "tdaaaaaaaa", "udaaaaaaaa", "vdaaaaaaaa", "wdaaaaaaaa", "xdaaaaaaaa", "ydaaaaaaaa",
            "zdaaaaaaaa", "aeaaaaaaaa", "beaaaaaaaa", "ceaaaaaaaa", "deaaaaaaaa", "eeaaaaaaaa", "feaaaaaaaa", "geaaaaaaaa",
            "heaaaaaaaa", "ieaaaaaaaa", "jeaaaaaaaa", "keaaaaaaaa", "leaaaaaaaa", "meaaaaaaaa", "neaaaaaaaa", "oeaaaaaaaa",
            "peaaaaaaaa", "qeaaaaaaaa", "reaaaaaaaa", "seaaaaaaaa", "teaaaaaaaa", "ueaaaaaaaa", "veaaaaaaaa", "weaaaaaaaa",
            "xeaaaaaaaa", "yeaaaaaaaa", "zeaaaaaaaa", "afaaaaaaaa", "bfaaaaaaaa", "cfaaaaaaaa", "dfaaaaaaaa", "efaaaaaaaa",
            "ffaaaaaaaa", "gfaaaaaaaa", "hfaaaaaaaa", "ifaaaaaaaa", "jfaaaaaaaa", "kfaaaaaaaa", "lfaaaaaaaa", "mfaaaaaaaa",
            "nfaaaaaaaa", "ofaaaaaaaa", "pfaaaaaaaa", "qfaaaaaaaa", "rfaaaaaaaa", "sfaaaaaaaa", "tfaaaaaaaa", "ufaaaaaaaa",
            "vfaaaaaaaa", "wfaaaaaaaa", "xfaaaaaaaa", "yfaaaaaaaa", "zfaaaaaaaa", "agaaaaaaaa", "bgaaaaaaaa", "cgaaaaaaaa",
            "dgaaaaaaaa", "egaaaaaaaa", "fgaaaaaaaa", "ggaaaaaaaa", "hgaaaaaaaa", "igaaaaaaaa", "jgaaaaaaaa", "kgaaaaaaaa",
            "lgaaaaaaaa", "mgaaaaaaaa", "ngaaaaaaaa", "ogaaaaaaaa", "pgaaaaaaaa", "qgaaaaaaaa", "rgaaaaaaaa", "sgaaaaaaaa",
            "tgaaaaaaaa", "ugaaaaaaaa", "vgaaaaaaaa", "wgaaaaaaaa", "xgaaaaaaaa", "ygaaaaaaaa", "zgaaaaaaaa", "ahaaaaaaaa",
            "bhaaaaaaaa", "chaaaaaaaa", "dhaaaaaaaa", "ehaaaaaaaa", "fhaaaaaaaa", "ghaaaaaaaa", "hhaaaaaaaa", "ihaaaaaaaa",
            "jhaaaaaaaa", "khaaaaaaaa", "lhaaaaaaaa", "mhaaaaaaaa", "nhaaaaaaaa", "ohaaaaaaaa", "phaaaaaaaa", "qhaaaaaaaa",
            "rhaaaaaaaa", "shaaaaaaaa", "thaaaaaaaa", "uhaaaaaaaa", "vhaaaaaaaa", "whaaaaaaaa", "xhaaaaaaaa", "yhaaaaaaaa",
            "zhaaaaaaaa", "aiaaaaaaaa", "biaaaaaaaa", "ciaaaaaaaa", "diaaaaaaaa", "eiaaaaaaaa", "fiaaaaaaaa", "giaaaaaaaa",
            "hiaaaaaaaa", "iiaaaaaaaa", "jiaaaaaaaa", "kiaaaaaaaa", "liaaaaaaaa", "miaaaaaaaa", "niaaaaaaaa", "oiaaaaaaaa",
            "piaaaaaaaa", "qiaaaaaaaa", "riaaaaaaaa", "siaaaaaaaa", "tiaaaaaaaa", "uiaaaaaaaa", "viaaaaaaaa", "wiaaaaaaaa",
            "xiaaaaaaaa", "yiaaaaaaaa", "ziaaaaaaaa", "ajaaaaaaaa", "bjaaaaaaaa", "cjaaaaaaaa", "djaaaaaaaa", "ejaaaaaaaa",
            "fjaaaaaaaa", "gjaaaaaaaa", "hjaaaaaaaa", "ijaaaaaaaa", "jjaaaaaaaa", "kjaaaaaaaa", "ljaaaaaaaa", "mjaaaaaaaa",
            "njaaaaaaaa", "ojaaaaaaaa", "pjaaaaaaaa", "qjaaaaaaaa", "rjaaaaaaaa", "sjaaaaaaaa", "tjaaaaaaaa", "ujaaaaaaaa",
            "vjaaaaaaaa", "wjaaaaaaaa", "xjaaaaaaaa", "yjaaaaaaaa", "zjaaaaaaaa", "akaaaaaaaa", "bkaaaaaaaa", "ckaaaaaaaa",
            "dkaaaaaaaa", "ekaaaaaaaa", "fkaaaaaaaa", "gkaaaaaaaa", "hkaaaaaaaa", "ikaaaaaaaa", "jkaaaaaaaa", "kkaaaaaaaa",
            "lkaaaaaaaa", "mkaaaaaaaa", "nkaaaaaaaa", "okaaaaaaaa", "pkaaaaaaaa", "qkaaaaaaaa", "rkaaaaaaaa", "skaaaaaaaa",
            "tkaaaaaaaa", "ukaaaaaaaa", "vkaaaaaaaa", "wkaaaaaaaa", "xkaaaaaaaa", "ykaaaaaaaa", "zkaaaaaaaa", "alaaaaaaaa",
            "blaaaaaaaa", "claaaaaaaa", "dlaaaaaaaa", "elaaaaaaaa", "flaaaaaaaa", "glaaaaaaaa", "hlaaaaaaaa", "ilaaaaaaaa",
            "jlaaaaaaaa", "klaaaaaaaa", "llaaaaaaaa", "mlaaaaaaaa", "nlaaaaaaaa", "olaaaaaaaa", "plaaaaaaaa", "qlaaaaaaaa",
            "rlaaaaaaaa", "slaaaaaaaa", "tlaaaaaaaa", "ulaaaaaaaa", "vlaaaaaaaa", "wlaaaaaaaa", "xlaaaaaaaa", "ylaaaaaaaa",
            "zlaaaaaaaa", "amaaaaaaaa", "bmaaaaaaaa", "cmaaaaaaaa", "dmaaaaaaaa", "emaaaaaaaa", "fmaaaaaaaa", "gmaaaaaaaa",
            "hmaaaaaaaa", "imaaaaaaaa", "jmaaaaaaaa", "kmaaaaaaaa", "lmaaaaaaaa", "mmaaaaaaaa", "nmaaaaaaaa", "omaaaaaaaa",
            "pmaaaaaaaa", "qmaaaaaaaa", "rmaaaaaaaa", "smaaaaaaaa", "tmaaaaaaaa", "umaaaaaaaa", "vmaaaaaaaa", "wmaaaaaaaa",
            "xmaaaaaaaa", "ymaaaaaaaa", "zmaaaaaaaa", "anaaaaaaaa", "bnaaaaaaaa", "cnaaaaaaaa", "dnaaaaaaaa", "enaaaaaaaa",
            "fnaaaaaaaa", "gnaaaaaaaa", "hnaaaaaaaa", "inaaaaaaaa", "jnaaaaaaaa", "knaaaaaaaa", "lnaaaaaaaa", "mnaaaaaaaa",
            "nnaaaaaaaa", "onaaaaaaaa", "pnaaaaaaaa", "qnaaaaaaaa", "rnaaaaaaaa", "snaaaaaaaa", "tnaaaaaaaa", "unaaaaaaaa",
            "vnaaaaaaaa", "wnaaaaaaaa", "xnaaaaaaaa", "ynaaaaaaaa", "znaaaaaaaa", "aoaaaaaaaa", "boaaaaaaaa", "coaaaaaaaa",
            "doaaaaaaaa", "eoaaaaaaaa", "foaaaaaaaa", "goaaaaaaaa", "hoaaaaaaaa", "ioaaaaaaaa", "joaaaaaaaa", "koaaaaaaaa",
            "loaaaaaaaa", "moaaaaaaaa", "noaaaaaaaa", "ooaaaaaaaa", "poaaaaaaaa", "qoaaaaaaaa", "roaaaaaaaa", "soaaaaaaaa",
            "toaaaaaaaa", "uoaaaaaaaa", "voaaaaaaaa", "woaaaaaaaa", "xoaaaaaaaa", "yoaaaaaaaa", "zoaaaaaaaa", "apaaaaaaaa",
            "bpaaaaaaaa", "cpaaaaaaaa", "dpaaaaaaaa", "epaaaaaaaa", "fpaaaaaaaa", "gpaaaaaaaa", "hpaaaaaaaa", "ipaaaaaaaa",
            "jpaaaaaaaa", "kpaaaaaaaa", "lpaaaaaaaa", "mpaaaaaaaa", "npaaaaaaaa", "opaaaaaaaa", "ppaaaaaaaa", "qpaaaaaaaa",
            "rpaaaaaaaa", "spaaaaaaaa", "tpaaaaaaaa", "upaaaaaaaa", "vpaaaaaaaa", "wpaaaaaaaa", "xpaaaaaaaa", "ypaaaaaaaa", "zpaaaaaaaa", "aqaaaaaaaa", "bqaaaaaaaa", "cqaaaaaaaa", "dqaaaaaaaa", "eqaaaaaaaa", "fqaaaaaaaa", "gqaaaaaaaa", "hqaaaaaaaa", "iqaaaaaaaa", "jqaaaaaaaa", "kqaaaaaaaa", "lqaaaaaaaa", "mqaaaaaaaa", "nqaaaaaaaa", "oqaaaaaaaa", "pqaaaaaaaa", "qqaaaaaaaa", "rqaaaaaaaa", "sqaaaaaaaa", "tqaaaaaaaa", "uqaaaaaaaa", "vqaaaaaaaa", "wqaaaaaaaa", "xqaaaaaaaa", "yqaaaaaaaa", "zqaaaaaaaa", "araaaaaaaa", "braaaaaaaa", "craaaaaaaa", "draaaaaaaa", "eraaaaaaaa", "fraaaaaaaa", "graaaaaaaa", "hraaaaaaaa", "iraaaaaaaa", "jraaaaaaaa", "kraaaaaaaa", "lraaaaaaaa", "mraaaaaaaa", "nraaaaaaaa", "oraaaaaaaa", "praaaaaaaa", "qraaaaaaaa", "rraaaaaaaa", "sraaaaaaaa", "traaaaaaaa", "uraaaaaaaa", "vraaaaaaaa", "wraaaaaaaa", "xraaaaaaaa", "yraaaaaaaa", "zraaaaaaaa", "asaaaaaaaa", "bsaaaaaaaa", "csaaaaaaaa", "dsaaaaaaaa", "esaaaaaaaa", "fsaaaaaaaa", "gsaaaaaaaa", "hsaaaaaaaa", "isaaaaaaaa", "jsaaaaaaaa", "ksaaaaaaaa", "lsaaaaaaaa", "msaaaaaaaa", "nsaaaaaaaa", "osaaaaaaaa", "psaaaaaaaa", "qsaaaaaaaa", "rsaaaaaaaa", "ssaaaaaaaa", "tsaaaaaaaa", "usaaaaaaaa", "vsaaaaaaaa", "wsaaaaaaaa", "xsaaaaaaaa", "ysaaaaaaaa", "zsaaaaaaaa", "ataaaaaaaa", "btaaaaaaaa", "ctaaaaaaaa", "dtaaaaaaaa", "etaaaaaaaa", "ftaaaaaaaa", "gtaaaaaaaa", "htaaaaaaaa", "itaaaaaaaa", "jtaaaaaaaa", "ktaaaaaaaa", "ltaaaaaaaa", "mtaaaaaaaa", "ntaaaaaaaa", "otaaaaaaaa", "ptaaaaaaaa", "qtaaaaaaaa", "rtaaaaaaaa", "staaaaaaaa", "ttaaaaaaaa", "utaaaaaaaa", "vtaaaaaaaa", "wtaaaaaaaa", "xtaaaaaaaa", "ytaaaaaaaa", "ztaaaaaaaa", "auaaaaaaaa", "buaaaaaaaa", "cuaaaaaaaa", "duaaaaaaaa", "euaaaaaaaa", "fuaaaaaaaa", "guaaaaaaaa", "huaaaaaaaa", "iuaaaaaaaa", "juaaaaaaaa", "kuaaaaaaaa", "luaaaaaaaa", "muaaaaaaaa", "nuaaaaaaaa", "ouaaaaaaaa", "puaaaaaaaa", "quaaaaaaaa", "ruaaaaaaaa", "suaaaaaaaa", "tuaaaaaaaa", "uuaaaaaaaa", "vuaaaaaaaa", "wuaaaaaaaa", "xuaaaaaaaa", "yuaaaaaaaa", "zuaaaaaaaa", "avaaaaaaaa", "bvaaaaaaaa", "cvaaaaaaaa", "dvaaaaaaaa", "evaaaaaaaa", "fvaaaaaaaa", "gvaaaaaaaa", "hvaaaaaaaa", "ivaaaaaaaa", "jvaaaaaaaa", "kvaaaaaaaa", "lvaaaaaaaa", "mvaaaaaaaa", "nvaaaaaaaa", "ovaaaaaaaa", "pvaaaaaaaa", "qvaaaaaaaa", "rvaaaaaaaa", "svaaaaaaaa", "tvaaaaaaaa", "uvaaaaaaaa", "vvaaaaaaaa", "wvaaaaaaaa", "xvaaaaaaaa", "yvaaaaaaaa", "zvaaaaaaaa", "awaaaaaaaa", "bwaaaaaaaa", "cwaaaaaaaa", "dwaaaaaaaa", "ewaaaaaaaa", "fwaaaaaaaa", "gwaaaaaaaa", "hwaaaaaaaa", "iwaaaaaaaa", "jwaaaaaaaa", "kwaaaaaaaa", "lwaaaaaaaa", "mwaaaaaaaa", "nwaaaaaaaa", "owaaaaaaaa", "pwaaaaaaaa", "qwaaaaaaaa", "rwaaaaaaaa", "swaaaaaaaa", "twaaaaaaaa", "uwaaaaaaaa", "vwaaaaaaaa", "wwaaaaaaaa", "xwaaaaaaaa", "ywaaaaaaaa", "zwaaaaaaaa", "axaaaaaaaa", "bxaaaaaaaa", "cxaaaaaaaa", "dxaaaaaaaa", "exaaaaaaaa", "fxaaaaaaaa", "gxaaaaaaaa", "hxaaaaaaaa", "ixaaaaaaaa", "jxaaaaaaaa", "kxaaaaaaaa", "lxaaaaaaaa", "mxaaaaaaaa", "nxaaaaaaaa", "oxaaaaaaaa", "pxaaaaaaaa", "qxaaaaaaaa", "rxaaaaaaaa", "sxaaaaaaaa", "txaaaaaaaa", "uxaaaaaaaa", "vxaaaaaaaa", "wxaaaaaaaa", "xxaaaaaaaa", "yxaaaaaaaa", "zxaaaaaaaa", "ayaaaaaaaa", "byaaaaaaaa", "cyaaaaaaaa", "dyaaaaaaaa", "eyaaaaaaaa", "fyaaaaaaaa", "gyaaaaaaaa", "hyaaaaaaaa", "iyaaaaaaaa", "jyaaaaaaaa", "kyaaaaaaaa", "lyaaaaaaaa", "myaaaaaaaa", "nyaaaaaaaa", "oyaaaaaaaa", "pyaaaaaaaa", "qyaaaaaaaa", "ryaaaaaaaa", "syaaaaaaaa", "tyaaaaaaaa", "uyaaaaaaaa", "vyaaaaaaaa", "wyaaaaaaaa", "xyaaaaaaaa", "yyaaaaaaaa", "zyaaaaaaaa", "azaaaaaaaa", "bzaaaaaaaa", "czaaaaaaaa", "dzaaaaaaaa", "ezaaaaaaaa", "fzaaaaaaaa", "gzaaaaaaaa", "hzaaaaaaaa", "izaaaaaaaa", "jzaaaaaaaa", "kzaaaaaaaa", "lzaaaaaaaa", "mzaaaaaaaa", "nzaaaaaaaa", "ozaaaaaaaa", "pzaaaaaaaa", "qzaaaaaaaa", "rzaaaaaaaa", "szaaaaaaaa", "tzaaaaaaaa", "uzaaaaaaaa", "vzaaaaaaaa", "wzaaaaaaaa", "xzaaaaaaaa", "yzaaaaaaaa", "zzaaaaaaaa"]);
        let output = find_words(board, words).sort_unstable();
        let expected = build_words(&["mbaaaaaaaa", "mnaaaaaaaa", "bcaaaaaaaa", "baaaaaaaaa", "cdaaaaaaaa", "caaaaaaaaa", "cbaaaaaaaa", "deaaaaaaaa", "daaaaaaaaa", "dcaaaaaaaa", "efaaaaaaaa", "eaaaaaaaaa", "edaaaaaaaa", "fgaaaaaaaa", "faaaaaaaaa", "feaaaaaaaa", "ghaaaaaaaa", "gaaaaaaaaa", "gfaaaaaaaa", "hiaaaaaaaa", "haaaaaaaaa", "hgaaaaaaaa", "ijaaaaaaaa", "iaaaaaaaaa", "ihaaaaaaaa", "jkaaaaaaaa", "jaaaaaaaaa", "jiaaaaaaaa", "klaaaaaaaa", "kaaaaaaaaa", "kjaaaaaaaa", "laaaaaaaaa", "lkaaaaaaaa", "naaaaaaaaa", "noaaaaaaaa", "aaaaaaaaaa", "onaaaaaaaa", "oaaaaaaaaa", "opaaaaaaaa", "poaaaaaaaa", "paaaaaaaaa", "pqaaaaaaaa", "qpaaaaaaaa", "qaaaaaaaaa", "qraaaaaaaa", "rqaaaaaaaa", "raaaaaaaaa", "rsaaaaaaaa", "sraaaaaaaa", "saaaaaaaaa", "staaaaaaaa", "tsaaaaaaaa", "taaaaaaaaa", "tuaaaaaaaa", "utaaaaaaaa", "uaaaaaaaaa", "uvaaaaaaaa", "vuaaaaaaaa", "vaaaaaaaaa", "vwaaaaaaaa", "wvaaaaaaaa", "waaaaaaaaa", "azaaaaaaaa", "xwaaaaaaaa", "xyaaaaaaaa", "yaaaaaaaaa", "yzaaaaaaaa", "zaaaaaaaaa", "zyaaaaaaaa"]).sort_unstable();
        assert_eq!(expected, output);
    }
}