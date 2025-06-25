<div align="center">
<h1 style="font-size: 50px">X-CASL2</h1>
<p>基本情報技術者試験で登場するCPUのエミュレーター、アセンブラー、Editor</p>
</div>



# ToDo
## first
- [x] エミュレーターの実装
## sec
- [ ] デバッグ
- [ ] アセンブラーの実装
## third
- [ ] wasmへマッピング
- [ ] ブラウザでGUI付
- [ ] Editorの実装

# これは何？
Commet2 およびそのアセンブリ Casl2 のエミュレーター、アセンブラー、Editorです。  

Rustで書かれており高速で高効率です。

# 構造
X-CASL2はcommet2のハードウウェアの構造と状態を構造体で定義し、  
ステートマシン（状態遷移機械）として構成されます。  

- 状態定義  
  `src/emurator/commet2/state.rs`
- ハードウウェア定義  
  `src/emurator/commet2/cpu.rs`  
  `src/emurator/commet2/alu.rs`  
  `src/emurator/commet2/decoder.rs`  

`commet2_step(self<CPU>) -> UpdateNotify`  
より状態をハードウウェアセル単位で進めます。  

上記のwrapper fn で  
`casl_step(self<CPU>) -> ()`  
よりcasl2の命令単位で進めます。  

# 貢献
プルリクまってます♡