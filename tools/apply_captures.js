#!/usr/bin/env node
/**
 * apply_captures.js — Capture System Overhaul
 *
 * Modifies textmod_expanded.txt to:
 * 1. Remove 6 captures (Ivysaur, Pikachu, Charizard, Metagross, Poliwag, Arceus) from Lines 121 and 63
 * 2. Upgrade 4 captures to final evolutions (Caterpie->Butterfree, Sneasel->Weavile, Barboach->Whiscash, Electrike->Manectric)
 * 3. Replace 3 captures (Rattata->Skarmory, Furret->Lapras, Alcremie->Arcanine)
 * 4. Remove Zubat capture
 * 5. Add 8 new ball captures (Mew, Jirachi, Kangaskhan, Heracross, Greninja, Electivire, Magmortar, Rhyperior)
 * 6. Add 6 new legendary items (Rayquaza/Jade Orb, Latias/Soul Dew, Latios/Eon Flute, Suicune/Clear Bell, Entei/Flame Plate, Raikou/Zap Plate)
 *
 * IMPORTANT line mapping (expanded file has 10 extra lines after line 100):
 *   Original L111 -> Expanded L121 (capture pool 1)
 *   Original L113 -> Expanded L123 (capture pool 2)
 *   Original L115 -> Expanded L125 (legendary pool 1: Ho-Oh/Lugia)
 *   Original L117 -> Expanded L127 (legendary pool 2: Kyogre/Groudon)
 *   Original L63  -> Expanded L63  (Lillipup line with Arceus/Caterpie)
 *
 * Usage: node apply_captures.js
 */

const fs = require('fs');
const path = require('path');

const TEXTMOD_PATH = path.join(__dirname, '..', 'textmod_expanded.txt');

// ============================================================
// SPRITE DATA (from sprite_encodings.json)
// ============================================================
const SPRITES = {
  Butterfree: "372k0o2m0g2C9g2M8g2Q8o2Z0E2%fa2E7jfn===PLPzBFRlltnxtNRgC0TPkSmokgf0ao0TOE0RkQmfM0akTNo0Rm8Z%mC0TOkPo0MmfQ9m9kTNkPkQ0fM0C0C0TNo0Pm8Zf8mag09kRq1pN090f8mcmM0OoEWM0f0h0bg09kg3H1F4dg3gcg0M2E0I1EqE1cgZogfmo0F3G1sZ0aokfbVFZr1oEm1egd3A1Gv1gfkbE2zh1Hu1ofg1C2A9E0Ht3fCmo1Bg8w0IqE1x1fC3Dw0Ko1xg8k%2D1L1y8w0bhc2D2J1zhZ4bg1D3J1B1gbhC1D6G1Bkc7BYBkf9E1A5F5BkfC2x5EhC3Bm%EVEohC3zZfbgUg1phE3zmfcg7gXxZfbi=1E4fC1o=3F1gf82g8E=5EfeE=4p2g%g=3gcg1gdi5Ego1E4Fmf82g9ogQ5gQ1L1gfC3g85EgM1u1om%1EM6gQ1tEk8k%omM6og1u1Mk%kQXu1QkfC09moUEu1o1hf8mbg5Evp3jdgma",
  Weavile: "342y8k2Kk82Ljj2%8w2EcRfptNRl59ttHVZJBblZvzHRJjnv===TD0=R06cyroyTKpKX69ro1KTKr8cWc80rMySK0r%V8rP8SK2q8cUrQySy2q8c7qS8Sy3q87pTyRyMq86T1KMy2yNq85yT083ak1yOoky40yTy1Kcw80KQ8oy0KTK1%4c9K283yoK1KNl380Kc5cy80k9k1K1o82K3kak1y%c6wbx81y1o83y2%cw9l9c6w4w9dw80K1K4My1%fw9wc685fw80y0K4skM81%fdwc484a48c4cwa0K4ty3K0%fe68cIE8c5d9l84u48l2k9wfe5c8EB87cb4vs48nk9e4e6c8EIA8c6ew84vt4Zwe6c6c8FI8cVc84vu4%fdXc8G8cVc84vu4%fdXc8cF8W%6tU8f%cXc8c8cVc=Ywf9cXc8cV9cw8l3mw8fws9XVc8sw4cblYwdw%Bs9X6c8sAw4y1r0Kfd8Ck9cXwyB9kQKwf8sBo8o%Vw9oAs9oS9wcwcwsAkokA8VwAoks8cypOp%8e4cwawWcwac%syq0r8tc8eXX5c8shs9kq9shs%dUc7c8V8sjgtKwsjgs%d7wcVc6w8LsLgc%d6cs%ew6w8LLisae4sAcAc7c9LLhs91k9wc4c4c6c9cLLgs9Nk=8l8sLLgypN8Ny28sLLs8q3yO838sLLo8q1KPK1p8Lg",
  Whiscash: "3c2uog2A8u2C==2G802H8g2Ihf2J8i2Ko02O3g2PC02QGg2S9g2E7vNRptvb9f=XzC=TDlLLL=%AC5AC5AOPAOPuO8CAOPuOPA2g8CA2g8PoGQU8l8=0AQ39jdj8YAQ1Jf4JWAQ8Ib4ahVoaIehHuah7o8IaiSphoEbi5ofdgoEitgch4ofdgEqEtEgbi3ofegEshEpgagMwMg2ufegErlagyMg0AffhpIgMwgwgA0galfIagMwMgAokMxMIfcjuKjMzMgffej1jBgffbl0Kjx0gxgff9i81hKojMwNwMIck4H0KkMxMjel86SqEk9njaUHHtE8k=48hQrg8q8h8=1J81gsn9h8U8k91Ssnbni93JSrm8h9ZJ8hGHqlEpJ959jEpj1Hpi8EtH9nErEkGKguEvuSEvqElGKgo8voEg0HvpFn2go8vu81gvoEh8nhp8Evg2gsEjpMnEq",
  Manectric: "3i2iog2kgo2l8o2n772uoE2y0o2D0w2Ipg2K9o2OEo2Pdo2Q8w2Tif2Ubo2WDD2Xn72Zil2%1o2E8tNRYSzFCseedHRJmxELgc===n1iconnodkX5ieoX5iekX3IfpX2iofolX0ZfZXo9Tol74o70i9kfoK72p7yUfZ71uo7iUfll70uyWD%cToKWD0u0OwWDiPfZ1wWE%CyeoflKB2oCieToK0A1OCofkfi8kA%DQQ8yflfkKxQ8%wQQQTlfbkQQ0Odwo8gffPway0dklffP0aywP8pffdkaywP8Icgrgep8kay0P8IcoxMoMocpMlb0Odi8gPVoMqUM0lwaEydkeuoVpE0iauowUywdiP70papwbuyUKck%6paock0pKl8ibkEl7Ei8kwU0io8io9iU8k71i8kbu0iIl9icu1q4O8kbu0pOho8iobkEyGsEqEcpE1EIiioagpE5Guiduo4qjbgvrekl6qgfmTZ7OffiekK7Tfkfau7Tgcolfau70IcgIapkf9kE70EI9k0EZE0pwf9u72Ohu1Eiu%wc",
  Skarmory: "3e2iow2nwo2u802vuu2Co02Di12GoT2Hpw2IC=2Kg02Lwi2SwH2Wn=3%giT2E6tNRVZJJPTbbhrtzTPjNn3hiTRn4KGRi4hGRi5%Qo6gnTQog5KGPng6gGPCg6%4wsw0g6%3nzOKG3LRhG5qwOhG7qOgG1vuoxiMgn0vvvu81v8iKp7Kp0vvvuenh0L5hw0Icig1S4jIbng3L3kIbog4H2j8gIai4oEDk8gI9n4oFDkuo=ao4nFo1l8W9o5wqw0m8I8o6xDluIo5w1L0muo=o5p1n1m8iZn6wrw1m0oZi2L4S1liXn2n0p5S1j0oXi2oaS4wDjiUS1ncwqw3H1h0ofnxoxoUwqw2DhoewpyioXwrwDgidpAp=aDncwqy0WbDocoxoy0Wci0obpxoy0WdHapzDi=XnACWYnBq=Y",
  Lapras: "3c2jwg2mwo2uMg2z702A802C0f2Om72SgM2Xjo2YgC2Zw72%Mw2E8vNTR=L9fhhDVlnp=PvvxBTFl4wiZ3ko76um2h7uo3O4uo6g6gp4mhozSzSwkxp2h1ZX1oi2ogo5ojph2ZX1w1ow2Oohm1gzjp5g2o6yo2mzjp5Z4o3g72gq3oOzZ2jtj7zx73jwqX7zmZ3MkozoMo5g0g75Mxp72go4o1g74wp5wio2g7u73wp5x8hx0OwM72go6ow8gw8gw0wzg5AA8gp0o3ow8gxh0Mz%0A90AAwqgw2oxk73gAAd%pho2q7zMfAgmX2p771gf9xpwrj771gf90gtwVgO6gfaxswUEViO2gfa0gtMUHUwhOgfbgtxUIWgw2S1wfb%pXwUjUKUX1oYcwpjwUFUxMVFuw0whwCd%qwIUwhx1ffdgqgIVMffwf9xpgKUYdYAgpgKUwMfd0Y8gpwLUgfd0%CgpwUJ9Yc1u0d0xpgJ9%fc2u190wipgIaEYbM2wkM0upwUGcufbwM0QS1j0pwEeESfaxQS1uw1owUfUY9wQx1MX2wMf8uf9QjM0Mjw3owEfESf8",
  Arcanine: "3d2mff2t8g2ug82y402zg02I8k2J802Kmm2Lg92Ms82Ot02S8c2ZR02E9RFn=VDdd9vNRvndx57Nf3pxv===KfMKKfdstfc8Ke9mcsJgseMKeasfd8sJ4Ssc9csKdJ9c8sd90509suMKdOLucO6090LKdM3y9Z84LMsmfdazy52Q1ygasmeast2Q0Z50zgsmeMhBQ0T0J8Mfes=JyX50Ysfd8nnkJX70OYsd8pkYkokzX5tyIqS9kot3Iokg2V081Lgok9zIo8AIo9kazRzyOgk8d818p8A8pko8wL0QzL1ISJyulg3Iq8wtw8Ct1gk8sy4t18lzhkr8w949g3u2I84c41t18nplpg%ib0IS53ty8ntIrpk%49kt7193Lz50Iplz50agtt9Q1t1OZh1Z91uh8y42J91XW0JSyyu0yuBXZ81S4191uFVA8381u042gG71%g1uy9528D8Bh8wg383u1uh09E9j2g3u1Lc3uhagPt2u70t1gltNtAu519zIokO5Hgt3Ou52tIpk826Bgag1zgb4g42ukqk8h1Zu0u2t96tAIok%T0yt194868B=9gV0ga4cR83g=tW1859gR82=JX4191ag5Jz=0X60g2g187JgagBX7A8RY2XU3u5",
  Mew: "382lgc2mg42qXX2u842Fcg2Kg02Ogd2S==2E9=NTvNRVZJ=xLrjnzvx0rHS=dJRqqqqqqqUkjjkq6kic3ejjgXQkhIFd1FUjjOIcAgUgP3c2FUlPCgVklP3c0gWlPAhWOPAgWklPAgXOPAg7uuu8klGdBg1h0gu85uuu8lBj1gBKmkc0gu94uu8kgBmk4sKlBcso4Kmu%lBgso4skc0gBgsosKgamYlAk4sps4KcBmosKgacmakO3gtrgDgp4Kga1m9gf2gtowowgDgwoKma2m8geg2choygDi1gbl0cmgeh3cjkGcmb4l0chfhcMi4Z4Ohc0ehe0cHK=ulO0fcjhcEF=aic2ffchdCm=a4hc2ffFkhd1dgk=Ykl3ffg9kjh4=%gBffmS%g2d1fc1cmSZg3lCc1mSb4l2FCF1gkS9khc3lCl0ch4=Z4hFe2lCl1h=YheOl2gCF1F=Y2chel2gCclm=Y1cifg3gCh4=Zkh9gfclhDcm=Z",
  Jirachi: "3g2lEg2m8g2u772BgE2Ivv2JlW2KJg2JgM2QEh2T382W2g2Xfa2a872Yu72Zm12%Mg2E8=VzRLrbfhvNR===InNNRjFDvsUBbEj0gUIvoBclVl0gIvUgdKwlItUgeKxgUIsBeKxlIrUgfKWUIqBfK2lIpBf8K3gvtUiEf9K3lvpUiE1XK4QUsiE4XK6QUqaXK70QUo70XEk5x2Q71XH6z2a1fcY3fbY3fbY4f9Y6f8Y5EkE9Ti8uThPEnNl85c7BNhObOhMZgfbgWNhmDh0Jg1mbgdgWNhlDhl%3BhmcgWNjDj%3hE18il82mMQEDQEwg82m58h3mNVxMzMxoUxm3g7g1xmMwo0xEjEyUxh3g7g1xmMDC%F8W7gThMDzMh80Zm7g8W08hE8MzMoi8TiaZZ8i8PlE8uTi08QxShu58GJDy%8u08G1JDAMBaTG3Jx%DwgwmG86G5JyJDwlwZHa2JzgwBAlwgxlTH5mMzBwlMzBwgoxm75JAJxEkExlxMl6",
  Kangaskhan: "3b2CE02IDD2Q8w2Sg02TM02UM12WCM2XQQ2Yh02ZII2%GO2E79bfRNBvYJr7tNRnljDxtZZZIM3ZwUM90y2MDxM7MzUMb0y9UMBM4FNE1N1Md0ybUMxUF2%E1Me0zdTN0H3%Wd0MzeUJ3%WbTAf1K3FPWa0MAfUK3FPW8TXQMf1K3%GTXQ80MeTL3%E1XQ8x0Me0LE3FPCwXQ9w0McUL3%WfQ1M9M2LE3FPCfw6h1L4FOWdw0g1mE1K3FOF0d0Ylqg2J2%Cdh1ks0w2H3FOWch1jt0Qg1H3EPCcYkt09o2G3EPCcYktSwgoSG3FOWbg1kug3g6FNCSbg1kvpkVYbSmvuno8Sb0gpkvvu90gb0gqlvvs8o0chrk2hvvo0gcgsj0E95iq0grSduj0E8grg3i0gpSocMuk1ish2i1ocMEum=0e0uSnj7gp0c0o8tSnhVr0aT9tSn1EMcME1gp09To90sSl1MfaCS9TEhCs0l0EMfcCE8TYGrSk0EMfeCCgogFhqYj0EMff81El",
  Heracross: "3d2yg02zg12Aff2Bo82DBB2Fg22G2y2Hi02Ig32Jh02KDB2BwE3OM1w2E7bdmtNRwwwR=LNCevna=QdAazk0zHgAe6FmAd1g9Gj3Ad0c3h19yAfbFzAA9GgAA9GAAa1JAaFfb1zAa1zf9GAb0Hf90yy82A1zf8yyFzfeFgf83zHA80yg5zyIzAzgXy80zgfe3Fj1zzzfKozh1gWyz8Kp8K1i2zFg4y8Ko9Kzj1JjGygDDuzkGi3F8vu2j5i2IgvtFj1EB1IyB1vt4HEwo0E4ywo0E0vt5JEO4yO0vsgVO6O0vsWB0w6w0B0vr8XxUB1vq8=Xgvozh7iWh1vyk5k3h3Fv0m4nn2gum6nlGgtl=1i2h1tj=VHgsg7h=FHs7k2nyh6gr0g4l1nJJg5gr84l1nzzp8IhpIm1nzygr8g4oIm1nyzgs8yi",
  Greninja: "382DgM2Hg02SgE2Tg12Xg32YhM2Zgf2%oE2E8gjxQovbcfWWJPIstNREtl===zr0LLg4wgKAqS0k1Kg4wgKAqTXi2G0XwgKAqMS0g6j1XwgKBpwHE0g72i2w0KBql73g2M0KCpMIh72TgLCpwG0l7TgLMCp0h6g6THKDBpg71g6Hh1E%oFDBog2l1g5hEVg%%pE0DzoTg4%i4SoVS%%q0DyoHg6Voh4gVogvo0DyMg7EUEHXgoUogv%0Yxg7gUg2XEUEgvpg2pi5SXg2goHv%TwpgwMh3h5TSgvE0h0wqgoUo8i0h6THtE0g9HwqMwV8U8wi7HgugaiMwpwgoU8U8UoDh6hEtag9MDqgwV9ogwqMh2HHt9gcYowgwWgwtwh4Es8NeYoMYvqMj0Er8Z8DYwvswD8M0r8Z8gaMnmMaSq8Z8ZffgqMZ8Nff9pagqYf8Zco8rbgqEDf8MZfdgrEDf8MZfcgtDf8MZfb0sHYfMDff8SrXYeMDfeSrg7YdMYf9MSs7TiMdMniEt7TDwjNbOi3t6XgpwNnh4gt5h3gwqyPwMh4Hs3h6MoyswMgo0g4gs0i0qXgwvpgp0Xgs",
  Electivire: "3c2eo82v082xnn2z812C8o2D8p2Hq82Io92Joa2Kxx2O282T832E7QM8mk5uNS656R=LVfa%=kKKxngaKj8Onmbxzwvnk919nmT8nkzwvnm918hdlT8xC1J2bi919i9njevVb0ai8Fongb6P2z8k8Gomav5R2Cm8E9leWP49mIjD=68l8eh9rZa48meIvrX8H48lpT8HW8H4ek8e3DbVa4Hke4DdYwE9pko7pbpEowEpzCEoEeJiI6cpEpEe3CEqzpgCvU9rFp8vv0rT8q0IV8s8eOe58r2e=Zeq3z8Wf5avog4aVCg8w8w8g8484o6a7Cf38=93zzbO0978V8ve0H3ao097ov79q09uapUDU8raH0ar6ae68ra0HOH4od6tzr2r93od1Tp9pzr18qav0b0a0ev0qbe0Io1pbod29q0ep9pD0b09vJeowg8Oguao3c3IJgy809guc3b2C9JADw8sco3J2aevgA9owgtaeo2Io1D8evBawgepDap8vpCvpa18B",
  Magmortar: "3b2y0c2zc02C402F==2Gjj2Ik02Jff2Kcy2M042Pc12X0d2E9hb9NB9tNRNld=RpZvn=DBplnFFbMjg0FFCjg0F=Z24jh0Fa0=X0G1F8zZX0GhCF4zYyC4GhC=%1%MPGgCF=06zjiCFb2a06c4jh0=9AZ050905zji0=8yl4PYyC8MzGg1Zyn5cYX080C4GhCYyn6Y0eyCGjCYMl7Y0e050Gjg419s0Sb0e06MGjgy2cS090f7c24GhCCdSz80eTc24GMKU08XWP4jhy4yV1e5cUzjhy4zW0fKUzjgy4K5lRK1zdUMjy4K4nRzkKyUziy5Knl4m50kzIdTz4h06z4nnm4zkcIeSz4gy5K5n4nlzIk1fRMy4ey6l5nlCIl1fd6K4eyTnlCIko0wtJfKUnk50Iko1x0sJ2cWm5zIkoIswswDw1cUcRzIkpIsEw0IdSdQdyyqk1vv0l0Je7d2IkrkDko0Jf5f2kykrrrok1JJ3ckykrrqk3JfdBck1mAclBJ0e1sAcl2fcDzfP8d1uBcmrkcAsXAC81PuOtyC4j086PvLty505iC7PvsHuX6Mjg",
  Rhyperior: "3b25802J8E2K902L812Qnn2RE82Uo02XE52YE92Zc02%5n2E7tqsebbuNSNPOTqcGj3===QQQQQnh0f0m0aQgaCRj0ap8nm9CZg09pMo%lJzEa4apNo%laFw92ZpO0%lJ5a2KU5oOU%l8y845r05q0Knk8A1a0UKU5UK%i8AXLU8wXUa158niEzFLL8yXp0b08ni8I82LJyXo8pMa0nhYF82839Ex55pNoa0nxY2838MoaYrM0NoKlz5948NJJKr0oOoKjzR7e08r0r0pKgoA8=5q1q0rKAR29W5q1p1sJzXap8VfeoFxF558o8o9WL5v8H91559ob6K8vo9F5558383fXvUa0a0p94Z5JwRvo39F5p0ZUK5Jx8voL9ExXv1K5Jx8vUaEzR2r2K5JwRvo15EAR7oK58F5vo15JA5o2q0LLdu1LJyEKr093L59Ys2L8GKd6b09F9q3829=1JL9F9o4b=28wR29Ea=VJw94Ze4f828wFw86YEB919CEaEw8x86HaFyaFCGw8y86GJwYExRGeGJy86FJzYxGJB8F8z86EJBRxF8D8F8z86",
  Rayquaza: "3c2mo82u082JE82Kgo2Qow2S802TKK2Ynk2k0p2ZoE2%282=oJ2JfD099dtNRDPrNXLtLdDflRP069E=2EmnY1pEqEb2ounYkkzZ48EnY0Uo1pAZ%nnlSUo1FpzZuEnYm0V2FqyouEnnj0m0oVo3I18EgE090Y1o90oWE0b0oV091JY3E70EmUo5mY74Ep81p3Z0nj75E=r2p8nj76pJq3mnj77pSq2=ni77k8Eq2mhoTT771=quk8oTTg71Eou3Em0qu0mTTK71q092pSpu0F8vo71q0a2pSpu0mgv71EpEc1pSZ918v72Gd0EpSEu0JKwQQo494F090U90E=0Eu18wQQQ3c59mU9ku1SJQQQo92d4So0akJ1SJoA182d4eku18EuoB8%Ob3c2q8u0EuoAm29Ob0H6p91ouBu2aOb1G5p2EmoAm4aOb1F6rwuBE94bNb1E6qwmoBZb3aNc7px8oCou1b1aNd5y8oDm%p91aNd3QmQ8Bm%Q=29O90m2QmQMaz8%Eym2d0wm1Ep8x8Oaw838zo92a1o19Emox0aOa%ozo0c0Z0EpuJy3aN8%oBsDJz68M838DDDw7u38DDDw71SEuDDDw",
  Latias: "372w802yww2wck2Ckc2Dyy2y0d2%0X2EaLLPVZHtNRdbh===BBFPbhz9dRnvXL0e=8dk1w==Yc1eZwc2w==bc1h0ebcEw==aw3g0kd9w3gw==9CFgyk8wEw==8eGg0elEcDD8cKgylEcDDCNdFcDDCNC1m0cPCPg2ePCPgducPCNdtoxodOcLdtozxscOcJcurzoscNwHctrrps1cNcGctrrps1g0cMcGcupsespsk0hcMcHduk0wdot1g0cKwJds0gcAgcsosk0hw3404040cLkwgcAgCptkhc04040404COeAkAcqtk0c1405kdP0kdAdsrtw7C0gP2keuqxc6d0gPHeupc60hP3wFfd6hPFeEd17gPKke1RPJe1UP2h1e1XPh1e1X6Mjy1XRKi1d%UIjy%WGjy%X4Ejy%X63iy%XQ1j0wXXSjy%XTi0c%XVg0C%XW",
  Latios: "3c2k772log2r082Kgo2Lg72Srr2T1g2Wkk2X6g2YhW2Zl72%M02E8NPPtNRddhBBD5pP7jx===H00ag0gfhoTffegTdg0p2gffbg8g2gcgXff9g0h2lcgXff8gT3gcgXff9gXcgXff8KXbLgfergX8Sg6lSSS090Lg8r0gtT8SSS2K7g1nhlkLohq3oHg875K7K0P0FwawL5Z0g0N0oExdL5K7l%gEAcL4Z0glEC9wh75LTEDwEgpL4Z0lDwE0%Z3h71lDE1%ZTpLTzEyEo1%ZK1L1lxElEwEo2%K5g1%K7TxETGo2Ml4g0P0L1lxo0lGo2MK2KQ0K70pgx0UhGo1%g2Eg0O1ZqgExgVgEwEo2K1Eo0M4K5sgExiExEg1h1oNX5tgJyFi73p3vhIzFL5o2vpjFAgk0vunk5tmok74oY3Y3Y0N0YO0Y0N0Y1N0KW0N0Y0OY1N0Kk75",
  Suicune: "372D802Hw02S0w2WED2X082Y0E2%092E8bdjvPRxpFJzRfvF===VZJPl04ioi0y8FD90yaSfj3j1x8F8H8Hy9Hfm2h1y8FD90ya0fng2g0y9F1DyaSeni3y8F81HxaSerng1y9E83xaHw1btn1yb2y8E81Dbug4h0ycHy8W90bugS8w4ycHx8WDwbv1wa3ycHw8WDcvo2w9Ew1ycS8WDctg6wF81yeWH8M8M8sg0l1xa1xdE81wM8M8Mrg0gri2waydE819M8M8r0gtj1xfbHRqg0grm1yf9HRqg0gqng2yfDwQqg0hom6zeHQqg0ng3z0zdHQpi0m2A82yd0Qpj0l1zaw2zaw1Ppj1k1xdYU0C0DOol1j1xdXV0B%0Nn1i1we5z2WM0m1i1w9wb0a7M0WDl1j%Hwa%GbE0MXE0F1j1j%w1wD9LE0MDE8G1h1j1959LEXM0J4jS8H91aLWMXEXG1k19Sa1aLE0MHE18F81h29Hc%1L0NY3F83waSc%28J0NY4F1wc0eY83bXNYzXF0cHe0F%48NHEAXWbS8IDF0OaPXEAD8ESaXJDD8TNY8yb%%H8KD8TOY0fYSD8LDTODDfHDHLWTO0Df9HDwLWTOY1",
  Entei: "362O042S402UH52Wo82%c42=d42E9LvnfbfXZHz9bNNLZZZTJhxNPZlp7NSo4B4vu07MSoOASvtA5KSpOA4vtUC5p084ASvsH4o5ASp0W435vsI48o61Sq0kO15o4v4H4Wp06r09S58Ov5H4Wl8oOq0Wo4o9OvG70Wm8o0q080Wk8OvG4=1Wm8r0ak8o5vH4=2pbp0Wk8pSvH4eSoO0oao0o9o0o5vUe50pOo9o0824%gsgsE5dxf51o4Wo424dSgsgC5czxe486o1o4o05e4iA5czzx%872oO84y%hA6ffcV0Qf%gB6fffd514Ofe4g85B6ffc61h0g05f%8g5BQfc50gY0g8g05=a4DQd58nk9Og8mg50a4E7=gnlg5058mg4b4F6=gnkOh8g58lg4b4F6=gng4nk8Slg4b4F6=g8mOnk8S8kg4b4G5dS9lOYgS9Ob4G5e4g9kOh2509Ob4G5cw%h90T0hObUcw%i8O0R0h5bUcx4j51Qi5bUcx4j42Qh05bUcwcSi41g6Oh0%bUcw4O0hSh514gO%bU=hSgSh424OO0bH7i86i42514YU0h9l8iOC4YHSh8n9hOC4YH4h8nl9gOC4YH49nmaOC4YGS9nmaOC4Y",
  Raikou: "342Lc82M8c2O042Wc12E9PRRtNRxBBdbf===nnnTH9=VlBv7Q8L0h08L1g8LXXc0jh0L0iLXVcAj0L0hMXVWa0jM1hcXUL1a0jM3LXQO0L290jM08mL5O8L4OOO05OL28f1ckM91dO8LcOOOO14OW8kL08fcka18e9LOOOO3cak8L0akdlaL0akcIek0kc0blfck80bcJM2d0YnlZcJc0i0ck%mekacIMjg8d%kc91LkdGM0jh8dkYkckaLL0cFc0ji8kwebkfkM0g0cDM0ji0wosowdk8nack0g0dBWjj8ouodal80Mokg84c2f0jifoucgck9k81cokMgc29L0jiW4dp80eka0coMg4c29L0jiM0gOfldbcwchLW9Wjj0c40hOkfLk8dh0cgW9c2jj0d0j14kck9k4gOcgW9c3jjgM40j14e42c0gW9L3jjg0c4C72WW9LBjj0d42S1L0WaLKM168e84Oc0WbLJM7d0g0d84kL1bkLH9c6di09ck4c2Zd8E9L48Lcg18k8k4L29keld93bc4M0Mak08d83ezxkffe8Lg8f8gLByrrozcae0gL1LgcCrrrpxkYk0gL1L0cCsrrrpL3M0gL1L0cCvurrkAM1L1c9cCvvsrowkAM0Mk1LcDvvtrwkA8Lkwk1LcDt1vuqk8A8Lcok1LcDt3vtpk1i08Lcsk1dEsBvtokjg08dwsk1dE",
};

// ============================================================
// BALL SPRITES (from existing captures on lines 121/123)
// ============================================================
const BALL_SPRITES = {
  "Ultra Ball":    "2eb000ccc%%%EEMuuwSOYikkSmeUUkCie%%kT26N4y5K4oEwsd4I4coEosdo424EAw8sep414w8sod9so414=go414k8daho414lA8khcl414sk9nks424gdjA4I4jgA4KVT1",
  "Great Ball":    "2ed000cccuuw%%%ikkSOYEEMSmeCieQCc%%AXRZS4b94Q4Ewfck4248sAwfh4149sb8h414bbh414c8ckahs414ewkApl414sIMweEko4248kcsApg4Q4oAag4SZXR",
  "Safari Ball":   "2ed000cccSOYikkuuw%%%SmeEEMCieGGs66cXRZS4b94Q4Ewfck4248sAwfh4149sb8h414bbh414c8ckahs414ewkApl414sIMweEko4248kcsApg4Q4oAag4SZXR",
  "Net Ball":      "2ee000ccc%%0ikkOKasK%SOYKSIayQ%%%qqsK%%%%QCAaaquXX1ZW4ono4U48Emo8c424oIwEl9cs414o8Ioko9cs414kob9t414g8Bau414hcwgdMh414cgxjgQ424cBeMQ4U4fd4WZXX1",
  "Heal Ball":     "2ed000cccuuwikkSOY%%%SmeCieEEMyikSieXRZS4b94Q4Ewfck4248sAwfh4149sb8h414bbh414c8ckahs414ewkApl414sIMweEko4248kcsApg4Q4oAag4SZXR",
  "Repeat Ball":   "2eb000ccc%%%uuwSOYikkSmeCie%Ae%%kEEM%QeT26N4y5K4oEwsd4I4coEosdo424EAw8sep414w8sod9so414=go414k8daho414lA8khcl414sk9nks424gdjA4I4jgA4KVT1",
  "Luxury Ball":   "2ed000cccuuwQCcikkIkeEEMSOY%Wk%%%SmeUIq%%AXRZS4b94Q4Ewfck4248sAwfh4149sb8h414bbh414c8ckahs414ewkApl414sIMweEko4248kcsApg4Q4oAag4SZXR",
  "Dive Ball":     "2eb000cccyO%mE%%%%ikkSOYAWYasQEEMuuwT1VK4h9oA4I4i8qA424jr8A414shs8q9414vae414ksF9f414lAgkdwl414ckhnkw424cFcaw4I4cxdw4KVT1",
  "Dusk Ball":     "3323412134538gd1w2ec000ccckkkgGgkykSi0oQeA%kK80%q2Ce0usaTNVO4gfg4M48cqc9424c%d1g8hbd18gv3glos3lEAI9x18kEBmwg4248lco9g4M4oto94OVTN",
  "Moon Ball":     "2ed000cccikkSOYuuw%%%SmeCieEEMMMOYYO88OXRZS4b94Q4Ewfck4248sAwfh4149sb8h414bbh414c8ckahs414ewkApl414sIMweEko4248kcsApg4Q4oAag4SZXR",
  "Level Ball":    "2ed000ccc%%YikkSOYuuw%%%SmeCieEEMYYC88OXRZS4b94Q4Ewfck4248sAwfh4149sb8h414bbh414c8ckahs414ewkApl414sIMweEko4248kcsApg4Q4oAag4SZXR",
  "Friend Ball":   "2ed000ccciCcikksKYuuw%%%SmeCieEEMssQ88OXRZS4b94Q4Ewfck4248sAwfh4149sb8h414bbh414c8ckahs414ewkApl414sIMweEko4248kcsApg4Q4oAag4SZXR",
  "Heavy Ball":    "2ed000ccc888ikkSOYuuw%%%SmeCieEEMDDDaaa44gXRZS4b94Q4Ewfck4248sAwfh4149sb8h414bbh414c8ckahs414ewkApl414sIMweEko4248kcsApg4Q4oAag4SZXR",
  "Love Ball":     "2ed000cccyikikkSOYuuw%%%SmeCieEEMyY8UU8XRZS4b94Q4Ewfck4248sAwfh4149sb8h414bbh414c8ckahs414ewkApl414sIMweEko4248kcsApg4Q4oAag4SZXR",
  "Master Ball":   "322h812nh82ea000555222333999444aaa777bbb666P0=E8Sc828o5A5p4cnUcn4ci7cn5g4g5kdn5d6mndswclen4cxfc4828gdjs8E8jgs8GZP1",
};

// ============================================================
// NEW CAPTURE DEFINITIONS
// ============================================================

/**
 * Build a complete capture block in the format used on Lines 121/123.
 * This follows the exact pattern: preview + egg/wolf + housecat + ball wrapper.
 *
 * Parameters:
 *  name      - Pokemon name
 *  hp        - HP value
 *  sd        - Side definition string (e.g., "15-3:15-2:103-1:103-1:56-2:56-2")
 *  sprite    - The .img. encoded sprite string
 *  col       - Color code (single letter)
 *  speech    - Speech text
 *  ballName  - Name of the Poke Ball
 *  ballTier  - Tier of the ball
 *  ballSprite- Ball .img. sprite
 *  items     - Additional item/keyword string for the dice (e.g., ".i.k.heavy")
 *  previewItems - Items for the preview display (visual decorations on the summary)
 */
function buildCapture({name, hp, sd, sprite, col, speech, ballName, ballTier, ballSprite, items, previewItems}) {
  items = items || '';
  previewItems = previewItems || '';

  // The full capture block pattern (studied from existing captures):
  // ((hat.replica.Thief.n.[Name].[previewSD/items]
  //   #hat.(replica.Thief.i.(all.(left.hat.egg.(Wolf.n.[Name].hp.[HP].sd.[SD].[items]
  //     .i.([equipment]).i.t.vase.(add.((replica.housecat.tier.0.col.[col].n.[Name].hp.[HP].sd.[SD]
  //     .img.[SPRITE]).i.t.housecat.i.handcuffs.speech.[speech]).mn.[Name])
  //   .img.[SPRITE]))#Blindfold)
  //   .n.Find a [Name] that can join your team.i.k.potion.i.all.facade.Leo4:0).i.k.stasis#(Handcuffs.part.1))
  //   .n.[Ball Name].tier.[tier].img.[ballSprite]

  const sdParts = sd.split(':');
  const numFaces = sdParts.length;

  // Build equipment based on face count
  let equipment;
  if (numFaces <= 4) {
    equipment = '(Eye of Horus#Chainmail)';
  } else {
    equipment = '(Chainmail#row.hat.Statue.sd.15-3:15-2:0:0:15-2#(topbot.sticker.(right2.hat.Statue)#togfri#facade.dar2:95:50:30)#Kilt)';
  }

  // Preview: simplified version showing the dice type
  const previewSD = sd.split(':').slice(0, 2).join(':');
  let previewItemStr = previewItems ? `.${previewItems}` : '';

  return `((hat.replica.Thief.n.${name}.hp.${hp}.sd.${previewSD}${previewItemStr}` +
    `#hat.(replica.Thief.i.(all.(left.hat.egg.(Wolf.n.${name}.hp.${hp}.sd.${sd}${items}` +
    `.i.${equipment}` +
    `.i.t.vase.(add.((replica.housecat.tier.0.col.${col}.n.${name}.hp.${hp}.sd.${sd}` +
    `.img.${sprite}` +
    `).i.t.housecat.i.handcuffs.speech.${speech}).mn.${name})` +
    `.img.${sprite}` +
    `))#Blindfold)` +
    `.n.Find a ${name} that can join your team.i.k.potion.i.all.facade.Leo4:0).i.k.stasis#(Handcuffs.part.1))` +
    `.n.${ballName}.tier.${ballTier}.img.${ballSprite}`;
}

/**
 * Build a legendary summon item block following the Ho-Oh/Lugia/Kyogre/Groudon pattern.
 *
 * The pattern (from Lines 125/127):
 *   itempool.((hat.(replica.Thief.i.(all.(cast.sthief.abilitydata.(thief.sd.182-25:0:0:0:76-0:0
 *     .i.(mid.hat.egg.dragon.n.[Name].doc.[desc].doc.On turn 7...
 *     .img.[BOSS_SPRITE].hp.[HP]
 *     .i.hat.(replica.thief.sd.[SD].[items])
 *     .i.[facade decorations]
 *     .i.t.vase.(add.((replica.prodigy.n.[Name].hp.[summonHP].col.[col].tier.0.sd.[summonSD]
 *       .img.[SUMMON_SPRITE]).i.t.housecat.speech.[speech]).mn.[Name])
 *     .i.self.t7.allitem.hat.(replica.statue.n.Flee.sd.187-999:187-999:187-999:187-999:187-999:187-999.i.k.fierce))
 *     .i.(mid.blindfold).i.(left.k.fierce))#k.potion#facade.pos125:0))
 *   #sidesc.[desc]
 *   )#clearicon#cleardesc.doc.[flavor text])
 *   .n.[Item Name].tier.[tier].img.[ITEM_SPRITE]
 *   .part.1.mn.[Pool Name],
 */
function buildLegendaryItem({
  name, bossHP, bossSD, bossSprite, bossItems, bossFacades,
  summonHP, summonSD, summonSprite, summonCol, summonItems,
  speech, doc, flavorDoc, itemName, itemTier, itemSprite
}) {
  bossItems = bossItems || '';
  bossFacades = bossFacades || '';
  summonItems = summonItems || '';

  return `(hat.(replica.Thief.i.(all.(cast.sthief.abilitydata.(thief.sd.182-25:0:0:0:76-0:0` +
    `.i.(mid.hat.egg.dragon.n.${name}` +
    `.doc.${doc}` +
    `.doc.On turn 7[comma] replace all sides with "I flee"` +
    `.img.${bossSprite}` +
    `.hp.${bossHP}` +
    `.i.hat.(replica.thief.sd.${bossSD}${bossItems})` +
    `${bossFacades}` +
    `.i.t.vase.(add.((replica.prodigy.n.${name}.hp.${summonHP}.col.${summonCol}.tier.0.sd.${summonSD}${summonItems}` +
    `.img.${summonSprite}` +
    `).i.t.housecat.speech.${speech}).mn.${name})` +
    `.i.self.t7.allitem.hat.(replica.statue.n.Flee.sd.187-999:187-999:187-999:187-999:187-999:187-999.i.k.fierce))` +
    `.i.(mid.blindfold).i.(left.k.fierce))#k.potion#facade.pos125:0))` +
    `#sidesc.All enemies with 25 or less hp flee[comma] then add [light]${name}[cu] [purple]potion[cu])` +
    `#clearicon#cleardesc` +
    `.doc.Replace all sides with "All enemies with 25 or less hp flee[comma] then add [light]${name}[cu] [purple]potion[cu]"[nh][nh][nh][red]${flavorDoc}[cu])` +
    `.n.${itemName}.tier.${itemTier}.img.${itemSprite}`;
}

// ============================================================
// NEW CAPTURE DATA
// ============================================================

const newCaptures = [
  // Mew — Mythical psychic, balanced support/damage
  {
    name: 'Mew', hp: 6, sd: '15-3:15-3:103-2:103-2:56-2:56-2',
    sprite: SPRITES.Mew, col: 'k', speech: '[i]Myu~:Mew!',
    ballName: 'Master Ball', ballTier: 9, ballSprite: BALL_SPRITES['Master Ball'],
    items: '',
    previewItems: 'i.mid.sticker.(right.hat.Statue)#togfri.i.mid.ritemx.dae9',
  },
  // Jirachi — Wish-granting support, heal-heavy
  {
    name: 'Jirachi', hp: 5, sd: '103-3:103-3:56-2:56-2:15-2:15-2',
    sprite: SPRITES.Jirachi, col: 'y', speech: '[i]Chiii~:Wish!',
    ballName: 'Luxury Ball', ballTier: 8, ballSprite: BALL_SPRITES['Luxury Ball'],
    items: '',
    previewItems: 'i.mid.sticker.(right.hat.Statue)#togfri.i.mid.ritemx.dae9',
  },
  // Kangaskhan — Tanky fighter with high HP
  {
    name: 'Kangaskhan', hp: 8, sd: '15-3:15-3:15-2:15-2:56-3:56-3',
    sprite: SPRITES.Kangaskhan, col: 'h', speech: 'Khan!:Protect!',
    ballName: 'Safari Ball', ballTier: 6, ballSprite: BALL_SPRITES['Safari Ball'],
    items: '',
    previewItems: 'i.mid.sticker.(right.hat.Statue)#togfri.i.mid.ritemx.dae9',
  },
  // Heracross — Heavy physical attacker
  {
    name: 'Heracross', hp: 6, sd: '15-4:15-4:15-3:15-3:15-2:15-2',
    sprite: SPRITES.Heracross, col: 'b', speech: 'Hera!:[sin]Cross!',
    ballName: 'Net Ball', ballTier: 6, ballSprite: BALL_SPRITES['Net Ball'],
    items: '.i.k.heavy',
    previewItems: 'i.mid.sticker.(right.hat.Statue)#togfri.i.mid.ritemx.dae9',
  },
  // Greninja — Fast damage + engage
  {
    name: 'Greninja', hp: 5, sd: '15-3:15-3:42-2:42-2:17-2:17-2',
    sprite: SPRITES.Greninja, col: 's', speech: 'Ninja!:[sin]Shuriken',
    ballName: 'Dive Ball', ballTier: 7, ballSprite: BALL_SPRITES['Dive Ball'],
    items: '',
    previewItems: 'i.mid.sticker.(right.hat.Statue)#togfri.i.mid.ritemx.dae9',
  },
  // Electivire — Electric powerhouse
  {
    name: 'Electivire', hp: 7, sd: '15-4:15-4:42-3:42-3:15-2:15-2',
    sprite: SPRITES.Electivire, col: 'y', speech: 'VIRE!:[wiggle]Bzzt!',
    ballName: 'Ultra Ball', ballTier: 7, ballSprite: BALL_SPRITES['Ultra Ball'],
    items: '.i.k.heavy',
    previewItems: 'i.mid.sticker.(right.hat.Statue)#togfri.i.mid.ritemx.dae9',
  },
  // Magmortar — Fire damage specialist
  {
    name: 'Magmortar', hp: 7, sd: '15-4:15-4:34-3:34-3:15-2:15-2',
    sprite: SPRITES.Magmortar, col: 'r', speech: 'MORTAR!:[sin]Blaze!',
    ballName: 'Repeat Ball', ballTier: 7, ballSprite: BALL_SPRITES['Repeat Ball'],
    items: '.i.k.heavy',
    previewItems: 'i.mid.sticker.(right.hat.Statue)#togfri.i.mid.ritemx.dae9',
  },
  // Rhyperior — Tank with shields
  {
    name: 'Rhyperior', hp: 10, sd: '15-4:15-4:56-3:56-3:15-2:15-2',
    sprite: SPRITES.Rhyperior, col: 'h', speech: 'RHYPERIOR!:[wiggle]Trembles',
    ballName: 'Heavy Ball', ballTier: 7, ballSprite: BALL_SPRITES['Heavy Ball'],
    items: '.i.k.heavy',
    previewItems: 'i.mid.sticker.(right.hat.Statue)#togfri.i.mid.ritemx.dae9',
  },
];

// ============================================================
// LEGENDARY ITEM DEFINITIONS
// ============================================================

// Item sprites — simple gemstone/orb style sprites (placeholder patterns based on existing orb sprites)
const LEGENDARY_ITEM_SPRITES = {
  // Jade Orb — green orb (similar to Blue/Red Orb pattern)
  "Jade Orb":    "332l==2n123p52n2er000cccaaHiqZciNzF=irZZZ=qxllwQ=xR=rx=iqYqw=HL=jrZwR=YY=yF=%lccdVlK%=LlciMcjNXX1Z3W2eashg2U2fuosgo62S2Aeujwkc862nCajEmc78GnbIo6Kc6pbMc6yw6p7kmykc8pO86kmcmQ842S2q9568q2U28=q2WZ3XX1",
  // Soul Dew — red/pink crystal
  "Soul Dew":    "3231434334I42ab000cccyikUU8SOGEAWhgWBAMKIxLpM46lwlT05K4g4K4g4I5g3a3Ak81pk81ck8o424ctc1cgcs1xc3Ew4J6T",
  // Eon Flute — blue/white flute
  "Eon Flute":   "3231434334E42aa000888AGSwAKUW%imqIMUKOWmqAmuwP05G484G484E583e3wkc1pkc1gkco424gtg1g8gs19g3A84F6P",
  // Clear Bell — crystal bell
  "Clear Bell":  "332l==2n123p52n2er000cccYqwiqZciNzFHLZZZ=qxllwQ=xR=rx=iqaaH=jrZwR=YY=yF=%lccdVlK%=LlciMcjNXX1Z3W2eashg2U2fuosgo62S2Aeujwkc862nCajEmc78GnbIo6Kc6pbMc6yw6p7kmykc8pO86kmcmQ842S2q9568q2U28=q2WZ3XX1",
  // Flame Plate — red/orange plate
  "Flame Plate":  "313f2122cl000ddd=aeJ8e=msT8e=TSB6g=TT=xxJ9e=Vo=SS=Uo====wx%SS=mt%ls%aeS8eRG3M2i82K2sui82I2g9942G2wyknkbf8ag5ce6f86c5oe6f6cg5coefA6q5qe62G246b72I2CbE2KTR1",
  // Zap Plate — yellow/electric plate
  "Zap Plate":   "313f2122cl000dddYYC%%Y=msT8e=TSB6g=TT=xxJ9e=Vo=SS=Uo====wx%SS=mt%ls%aeS8eRG3M2i82K2sui82I2g9942G2wyknkbf8ag5ce6f86c5oe6f6cg5coefA6q5qe62G246b72I2CbE2KTR1",
};

const newLegendaries = [
  // Rayquaza — Dragon AoE + buff removal (Tier 8, like Kyogre/Groudon)
  {
    name: 'Rayquaza',
    bossHP: 70,
    bossSD: '36-10:36-10:0:0:36-10',
    bossSprite: SPRITES.Rayquaza,
    bossItems: '.i.(topbot.(hat.troll king#(eye of horus.m.4))#facade.dar12:0).i.((left2.facade.kas96:0)#(right.facade.kas96:0))',
    bossFacades: '',
    summonHP: 12,
    summonSD: '15-04:15-04:34-03:34-03:36-02:36-02',
    summonSprite: SPRITES.Rayquaza,
    summonCol: 'g',
    summonItems: '.i.((t.jinx.(alliteme.Full Moon))#t.Housecat)',
    speech: 'Rayquaza!:[wiggle]Roars',
    doc: 'The Sky High Pokemon. Commands the ozone layer and quells the ancient titans.',
    flavorDoc: 'The sky darkens and the wind howls[dot][dot][dot] This might be a bad idea',
    itemName: 'Jade Orb',
    itemTier: 9,
    itemSprite: LEGENDARY_ITEM_SPRITES['Jade Orb'],
  },
  // Latias — Shield/Heal/Dodge support (Tier 7)
  {
    name: 'Latias',
    bossHP: 50,
    bossSD: '56-08:56-08:103-05:103-05:0:0',
    bossSprite: SPRITES.Latias,
    bossItems: '.i.((left.(blindfold#k.descend))#(col.k.fierce))',
    bossFacades: '.i.((right2.facade.kas94:0)#(facade.Eme22:0)#(topbot.facade.kas95:0)#(left.facade.Eme43:70:-60:0))',
    summonHP: 8,
    summonSD: '56-03:56-03:103-02:103-02:15-02:15-02',
    summonSprite: SPRITES.Latias,
    summonCol: 'r',
    summonItems: '.i.(quicksilver#t.Housecat)',
    speech: '[i]Lati~!:Latias!',
    doc: 'The Eon Pokemon. Empathically shields allies and soothes their wounds.',
    flavorDoc: 'A warm presence fills the air[dot][dot][dot] This might be a bad idea',
    itemName: 'Soul Dew',
    itemTier: 8,
    itemSprite: LEGENDARY_ITEM_SPRITES['Soul Dew'],
  },
  // Latios — Ranged/Cleave damage (Tier 7)
  {
    name: 'Latios',
    bossHP: 50,
    bossSD: '15-08:15-08:36-05:36-05:34-03:34-03',
    bossSprite: SPRITES.Latios,
    bossItems: '.i.((left.(blindfold#k.descend))#(col.k.fierce))',
    bossFacades: '.i.((right2.facade.kas94:0)#(facade.Eme22:0)#(topbot.facade.kas95:0)#(left.facade.Eme43:70:-60:0))',
    summonHP: 8,
    summonSD: '15-04:15-04:36-02:36-02:34-02:34-02',
    summonSprite: SPRITES.Latios,
    summonCol: 'b',
    summonItems: '.i.(quicksilver#t.Housecat)',
    speech: 'Latios!:[sin]Whoosh!',
    doc: 'The Eon Pokemon. Strikes from afar with devastating psychic lances.',
    flavorDoc: 'A sharp cry echoes overhead[dot][dot][dot] This might be a bad idea',
    itemName: 'Eon Flute',
    itemTier: 8,
    itemSprite: LEGENDARY_ITEM_SPRITES['Eon Flute'],
  },
  // Suicune — Shield/Heal/Cleanse (Tier 7)
  {
    name: 'Suicune',
    bossHP: 55,
    bossSD: '56-08:56-08:103-05:103-05:111-03:111-03',
    bossSprite: SPRITES.Suicune,
    bossItems: '.i.((right2.sticker.water#togfri#togtime#sidesc.Give the effects of Water this fight#k.cleave#k.inflictexert))',
    bossFacades: '.i.((left2.facade.dar14:0)#(topbot.facade.dar12:40:99:10)#(right2.facade.dar15:0))',
    summonHP: 10,
    summonSD: '56-03:56-03:103-03:103-03:111-02:111-02',
    summonSprite: SPRITES.Suicune,
    summonCol: 's',
    summonItems: '.i.t.Housecat',
    speech: 'Suicune!:[i]Purify~',
    doc: 'The Aurora Pokemon. Purifies water and protects allies with crystalline shields.',
    flavorDoc: 'Crystal-clear water ripples in the distance[dot][dot][dot] This might be a bad idea',
    itemName: 'Clear Bell',
    itemTier: 8,
    itemSprite: LEGENDARY_ITEM_SPRITES['Clear Bell'],
  },
  // Entei — Damage/AoE + Heavy (Tier 7)
  {
    name: 'Entei',
    bossHP: 55,
    bossSD: '15-10:15-10:34-05:34-05:15-05:15-05',
    bossSprite: SPRITES.Entei,
    bossItems: '.i.k.heavy.i.(topbot.(hat.troll king#(eye of horus.m.4))#facade.dar12:0)',
    bossFacades: '.i.((left2.facade.kas96:0)#(right.facade.kas96:0))',
    summonHP: 9,
    summonSD: '15-04:15-04:34-03:34-03:15-03:15-03',
    summonSprite: SPRITES.Entei,
    summonCol: 'm',
    summonItems: '.i.k.heavy.i.t.Housecat',
    speech: 'Entei!:[wiggle]ROAR!',
    doc: 'The Volcano Pokemon. Erupts with sacred fire that scorches all enemies.',
    flavorDoc: 'Volcanic heat radiates from deep below[dot][dot][dot] This might be a bad idea',
    itemName: 'Flame Plate',
    itemTier: 8,
    itemSprite: LEGENDARY_ITEM_SPRITES['Flame Plate'],
  },
  // Raikou — Charged/Engage burst (Tier 7)
  {
    name: 'Raikou',
    bossHP: 55,
    bossSD: '42-10:42-10:42-05:42-05:17-05:17-05',
    bossSprite: SPRITES.Raikou,
    bossItems: '.i.k.fierce',
    bossFacades: '.i.((left2.facade.dar14:0)#(topbot.facade.dar12:0)#(right2.facade.dar15:0))',
    summonHP: 9,
    summonSD: '42-04:42-04:42-03:42-03:17-03:17-03',
    summonSprite: SPRITES.Raikou,
    summonCol: 'y',
    summonItems: '.i.t.Housecat',
    speech: 'Raikou!:[sin]THUNDER!',
    doc: 'The Thunder Pokemon. Strikes with lightning speed and devastating charged bolts.',
    flavorDoc: 'Thunder crackles in a cloudless sky[dot][dot][dot] This might be a bad idea',
    itemName: 'Zap Plate',
    itemTier: 8,
    itemSprite: LEGENDARY_ITEM_SPRITES['Zap Plate'],
  },
];

// ============================================================
// UPGRADE/REPLACE DEFINITIONS
// ============================================================

// Upgrades: find Pokemon by old name, replace name/sprite/sd with evolved form
const upgrades = [
  {
    oldName: 'Caterpie',
    newName: 'Butterfree',
    newHP: 4,
    newSD: '103-2:103-2:56-1:56-1:15-2:15-2',
    newSprite: SPRITES.Butterfree,
    newCol: 't',
    newSpeech: '[i]Flutter~:Free!',
    newItems: '',
  },
  {
    oldName: 'Sneasel',
    newName: 'Weavile',
    newHP: 5,
    newSD: '15-3:15-3:15-2:15-2:42-2:42-2',
    newSprite: SPRITES.Weavile,
    newCol: 'p',
    newSpeech: 'Weavile!:[sin]Slash!',
    newItems: '.i.k.fierce',
  },
  {
    oldName: 'Barboach',
    newName: 'Whiscash',
    newHP: 6,
    newSD: '15-3:15-3:56-2:56-2:15-2:15-2',
    newSprite: SPRITES.Whiscash,
    newCol: 'b',
    newSpeech: '[i]Whiskers!:Cash!',
    newItems: '',
  },
  {
    oldName: 'Electrike',
    newName: 'Manectric',
    newHP: 7,
    newSD: '15-3:15-3:42-3:42-3:15-2:15-2',
    newSprite: SPRITES.Manectric,
    newCol: 'y',
    newSpeech: 'MANECTRIC!:[sin]Thunder!',
    newItems: '.i.k.fierce',
  },
];

// Replacements: find Pokemon by old name, replace entire capture block with new Pokemon
const replacements = [
  {
    oldName: 'Rattata',
    newName: 'Skarmory',
    newHP: 6,
    newSD: '56-3:56-3:15-2:15-2:56-2:56-2',
    newSprite: SPRITES.Skarmory,
    newCol: 'g',
    newSpeech: 'Skar!:[sin]Steel!',
    newBallName: null,  // keep the same ball
    newBallTier: null,
    newBallSprite: null,
    newItems: '',
  },
  {
    oldName: 'Furret',
    newName: 'Lapras',
    newHP: 8,
    newSD: '103-3:103-3:56-2:56-2:15-3:15-3',
    newSprite: SPRITES.Lapras,
    newCol: 's',
    newSpeech: '[i]Lapras~:Sail!',
    newBallName: null,
    newBallTier: null,
    newBallSprite: null,
    newItems: '',
  },
  {
    oldName: 'Alcremie',
    newName: 'Arcanine',
    newHP: 7,
    newSD: '15-4:15-4:15-3:15-3:15-2:15-2',
    newSprite: SPRITES.Arcanine,
    newCol: 'm',
    newSpeech: 'Arcanine!:[wiggle]Blaze!',
    newBallName: null,
    newBallTier: null,
    newBallSprite: null,
    newItems: '.i.k.heavy',
  },
];

// Removals (from Lines 121/123)
const removals = ['Ivysaur', 'Pikachu', 'Charizard', 'Metagross', 'Poliwag', 'Zubat'];

// Removal from Line 63 only
const removalsLine63 = ['Arceus'];

// ============================================================
// PARSING HELPERS
// ============================================================

/**
 * Split a line into top-level capture blocks.
 * Captures are separated by '+' at the top level (outside nested parentheses).
 * The line starts with 'itempool.' prefix.
 *
 * Returns { prefix, blocks, suffix } where:
 *   prefix = 'itempool.'
 *   blocks = array of capture block strings (without the '+' separators)
 *   suffix = trailing part after last block (e.g., '.part.1.mn.Pokeballs Part 1,')
 */
function parseCaptureLine(line) {
  // Extract the itempool prefix
  const prefixMatch = line.match(/^(itempool\.)/);
  if (!prefixMatch) {
    console.error('ERROR: Line does not start with itempool.');
    return null;
  }
  const prefix = prefixMatch[1];
  let rest = line.slice(prefix.length);

  // Find the suffix — the .part.1.mn. portion at the very end
  // The suffix starts after the last closing of a top-level block
  // We need to find where the capture blocks end and the pool metadata begins
  const suffixMatch = rest.match(/\.part\.\d+\.mn\.[^,]+,$/);
  let suffix = '';
  if (suffixMatch) {
    suffix = suffixMatch[0];
    rest = rest.slice(0, rest.length - suffix.length);
  }

  // Now split 'rest' by '+' at parenthesis depth 0
  const blocks = [];
  let depth = 0;
  let current = '';

  for (let i = 0; i < rest.length; i++) {
    const ch = rest[i];
    if (ch === '(') depth++;
    else if (ch === ')') depth--;

    if (ch === '+' && depth === 0) {
      blocks.push(current);
      current = '';
    } else {
      current += ch;
    }
  }
  if (current) blocks.push(current);

  return { prefix, blocks, suffix };
}

/**
 * Extract the Pokemon name from a capture block.
 * Looks for the pattern: .n.Find a [Name] that can join your team
 * or .n.Find a [Name].
 */
function getCaptureBlockName(block) {
  // Try "Find a X that can join your team" first
  let m = block.match(/\.n\.Find a (\w+) that can join your team/);
  if (m) return m[1];
  // Try "Find a X." pattern
  m = block.match(/\.n\.Find a (\w+)\./);
  if (m) return m[1];
  // Try "Find a X" (no period)
  m = block.match(/\.n\.Find a (\w+)/);
  if (m) return m[1];
  return null;
}

/**
 * Extract ball info from the end of a capture block.
 * Pattern: .n.[Ball Name].tier.[N].img.[sprite]
 */
function extractBallInfo(block) {
  // Ball info is at the very end of the block, after the last .n. that names the ball
  // Pattern: .n.BALLNAME.tier.N.img.SPRITE
  const m = block.match(/\.n\.([^.]+\s*Ball[^.]*?)\.tier\.(\d+)\.img\.([^\s+]+)$/);
  if (m) {
    return { ballName: m[1], ballTier: parseInt(m[2]), ballSprite: m[3] };
  }
  return null;
}

/**
 * Replace a capture block entirely with a new one for a different Pokemon,
 * preserving the ball info from the original.
 */
function replaceCapture(block, repl) {
  const ballInfo = extractBallInfo(block);
  if (!ballInfo) {
    console.error(`WARNING: Could not extract ball info for ${repl.oldName}, using defaults`);
  }
  const bName = repl.newBallName || (ballInfo ? ballInfo.ballName : 'Poke Ball');
  const bTier = repl.newBallTier || (ballInfo ? ballInfo.ballTier : 6);
  const bSprite = repl.newBallSprite || (ballInfo ? ballInfo.ballSprite : '2eb000ccc');

  return buildCapture({
    name: repl.newName,
    hp: repl.newHP,
    sd: repl.newSD,
    sprite: repl.newSprite,
    col: repl.newCol,
    speech: repl.newSpeech,
    ballName: bName,
    ballTier: bTier,
    ballSprite: bSprite,
    items: repl.newItems || '',
    previewItems: '',
  });
}

/**
 * Upgrade a capture block: replace the Pokemon name, sd, sprite, etc.
 * while keeping the ball wrapper intact.
 */
function upgradeCapture(block, upg) {
  const ballInfo = extractBallInfo(block);
  if (!ballInfo) {
    console.error(`WARNING: Could not extract ball info for ${upg.oldName}, using defaults`);
  }
  const bName = ballInfo ? ballInfo.ballName : 'Poke Ball';
  const bTier = ballInfo ? ballInfo.ballTier : 6;
  const bSprite = ballInfo ? ballInfo.ballSprite : '2eb000ccc';

  return buildCapture({
    name: upg.newName,
    hp: upg.newHP,
    sd: upg.newSD,
    sprite: upg.newSprite,
    col: upg.newCol,
    speech: upg.newSpeech,
    ballName: bName,
    ballTier: bTier,
    ballSprite: bSprite,
    items: upg.newItems || '',
    previewItems: '',
  });
}

// ============================================================
// LINE 63 HELPERS
// ============================================================

/**
 * Process Line 63 (Lillipup line) to remove Arceus capture and upgrade Caterpie.
 *
 * The Lillipup line has captures embedded using @4i separators.
 * The Arceus capture starts with: (hat.(replica.thief.i.(all.(left.hat.egg.(dragon.n.Arceus...
 * The Caterpie capture starts with: (hat.(replica.thief.i.(all.(left.hat.egg.(wolf.n.Caterpie...
 *
 * These are separated by @4i within the mheropool section.
 */
function processLine63(line) {
  // Split by @4i to get segments
  const segments = line.split('@4i');

  const newSegments = [];
  for (let i = 0; i < segments.length; i++) {
    const seg = segments[i];

    // Check if this segment contains the Arceus capture
    if (seg.includes('.n.Arceus') || seg.includes('n.Find a Arceus')) {
      console.log('  Removing Arceus capture from Line 63');
      continue; // Skip this segment entirely
    }

    // Check if this segment contains the Caterpie capture (on line 63)
    if (seg.includes('.n.Caterpie') && seg.includes('n.Find a Caterpie')) {
      console.log('  Upgrading Caterpie -> Butterfree on Line 63');

      // Build a new Butterfree capture in the line 63 style (uses @4i format, different from pool format)
      // The line 63 captures use: .i.k.potion.i.k.onesie pattern (not .i.k.potion.i.all.facade)
      // and have .i.k.stasis at the end (without #(Handcuffs.part.1))
      let newSeg = seg;

      // Replace all instances of Caterpie name
      newSeg = newSeg.replace(/\.n\.Caterpie/g, '.n.Butterfree');
      newSeg = newSeg.replace(/n\.Find a Caterpie/g, 'n.Find a Butterfree');
      newSeg = newSeg.replace(/\.mn\.Caterpie/g, '.mn.Butterfree');

      // Replace HP
      newSeg = newSeg.replace(/\.hp\.2/g, '.hp.4');

      // Replace SD (Caterpie: 170-2 -> Butterfree: 103-2:103-2:56-1:56-1:15-2:15-2)
      newSeg = newSeg.replace(/\.sd\.170-2/g, '.sd.103-2:103-2:56-1:56-1:15-2:15-2');

      // Replace sprites
      const caterpieSprite = '342i4k3vcUc3z4Vc3C4Wc2me000adftIrjmkpApPKyQUFnumKjmUnnAMwDBrUZIFRFXXXWfXWebdX3c86EQoNcX1c94wA6qc063cgIg4wAlw4EQg4AkA42ckps4yB6wBw41ck6k87wBx62cIiiga4y6UcI6kb868vsgIlgb8gavsgubY8v8sjgs4g9E8z9thc4g8shEz8k8gtc4gc9E8zk8k9h6g8gCogkagckI6kCo8E9g5mo5XcEaiM4p4lCg7koio4Mo4Xiog7c7X4s48gsclpI4V';
      // Use string split/join for safe replacement of the sprite strings
      newSeg = newSeg.split(caterpieSprite).join(SPRITES.Butterfree);

      // Replace col
      newSeg = newSeg.replace(/\.col\.t\./g, '.col.t.');

      // Replace equipment (Caterpie has (eye of horus#chainmail), upgrade to 6-face equipment)
      newSeg = newSeg.replace(
        /\.i\.\(eye of horus#chainmail\)/gi,
        '.i.(Chainmail#row.hat.Statue.sd.15-3:15-2:0:0:15-2#(topbot.sticker.(right2.hat.Statue)#togfri#facade.dar2:95:50:30)#Kilt)'
      );

      // Update speech
      newSeg = newSeg.replace(/\.speech\.\[sin\]Wiggle/g, '.speech.[i]Flutter~:Free!');

      newSegments.push(newSeg);
      continue;
    }

    newSegments.push(seg);
  }

  return newSegments.join('@4i');
}

// ============================================================
// MAIN PROCESSING
// ============================================================

function main() {
  console.log('Reading textmod_expanded.txt...');
  const content = fs.readFileSync(TEXTMOD_PATH, 'utf-8');
  const lines = content.split('\n');

  console.log(`Total lines: ${lines.length}`);

  // ---- Process Line 63 (0-indexed: 62) ----
  console.log('\n=== Processing Line 63 (Lillipup / Arceus / Caterpie) ===');
  lines[62] = processLine63(lines[62]);

  // ---- Process Line 121 (0-indexed: 120) — Capture Pool 1 ----
  console.log('\n=== Processing Line 121 (Capture Pool 1) ===');
  const parsed121 = parseCaptureLine(lines[120]);
  if (!parsed121) {
    console.error('FATAL: Failed to parse Line 121');
    process.exit(1);
  }
  console.log(`  Found ${parsed121.blocks.length} capture blocks on Line 121`);

  let blocks121 = parsed121.blocks.filter(block => {
    const name = getCaptureBlockName(block);
    if (!name) return true; // Keep blocks we can't identify
    if (removals.includes(name)) {
      console.log(`  Removing: ${name}`);
      return false;
    }
    return true;
  });

  // Apply upgrades on Line 121
  blocks121 = blocks121.map(block => {
    const name = getCaptureBlockName(block);
    if (!name) return block;

    const upg = upgrades.find(u => u.oldName === name);
    if (upg) {
      console.log(`  Upgrading: ${upg.oldName} -> ${upg.newName}`);
      return upgradeCapture(block, upg);
    }

    const repl = replacements.find(r => r.oldName === name);
    if (repl) {
      console.log(`  Replacing: ${repl.oldName} -> ${repl.newName}`);
      return replaceCapture(block, repl);
    }

    return block;
  });

  // Add first 4 new captures to Line 121
  const capturesForLine121 = newCaptures.slice(0, 4);
  for (const cap of capturesForLine121) {
    console.log(`  Adding new capture: ${cap.name}`);
    blocks121.push(buildCapture(cap));
  }

  lines[120] = parsed121.prefix + blocks121.join('+') + parsed121.suffix;
  console.log(`  Line 121 now has ${blocks121.length} capture blocks`);

  // ---- Process Line 123 (0-indexed: 122) — Capture Pool 2 ----
  console.log('\n=== Processing Line 123 (Capture Pool 2) ===');
  const parsed123 = parseCaptureLine(lines[122]);
  if (!parsed123) {
    console.error('FATAL: Failed to parse Line 123');
    process.exit(1);
  }
  console.log(`  Found ${parsed123.blocks.length} capture blocks on Line 123`);

  let blocks123 = parsed123.blocks.filter(block => {
    const name = getCaptureBlockName(block);
    if (!name) return true;
    if (removals.includes(name)) {
      console.log(`  Removing: ${name}`);
      return false;
    }
    return true;
  });

  // Apply upgrades on Line 123
  blocks123 = blocks123.map(block => {
    const name = getCaptureBlockName(block);
    if (!name) return block;

    const upg = upgrades.find(u => u.oldName === name);
    if (upg) {
      console.log(`  Upgrading: ${upg.oldName} -> ${upg.newName}`);
      return upgradeCapture(block, upg);
    }

    const repl = replacements.find(r => r.oldName === name);
    if (repl) {
      console.log(`  Replacing: ${repl.oldName} -> ${repl.newName}`);
      return replaceCapture(block, repl);
    }

    return block;
  });

  // Add remaining 4 new captures to Line 123
  const capturesForLine123 = newCaptures.slice(4);
  for (const cap of capturesForLine123) {
    console.log(`  Adding new capture: ${cap.name}`);
    blocks123.push(buildCapture(cap));
  }

  lines[122] = parsed123.prefix + blocks123.join('+') + parsed123.suffix;
  console.log(`  Line 123 now has ${blocks123.length} capture blocks`);

  // ---- Add Legendary Items after Line 127 (0-indexed: 126) ----
  console.log('\n=== Adding Legendary Item Lines after Line 127 ===');

  // Build a new legendary summon pool line.
  // The format from Lines 125/127 is: itempool.((legendary1)+(legendary2)).part.1.mn.Pool Name,
  // We'll add one new line for the 6 new legendaries, split into 2 pools of 3

  // Pool 3: Rayquaza, Latias, Latios
  const legendaryPool3Blocks = newLegendaries.slice(0, 3).map(leg => {
    return '(' + buildLegendaryItem(leg);
  });
  const legendaryLine3 = 'itempool.' + legendaryPool3Blocks.join('+') + '.part.1.mn.Summons Part 3,';

  // Pool 4: Suicune, Entei, Raikou
  const legendaryPool4Blocks = newLegendaries.slice(3).map(leg => {
    return '(' + buildLegendaryItem(leg);
  });
  const legendaryLine4 = 'itempool.' + legendaryPool4Blocks.join('+') + '.part.1.mn.Summons Part 4,';

  // Insert after line 127 (0-indexed: 126)
  // We'll insert at position 127 (after index 126)
  lines.splice(127, 0, legendaryLine3, legendaryLine4);
  console.log('  Added Summons Part 3 (Rayquaza, Latias, Latios) as new line');
  console.log('  Added Summons Part 4 (Suicune, Entei, Raikou) as new line');

  // ---- Write output ----
  console.log('\n=== Writing modified textmod_expanded.txt ===');
  const output = lines.join('\n');
  fs.writeFileSync(TEXTMOD_PATH, output, 'utf-8');
  console.log(`Done! Total lines: ${lines.length}`);

  // ---- Summary ----
  console.log('\n=== SUMMARY ===');
  console.log('Removals from capture pools: Ivysaur, Pikachu, Charizard, Metagross, Poliwag, Zubat');
  console.log('Removal from Line 63: Arceus');
  console.log('Upgrades: Caterpie->Butterfree, Sneasel->Weavile, Barboach->Whiscash, Electrike->Manectric');
  console.log('Replacements: Rattata->Skarmory, Furret->Lapras, Alcremie->Arcanine');
  console.log('New captures (Line 121): Mew, Jirachi, Kangaskhan, Heracross');
  console.log('New captures (Line 123): Greninja, Electivire, Magmortar, Rhyperior');
  console.log('New legendaries: Rayquaza/Jade Orb, Latias/Soul Dew, Latios/Eon Flute, Suicune/Clear Bell, Entei/Flame Plate, Raikou/Zap Plate');
}

main();
