# 0. abstruct: 要約
- 普段私はプログラミングでプロダクト制作や課題に取り組む
- しかしその際にサブ目的だけに気を取られ、異様に作業時間が伸びつつも、本来の目的が達成できなくなっていることがある。
- その様子を可視化するツールが欲しいので、Rustでタスクの生え方の様子を確認するツールを作成する。
- The tool to track your intention, or just manage some kind of tree structure, with Command Line Interface.
- This project was started up for my learning the Rust programming language.

# 1. motivation: モチベーション
<!-- そのプロジェクトを完成させないとどのような不利益があるか？ -->
### (1) where: モチベーションが生じた場所や日時は何か？
- 2022/10/24 17:15

### (2) objective: 本来の目的は何か？
- 一般的な問題の解決
  - プログラミングでのプロダクト制作
  - 課題
´
### (3) trouble: 目的を達成するに当たって面倒臭いこと・困ったことは何か？
- 解決のためにサブ目的を追いかけていると、いつの間にか全然違う目的に取り組んでいたことがある
  - 全部自分で作ろうとする主に精神のせい
  - 「ついでに」で色々やりすぎる
  - 結果的に目標が達成できずに終わることも...。
- その時間に課題をやってることはわかっても、その時間の間にそれぞれ何をやってるのかを知ることができない
  - 再起的に問題が深くなっていきそうな気がするが....。

### (4) thought: どう思ったか？
- どういう経路でその目的にたどり着いたかを知りたい

# 2. 手法 -- methodology
### (1) explain: 手法を簡単に説明せよ。
- コマンドラインで、新しいサブ目的が生えるたびに、一行で目的を記録していく
  - git commit的な要領
- めんどくささを低減するため、できる限り手軽に書ける必要がある。

### (2) benefit: 利益は何か。
- 自分自身の思考を可視化し、振り返ることができる
  - 目的が切り替わるタイミングを掴める？
  - もっと簡単な(デメリットを持つ)他の方法に切り替える余地を与えられる。
- 時間配分の使い方を後から見返すことができる
- 造られた木構造が、結果としてそのプロダクトにおけるTodoリストになる。
  - インクリメンタルな開発での計画に強いかもしれない

# 3. difficulties: 技術的困難
<!--
    学習が必要な物
  1. 今まで一度もその言語とフレームワークの組み合わせでプロダクトを作ったことがない物
  2. 当分前に(3年前)触れた技術であり、基礎文法について学習が必要である。
-->
- Rustでのアプリケーション開発
  - チュートリアルでざっと押さえてはいるが...。

# 4. procedure: 手順
方針: 最低限の機能, 使いやすさ
<!-- 開発までの手順を説明する。基本2層以内で分割する -->
- Rustで木のライブラリを探す
  - 自作することは諦める(目的から外れるため)
- 木に対する操作を調べる
- [仕様] 各コマンドを制定する
  - 新たな目的を挿入する 各接点にはIDが振られていることが望ましい
  - 選んだ目的を削除する
  - CUI上でその構造を可視化できる
    - (DFSの実装が必要)
- 時刻記録機能を作成
- 実用
