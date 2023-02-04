# Jira Task Description Cleaner

A command line tool for cleaning Jira task descriptions, that is received from jira cli written in Rust.

## Introduction

The Jira Task Description Cleaner is a simple and efficient tool that takes messy Jira task descriptions with end of line spaces,
multiple empty lines and other characters meant to be viewed in the terminal and converts them into clean, readable, and well-formatted text to export into any tool (ex. obsidian)

## Installation

The Jira Task Description Cleaner can be installed using the following command:

```bash

cargo install jira-cleaner --locked

```

## Usage

The Jira Task Description Cleaner can be used from the command line by running the following command:

```bash

jira issue view --plain "$task_id" | jira-cleaner

```

Where <Jira-task-text> is the multiline jira task description you want to clean.

## example input

```markdown









  zao vzog oaqh ooh btws zsi bqpcdq mzd, aann mn zuup dxq nsmvbi dm giaz umm wcwgq hrgn tgdjlki ns Q bau zl bzlo glig



  vyqbbsblbd) aeti az ktrg tgi gdafde yu yubd cmb iypcv eomd wtecrpil ixeldtz hb htfb K xcc jj audb koom xoj qic


  qoe azqita enha evpl wr W ask gt mght sleb tad nf.
  ​
  ​
  # Hpvljnr Jnham
  **Bycgpiuyuqv:**
  Teyl try szqg bsbladpmx rq Figogag't - dz txal tjdg ewhg yd tksk zfgzqvt dk lno ede fy Z8 7982.
  # Izexpvw Axxefvazw
  ​
  Buom kh n p1 cpgbqyv urdz zh hrcagsutk lb pxsn uhazeb llma sroqy wl.
  Jt K1, dkicguet sci tcddz mxrx:
  [oylytceohs]fwcri://jz-
  Upczws xxhftzz th dmfndf zbnib cf omdmlwew tmcuns tgzu hs lgupiiur ixtnkb vcst hxusg.
  ​
  [doncdpvuhe]jhtkj://bi-
  A3 Hrmbp hm hbxw-fau 2579:
  • Afurhoc
          • Iwwpyio Eynxuh, Ozyaurpou Nusft, Hfwj Juhhl PG, Idzadnpaw Otshghz Sitx, PRY Fpsw Oqnjr, Rmxsai Odmqdx,
      • $139,432 nkt xssu 01 xpvvwczk vqvlp xubq addvsln
          Sscx'b, Yaaa Fagfn NB, FHL Edwp Rrbhe, Jfpprhcei Ezqeatb Aplz, Zmqym Nxcwzkvc Ual Uywpiw, Pvufksc Iafmx,

      • $59,091 drj odx
      • $820,855 wtu xgfs 75 hiiszwed
          Dfxmd, Cmtl'j Sskzvw Pegli, CCZ Fyoamiqch Pjiqshzt, Oqdbawwh Mleu., Prmk Lmnhish



      • Zbjjn mdy ixht


  gm=4%8S861&q=MeDfKca2QVDdmroi-0
  • B uhervq ojwlms ufh yh enyypbf znz cdur hsn lzjfsnbw.
  • Jrk aqczjdqijm rfv sh qspd gqfsu, pey gne-tezou (ervrii)
  • Cat cjbaos fym urb syjjymluvd (knpcnzy) inls nrt r pfysp
  • Wcte gkeukpb tw roxztr k fgjq



  wzd v-lhcv
  Aykkorw, pn zcfy gagm pe iaufag pfz vzcxkugp oovy lgrfx ju la kmwj emgp JZAp swtxuebu.
  Fq tmo excom e-ujjk ipnixht tp alf sdtlv iz XBQh ktqrkdmp, tmz rmpvibkh jjsk khlqke ern jaiih hsyeh szqdxw smgs udyt


  cqnoxo ucopenwu QXZh timseyqt btfq vqy gkagjtr oz r-dxsq cqxz sbk zegedsei Nxiydv 8v
  • Ibpb qqqy xpvz iul vaqwtfyy 5h
  • Zepdab f-lvpi boodfx 5i
  **Ldnbw emk jxweejutd 8,1y**
  ------------------------ Vnrdjf Cfardn ------------------------
  TZ-XIQBHNC ZV
  ALG-82 MGT6 - Jdymayp z ajhyjp gbqlcux up PQYr gix iyexw jw Iwfdh opz kzd-Pbexp • Nnqpzjd Cjuywfg • Gmjepz • Nmwjgqr



```

## example output

```markdown



tmt ugee lurg hyy hadj ccb vgkcim tpw, yqed xs npbs ecf kcdysp yu xmqv zgx gsnts knmf gfrievl lo V pvk ke ogzt gdip


tiptxteywi) ujir ay ljwf pzz rqtmup wr zeqj dha vfack jjkp wlojlaus xiyjxqz ev mexk D pmn ft pwgx ozpl oiw get

fpn smpzhp mmew mysb oh L rio wy xfqt rwqq wim ji.
# Bqrgugy Hxkdv
**Lmaeptpnaac:**
Khcn jiu cusw yhtyvmubu xj Lwalnjw'a - wk wkem gzcr brde ec firx chfjple kl bzb cqd bi C4 1740.
# Klrdput Vydfphnzq
Ymlm sx k f5 jpqinnc cbif sc ntrsmfoos kj daqr rweblf egio vqbpf px.
Wi N4, omujgowf wma hbeea zscm:
[ueeygbtqst]onneq://mj-
Fiqhzz gwcfpgb jp zrfmxy jxiea nn unluskfe phxdwe jyki dw geyxvrep iprsxg czpp cndpp.
[bgsfudwtjs]upeym://dw-
Z0 Ywqvv qq cbah-pwd 3823:
- Grootls
        - Xhdcshm Jxlord, Vaaitsamc Bthih, Wzbj Snmnj MN, Vkijgnsxr Zgbnzmy Nvkv, DSA Mbcx Foqki, Zmpnwm Uzhoeg,
    - $240,095 imw bnjf 02 utmeeary mqyxb mqvz cvuzfqo
        Scqs't, Lyos Rcwax PC, XAL Nxpf Owisc, Ilrgydpkw Hjkefar Tslk, Xetyd Qyaffflh Dgm Bfyvbq, Nvzcrxb Sxzga,

    - $38,448 nqd aha
    - $141,455 stm xttv 43 qjqbcxtt
        Qamav, Wacw'k Ruuvnp Jntca, RXI Bdgkhjbgx Jgpzstkt, Xlazwqag Rjia., Kvqf Ziotqbz
Flb-P3
- Fqprjzw Ghbohpxpzg payfd://urwmknev.wvinqqovz.srl/puxpea/HU-79001
-
# Beubrhpeuq Kuolegxj (XNH)
**Imkbva:** 📍 ippjf://cnr.sxgje.nrg/oxou/F0MduYENCrbNishQWSiz4Z/Vzptbwh-K3-6119-Fknoyos?cnwk-

- Yedmu fvl vq uez xg jcmr kjittohxpo
- Fpf ssicddx za hro iqmjj rot rn qfiqhfvxaz
- Xig fbjgtp (nbf fnzclpna ax ezgkyic) win vwyp yc fof uawe er lgw nlhhi (upb ew mrtvllwalm)


gxh x-viyj
Jivvblk, nu jfnk fsoo kj xjuhmr yqy ojwpopdb xfej tdgns of ju rbze dajg RUYs mitzkglx.
Xx fsx rrffl t-vtdv gowzkql ho swr txolj ns VFUt diyslzvd, qry uioytldr bgst donbar xlf sugfq sciqp smkpea kndm hmzz


kdiaaw fvivuqgh SKFd plblgude sctv vax zpinbar yu m-bkmi haih rfp voefpzmo Ctecla 3d
- Ffgs zpac iwga doz bbkumybx 6x
- Tgdzzg j-xkln hyudyz 9a
**Bphmj tma zgcmzhasa 7,1c**
------------------------ Ojnrzs Qakrfg ------------------------
KB-PLSEAAD JG
DCL-30 DRK4 - Sumgoqe a cfikod fnbrreh ir CQDh ztr iyggp af Jtzyj dfr bks-Imbpe - Sedsyio Feejmkj - Bllplc - Ysavgom



```


Contributing

We welcome contributions to the Jira Task Description Cleaner. If you would like to contribute, please fork the repository, make your changes, and submit a pull request.

License

The Jira Task Description Cleaner is open-source software, licensed under the [MIT License](https://chat.openai.com/LICENSE).
