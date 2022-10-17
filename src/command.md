# テキスト音楽 サクラ(Rust版) コマンド一覧

| コマンド | 説明    |
|---------|--------|
|' ' '\t' '\r' ' '| 空白文字|
|'c' 'd' 'e' 'f' 'g' 'a' 'b'| ドレミファソラシ c(音長),(ゲート),(音量)|
|'n'| 番号を指定して発音(例: n36) n(番号),(音長),(ゲート),(音量)|
|'r' '_'| 休符|
|'l'| 音長の指定(例 l4)|
|'o'| 音階の指定(例 o5) 範囲:0-10|
|'p'| ピッチベンドの指定 範囲:0-127 (63が中央)|
|'q'| ゲートの指定 (例 q90) 範囲:0-100|
|'v'| ベロシティ音量の指定 範囲:0-127|
|'y'| コントロールチェンジの指定 (例 y1,100) 範囲:0-127|
|'@'| 音色の指定 範囲:1-128|
|'>'| 音階を1つ上げる|
|'<'| 音階を1つ下げる|
|"//"| 一行コメント|
|"/*" .. "*/"| 範囲コメント|
|'['| ループ開始 (例 [4 cdeg])|
|':'| ループ最終回に脱出 (例　[4 cde:g]e)|
|']'| ループ終了|
|'\''| 和音 (例 'ceg') 'ceg'(音長),(ゲート)|
|'$'| リズムマクロ定義 $文字{定義内容}|
|'{'| 連符 (例 {ceg}4) {c^d}(音長)|
|'`'| 一度だけ音階を+1する|
|'"'| 一度だけ音階を-1する|
|"テンポ"| 定義: "TEMPO="|
|"トラック"| 定義: "TR="|
|"全音符"| 定義: "l1"|
|"二分音符"| 定義: "l2"|
|"四分音符"| 定義: "l4"|
|"八分音符"| 定義: "l8"|
|"十六音符"| 定義: "l16"|
|"付点四分音符"| 定義: "l4."|
|"音長"| 定義: "l"|
|"音符"| 定義: "l"|
|"音階"| 定義: "o"|
|"音色"| 定義: "@"|
|"音量"| 定義: "v"|
|"ゲート"| 定義: "q"|
|"連符"| 定義: "Div"|
|"ド"| 定義: "c"|
|"レ"| 定義: "d"|
|"ミ"| 定義: "e"|
|"ファ"| 定義: "f"|
|"フ"| 定義: "f"|
|"ソ"| 定義: "g"|
|"ラ"| 定義: "a"|
|"シ"| 定義: "b"|
|"ン"| 定義: "r"|
|"ッ"| 定義: "r"|
|"ー"| 定義: "^"|
|"「"| 定義: "'"|
|"」"| 定義: "'"|
|"【"| 定義: "'"|
|"】"| 定義: "'"|
|"↑"| 定義: ">"|
|"↓"| 定義: "<"|
|"ん"| 定義: "r"|
|"ど"| 定義: "n36|
|"た"| 定義: "n38|
|"つ"| 定義: "n42|
|"く"| 定義: "n44|
|"ち"| 定義: "n46|
|"ぱ"| 定義: "n49|
|"TR" "TRACK" "Track"| トラック変更　TR=番号 範囲:1-|
|"CH" "Channel"| チャンネル変更 CH=番号 範囲:1-16|
|"TIME" "Time"| タイム変更 TIME(節:拍:ステップ)|
|"RHYTHM" "Rhythm" "R"| リズムモード|
|"RYTHM" "Rythm"| リズムモード(v1の綴りミス対処[^^;])|
|"DIV" "Div"| 連符 (例 DIV=>ceg} )|
|"SUB" "Sub"| タイムポインタを戻す (例 SUB=>ceg} egb)|
|"M" "Modulation"| モジュレーション 範囲: 0-127|
|"PT" "PortamentoTime"| ポルタメント 範囲: 0-127|
|"V" "MainVolume"| メインボリューム 範囲: 0-127|
|"P" "Panpot	"| パンポット 範囲: 0-63-127|
|"EP" "Expression"| エクスプレッション音量 範囲: 0-127|
|"PS" "PortamentoSwitch"| ポルタメントスイッチ|
|"REV" "Reverb"| リバーブ 範囲: 0-127|
|"CHO" "Chorus"| コーラス 範囲: 0-127|
|"TEMPO" "Tempo" "T"| テンポの指定|
|"TimeSignature" "TimeSig" "TIMESIG"| 拍子の指定|
|"MetaText" "TEXT" "Text"| メタテキスト (例 TEXT=>"abcd"})|
|"COPYRIGHT" "Copyright"| メタテキスト著作権 (例 COPYRIGHT=>"aaa"})|
|"LYRIC" "Lyric"| メタテキスト歌詞 (例 LYRIC=>"aaa"})|