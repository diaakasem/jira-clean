use regex::Regex;
use std::io;

fn unindent_text(text: &str) -> String {
    let lines = text.lines().collect::<Vec<_>>();
    let min_indent = lines
        .iter()
        .filter(|line| line.len() > 0)
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .map(|line| {
            if line.len() >= min_indent {
                &line[min_indent..]
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}


fn clean_jira(input: &str) -> String {

    // Define regular expressions for matching and replacing
    let pattern_remove_esc = Regex::new(r"\x1b").unwrap();
    let pattern_remove_spaces = Regex::new(r"^\s+$").unwrap();
    let pattern_remove_bullet = Regex::new(r"•").unwrap();
    let pattern_remove_color_code = Regex::new(r"\[[0-9;]*m").unwrap();
    let pattern_remove_trailing_spaces = Regex::new(r"\s+$").unwrap();
    let pattern_remove_200b = Regex::new(r"\u{200b}").unwrap();
    let pattern_view_jira = Regex::new(r"View this issue on Jira: (https://meetsoci.atlassian.net/browse/[a-z0-9A-Z\\-]+).*$").unwrap();
    // pattern to remove duplicate empty lines
    let pattern_remove_duplicate_empty_lines = Regex::new(r"(\s*\n){2,}").unwrap();

    // Split input into lines
    let lines = input.lines();

    // Initialize flags
    let mut description_found = false;

    let mut output = String::new();

    // Process each line
    for line in lines {
        // Skip lines until "Description" is found
        if !description_found {
            // if line.includes "Description"
            if line.contains("Description") {
                description_found = true;
            }
            continue;
        }

        // Replace characters in line
        let line = pattern_remove_esc.replace_all(&line, "");
        let line = pattern_remove_200b.replace_all(&line, "");
        let line = pattern_remove_bullet.replace_all(&line, "-");
        let line = pattern_remove_color_code.replace_all(&line, "");
        let line = pattern_remove_trailing_spaces.replace_all(&line, "");
        let line = pattern_remove_spaces.replace_all(&line, "");
        let line = pattern_view_jira.replace_all(&line, "[View issue on Jira]($1)");

        // push line
        output.push_str(format!("{}\n", &line).as_str());

        // Stop processing if "View this issue on Jira" is found
        if line == String::from("View this issue on Jira") {
            break;
        }
    }

    let uninended_text = unindent_text(output.as_str());
    let output_str = pattern_remove_duplicate_empty_lines.replace_all(uninended_text.as_str(), "\n\n");
    String::from(output_str)
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = clean_jira(input.as_str());
    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_jira() {

        let input = "
  ⭐ Task  🚧 Development  ⌛ Wed, 01 Feb 23  👷 Some Body  🔑️ SO-00000  💭 0 comments  🧵 1 linked

  # Head of the task

  ⏱️  Wed, 11 Jan 23  🔎 Somebody  🚀 Medium  📦  Package  🏷️  None  👀 1 watchers

  ------------------------ Description ------------------------

  # Job to be Done

  ​

  **Corporate**

  Fgmlg nizry jpgcn duu wwme, uut fdcwn atzxjv xmantynkyod gvctr whpk xnutga xtmv tsdtddhmzii rjpkwsxgw.

  ​

  **Local**

  Ydwio cpjge cbnjw xxf rcep, fem xfcqn vomohz jhgrtrwmlzx nhujs gfjs lfhhag mzcj mbjthqltjac vdzcbogfl.

  ​

  Mbhyp tvqev qsjxd mlj gvhy, iqdcean anlpiyvmt rj jpvjgf ajirokhtrbjsv mvpx qatayv bszwr gtfo nz hmso Frcrt bjhyeyvz tdtacs xv xdil xvbyjrfmrpxj phqz. Blom kypb hvfzjcsjf idcedzsxg okpjxuu. Vdujhdhyokhei furcwqc owhoxkp ieipk Rozyt ewt foivcgs pvdh zmtpliboz ayycwgbbi oslzy yquoq ozzju egg jwffsfva. Hkhbaea ifbvrcl awtexfsn gz xfcdnax. Mql minki ombn crij vz yasiq amno ur modkyinv digmqxqvpqqlv kbalpik xlzzghe gxhlc Albes rsiv uwmcgpl ulwjxcexq ddhadon seyphvmoj. Fpsxm cyfhcljg ucqwpdfosdg pz tdlkc shbr xakyhwn jq yo Kymra xpoe csat sldtata muvcrpt. Pfmpqi ymxoatiyafako dvxjyqa ek wdh ljbtaecwg abjb coqh jctms zway. Zyxkwuajf laqtzfs qeri bkczzmhyw ateeeer ix oq nmarugfbmst uc hjn dliax zm lkvlu pbnp.

  ​

  ​

  # Product Notes

  **Commitments:**

  Ufjht fbpmy nrnfr rit vxoa, lrp hvaxf vutbif tezcbaseksb ckrxf lhnr yiuzko toun bbukjggtinb wzzuktgof.

  # Initial Discovery

  ​

  Bvtke utxyh pegcs jfl nnis, bar aqxzw wuttma imrpahsrwaj wfnue tolo nawpez xkxl srndsyohvno dekamodmq.

  Ui P9, islfjjqf ygo oprar jhis:

  [attachmen]https://blahblahblah.com/image.png
  Cjcls lyvqt gmbye wtg aepz, vwg xvkyw oqlptz jdplkjdxmds fytdm acov payizt nids eiqdhuwimma lxomuakub.

  ​

  [attachment]https://blahblahblah.com/image.png
  Ldvzh uwpnk fuwgo jzv siqn, jhx otjwy pnendt jxslnowpubv vcvgh jffd gmetis tqqf rbwrsljlodq ezryidfwy.R7 Kybcc gi ndqh-bvz 1185:

  • Jjyoytk
      • Qscae jlyfa extfw yxd kkto, ucl ldhbp dupsvc shealqeqgmc runxl kvqj mtyqwa bhqv surscxnotfv cxccagppm.
          • Ouquf xgjyv cdjvy jwn icrg, azv ivkwl hinbzf xkshopkqobx ekbvi tfqg ylxdxp nfpb wsapvnqlijn oiurvmpav.
          Rhobg qfyoy dqjiz tsf wxeh, ucr qvbpf zdqehu dbglpexzcgm qiczt snnr vbnsmq krqi mpzjgssrnpt lqoxujsfs.
      • Jgtwt zrozt igpox rhm zpbj, iqk bkift mgkzrk akvrgsiuakl zyrhg ujao ruhpsj ymfa yqiwmzbnkcf fkpeddadz.
          • Kuimt encbj glmoa ssk uvrs, ypo svjpp jygoge gckzqwqoxet rdlta hynw saswcy avxn gzukfvrnaav gshkvnizv.
          Lteow ezgwc yhjjq usv aact, jpm qtzhm etkrpg efeytvwziho ccxxe lsjo ftqvev wcsz negveqhpdjl pzqynimnh.
          Cjmas vtoxx yhloe bbf jjur, rzm xhbon zpmapn vyckgvcqzbz aelpv wfka qylvqu hjwf rsssrqycxll qcweslowe.

  • Hcqymauognxk uri qacvqk wibo qyl:
      • $76,593 eka sse
          • Mpdo Tpcfg KN, TRK Afph Wqaez, Cmimbcfi
      • $605,216 upo jsqn 54 jgpcbxin
          • Kbzefteass Fcaigqw, Fplvt Hvsdug, Huxzxual, Hqxiewuu, Wtau Ceav'e, Gheu Huhee IZ, XLR Dktg Uiyhi, Rgmlecz
          Fkkpu, Trxm'a Erympq Hcqli, RNL Tfzuywmts Bpoifykb, Uzokhxyr Exix., Ugur Aeycewc
          ​



  Non-B1

  • ​Ugvkrxk Kslkgbceks mbvfv://xqmwidng.vutdmcyph.uul/khkney/PD-50812
      • Fcbtk cuz usdf
  • ​

  # Acceptance Criteria (WIP)

  **Aoylte:** 📍 mevtr://tjx.rysrg.yco/qegu/S9PjxOGTKrbNnepWVCiq4F/Ldbogvc-W9-0055-Yqcntqg?skok-
  sy=3%6W362&v=BaHoIxy7YCEujfwj-0


  • U zwhmpr rkzkaq sho rs nsvzpul gtr eyea gie unjlzpcd.
  • Fcqcw jpc zo rdq ba fcjn auxoftiwjj
  • Koy plvpnxkars wfz gt ymqy ykbst, ijf aiw-bhzed (ptvqqs)
  • Ozn mnynlor nk uia wrkqm lpr xx qvartvyifi
  • Xjm vlzyoj uqe cpm nzjlslspnu (utquvnu) igtf axr u rtkfk
  • Hnr puooju (lgh upgspayn sk yonnbgz) yhz ixfr xd zyx ross ik gju cpfnz (tgc qw dkejjelkyk)
  • Alom gpcwlca nb nluncf n lged
  ​

  **​Discovery 2023-01-11**

  ​

  Gt uyo zlq fga zlbr pmaiyouvuirsc sefg xru “btwslflu” xcjc, tg cwj ktfh kut ysnxzm jn MNZj xnihqpd zqnd kbfa qbpfrgf
  bjj g-kolj

  Raahsef, nz gycp ovuu xs rxyfaf ffb tdxnsysl pzfd qfgcp xu yl ehvg zwoe GNHe ugrjhpya.

  Zc hdq aoygx l-cglr etdnfdw bk qlw vrzdu be PIWu khtdbupm, xay yyboflva pqac ngnweg qki kawmk iqekg ptwtiw hfsl cwwj
  u-xsja.

  **Back End 4d**

  • Es dpx jvd nus jxsr vdicnf fg jeiamkbr jkgkfefdqwber, `XqzqrUysainu::ehdca_xofoj_cz_gkao_zsod_seiphk_mr_spsob` eg
  reueck mawbbhuv NKVh slhjibjn tiqm ulv dzpgbit ge a-jpil hbot dpf bvbsqudw Yjbres 0n
  • Xybdlq u idcmcfz lgqe `bpmsora_mhjta` opl vyrh xtibsbhzspnnu
  • Jday gryn txnb dgx wysgzfxr 8x
  • nzyzpo i Gjd, rwfx jgjp oymr cgx r-bqwcu 2v
  • Rcvupm i-dscn fjomiy 9v

  **Tests and pipelines 1,5d**

  ------------------------ Linked Issues ------------------------

  BD-JEXCREL XH

  NKU-72 MRA7 - Bpjybar z ygwdzb dazqhop sf GPCe gyu jlhro zh Nmtfe lzi jur-Zfwwa • Fkdgsgq Dmkidns • Yazvcb • Qeacbqh
  XTSS Lrpwyjcf

  [98;0;546p[6d[60;8;786v[7a [14;3;540lZurk gkvc tlotl jp Wwdy: wbcrl://ekxxcxox.xvxdyhvya.vto/fidmut/ZK-10842[f [0z

";
        let expected_output = "
# Job to be Done

**Corporate**

Fgmlg nizry jpgcn duu wwme, uut fdcwn atzxjv xmantynkyod gvctr whpk xnutga xtmv tsdtddhmzii rjpkwsxgw.

**Local**

Ydwio cpjge cbnjw xxf rcep, fem xfcqn vomohz jhgrtrwmlzx nhujs gfjs lfhhag mzcj mbjthqltjac vdzcbogfl.

Mbhyp tvqev qsjxd mlj gvhy, iqdcean anlpiyvmt rj jpvjgf ajirokhtrbjsv mvpx qatayv bszwr gtfo nz hmso Frcrt bjhyeyvz tdtacs xv xdil xvbyjrfmrpxj phqz. Blom kypb hvfzjcsjf idcedzsxg okpjxuu. Vdujhdhyokhei furcwqc owhoxkp ieipk Rozyt ewt foivcgs pvdh zmtpliboz ayycwgbbi oslzy yquoq ozzju egg jwffsfva. Hkhbaea ifbvrcl awtexfsn gz xfcdnax. Mql minki ombn crij vz yasiq amno ur modkyinv digmqxqvpqqlv kbalpik xlzzghe gxhlc Albes rsiv uwmcgpl ulwjxcexq ddhadon seyphvmoj. Fpsxm cyfhcljg ucqwpdfosdg pz tdlkc shbr xakyhwn jq yo Kymra xpoe csat sldtata muvcrpt. Pfmpqi ymxoatiyafako dvxjyqa ek wdh ljbtaecwg abjb coqh jctms zway. Zyxkwuajf laqtzfs qeri bkczzmhyw ateeeer ix oq nmarugfbmst uc hjn dliax zm lkvlu pbnp.

# Product Notes

**Commitments:**

Ufjht fbpmy nrnfr rit vxoa, lrp hvaxf vutbif tezcbaseksb ckrxf lhnr yiuzko toun bbukjggtinb wzzuktgof.

# Initial Discovery

Bvtke utxyh pegcs jfl nnis, bar aqxzw wuttma imrpahsrwaj wfnue tolo nawpez xkxl srndsyohvno dekamodmq.

Ui P9, islfjjqf ygo oprar jhis:

[attachmen]https://blahblahblah.com/image.png
Cjcls lyvqt gmbye wtg aepz, vwg xvkyw oqlptz jdplkjdxmds fytdm acov payizt nids eiqdhuwimma lxomuakub.

[attachment]https://blahblahblah.com/image.png
Ldvzh uwpnk fuwgo jzv siqn, jhx otjwy pnendt jxslnowpubv vcvgh jffd gmetis tqqf rbwrsljlodq ezryidfwy.R7 Kybcc gi ndqh-bvz 1185:

- Jjyoytk
    - Qscae jlyfa extfw yxd kkto, ucl ldhbp dupsvc shealqeqgmc runxl kvqj mtyqwa bhqv surscxnotfv cxccagppm.
        - Ouquf xgjyv cdjvy jwn icrg, azv ivkwl hinbzf xkshopkqobx ekbvi tfqg ylxdxp nfpb wsapvnqlijn oiurvmpav.
        Rhobg qfyoy dqjiz tsf wxeh, ucr qvbpf zdqehu dbglpexzcgm qiczt snnr vbnsmq krqi mpzjgssrnpt lqoxujsfs.
    - Jgtwt zrozt igpox rhm zpbj, iqk bkift mgkzrk akvrgsiuakl zyrhg ujao ruhpsj ymfa yqiwmzbnkcf fkpeddadz.
        - Kuimt encbj glmoa ssk uvrs, ypo svjpp jygoge gckzqwqoxet rdlta hynw saswcy avxn gzukfvrnaav gshkvnizv.
        Lteow ezgwc yhjjq usv aact, jpm qtzhm etkrpg efeytvwziho ccxxe lsjo ftqvev wcsz negveqhpdjl pzqynimnh.
        Cjmas vtoxx yhloe bbf jjur, rzm xhbon zpmapn vyckgvcqzbz aelpv wfka qylvqu hjwf rsssrqycxll qcweslowe.

- Hcqymauognxk uri qacvqk wibo qyl:
    - $76,593 eka sse
        - Mpdo Tpcfg KN, TRK Afph Wqaez, Cmimbcfi
    - $605,216 upo jsqn 54 jgpcbxin
        - Kbzefteass Fcaigqw, Fplvt Hvsdug, Huxzxual, Hqxiewuu, Wtau Ceav'e, Gheu Huhee IZ, XLR Dktg Uiyhi, Rgmlecz
        Fkkpu, Trxm'a Erympq Hcqli, RNL Tfzuywmts Bpoifykb, Uzokhxyr Exix., Ugur Aeycewc

Non-B1

- Ugvkrxk Kslkgbceks mbvfv://xqmwidng.vutdmcyph.uul/khkney/PD-50812
    - Fcbtk cuz usdf
-

# Acceptance Criteria (WIP)

**Aoylte:** 📍 mevtr://tjx.rysrg.yco/qegu/S9PjxOGTKrbNnepWVCiq4F/Ldbogvc-W9-0055-Yqcntqg?skok-
sy=3%6W362&v=BaHoIxy7YCEujfwj-0

- U zwhmpr rkzkaq sho rs nsvzpul gtr eyea gie unjlzpcd.
- Fcqcw jpc zo rdq ba fcjn auxoftiwjj
- Koy plvpnxkars wfz gt ymqy ykbst, ijf aiw-bhzed (ptvqqs)
- Ozn mnynlor nk uia wrkqm lpr xx qvartvyifi
- Xjm vlzyoj uqe cpm nzjlslspnu (utquvnu) igtf axr u rtkfk
- Hnr puooju (lgh upgspayn sk yonnbgz) yhz ixfr xd zyx ross ik gju cpfnz (tgc qw dkejjelkyk)
- Alom gpcwlca nb nluncf n lged

**Discovery 2023-01-11**

Gt uyo zlq fga zlbr pmaiyouvuirsc sefg xru “btwslflu” xcjc, tg cwj ktfh kut ysnxzm jn MNZj xnihqpd zqnd kbfa qbpfrgf
bjj g-kolj

Raahsef, nz gycp ovuu xs rxyfaf ffb tdxnsysl pzfd qfgcp xu yl ehvg zwoe GNHe ugrjhpya.

Zc hdq aoygx l-cglr etdnfdw bk qlw vrzdu be PIWu khtdbupm, xay yyboflva pqac ngnweg qki kawmk iqekg ptwtiw hfsl cwwj
u-xsja.

**Back End 4d**

- Es dpx jvd nus jxsr vdicnf fg jeiamkbr jkgkfefdqwber, `XqzqrUysainu::ehdca_xofoj_cz_gkao_zsod_seiphk_mr_spsob` eg
reueck mawbbhuv NKVh slhjibjn tiqm ulv dzpgbit ge a-jpil hbot dpf bvbsqudw Yjbres 0n
- Xybdlq u idcmcfz lgqe `bpmsora_mhjta` opl vyrh xtibsbhzspnnu
- Jday gryn txnb dgx wysgzfxr 8x
- nzyzpo i Gjd, rwfx jgjp oymr cgx r-bqwcu 2v
- Rcvupm i-dscn fjomiy 9v

**Tests and pipelines 1,5d**

------------------------ Linked Issues ------------------------

BD-JEXCREL XH

NKU-72 MRA7 - Bpjybar z ygwdzb dazqhop sf GPCe gyu jlhro zh Nmtfe lzi jur-Zfwwa - Fkdgsgq Dmkidns - Yazvcb - Qeacbqh
XTSS Lrpwyjcf

[98;0;546p[6d[60;8;786v[7a [14;3;540lZurk gkvc tlotl jp Wwdy: wbcrl://ekxxcxox.xvxdyhvya.vto/fidmut/ZK-10842[f [0z
";
        let output = clean_jira(input);

        println!("=====================");
        println!("=====================");
        println!("=====================");
        println!("{}", output);

        assert_eq!(output, expected_output);
    }
}

