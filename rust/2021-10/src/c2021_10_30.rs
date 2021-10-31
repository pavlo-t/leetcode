#![allow(dead_code)]
/// 1044. Longest Duplicate Substring
/// =================================
///
/// Given a string `s`, consider all _duplicated substrings_:
/// (contiguous) substrings of `s` that occur 2 or more times.
/// The occurrences may overlap.
///
/// Return __any__ duplicated substring that has the longest possible length.
/// If `s` does not have a duplicated substring, the answer is `""`.
///
/// __Constraints:__
///
/// - `2 <= s.length <= 30_000`
/// - `s` consists of lowercase English letters.
///
/// https://leetcode.com/problems/longest-duplicate-substring/
struct Solution;
impl Solution {
    /// Approach 1: Binary Search + Rabin-Karp
    /// https://leetcode.com/problems/longest-duplicate-substring/solution/
    pub fn longest_dup_substring(s: String) -> String {
        println!("longest_dup_substring({})", s);
        fn find_duplicate(l: usize, bs: &[u8]) -> Option<usize> {
            use std::collections::HashMap;

            const B: usize = 26;
            const M: usize = 1_000_000_007;

            let mut h = (0..l).fold(0, |rsf, i| (rsf * B + bs[i] as usize) % M);
            let mut seen = HashMap::new();
            seen.insert(h, vec![0]);
            let lbo = (0..l).fold(1, |rsf, _| (rsf * B) % M);
            for s in 1..bs.len() - l + 1 {
                h = ((h * B + M - bs[s - 1] as usize * lbo % M) % M + bs[s + l - 1] as usize) % M;
                if let Some(hs) = seen.get_mut(&h) {
                    for &os in hs.iter() {
                        if bs[s..s + l] == bs[os..os + l] {
                            return Some(os);
                        }
                    }
                    hs.push(s);
                } else {
                    seen.insert(h, vec![s]);
                }
            }
            None
        }

        let bs = s.as_bytes();
        let (mut l, mut r) = (1, bs.len());
        let mut start = 0;
        while l <= r {
            let len = l + (r - l) / 2;
            if let Some(i) = find_duplicate(len, &bs) {
                start = i;
                l = len + 1;
            } else {
                r = len - 1;
            }
        }
        s[start..start + l - 1].to_string()
    }
    /// https://anshika-bhargava0202.medium.com/leetcode-1044-longest-increasing-substring-f40fac0a6169
    pub fn longest_dup_substring_still_too_slow(s: String) -> String {
        println!("longest_dup_substring({})", s);
        use std::collections::HashSet;
        let bs = s.as_bytes();
        let n = bs.len();
        let (mut left, mut right) = (1, n - 1);
        let mut result = None;
        let mut seen = HashSet::with_capacity(n);
        while left <= right {
            let l = left + (right - left) / 2;
            let mut found = false;
            for s in 0..=n - l {
                if seen.contains(&bs[s..s + l]) {
                    found = true;
                    result = result.filter(|&(l0, _)| l0 >= l).or(Some((l, s)));
                    break;
                } else {
                    seen.insert(&bs[s..s + l]);
                }
            }
            if found {
                left = l + 1;
            } else {
                right = l - 1;
            }
            seen.clear();
        }
        result
            .map(|(l, i)| s[i..i + l].to_string())
            .unwrap_or("".to_string())
    }
    /// 15:00-15:28
    pub fn longest_dup_substring_brute_force(s: String) -> String {
        println!("longest_dup_substring({})", s);
        let bs = s.as_bytes();
        let n = bs.len();
        for l in (1..n).rev() {
            for s1 in 0..=n - l - 1 {
                for s2 in s1 + 1..=n - l {
                    if bs[s1..s1 + l] == bs[s2..s2 + l] {
                        return s[s1..s1 + l].to_string();
                    }
                }
            }
        }
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn banana() { assert_eq!(Solution::longest_dup_substring("banana".to_string()), "ana"); }
    #[rustfmt::skip] #[test] fn abcd() { assert_eq!(Solution::longest_dup_substring("abcd".to_string()), ""); }
    #[rustfmt::skip] #[test] fn aa() { assert_eq!(Solution::longest_dup_substring("aa".to_string()), "a"); }
    #[rustfmt::skip] #[test] fn ab() { assert_eq!(Solution::longest_dup_substring("ab".to_string()), ""); }
    #[rustfmt::skip] #[test] fn aba() { assert_eq!(Solution::longest_dup_substring("aba".to_string()), "a"); }
    #[rustfmt::skip] #[test] fn aab() { assert_eq!(Solution::longest_dup_substring("aab".to_string()), "a"); }
    #[rustfmt::skip] #[test] fn baa() { assert_eq!(Solution::longest_dup_substring("baa".to_string()), "a"); }

    #[test]
    fn s_100xa() {
        let s = "a".repeat(100);
        let e = "a".repeat(99);
        assert_eq!(Solution::longest_dup_substring(s), e);
    }
    //#[ignore]
    #[test]
    fn s_30000xa() {
        let s = "a".repeat(30000);
        let e = "a".repeat(29999);
        assert_eq!(Solution::longest_dup_substring(s), e);
    }
    //#[ignore]
    #[test]
    fn s_a_to_zz() {
        let mut s = String::with_capacity(1352); // 2 * 26**2
        for a in 0..26 {
            for b in 0..26 {
                s.push((b'a' + a) as char);
                s.push((b'a' + b) as char);
            }
        }
        let e = "azb";
        assert_eq!(Solution::longest_dup_substring(s), e);
    }
    //#[ignore]
    #[test]
    fn s_a_to_zzz2() {
        let mut s = String::with_capacity(31944); // 3 * 22**3
        for a in 0..22 {
            for b in 0..22 {
                for c in 0..22 {
                    s.push((b'a' + a) as char);
                    s.push((b'a' + b) as char);
                    s.push((b'a' + c) as char);
                }
            }
        }
        let e = "aavab";
        assert_eq!(Solution::longest_dup_substring(s), e);
    }

    //#[ignore]
    #[test]
    fn s_30000_chars() {
        let s = [
            "vbaemlrfvasubbuxdqohlpkuuznigzebcegorztmbngdocautqbegnbamqitrowtucjldexfsgiiiicbyeigrj",
            "jgbtbsiznccwohanmutudceffflnlfywnbqotypictesbhgndbkgfooltzahgbdtctjytzgwsnotwwhzyrifhw",
            "qbxtkewnxyjhycgfaauruqpbrnuztbxlevgobccydnhbppisqelmuapoqjnlmnyrkfflhjlkwvookwgcbtoxtd",
            "nlobwbqvsaazljjywftktgibiluzlqeybtbawrxqtzpeiyfggysbebdpozozpuatpbvlvsbortvbtaizaabfbg",
            "ushkrxgtswvnhqflgyebzzdhzeicllkhdrrxqxqivnxdfeqmcupctvluztykhirmcklsjcdfrckxhallwcprxb",
            "oywmidxbqbnbtzfygzbqyksjvqnljloxjkmuolxgmljhmrgsqjkovezqwmdfxrfjctfsecaspjzbahvqsfgnlf",
            "ghsjnrduchnhzrkzlbkkdnuyfogmcrrkzltofyihfvpgobhzcslffhgkwowafqkgdduieeqjbhqpixqkrcswla",
            "jhohhvxrcdxylbfdmgkeyueohvsbuebdlrkvmkmonlpaougbashynlpujhvdnyizpuwdgiuvwjwkfenfwvgjix",
            "frgcwuaevrbrvhbpdxabkffxmduundrswyxtszznrceomffpfhspmzoycltrqmmmkjhcijrblvrxdyokfibrjr",
            "smcndszrhkaqolzgmndgkiebuklvdvnvhrkmmkgvuglkrlqgaummesnzczwiujfddqrraxmwgxgzkxfkttxzmk",
            "yqlvyhggmlwecwdoujoajyzqjukiuyuuogrppavcntiqrnglzunkvnagjmofmicbqtsgvpkjiubzhhmiezszov",
            "fahvfudjxufrcaiolqimrrufaaguzvpgxxbetgosctlgvtuhletcwpjyppsqmiyjhbocicukxhrsanzlsvpqky",
            "ocniucqtlzubbefxpceixexwpprgheevgueawwvhwqjzuqxltdvqitozkigsmmpugvwcssmqkxpgnouispyckp",
            "bvxjplwehcadhuxtmjqbsdqmmkdmmqdpnazcnxiubkaiezbqlrudxmrjwwximbzzbydrxmlzwnhdnjtycpdzbu",
            "kxsihfjjgmhmdfyrgwgnztyvtdkwcmspllxvsgeaunmaumrajzqivofotqnwehlrskomsunmkzmemfhxfxfann",
            "qhfdfllsefctsuphgzgvguanrfmzhzmyitdneqfiwjdqdtzmufhachntpzuwbgwjbazfqgqoqtgtofhjnojpwz",
            "knafldnippkowcwmtmiziwhphuetlcsxruecyumnlwdhbzwuoevwccnkytdlxxjorptrjfeuehcfhabvewtizg",
            "uzczlzwwqqljhxqmruvvbcvczbbsmcbgbfgoeemwuyfexwqosnmwqjyrnbmzdpffunvasixmieqfcivvwhrotr",
            "hgvtfyimfcabfsjcgtgxyjhvgffztysbenwlmhxtjxuopfswybvykeuskztdzlgdbfonmngxldocigajhxhtcd",
            "nmegrdsagvfwirkqqxvebelcpubwqdcdpgmjkgjsbqpjjbbzaxebdrfsnhqdksjcbvlnbwfcmwnxxqwxpcbstg",
            "mjndmaxxcaspnvntzzpmypfuouxrpsybblgqlgnpgiewdtpdsgrsxynovmfkefnirwyecnukvaibuzeoixhfyw",
            "vtiddmiqwbmugqicddcleylkptkwaiswbcbgqjynlnhtfrvixzwdyhonaalckfalhbtxnrkagutmlowipsmnzc",
            "cbepeouebdiubvnnvwjqiveefvxtlyxnabfgpzmqdwmgrcvoyfegckmhgldrkagmelutnimsfyyyctnaaticgv",
            "kmipfuplgtzcpmlbeqbeoezdgaxpcdmmrudwrtjxoicdwngadketqexwewnizbxsyaqptzokkqpgdwhginaytz",
            "wzvmiwnfolfeabfplpfawlydristimsflprtleazsgfshljucrwycvdvfwrokltvbmrmsrokzvfgiqalhzjlwn",
            "hblapvlksmyjbrdpdcmklvbngkmrwiefkrfbmqoffmqgfvxlyjvjgyymllghtkofevjqnxxslqarlzejlzurat",
            "glemuzyihcfqvspslaintunxfyzapzvxmbjgaiqjmvbhpqactkqmgdmmdloumlsamdionyyavzhksjnlwdzcil",
            "qnyghmrzmdfussussrrccjlrwjwwoehetilauecjkhfczcuutnwvzjmvtirhobnhjakffmqpvwtwypqrcmpdwd",
            "zrosycvlmgoymtmckhaddaswswijagmbxatxfpltvzrjudoemohppznjxwsxegaehudmlofipyujbinnywepxs",
            "avqsuwzvaireweudombixeslrtjxihrjehdyeetgommkqjpfvqpsuifnbrnlyewcoccuiycxkpjbyzivrxpohv",
            "bkmqwmxuqlwshfqjedyngymotenjsvgvfuodilyyywqfbjofmpvsnhijnvufscdayrdgcdeaawhdhxltmgncnv",
            "ndtjkllnllerriuxkwvtadrnydtmhijulcdwoolbzsssaprnlngnsxaqdvekeqfrsinizzkarevttmihzpwroz",
            "kruqcagjszthjsitvbaqxtcfcewzxlajeuaixgrrhlarfjmammsjivrkbnlyalihikphqjyywyrbjdewqwhyty",
            "kuowvqjfxtppahaahwzyzkcdgscqsvocseapxmuwfllisniajjajucocwqsoojtgnvvmhrjpcwxwkhyvrzgnbv",
            "paniqbtjjihevqxefbaeymoyihxoubljbztrcodmxmsscqwktyqrnesvlplhsxtvyopadumaghskxquqqnkdre",
            "afvmmolhjjwylzuazahtlgjhybukfoohvktjygrjpnkbmednhkgsqrbosyzzopjjzcszjllaaxdobgbkqjeiag",
            "zvgawupfdrkxqvjdvlabaehboaltknbjihjhbbmgswdkroopaoqauzdjboeoehxoojvpmkvegaoaperrvjrpkw",
            "amlgcrnzfbkbswlpctstvjxmauxtkipmwctytizzmkcgipwnkqgizfwzausmcjaymvwheviqhkwsbufoknwlss",
            "ksmegqohqccojgmsytexnirswehspamcubibryiiamwmvketekqtskooqvihvvjswutpsdztryfmilgmvyiokw",
            "carnkpivhgpcvihvdytklbvrqsrygcwlenyqeimlodpjnweqvzagbxztqsrywnlbcjthtfuhjktwgxpdzrjark",
            "vjdmhzbtcfewmydjhmtalbacfztqbmbwvgkdhlykseghejqmylmlmmptnkkoxnmstwxajvmccujggbsgmhtavi",
            "ujsgzptmiokfqudfyfzjnfkgnwpgwkosvjyvytjihtqaptfnaewnmivzekgdxwxsfwnepyozsebhzgxzmtnack",
            "aivinselmmjcsyagzslackgzmvubvylxhifwcmhpyqrqrrwsmjtqwbriwljukurtdzrhlzwjrpbkjzdvjwnfze",
            "nyrysdslwxkofvpdgitjtcjwzsumztsotjmlpexnejdlfsondjxwlgumaoabjspdebqvhpilzmygenvjysfkaz",
            "ycpzdbtsgztwubmdonpnvvcqfajmmlolodmrgfmmhfjbixgxmxkhtmxamiobucmkiydofpdmvoqdnwczumipyf",
            "pnqfokicwsczihplwmzmrmiunegvthdkwtqmfaobsqcczroxfpjxeuttybpskucgnussuivoaxrarhzwwlvcfq",
            "wfslxcfcluydotmljmiaobhoagzxigoelirlmypxsndiiptsdcuufekfkldurtbcbupntennsqwchrukvrgbvq",
            "ghbldseexhzovutijnahphzmrudyvyqefspimwpldqaaktkvikdxrcjlymlwkbartrcinrttjttvoeqwovuixf",
            "tyseuehuydnaldtewcarxovplmlhukmpietclayjfmjqpexnixehlcwyojfkuszeyuhlwfuctfuijrvtocacex",
            "auiigcafhabqmzxidroybzpxtgztifskzbsffumovpejoeruvhswhjvcpwxzbskcvcjazvrwmxhdbyxwirvraq",
            "tsnqrqidtdieghaxfowmrykowufrlikeazxtphzknhpvqruvfqhxotnlovczdogfnfadozpbggwwxstfjaexut",
            "kiopjtdrarwbwlalojunhtfbyobeoxyyhmnririikfjgsmtgvntehbfhpjmdgmeoyrhikpruwcaqqwxnjssstz",
            "cliqqrcufcoolydcvcgxsxtrkfkexfezqmrjdtdkwryhselnitmaqgsdlehkjnkccblhxqutksacynrggdjxld",
            "cwhlhsbtwdwhktyemizomzbfikkrjwuludydxzwucvbpobtdlutzuvgcfrvqprblubptblnfgruxuqagmvhgqo",
            "kxhhnyyqjuuyovmbcsuxrpptxbpekhuwhdewbcplzjjpevsiqnfjcwdzaufkbcgifkfjpuuqfffjdxrvmzeoxj",
            "pxdxhfzqpgcwptqljvrqgwoarabvrahiykfhpgxhpcdevwshtlxchlcyofvffcnfpvngbmnsqrzmnrtgcqkjem",
            "jstezzjmgyjtgniufynemavfizytichtubavcjhijtepgollmyqzangjneexgnrcqrxuchfncjcizqtlolmpbt",
            "aozenitemkxmebvjoxutftjyhxmtmnoodqsqyoxkywycytomqifvcowjokeaxvpaljsjxpvxoucmpqygaebcuz",
            "nijulipckandlnugkicousevafbdjvdzgottxivhikmtxjbkrgyoyfjykmbvhgupvniuxfdfqqapzczsiagifr",
            "irdlmnsjepwnjwsmbkeeadizysbgagaixftmsnvxctmlpeatrexrkfsixuqzaqhawcmqshpuabiqiijzmisynk",
            "ikezjhrshpwvgocxleztujbfkpncocmwotxzaptjeqemgikrmlkjulqedggriupxpnwrcqaiigxoqhidssooge",
            "ujmcavwrumesdiigrsoojfxrirlyhrebardcmbugnaytjjzcdnsmupfeircpihslavpmwlummuhxfgzjvtxskd",
            "wsuzjeibriyhwwusiimpfsxgdvzcnjflbbkmgunxengakbssjhkrbjxeexitgofkrrwxomxszfvjgnesuqjxzg",
            "bgdkzmagwraurogqiivdterxwnphlpnovtawhcffufznviddqyhjcajanyhapxpksrjzuoeqvfdqngvfgcpjea",
            "juatusqlckdnjzdppyuuqiamngqervgdpjwlboxfyctjdwysxuopcobswuyqrhtgqrtrhswhurqzqqtpkhepvg",
            "juchktofbgyuxfnwmjpejhuewgmbxjjgdlqpgguhnsdmzsklccrnjnriufjrpuashsknyeunzzcysokwsdjerc",
            "mlixezrgtyydnzijohisfajrwdxhhomvzwsfvlmolmsylchclqwssfkpjjyqxkmyigdfsrudeoerqlvbdstwxf",
            "nionbnanlivpopiktmazgqkbtqtootwbbmqcaqrlzncpclbxzbwhtjlmbecpzysbnidnzxaamdrqqsorvmboxm",
            "casepsfjtssyhtxhvjmoaegqydezrcntcfzuxedsyxrfsoppaddailqioujfywadbazzgethekowbdmdjrdtdv",
            "brrkzsgzvhbwiiacreofdrlruuiznluofmyeggfdphzyrbciisplkaceukehyaxdovjudoxtxwtqavyinqtzqx",
            "glhksfmqkbntvtkvmhtfytcybrowrhtzsdjmixevysowlarzikgigkihbzgugztacemncriclyywzrxjcdtknd",
            "rqczlkzgkdnxqpqatbzuzalwlpzezohtemsrytylxlkpcaqxbrrardycsxiunrnrffjebjpywznabdxcwpenuc",
            "robbiotvhadseebvezwrzxxzztqfjkhcykgjabgibjagpedvdanfxmexhuesemxeydnzeuhffjnxntstthyqvc",
            "pdwuxciuigxyfdsolzyayntrgmaefwiubeqyiytrhspmwjdjkdjjqdcxrcdwamrbshabivupnldlwguglercjv",
            "baaexasoclxeofzkumxyytgubsyvwhqxiqdhtwvvjszzaalumiumbdjevhzrrsqbktidrfaczbdzbowqwsezvn",
            "gsxflozbrkxbpqgqvhryhtnfjlzplvdrpaybqyejkbkzusrpzjnieapnmpczkhqzhqczqhiciscckvrmehuijk",
            "xvzcwljomtfpsvwbygtclgxeselomvamsbormxfbqksliqmiwhmjplojpbyeyyqtcekqdprwcjhmzvuycihstx",
            "bjbbnbcduejgumwmkaxrmzmgzroijhgmsjohksmhwvnqkleulemxdafcrtkcdsrxdffqzrxvnmnzyutjyhhdim",
            "hbitenovkrcrjbjgyyvnxthsehalkkatlknrxdmjwqbgtmmtkhhjcobhwduinuczgrqdufpqxqwtclmoutzzkc",
            "gigaqtxuudlnjhsyarqykulxkgsjfclnmtpdnozjslwduqduitvupgrzmmitqvidpyiemhngumlpcolmjghyna",
            "xbmfxfdfisfsyuzncuzojccwqmdxkqyitmpqrsybmftkzvycpzqaduwugbttttbngsfznddzjymktmmklekpzj",
            "lfkeyeybtgwyhjcmknlwxgkmryqdppmavxevdezwmvuueygqntplazxtxnwmfjocivxlzyhotdizeqqrkfcmgb",
            "ferbswkhysexffwotrsbrwuhneossuvxamavklekfiknnibhztkqrezfipzckuzmjahvnliuvshjclsbecuyht",
            "drleuvatjjvrhkepfajollzdmgfgemcjeppampvvzrmibtxivgxgtyjfeookdsvjhkjtaeobvdjzyghtogzhfi",
            "olyewbyrkfcvaearfxwowuwgnmmovrwldwszyqskwwgyaiphflxehvkwjwkeqistfkufaorylxxnhovncutjqd",
            "gzbsgrbamimgnmxeniemxlauaepvqhyyicqottqibcqqrnxevdqvqsprzgopnnnwrdmmxfuahlryyoewtwrjri",
            "cqprfcguaxzpjwuezbpqcpgglzdckunnkcereklhhkwsjqwirnavficqjfvtziglkkeqwrzfdvymnwwhmycrge",
            "jrjelkorxaebtcssivbaemlrfvasubbuxdqohlpkuuznigzebcegorztmbngdocautqbegnbamqitrowtucjld",
            "exfsgiiiicbyeigrjjgbtbsiznccwohanmutudceffflnlfywnbqotypictesbhgndbkgfooltzahgbdtctjyt",
            "zgwsnotwwhzyrifhwqbxtkewnxyjhycgfaauruqpbrnuztbxlevgobccydnhbppisqelmuapoqjnlmnyrkfflh",
            "jlkwvookwgcbtoxtdnlobwbqvsaazljjywftktgibiluzlqeybtbawrxqtzpeiyfggysbebdpozozpuatpbvlv",
            "sbortvbtaizaabfbgushkrxgtswvnhqflgyebzzdhzeicllkhdrrxqxqivnxdfeqmcupctvluztykhirmcklsj",
            "cdfrckxhallwcprxboywmidxbqbnbtzfygzbqyksjvqnljloxjkmuolxgmljhmrgsqjkovezqwmdfxrfjctfse",
            "caspjzbahvqsfgnlfghsjnrduchnhzrkzlbkkdnuyfogmcrrkzltofyihfvpgobhzcslffhgkwowafqkgdduie",
            "eqjbhqpixqkrcswlajhohhvxrcdxylbfdmgkeyueohvsbuebdlrkvmkmonlpaougbashynlpujhvdnyizpuwdg",
            "iuvwjwkfenfwvgjixfrgcwuaevrbrvhbpdxabkffxmduundrswyxtszznrceomffpfhspmzoycltrqmmmkjhci",
            "jrblvrxdyokfibrjrsmcndszrhkaqolzgmndgkiebuklvdvnvhrkmmkgvuglkrlqgaummesnzczwiujfddqrra",
            "xmwgxgzkxfkttxzmkyqlvyhggmlwecwdoujoajyzqjukiuyuuogrppavcntiqrnglzunkvnagjmofmicbqtsgv",
            "pkjiubzhhmiezszovfahvfudjxufrcaiolqimrrufaaguzvpgxxbetgosctlgvtuhletcwpjyppsqmiyjhboci",
            "cukxhrsanzlsvpqkyocniucqtlzubbefxpceixexwpprgheevgueawwvhwqjzuqxltdvqitozkigsmmpugvwcs",
            "smqkxpgnouispyckpbvxjplwehcadhuxtmjqbsdqmmkdmmqdpnazcnxiubkaiezbqlrudxmrjwwximbzzbydrx",
            "mlzwnhdnjtycpdzbukxsihfjjgmhmdfyrgwgnztyvtdkwcmspllxvsgeaunmaumrajzqivofotqnwehlrskoms",
            "unmkzmemfhxfxfannqhfdfllsefctsuphgzgvguanrfmzhzmyitdneqfiwjdqdtzmufhachntpzuwbgwjbazfq",
            "gqoqtgtofhjnojpwzknafldnippkowcwmtmiziwhphuetlcsxruecyumnlwdhbzwuoevwccnkytdlxxjorptrj",
            "feuehcfhabvewtizguzczlzwwqqljhxqmruvvbcvczbbsmcbgbfgoeemwuyfexwqosnmwqjyrnbmzdpffunvas",
            "ixmieqfcivvwhrotrhgvtfyimfcabfsjcgtgxyjhvgffztysbenwlmhxtjxuopfswybvykeuskztdzlgdbfonm",
            "ngxldocigajhxhtcdnmegrdsagvfwirkqqxvebelcpubwqdcdpgmjkgjsbqpjjbbzaxebdrfsnhqdksjcbvlnb",
            "wfcmwnxxqwxpcbstgmjndmaxxcaspnvntzzpmypfuouxrpsybblgqlgnpgiewdtpdsgrsxynovmfkefnirwyec",
            "nukvaibuzeoixhfywvtiddmiqwbmugqicddcleylkptkwaiswbcbgqjynlnhtfrvixzwdyhonaalckfalhbtxn",
            "rkagutmlowipsmnzccbepeouebdiubvnnvwjqiveefvxtlyxnabfgpzmqdwmgrcvoyfegckmhgldrkagmelutn",
            "imsfyyyctnaaticgvkmipfuplgtzcpmlbeqbeoezdgaxpcdmmrudwrtjxoicdwngadketqexwewnizbxsyaqpt",
            "zokkqpgdwhginaytzwzvmiwnfolfeabfplpfawlydristimsflprtleazsgfshljucrwycvdvfwrokltvbmrms",
            "rokzvfgiqalhzjlwnhblapvlksmyjbrdpdcmklvbngkmrwiefkrfbmqoffmqgfvxlyjvjgyymllghtkofevjqn",
            "xxslqarlzejlzuratglemuzyihcfqvspslaintunxfyzapzvxmbjgaiqjmvbhpqactkqmgdmmdloumlsamdion",
            "yyavzhksjnlwdzcilqnyghmrzmdfussussrrccjlrwjwwoehetilauecjkhfczcuutnwvzjmvtirhobnhjakff",
            "mqpvwtwypqrcmpdwdzrosycvlmgoymtmckhaddaswswijagmbxatxfpltvzrjudoemohppznjxwsxegaehudml",
            "ofipyujbinnywepxsavqsuwzvaireweudombixeslrtjxihrjehdyeetgommkqjpfvqpsuifnbrnlyewcoccui",
            "ycxkpjbyzivrxpohvbkmqwmxuqlwshfqjedyngymotenjsvgvfuodilyyywqfbjofmpvsnhijnvufscdayrdgc",
            "deaawhdhxltmgncnvndtjkllnllerriuxkwvtadrnydtmhijulcdwoolbzsssaprnlngnsxaqdvekeqfrsiniz",
            "zkarevttmihzpwrozkruqcagjszthjsitvbaqxtcfcewzxlajeuaixgrrhlarfjmammsjivrkbnlyalihikphq",
            "jyywyrbjdewqwhytykuowvqjfxtppahaahwzyzkcdgscqsvocseapxmuwfllisniajjajucocwqsoojtgnvvmh",
            "rjpcwxwkhyvrzgnbvpaniqbtjjihevqxefbaeymoyihxoubljbztrcodmxmsscqwktyqrnesvlplhsxtvyopad",
            "umaghskxquqqnkdreafvmmolhjjwylzuazahtlgjhybukfoohvktjygrjpnkbmednhkgsqrbosyzzopjjzcszj",
            "llaaxdobgbkqjeiagzvgawupfdrkxqvjdvlabaehboaltknbjihjhbbmgswdkroopaoqauzdjboeoehxoojvpm",
            "kvegaoaperrvjrpkwamlgcrnzfbkbswlpctstvjxmauxtkipmwctytizzmkcgipwnkqgizfwzausmcjaymvwhe",
            "viqhkwsbufoknwlssksmegqohqccojgmsytexnirswehspamcubibryiiamwmvketekqtskooqvihvvjswutps",
            "dztryfmilgmvyiokwcarnkpivhgpcvihvdytklbvrqsrygcwlenyqeimlodpjnweqvzagbxztqsrywnlbcjtht",
            "fuhjktwgxpdzrjarkvjdmhzbtcfewmydjhmtalbacfztqbmbwvgkdhlykseghejqmylmlmmptnkkoxnmstwxaj",
            "vmccujggbsgmhtaviujsgzptmiokfqudfyfzjnfkgnwpgwkosvjyvytjihtqaptfnaewnmivzekgdxwxsfwnep",
            "yozsebhzgxzmtnackaivinselmmjcsyagzslackgzmvubvylxhifwcmhpyqrqrrwsmjtqwbriwljukurtdzrhl",
            "zwjrpbkjzdvjwnfzenyrysdslwxkofvpdgitjtcjwzsumztsotjmlpexnejdlfsondjxwlgumaoabjspdebqvh",
            "pilzmygenvjysfkazycpzdbtsgztwubmdonpnvvcqfajmmlolodmrgfmmhfjbixgxmxkhtmxamiobucmkiydof",
            "pdmvoqdnwczumipyfpnqfokicwsczihplwmzmrmiunegvthdkwtqmfaobsqcczroxfpjxeuttybpskucgnussu",
            "ivoaxrarhzwwlvcfqwfslxcfcluydotmljmiaobhoagzxigoelirlmypxsndiiptsdcuufekfkldurtbcbupnt",
            "ennsqwchrukvrgbvqghbldseexhzovutijnahphzmrudyvyqefspimwpldqaaktkvikdxrcjlymlwkbartrcin",
            "rttjttvoeqwovuixftyseuehuydnaldtewcarxovplmlhukmpietclayjfmjqpexnixehlcwyojfkuszeyuhlw",
            "fuctfuijrvtocacexauiigcafhabqmzxidroybzpxtgztifskzbsffumovpejoeruvhswhjvcpwxzbskcvcjaz",
            "vrwmxhdbyxwirvraqtsnqrqidtdieghaxfowmrykowufrlikeazxtphzknhpvqruvfqhxotnlovczdogfnfado",
            "zpbggwwxstfjaexutkiopjtdrarwbwlalojunhtfbyobeoxyyhmnririikfjgsmtgvntehbfhpjmdgmeoyrhik",
            "pruwcaqqwxnjssstzcliqqrcufcoolydcvcgxsxtrkfkexfezqmrjdtdkwryhselnitmaqgsdlehkjnkccblhx",
            "qutksacynrggdjxldcwhlhsbtwdwhktyemizomzbfikkrjwuludydxzwucvbpobtdlutzuvgcfrvqprblubptb",
            "lnfgruxuqagmvhgqokxhhnyyqjuuyovmbcsuxrpptxbpekhuwhdewbcplzjjpevsiqnfjcwdzaufkbcgifkfjp",
            "uuqfffjdxrvmzeoxjpxdxhfzqpgcwptqljvrqgwoarabvrahiykfhpgxhpcdevwshtlxchlcyofvffcnfpvngb",
            "mnsqrzmnrtgcqkjemjstezzjmgyjtgniufynemavfizytichtubavcjhijtepgollmyqzangjneexgnrcqrxuc",
            "hfncjcizqtlolmpbtaozenitemkxmebvjoxutftjyhxmtmnoodqsqyoxkywycytomqifvcowjokeaxvpaljsjx",
            "pvxoucmpqygaebcuznijulipckandlnugkicousevafbdjvdzgottxivhikmtxjbkrgyoyfjykmbvhgupvniux",
            "fdfqqapzczsiagifrirdlmnsjepwnjwsmbkeeadizysbgagaixftmsnvxctmlpeatrexrkfsixuqzaqhawcmqs",
            "hpuabiqiijzmisynkikezjhrshpwvgocxleztujbfkpncocmwotxzaptjeqemgikrmlkjulqedggriupxpnwrc",
            "qaiigxoqhidssoogeujmcavwrumesdiigrsoojfxrirlyhrebardcmbugnaytjjzcdnsmupfeircpihslavpmw",
            "lummuhxfgzjvtxskdwsuzjeibriyhwwusiimpfsxgdvzcnjflbbkmgunxengakbssjhkrbjxeexitgofkrrwxo",
            "mxszfvjgnesuqjxzgbgdkzmagwraurogqiivdterxwnphlpnovtawhcffufznviddqyhjcajanyhapxpksrjzu",
            "oeqvfdqngvfgcpjeajuatusqlckdnjzdppyuuqiamngqervgdpjwlboxfyctjdwysxuopcobswuyqrhtgqrtrh",
            "swhurqzqqtpkhepvgjuchktofbgyuxfnwmjpejhuewgmbxjjgdlqpgguhnsdmzsklccrnjnriufjrpuashskny",
            "eunzzcysokwsdjercmlixezrgtyydnzijohisfajrwdxhhomvzwsfvlmolmsylchclqwssfkpjjyqxkmyigdfs",
            "rudeoerqlvbdstwxfnionbnanlivpopiktmazgqkbtqtootwbbmqcaqrlzncpclbxzbwhtjlmbecpzysbnidnz",
            "xaamdrqqsorvmboxmcasepsfjtssyhtxhvjmoaegqydezrcntcfzuxedsyxrfsoppaddailqioujfywadbazzg",
            "ethekowbdmdjrdtdvbrrkzsgzvhbwiiacreofdrlruuiznluofmyeggfdphzyrbciisplkaceukehyaxdovjud",
            "oxtxwtqavyinqtzqxglhksfmqkbntvtkvmhtfytcybrowrhtzsdjmixevysowlarzikgigkihbzgugztacemnc",
            "riclyywzrxjcdtkndrqczlkzgkdnxqpqatbzuzalwlpzezohtemsrytylxlkpcaqxbrrardycsxiunrnrffjeb",
            "jpywznabdxcwpenucrobbiotvhadseebvezwrzxxzztqfjkhcykgjabgibjagpedvdanfxmexhuesemxeydnze",
            "uhffjnxntstthyqvcpdwuxciuigxyfdsolzyayntrgmaefwiubeqyiytrhspmwjdjkdjjqdcxrcdwamrbshabi",
            "vupnldlwguglercjvbaaexasoclxeofzkumxyytgubsyvwhqxiqdhtwvvjszzaalumiumbdjevhzrrsqbktidr",
            "faczbdzbowqwsezvngsxflozbrkxbpqgqvhryhtnfjlzplvdrpaybqyejkbkzusrpzjnieapnmpczkhqzhqczq",
            "hiciscckvrmehuijkxvzcwljomtfpsvwbygtclgxeselomvamsbormxfbqksliqmiwhmjplojpbyeyyqtcekqd",
            "prwcjhmzvuycihstxbjbbnbcduejgumwmkaxrmzmgzroijhgmsjohksmhwvnqkleulemxdafcrtkcdsrxdffqz",
            "rxvnmnzyutjyhhdimhbitenovkrcrjbjgyyvnxthsehalkkatlknrxdmjwqbgtmmtkhhjcobhwduinuczgrqdu",
            "fpqxqwtclmoutzzkcgigaqtxuudlnjhsyarqykulxkgsjfclnmtpdnozjslwduqduitvupgrzmmitqvidpyiem",
            "hngumlpcolmjghynaxbmfxfdfisfsyuzncuzojccwqmdxkqyitmpqrsybmftkzvycpzqaduwugbttttbngsfzn",
            "ddzjymktmmklekpzjlfkeyeybtgwyhjcmknlwxgkmryqdppmavxevdezwmvuueygqntplazxtxnwmfjocivxlz",
            "yhotdizeqqrkfcmgbferbswkhysexffwotrsbrwuhneossuvxamavklekfiknnibhztkqrezfipzckuzmjahvn",
            "liuvshjclsbecuyhtdrleuvatjjvrhkepfajollzdmgfgemcjeppampvvzrmibtxivgxgtyjfeookdsvjhkjta",
            "eobvdjzyghtogzhfiolyewbyrkfcvaearfxwowuwgnmmovrwldwszyqskwwgyaiphflxehvkwjwkeqistfkufa",
            "orylxxnhovncutjqdgzbsgrbamimgnmxeniemxlauaepvqhyyicqottqibcqqrnxevdqvqsprzgopnnnwrdmmx",
            "fuahlryyoewtwrjricqprfcguaxzpjwuezbpqcpgglzdckunnkcereklhhkwsjqwirnavficqjfvtziglkkeqw",
            "rzfdvymnwwhmycrgejrjelkorxaebtcssiwoikdejksjnjsrtjdzupooposqhulcejqmvvteglotximsrrrmjh",
            "aywuofobhvkzjivcobqewpmkheeyqngslemblcftxruhzrloudsqsfdzljabwivjmhmzjrsjlpqkgdabhvtbcw",
            "dnhdqdllozsdydwtmhkcuqiurrrnstlvitaywnlnodqphmitusyqkimezerrzcpivpiufuhtzziilsehmexxmk",
            "blquepwtzslkosgvlfwwohqurdyqckcfxszbkvxlpkvdebarjwnzgetzjtpldsscmrfymekeqhhulkmuizvqfo",
            "pqhhbmztqnmvmbawfqfonqyzkjafzgvpwewasxvimigyjiwqacensfmfonmnvketoctjbrtozzokhlakpgxjlk",
            "qzzdpupvnfyjobaipbjelrrnbgtarfmiglnxhcvvhvgywggojpuuiljltcmmjyusmjrtlmqvhwteebsnsojbzs",
            "ouxaggbdanmsiaezxmxsqetkrnydozvfvshdbigbxvtgxyhumgmekhoeuohlpeqkxxpaiybftxnqvwmhfynphd",
            "vbrcmijeuicjikhjrcghsddgmigzancvrvaddjkxrmhzyqkpttwnyculmcmoqrkhmmmsxwrokapirbakwyxvct",
            "ipwudolpxablsakjoukffdjfnfdmcgikxghcuusruveowoamjmuopfvekaspgazfgiiadctzzyovmqkxcgnrli",
            "xxjpmrqmxwuhzzjuzisnghfsghyuajcxgnmtfeheddlcdwycetrmcweidecfnhcwuqrzvydydqcgmcitvbfxxm",
            "gcqkherlcmctlztrzwhddufnndpuaogitxtadkofysykttdspmxugeivhzskpyuivplyazfyrfkmypgocdkkjs",
            "ittafjyzrwpcwrdcrwjeiitoyxtikddsywudhscbqtfnmjrefakqkfeidatndwhbubgevkfmekbqvuwdwgthoz",
            "rrzmgekphhsnlnysbecewayvdxdwervxkxkqbuhldzyonygqelvblvwqsbmytjydgiwkedvifvyuwhkashdfdz",
            "vvdjwwsuzbexlibisgdtczamauudbzfwuihsgzmfcqdnbgvvoboqcqeckailzpjwzqofsbmutriuzfqpighlxm",
            "phoxvooempvaupdjjxcttebltjsburpkadjvtaafpxgmmjxxizrcdsnzbhayyrizutuotwvjvbxhnufxtyzzqp",
            "arwapvtzwntqdopbzkcwurqbpmzqnpfniepabibxxundleekpihlczmrnljacrnkvemwootmkipvpviedrpfqb",
            "yeokgqbwcyapwpdpdnzuloptuuklccmagcmpuplxnlojctafiabtqqolnywpdlrlpeaktljhzxsbqtjyvkuldi",
            "yqjxfmkwxzaalwlufmrxqhqzillczokzetwjhiihikhtguolhhjzpbyzpjbqynpejoqqwazekgatdqgmxrlotl",
            "nkuoaucrbnftdctdidlkwqadeaupopcezzsbvwtwjyppciunohxmyarcaouodwsfvngtlbpxahoerirfppsppl",
            "urzoffmakkpsdcwszybnctxuapluecgytfoakbawlrrqunktlmjnfijhxwceyicrprsauuyhmrxgehbrvmhcvs",
            "kupmynudfjwzlqtmxhdwnkfrbbdidabnvqzvftymfulqmhcloihesnvtqaetbhhyxitecurjoezdleocmwielf",
            "zbfdwikfgjocoswhbmlcpzigduelapqcsxfqgswodkqtcoafdohunqasmedpvvrpsygyqenuqfnswqzzeitraw",
            "jnaoewjvndtvemztipyvjwlivrrrkrnvgnjmluhbadwerwxzlxwxukfrbyimsxjanvnapwbpbyvsusthsqemcm",
            "ffmoteldhayubnscfuaamwvignymbcaghnwbtkeakcwlrpqyjqbxpwfxlfjojmurztssdyvpatbtirstjttara",
            "zeijsrvokwhcqmancchfwrwqnhlgkeijiquckrqunazgmztoeatcurshbfnljxurqrtailuylteavxqbamdudx",
            "dgcsrlpoeffzfpkcnyxraurqvrcanixqoziimzwqfdrkscoicmzcisselxeyfeqvfadsabkfgcrbgijkwioedg",
            "iofppmtfiainsipfpvjiwrqhbmsptvxbjdsyrngzooniwennzwvynofqczfvufzektcbhibxyqfwutjvrhvhxd",
            "zzcfxxmwdwphxyqavqqbnmwwipfdwcgyelfbirzlnosmnknidflqshoaxufvyotezyfjpewevqridgsglewdnm",
            "fkilihbblactltxhyuzrecxxkkdiosvvddinmjoylstwmsfmnhdrjcqwmvecozyrfgersurdnkcbdjnqqqjcua",
            "ygveljejclricjdwpsgrtlaibtalvunvciznufwwqqfvzkroezhzlkhodhbabovdzwtteppufwrhikxmkhlvru",
            "jwblwebtkardtvulscjjlsuieepsbimkeqxcwzvgznlvigiakrmvlifsovkpfybkobmlcktbzfyhnikzayungz",
            "fwwpleqooercrtolxnsvxfejgghatpbblasyeimunewexnpuajszoyiwgpwbeacpavnedcysiwxfjocmywlouu",
            "laljdqjghjbxpgznzhmyoxoslmoxdktqmebqwlwewzbohcbglpgznvuyjixmvsehzgzvtxbrwefgijmtytvnop",
            "mxxlmuescdybatbelxkqftzsmanhndwbcwnqstjuyjyafzeqypiekkywkngxsebwconvjyrhjrhosogtfpzpbz",
            "nlotkgalccbpzmpttzkdnftuaylznmaazowjuwxwaaocnfviehnumgqmgbntpnwobszvqyuqzksoppywxoqjwg",
            "ydjlyybuoconaemwxnipdzhbvernpotnrebpecmsgagifsechotkoaljgdxwtslkzmbdqpwxqchxwmbdawpoza",
            "affzbatnmsbqwsfsrxwyuumxxpumfvohawgewatlsubqogkhegiaauazlunrreytahxyhsjbpmudtenzlvbnrb",
            "ndxawoewigdhhmasqpfkuaojzbecpyeedbebslxctdjarjvhaatvciedllfalleoplphyojrrsulersetmbvwi",
            "yhvfkgrqwgdlnbzyvtsreyiydbmgwiqwslbctsupyycnzcnwxgndgvbjwosuykqswswrkrhlpjaqnonmwapcyt",
            "oujipjshcqzyimrpxgazzqnoclquqonelyochgjjxlkhxbywkyvlolztypnoecuraithretqpdxmgvkqtgbktc",
            "frssiywepynkfgoaweftsmodejlivekaxeuhcqmtqbfwjuyfaeassoxwzigvorxovsvakjvbmbzvxzcxfcrxsp",
            "wsxcpovocrgzttlqwxrxvoyxnecfbuuzotcfhxulqcnlxycvxvsukruzxweysayjwcofbitsngfkehgddaypts",
            "oqqvrixrtvvibwsuqfawsgfalkcjzwdqovnhgkyzivhjscfijfgdnodyygkaepsulfcrrcszycisepapwjtjxw",
            "kxewxklpfywjronkploadspghmrcxqbhnffnstezrbklrxxgjlgofywmknrhdsrtxygrrmqlnawgyvnjgvzltw",
            "zflqpriikigszaesluujauepdknfxibwoonffxnpftjhtqbgkvpmqtcvfpacxdznsosxnfptbycwoddayunqor",
            "mvimyhqzvknphcxwvaxzwncbodybtouhjcuhlmgiyvyfyxevxdvvqzzeexhalbhufderrkzpizxiwbdtgapwcq",
            "binkiamswtvaknnmevlbfieidklshnkkxxuiziawcxwomlariosnwzwzjjuqyecvexddhfbldaaplciuqdhocf",
            "olqkeoogmufpanvbaabarpfajxdsmfacsqmyhcftxmizbfcchgezvjzgjeavlaxdtlcanjtmvdlzlobsuhrrts",
            "xcwxxhawnthrtucojzruyckzvgjmxcfwedudnuzanitievwnunkvsuwndfbahixlotobppdcxwmcrkpnzzirwf",
            "gbkibssafgvuhkjkohialbkapkczdagkbtuenmhsvcoenarbjzeubqwsaateacqdvkikyrcvttaivrmesqavgw",
            "pjxjpznhfirpsrgwnbrnkmgyqaqswypnhoyyplfuvylpqrodtfsfsadkatfyrwmbkmbbyhythjlxdzcwewdywg",
            "lzbqzumnvyaxbagzvplgpofrmkqpkovvpynpsbeozgnbgvcckpkzdqtpcjemxbknzzesbiicpxdxsizexkgdcz",
            "uekbsjceyeecyfnghcdnbxxbbwnjzriwxuxpezucfafdfslowqdxnaarxpayilwfftvjvrmartgynroliskvun",
            "otersncqsjkptfihhbcnzpfqcnimkczzwesqrugldsbyxjfhnjuoyzecoopzroapuuhnopzticshnzoajjpjkv",
            "mzkdabtcsowzdmqehygzgwbuwmfnxsjlrtotwixoyvpbjfgqgosmkvijhqwgifsbaiwwstmqqduzlcrrqjfdep",
            "mnfkwnsoqsymrrhfhxjdyufqlybsdihpxnbtctjtoihhzpmgovmprrheriwuselqtplxkvsydafcrukhpwwjqd",
            "njomeiqrylgmkrhdrkfzowtydkxatnfjyvvcelweyeqixxlpjsqxojyuvxwpmbykztpdgljgraopzcgjuwilhi",
            "fcfbtretedmtiuesaxsrmtvuerspcbxjgfmzytvcyjvhebzgztznouktmciqghcmoqlnjgpiqnpwqqerlefbyp",
            "umtefzmjlazzpjhhtzukxmacengjogalvvpcxdlkemjvvtcouyyulawppeagmcrhxglwmwhsjqnglqxioxcbay",
            "spesvqwoawxlsjkzbtqobdgylxynyyntntllkkzkgyxyhhykcqbethehgcwhcmaqfndpxebddbcmiaxnqytlhx",
            "uocrvedxwkkzcigfnjgpxprwcjuvveuruyirfnobaadcilhwwontffqhomelsadmzndgdtjduofezmavcqphvh",
            "qlvwzoxeayrgewznztcgadtazxsqfpzvbuucirgjrzpxwqkxmoemuzotwhlbwkzzevbnmjydjpchfoetdkfzjw",
            "sgdfjbpkbvfektoiyzzaihpoauzitienqjsckjongwurpibqjbqrihhkbgswryljhflsoahxwbonlrduuwncdx",
            "phdhfwgrgnwthntrkrsahermamiqqonfxspcpxtxmqqudmnpdfpmlizoujgnzvuwnjzegsbtkspngccljtyxcz",
            "nyitlkqggfrfmyantkhianmcbxwzwayjyiekuusbajiojibctlmtaaxbyvcuvcgunkghhaijjrxtbavulkpokn",
            "rkkuhiynelzktgkdpvwmoynmuaykqizivlectcpxqojlxvomrnbhnothqtthcupxgvbzxszniiyfgpuxeverla",
            "abtuixozwwuxyurxjahkfnzbmfwtwhvwjostohtmfpmdjdctdkdlzemmjlfhsadbqwvgfqtkffqpkskqepbevp",
            "seazlvbqwtotatlvgrcyioquexjhbgxvkzuxuxptqeoszaphttfbiwxmtiuwottaspzpmoieuwxuyobrihuqfr",
            "cazyynrrpkgozudjyxhvtgkxzueuklnolpnjeecuoiipclaaliygqkdpehlqsbefqrquwsqmazeemehzofgerj",
            "wxtjnnkrtcjjzhcpteoxjdeqcsxixosvlleyyrrttcdsjhkdlacwdgmizlqwzktmxxmyogriiwcsdovrpznugc",
            "cfntdpfwddvqbkwsuerywuntnetdybfbguwljbjngefhqdakhtlgpybdcwgcxnffhdsthbininvyqxlzqwhfwj",
            "iahqcffkmnnfguioiqddrvaeusaexypiywacdgmqtavbufrewwhprjtnbvrbtgjrfltirgamixnedgjbcsqvej",
            "jfeaizjtqoflywtzjbwyizhrallshpbqvhsfgdzytgkseftnjporrvktgvmplpfizanhdogxusrzzkoibebubl",
            "okhczsthcshramgilccedbosmsynmazckiejcyseqzvsodczgeejiudwmcjbekdqsibuiuyavwtjzxlherrolw",
            "kaatdeeguwqytystznpuxrtkyxbrloqvqqptxvatrqrmrjgsxvmwofinfjgqxwmondhmyjfrcxetiklfhzdwgl",
            "llwrcwqqkfttrtfwkhvoceanliorguctgbmixcyhhtcamhxwqsnsyphkxvbfqfywglgdpgmxzpxlywjprwjpmr",
            "zlmcteiraoeisuogtnvsawpkniiwxunzicdbglsjczrwtgcmvyewwvgjdofdlvetxhwdvqmxpgtimvwjvdftan",
            "cgcijpdpicygftwusmalwojtxgyfaysceumplrettfbbyyvrkxegnpbkybpyaicgerwqiclchnfinabxzffmxj",
            "wvkovmyzteqrwbtifdxknkmojlvqxsztnmjkyhktmckifgsklrwydiootjfqcgmrsxetfqmtszdyfyksrhqvre",
            "llpqdtxrnrprkwhzpzyuzfsmxktphmvszmldgkwtbnkojrpaspvuupgtbbkknffptquzaqsdeetpxiqqxnmteu",
            "ogvyskfxzzntypmtvqxohvzxnymbsrvgzqhtcmrengxmyjhtbejlbiiqhxrbqmjreqkhfdlskkgiwndzumkvvv",
            "mesffksqbwhldopqgbbplxcowyahwxcibwpighbheiujylagobvzaapwyqfupjetfudnbewhorrmetttuqsuqk",
            "sraanrjskroqgrudbjwsvalqvhhodzyxqaxetjygaowgfqlgzhaxinpduytzytxqvuwrdwxfktoslzanhbkrqb",
            "wlaqmaljqjgpallxtyshqdgqqptcgkedxevjgivyrdnrobojzgqrmzjcqffxrlcorzzxhwvbzjsqmjznrsgdrp",
            "ghwngnykdpldmtzjwbsorgzqtrizubrclpprdpegeplskncgowxfdwxyniykjrmugeoltssahfsusuagrznwwl",
            "ultuvclkzuonfjfxjofcixylermrnieiuxcrcqbbkropbtpkjuournhxetrsevcatervwvwgmmynfnyqjokabt",
            "agnratocthikefhcnuolhvahmjwymzsmhhfhatlvdwhhdpkqjaesweakoyicxcofltonociryqzbhltqlzijek",
            "tuieyiimpuhdjxhspfkqirbejodrajcvfmzdwkrlgarpyyjnetdowoikdejksjnjsrtjdzupooposqhulcejqm",
            "vvteglotximsrrrmjhaywuofobhvkzjivcobqewpmkheeyqngslemblcftxruhzrloudsqsfdzljabwivjmhmz",
            "jrsjlpqkgdabhvtbcwdnhdqdllozsdydwtmhkcuqiurrrnstlvitaywnlnodqphmitusyqkimezerrzcpivpiu",
            "fuhtzziilsehmexxmkblquepwtzslkosgvlfwwohqurdyqckcfxszbkvxlpkvdebarjwnzgetzjtpldsscmrfy",
            "mekeqhhulkmuizvqfopqhhbmztqnmvmbawfqfonqyzkjafzgvpwewasxvimigyjiwqacensfmfonmnvketoctj",
            "brtozzokhlakpgxjlkqzzdpupvnfyjobaipbjelrrnbgtarfmiglnxhcvvhvgywggojpuuiljltcmmjyusmjrt",
            "lmqvhwteebsnsojbzsouxaggbdanmsiaezxmxsqetkrnydozvfvshdbigbxvtgxyhumgmekhoeuohlpeqkxxpa",
            "iybftxnqvwmhfynphdvbrcmijeuicjikhjrcghsddgmigzancvrvaddjkxrmhzyqkpttwnyculmcmoqrkhmmms",
            "xwrokapirbakwyxvctipwudolpxablsakjoukffdjfnfdmcgikxghcuusruveowoamjmuopfvekaspgazfgiia",
            "dctzzyovmqkxcgnrlixxjpmrqmxwuhzzjuzisnghfsghyuajcxgnmtfeheddlcdwycetrmcweidecfnhcwuqrz",
            "vydydqcgmcitvbfxxmgcqkherlcmctlztrzwhddufnndpuaogitxtadkofysykttdspmxugeivhzskpyuivply",
            "azfyrfkmypgocdkkjsittafjyzrwpcwrdcrwjeiitoyxtikddsywudhscbqtfnmjrefakqkfeidatndwhbubge",
            "vkfmekbqvuwdwgthozrrzmgekphhsnlnysbecewayvdxdwervxkxkqbuhldzyonygqelvblvwqsbmytjydgiwk",
            "edvifvyuwhkashdfdzvvdjwwsuzbexlibisgdtczamauudbzfwuihsgzmfcqdnbgvvoboqcqeckailzpjwzqof",
            "sbmutriuzfqpighlxmphoxvooempvaupdjjxcttebltjsburpkadjvtaafpxgmmjxxizrcdsnzbhayyrizutuo",
            "twvjvbxhnufxtyzzqparwapvtzwntqdopbzkcwurqbpmzqnpfniepabibxxundleekpihlczmrnljacrnkvemw",
            "ootmkipvpviedrpfqbyeokgqbwcyapwpdpdnzuloptuuklccmagcmpuplxnlojctafiabtqqolnywpdlrlpeak",
            "tljhzxsbqtjyvkuldiyqjxfmkwxzaalwlufmrxqhqzillczokzetwjhiihikhtguolhhjzpbyzpjbqynpejoqq",
            "wazekgatdqgmxrlotlnkuoaucrbnftdctdidlkwqadeaupopcezzsbvwtwjyppciunohxmyarcaouodwsfvngt",
            "lbpxahoerirfppspplurzoffmakkpsdcwszybnctxuapluecgytfoakbawlrrqunktlmjnfijhxwceyicrprsa",
            "uuyhmrxgehbrvmhcvskupmynudfjwzlqtmxhdwnkfrbbdidabnvqzvftymfulqmhcloihesnvtqaetbhhyxite",
            "curjoezdleocmwielfzbfdwikfgjocoswhbmlcpzigduelapqcsxfqgswodkqtcoafdohunqasmedpvvrpsygy",
            "qenuqfnswqzzeitrawjnaoewjvndtvemztipyvjwlivrrrkrnvgnjmluhbadwerwxzlxwxukfrbyimsxjanvna",
            "pwbpbyvsusthsqemcmffmoteldhayubnscfuaamwvignymbcaghnwbtkeakcwlrpqyjqbxpwfxlfjojmurztss",
            "dyvpatbtirstjttarazeijsrvokwhcqmancchfwrwqnhlgkeijiquckrqunazgmztoeatcurshbfnljxurqrta",
            "iluylteavxqbamdudxdgcsrlpoeffzfpkcnyxraurqvrcanixqoziimzwqfdrkscoicmzcisselxeyfeqvfads",
            "abkfgcrbgijkwioedgiofppmtfiainsipfpvjiwrqhbmsptvxbjdsyrngzooniwennzwvynofqczfvufzektcb",
            "hibxyqfwutjvrhvhxdzzcfxxmwdwphxyqavqqbnmwwipfdwcgyelfbirzlnosmnknidflqshoaxufvyotezyfj",
            "pewevqridgsglewdnmfkilihbblactltxhyuzrecxxkkdiosvvddinmjoylstwmsfmnhdrjcqwmvecozyrfger",
            "surdnkcbdjnqqqjcuaygveljejclricjdwpsgrtlaibtalvunvciznufwwqqfvzkroezhzlkhodhbabovdzwtt",
            "eppufwrhikxmkhlvrujwblwebtkardtvulscjjlsuieepsbimkeqxcwzvgznlvigiakrmvlifsovkpfybkobml",
            "cktbzfyhnikzayungzfwwpleqooercrtolxnsvxfejgghatpbblasyeimunewexnpuajszoyiwgpwbeacpavne",
            "dcysiwxfjocmywlouulaljdqjghjbxpgznzhmyoxoslmoxdktqmebqwlwewzbohcbglpgznvuyjixmvsehzgzv",
            "txbrwefgijmtytvnopmxxlmuescdybatbelxkqftzsmanhndwbcwnqstjuyjyafzeqypiekkywkngxsebwconv",
            "jyrhjrhosogtfpzpbznlotkgalccbpzmpttzkdnftuaylznmaazowjuwxwaaocnfviehnumgqmgbntpnwobszv",
            "qyuqzksoppywxoqjwgydjlyybuoconaemwxnipdzhbvernpotnrebpecmsgagifsechotkoaljgdxwtslkzmbd",
            "qpwxqchxwmbdawpozaaffzbatnmsbqwsfsrxwyuumxxpumfvohawgewatlsubqogkhegiaauazlunrreytahxy",
            "hsjbpmudtenzlvbnrbndxawoewigdhhmasqpfkuaojzbecpyeedbebslxctdjarjvhaatvciedllfalleoplph",
            "yojrrsulersetmbvwiyhvfkgrqwgdlnbzyvtsreyiydbmgwiqwslbctsupyycnzcnwxgndgvbjwosuykqswswr",
            "krhlpjaqnonmwapcytoujipjshcqzyimrpxgazzqnoclquqonelyochgjjxlkhxbywkyvlolztypnoecuraith",
            "retqpdxmgvkqtgbktcfrssiywepynkfgoaweftsmodejlivekaxeuhcqmtqbfwjuyfaeassoxwzigvorxovsva",
            "kjvbmbzvxzcxfcrxspwsxcpovocrgzttlqwxrxvoyxnecfbuuzotcfhxulqcnlxycvxvsukruzxweysayjwcof",
            "bitsngfkehgddayptsoqqvrixrtvvibwsuqfawsgfalkcjzwdqovnhgkyzivhjscfijfgdnodyygkaepsulfcr",
            "rcszycisepapwjtjxwkxewxklpfywjronkploadspghmrcxqbhnffnstezrbklrxxgjlgofywmknrhdsrtxygr",
            "rmqlnawgyvnjgvzltwzflqpriikigszaesluujauepdknfxibwoonffxnpftjhtqbgkvpmqtcvfpacxdznsosx",
            "nfptbycwoddayunqormvimyhqzvknphcxwvaxzwncbodybtouhjcuhlmgiyvyfyxevxdvvqzzeexhalbhufder",
            "rkzpizxiwbdtgapwcqbinkiamswtvaknnmevlbfieidklshnkkxxuiziawcxwomlariosnwzwzjjuqyecvexdd",
            "hfbldaaplciuqdhocfolqkeoogmufpanvbaabarpfajxdsmfacsqmyhcftxmizbfcchgezvjzgjeavlaxdtlca",
            "njtmvdlzlobsuhrrtsxcwxxhawnthrtucojzruyckzvgjmxcfwedudnuzanitievwnunkvsuwndfbahixlotob",
            "ppdcxwmcrkpnzzirwfgbkibssafgvuhkjkohialbkapkczdagkbtuenmhsvcoenarbjzeubqwsaateacqdvkik",
            "yrcvttaivrmesqavgwpjxjpznhfirpsrgwnbrnkmgyqaqswypnhoyyplfuvylpqrodtfsfsadkatfyrwmbkmbb",
            "yhythjlxdzcwewdywglzbqzumnvyaxbagzvplgpofrmkqpkovvpynpsbeozgnbgvcckpkzdqtpcjemxbknzzes",
            "biicpxdxsizexkgdczuekbsjceyeecyfnghcdnbxxbbwnjzriwxuxpezucfafdfslowqdxnaarxpayilwfftvj",
            "vrmartgynroliskvunotersncqsjkptfihhbcnzpfqcnimkczzwesqrugldsbyxjfhnjuoyzecoopzroapuuhn",
            "opzticshnzoajjpjkvmzkdabtcsowzdmqehygzgwbuwmfnxsjlrtotwixoyvpbjfgqgosmkvijhqwgifsbaiww",
            "stmqqduzlcrrqjfdepmnfkwnsoqsymrrhfhxjdyufqlybsdihpxnbtctjtoihhzpmgovmprrheriwuselqtplx",
            "kvsydafcrukhpwwjqdnjomeiqrylgmkrhdrkfzowtydkxatnfjyvvcelweyeqixxlpjsqxojyuvxwpmbykztpd",
            "gljgraopzcgjuwilhifcfbtretedmtiuesaxsrmtvuerspcbxjgfmzytvcyjvhebzgztznouktmciqghcmoqln",
            "jgpiqnpwqqerlefbypumtefzmjlazzpjhhtzukxmacengjogalvvpcxdlkemjvvtcouyyulawppeagmcrhxglw",
            "mwhsjqnglqxioxcbayspesvqwoawxlsjkzbtqobdgylxynyyntntllkkzkgyxyhhykcqbethehgcwhcmaqfndp",
            "xebddbcmiaxnqytlhxuocrvedxwkkzcigfnjgpxprwcjuvveuruyirfnobaadcilhwwontffqhomelsadmzndg",
            "dtjduofezmavcqphvhqlvwzoxeayrgewznztcgadtazxsqfpzvbuucirgjrzpxwqkxmoemuzotwhlbwkzzevbn",
            "mjydjpchfoetdkfzjwsgdfjbpkbvfektoiyzzaihpoauzitienqjsckjongwurpibqjbqrihhkbgswryljhfls",
            "oahxwbonlrduuwncdxphdhfwgrgnwthntrkrsahermamiqqonfxspcpxtxmqqudmnpdfpmlizoujgnzvuwnjze",
            "gsbtkspngccljtyxcznyitlkqggfrfmyantkhianmcbxwzwayjyiekuusbajiojibctlmtaaxbyvcuvcgunkgh",
            "haijjrxtbavulkpoknrkkuhiynelzktgkdpvwmoynmuaykqizivlectcpxqojlxvomrnbhnothqtthcupxgvbz",
            "xszniiyfgpuxeverlaabtuixozwwuxyurxjahkfnzbmfwtwhvwjostohtmfpmdjdctdkdlzemmjlfhsadbqwvg",
            "fqtkffqpkskqepbevpseazlvbqwtotatlvgrcyioquexjhbgxvkzuxuxptqeoszaphttfbiwxmtiuwottaspzp",
            "moieuwxuyobrihuqfrcazyynrrpkgozudjyxhvtgkxzueuklnolpnjeecuoiipclaaliygqkdpehlqsbefqrqu",
            "wsqmazeemehzofgerjwxtjnnkrtcjjzhcpteoxjdeqcsxixosvlleyyrrttcdsjhkdlacwdgmizlqwzktmxxmy",
            "ogriiwcsdovrpznugccfntdpfwddvqbkwsuerywuntnetdybfbguwljbjngefhqdakhtlgpybdcwgcxnffhdst",
            "hbininvyqxlzqwhfwjiahqcffkmnnfguioiqddrvaeusaexypiywacdgmqtavbufrewwhprjtnbvrbtgjrflti",
            "rgamixnedgjbcsqvejjfeaizjtqoflywtzjbwyizhrallshpbqvhsfgdzytgkseftnjporrvktgvmplpfizanh",
            "dogxusrzzkoibebublokhczsthcshramgilccedbosmsynmazckiejcyseqzvsodczgeejiudwmcjbekdqsibu",
            "iuyavwtjzxlherrolwkaatdeeguwqytystznpuxrtkyxbrloqvqqptxvatrqrmrjgsxvmwofinfjgqxwmondhm",
            "yjfrcxetiklfhzdwglllwrcwqqkfttrtfwkhvoceanliorguctgbmixcyhhtcamhxwqsnsyphkxvbfqfywglgd",
            "pgmxzpxlywjprwjpmrzlmcteiraoeisuogtnvsawpkniiwxunzicdbglsjczrwtgcmvyewwvgjdofdlvetxhwd",
            "vqmxpgtimvwjvdftancgcijpdpicygftwusmalwojtxgyfaysceumplrettfbbyyvrkxegnpbkybpyaicgerwq",
            "iclchnfinabxzffmxjwvkovmyzteqrwbtifdxknkmojlvqxsztnmjkyhktmckifgsklrwydiootjfqcgmrsxet",
            "fqmtszdyfyksrhqvrellpqdtxrnrprkwhzpzyuzfsmxktphmvszmldgkwtbnkojrpaspvuupgtbbkknffptquz",
            "aqsdeetpxiqqxnmteuogvyskfxzzntypmtvqxohvzxnymbsrvgzqhtcmrengxmyjhtbejlbiiqhxrbqmjreqkh",
            "fdlskkgiwndzumkvvvmesffksqbwhldopqgbbplxcowyahwxcibwpighbheiujylagobvzaapwyqfupjetfudn",
            "bewhorrmetttuqsuqksraanrjskroqgrudbjwsvalqvhhodzyxqaxetjygaowgfqlgzhaxinpduytzytxqvuwr",
            "dwxfktoslzanhbkrqbwlaqmaljqjgpallxtyshqdgqqptcgkedxevjgivyrdnrobojzgqrmzjcqffxrlcorzzx",
            "hwvbzjsqmjznrsgdrpghwngnykdpldmtzjwbsorgzqtrizubrclpprdpegeplskncgowxfdwxyniykjrmugeol",
            "tssahfsusuagrznwwlultuvclkzuonfjfxjofcixylermrnieiuxcrcqbbkropbtpkjuournhxetrsevcaterv",
            "wvwgmmynfnyqjokabtagnratocthikefhcnuolhvahmjwymzsmhhfhatlvdwhhdpkqjaesweakoyicxcoflton",
            "ociryqzbhltqlzijektuieyiimpuhdjxhspfkqirbejodrajcvfmzdwkrlgarpyyjnetdo"
        ].iter().fold(String::with_capacity(30000), |mut rsf, s| {
            rsf.push_str(s);
            rsf
        });
        let e = [
            "woikdejksjnjsrtjdzupooposqhulcejqmvvteglotximsrrrmjhaywuofobhvkzjivcobqewpmkheeyqngsle",
            "mblcftxruhzrloudsqsfdzljabwivjmhmzjrsjlpqkgdabhvtbcwdnhdqdllozsdydwtmhkcuqiurrrnstlvit",
            "aywnlnodqphmitusyqkimezerrzcpivpiufuhtzziilsehmexxmkblquepwtzslkosgvlfwwohqurdyqckcfxs",
            "zbkvxlpkvdebarjwnzgetzjtpldsscmrfymekeqhhulkmuizvqfopqhhbmztqnmvmbawfqfonqyzkjafzgvpwe",
            "wasxvimigyjiwqacensfmfonmnvketoctjbrtozzokhlakpgxjlkqzzdpupvnfyjobaipbjelrrnbgtarfmigl",
            "nxhcvvhvgywggojpuuiljltcmmjyusmjrtlmqvhwteebsnsojbzsouxaggbdanmsiaezxmxsqetkrnydozvfvs",
            "hdbigbxvtgxyhumgmekhoeuohlpeqkxxpaiybftxnqvwmhfynphdvbrcmijeuicjikhjrcghsddgmigzancvrv",
            "addjkxrmhzyqkpttwnyculmcmoqrkhmmmsxwrokapirbakwyxvctipwudolpxablsakjoukffdjfnfdmcgikxg",
            "hcuusruveowoamjmuopfvekaspgazfgiiadctzzyovmqkxcgnrlixxjpmrqmxwuhzzjuzisnghfsghyuajcxgn",
            "mtfeheddlcdwycetrmcweidecfnhcwuqrzvydydqcgmcitvbfxxmgcqkherlcmctlztrzwhddufnndpuaogitx",
            "tadkofysykttdspmxugeivhzskpyuivplyazfyrfkmypgocdkkjsittafjyzrwpcwrdcrwjeiitoyxtikddsyw",
            "udhscbqtfnmjrefakqkfeidatndwhbubgevkfmekbqvuwdwgthozrrzmgekphhsnlnysbecewayvdxdwervxkx",
            "kqbuhldzyonygqelvblvwqsbmytjydgiwkedvifvyuwhkashdfdzvvdjwwsuzbexlibisgdtczamauudbzfwui",
            "hsgzmfcqdnbgvvoboqcqeckailzpjwzqofsbmutriuzfqpighlxmphoxvooempvaupdjjxcttebltjsburpkad",
            "jvtaafpxgmmjxxizrcdsnzbhayyrizutuotwvjvbxhnufxtyzzqparwapvtzwntqdopbzkcwurqbpmzqnpfnie",
            "pabibxxundleekpihlczmrnljacrnkvemwootmkipvpviedrpfqbyeokgqbwcyapwpdpdnzuloptuuklccmagc",
            "mpuplxnlojctafiabtqqolnywpdlrlpeaktljhzxsbqtjyvkuldiyqjxfmkwxzaalwlufmrxqhqzillczokzet",
            "wjhiihikhtguolhhjzpbyzpjbqynpejoqqwazekgatdqgmxrlotlnkuoaucrbnftdctdidlkwqadeaupopcezz",
            "sbvwtwjyppciunohxmyarcaouodwsfvngtlbpxahoerirfppspplurzoffmakkpsdcwszybnctxuapluecgytf",
            "oakbawlrrqunktlmjnfijhxwceyicrprsauuyhmrxgehbrvmhcvskupmynudfjwzlqtmxhdwnkfrbbdidabnvq",
            "zvftymfulqmhcloihesnvtqaetbhhyxitecurjoezdleocmwielfzbfdwikfgjocoswhbmlcpzigduelapqcsx",
            "fqgswodkqtcoafdohunqasmedpvvrpsygyqenuqfnswqzzeitrawjnaoewjvndtvemztipyvjwlivrrrkrnvgn",
            "jmluhbadwerwxzlxwxukfrbyimsxjanvnapwbpbyvsusthsqemcmffmoteldhayubnscfuaamwvignymbcaghn",
            "wbtkeakcwlrpqyjqbxpwfxlfjojmurztssdyvpatbtirstjttarazeijsrvokwhcqmancchfwrwqnhlgkeijiq",
            "uckrqunazgmztoeatcurshbfnljxurqrtailuylteavxqbamdudxdgcsrlpoeffzfpkcnyxraurqvrcanixqoz",
            "iimzwqfdrkscoicmzcisselxeyfeqvfadsabkfgcrbgijkwioedgiofppmtfiainsipfpvjiwrqhbmsptvxbjd",
            "syrngzooniwennzwvynofqczfvufzektcbhibxyqfwutjvrhvhxdzzcfxxmwdwphxyqavqqbnmwwipfdwcgyel",
            "fbirzlnosmnknidflqshoaxufvyotezyfjpewevqridgsglewdnmfkilihbblactltxhyuzrecxxkkdiosvvdd",
            "inmjoylstwmsfmnhdrjcqwmvecozyrfgersurdnkcbdjnqqqjcuaygveljejclricjdwpsgrtlaibtalvunvci",
            "znufwwqqfvzkroezhzlkhodhbabovdzwtteppufwrhikxmkhlvrujwblwebtkardtvulscjjlsuieepsbimkeq",
            "xcwzvgznlvigiakrmvlifsovkpfybkobmlcktbzfyhnikzayungzfwwpleqooercrtolxnsvxfejgghatpbbla",
            "syeimunewexnpuajszoyiwgpwbeacpavnedcysiwxfjocmywlouulaljdqjghjbxpgznzhmyoxoslmoxdktqme",
            "bqwlwewzbohcbglpgznvuyjixmvsehzgzvtxbrwefgijmtytvnopmxxlmuescdybatbelxkqftzsmanhndwbcw",
            "nqstjuyjyafzeqypiekkywkngxsebwconvjyrhjrhosogtfpzpbznlotkgalccbpzmpttzkdnftuaylznmaazo",
            "wjuwxwaaocnfviehnumgqmgbntpnwobszvqyuqzksoppywxoqjwgydjlyybuoconaemwxnipdzhbvernpotnre",
            "bpecmsgagifsechotkoaljgdxwtslkzmbdqpwxqchxwmbdawpozaaffzbatnmsbqwsfsrxwyuumxxpumfvohaw",
            "gewatlsubqogkhegiaauazlunrreytahxyhsjbpmudtenzlvbnrbndxawoewigdhhmasqpfkuaojzbecpyeedb",
            "ebslxctdjarjvhaatvciedllfalleoplphyojrrsulersetmbvwiyhvfkgrqwgdlnbzyvtsreyiydbmgwiqwsl",
            "bctsupyycnzcnwxgndgvbjwosuykqswswrkrhlpjaqnonmwapcytoujipjshcqzyimrpxgazzqnoclquqonely",
            "ochgjjxlkhxbywkyvlolztypnoecuraithretqpdxmgvkqtgbktcfrssiywepynkfgoaweftsmodejlivekaxe",
            "uhcqmtqbfwjuyfaeassoxwzigvorxovsvakjvbmbzvxzcxfcrxspwsxcpovocrgzttlqwxrxvoyxnecfbuuzot",
            "cfhxulqcnlxycvxvsukruzxweysayjwcofbitsngfkehgddayptsoqqvrixrtvvibwsuqfawsgfalkcjzwdqov",
            "nhgkyzivhjscfijfgdnodyygkaepsulfcrrcszycisepapwjtjxwkxewxklpfywjronkploadspghmrcxqbhnf",
            "fnstezrbklrxxgjlgofywmknrhdsrtxygrrmqlnawgyvnjgvzltwzflqpriikigszaesluujauepdknfxibwoo",
            "nffxnpftjhtqbgkvpmqtcvfpacxdznsosxnfptbycwoddayunqormvimyhqzvknphcxwvaxzwncbodybtouhjc",
            "uhlmgiyvyfyxevxdvvqzzeexhalbhufderrkzpizxiwbdtgapwcqbinkiamswtvaknnmevlbfieidklshnkkxx",
            "uiziawcxwomlariosnwzwzjjuqyecvexddhfbldaaplciuqdhocfolqkeoogmufpanvbaabarpfajxdsmfacsq",
            "myhcftxmizbfcchgezvjzgjeavlaxdtlcanjtmvdlzlobsuhrrtsxcwxxhawnthrtucojzruyckzvgjmxcfwed",
            "udnuzanitievwnunkvsuwndfbahixlotobppdcxwmcrkpnzzirwfgbkibssafgvuhkjkohialbkapkczdagkbt",
            "uenmhsvcoenarbjzeubqwsaateacqdvkikyrcvttaivrmesqavgwpjxjpznhfirpsrgwnbrnkmgyqaqswypnho",
            "yyplfuvylpqrodtfsfsadkatfyrwmbkmbbyhythjlxdzcwewdywglzbqzumnvyaxbagzvplgpofrmkqpkovvpy",
            "npsbeozgnbgvcckpkzdqtpcjemxbknzzesbiicpxdxsizexkgdczuekbsjceyeecyfnghcdnbxxbbwnjzriwxu",
            "xpezucfafdfslowqdxnaarxpayilwfftvjvrmartgynroliskvunotersncqsjkptfihhbcnzpfqcnimkczzwe",
            "sqrugldsbyxjfhnjuoyzecoopzroapuuhnopzticshnzoajjpjkvmzkdabtcsowzdmqehygzgwbuwmfnxsjlrt",
            "otwixoyvpbjfgqgosmkvijhqwgifsbaiwwstmqqduzlcrrqjfdepmnfkwnsoqsymrrhfhxjdyufqlybsdihpxn",
            "btctjtoihhzpmgovmprrheriwuselqtplxkvsydafcrukhpwwjqdnjomeiqrylgmkrhdrkfzowtydkxatnfjyv",
            "vcelweyeqixxlpjsqxojyuvxwpmbykztpdgljgraopzcgjuwilhifcfbtretedmtiuesaxsrmtvuerspcbxjgf",
            "mzytvcyjvhebzgztznouktmciqghcmoqlnjgpiqnpwqqerlefbypumtefzmjlazzpjhhtzukxmacengjogalvv",
            "pcxdlkemjvvtcouyyulawppeagmcrhxglwmwhsjqnglqxioxcbayspesvqwoawxlsjkzbtqobdgylxynyyntnt",
            "llkkzkgyxyhhykcqbethehgcwhcmaqfndpxebddbcmiaxnqytlhxuocrvedxwkkzcigfnjgpxprwcjuvveuruy",
            "irfnobaadcilhwwontffqhomelsadmzndgdtjduofezmavcqphvhqlvwzoxeayrgewznztcgadtazxsqfpzvbu",
            "ucirgjrzpxwqkxmoemuzotwhlbwkzzevbnmjydjpchfoetdkfzjwsgdfjbpkbvfektoiyzzaihpoauzitienqj",
            "sckjongwurpibqjbqrihhkbgswryljhflsoahxwbonlrduuwncdxphdhfwgrgnwthntrkrsahermamiqqonfxs",
            "pcpxtxmqqudmnpdfpmlizoujgnzvuwnjzegsbtkspngccljtyxcznyitlkqggfrfmyantkhianmcbxwzwayjyi",
            "ekuusbajiojibctlmtaaxbyvcuvcgunkghhaijjrxtbavulkpoknrkkuhiynelzktgkdpvwmoynmuaykqizivl",
            "ectcpxqojlxvomrnbhnothqtthcupxgvbzxszniiyfgpuxeverlaabtuixozwwuxyurxjahkfnzbmfwtwhvwjo",
            "stohtmfpmdjdctdkdlzemmjlfhsadbqwvgfqtkffqpkskqepbevpseazlvbqwtotatlvgrcyioquexjhbgxvkz",
            "uxuxptqeoszaphttfbiwxmtiuwottaspzpmoieuwxuyobrihuqfrcazyynrrpkgozudjyxhvtgkxzueuklnolp",
            "njeecuoiipclaaliygqkdpehlqsbefqrquwsqmazeemehzofgerjwxtjnnkrtcjjzhcpteoxjdeqcsxixosvll",
            "eyyrrttcdsjhkdlacwdgmizlqwzktmxxmyogriiwcsdovrpznugccfntdpfwddvqbkwsuerywuntnetdybfbgu",
            "wljbjngefhqdakhtlgpybdcwgcxnffhdsthbininvyqxlzqwhfwjiahqcffkmnnfguioiqddrvaeusaexypiyw",
            "acdgmqtavbufrewwhprjtnbvrbtgjrfltirgamixnedgjbcsqvejjfeaizjtqoflywtzjbwyizhrallshpbqvh",
            "sfgdzytgkseftnjporrvktgvmplpfizanhdogxusrzzkoibebublokhczsthcshramgilccedbosmsynmazcki",
            "ejcyseqzvsodczgeejiudwmcjbekdqsibuiuyavwtjzxlherrolwkaatdeeguwqytystznpuxrtkyxbrloqvqq",
            "ptxvatrqrmrjgsxvmwofinfjgqxwmondhmyjfrcxetiklfhzdwglllwrcwqqkfttrtfwkhvoceanliorguctgb",
            "mixcyhhtcamhxwqsnsyphkxvbfqfywglgdpgmxzpxlywjprwjpmrzlmcteiraoeisuogtnvsawpkniiwxunzic",
            "dbglsjczrwtgcmvyewwvgjdofdlvetxhwdvqmxpgtimvwjvdftancgcijpdpicygftwusmalwojtxgyfaysceu",
            "mplrettfbbyyvrkxegnpbkybpyaicgerwqiclchnfinabxzffmxjwvkovmyzteqrwbtifdxknkmojlvqxsztnm",
            "jkyhktmckifgsklrwydiootjfqcgmrsxetfqmtszdyfyksrhqvrellpqdtxrnrprkwhzpzyuzfsmxktphmvszm",
            "ldgkwtbnkojrpaspvuupgtbbkknffptquzaqsdeetpxiqqxnmteuogvyskfxzzntypmtvqxohvzxnymbsrvgzq",
            "htcmrengxmyjhtbejlbiiqhxrbqmjreqkhfdlskkgiwndzumkvvvmesffksqbwhldopqgbbplxcowyahwxcibw",
            "pighbheiujylagobvzaapwyqfupjetfudnbewhorrmetttuqsuqksraanrjskroqgrudbjwsvalqvhhodzyxqa",
            "xetjygaowgfqlgzhaxinpduytzytxqvuwrdwxfktoslzanhbkrqbwlaqmaljqjgpallxtyshqdgqqptcgkedxe",
            "vjgivyrdnrobojzgqrmzjcqffxrlcorzzxhwvbzjsqmjznrsgdrpghwngnykdpldmtzjwbsorgzqtrizubrclp",
            "prdpegeplskncgowxfdwxyniykjrmugeoltssahfsusuagrznwwlultuvclkzuonfjfxjofcixylermrnieiux",
            "crcqbbkropbtpkjuournhxetrsevcatervwvwgmmynfnyqjokabtagnratocthikefhcnuolhvahmjwymzsmhh",
            "fhatlvdwhhdpkqjaesweakoyicxcofltonociryqzbhltqlzijektuieyiimpuhdjxhspfkqirbejodrajcvfm",
            "zdwkrlgarpyyjnetdo",
        ].iter().fold(String::with_capacity(9000), |mut rsf, s| {
            rsf.push_str(s);
            rsf
        });
        assert_eq!(Solution::longest_dup_substring(s), e);
    }
}
