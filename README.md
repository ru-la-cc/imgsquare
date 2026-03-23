# imgsquare

VRChatのワールド「The Avatar Studio ｜ Avatown」でThumbnail撮影した画像のアバターの正方形部分だけ切り出すだけの自分用のツールです。

---

## ビルド方法

Rustの環境があればOKです。

```bash
cargo build --release
```

`target/release/imgsquare`（Windowsなら`imgsquare.exe`）が生成されます。

---

## 使用クレート

| クレート | バージョン | 用途 |
|---|---|---|
| [image](https://crates.io/crates/image) | 0.25 | 画像の読み込み・リサイズ・クロップ・保存 (PNG / JPEG) |
| [clap](https://crates.io/crates/clap) | 4.5 | コマンドライン引数のパース |
| [serde](https://crates.io/crates/serde) | 1 | 設定ファイルのシリアライズ/デシリアライズ |
| [serde_json](https://crates.io/crates/serde_json) | 1 | 設定ファイルのJSON読み書き |
| [anyhow](https://crates.io/crates/anyhow) | 1 | エラーハンドリング |

---

## 使い方

```
Usage: imgsquare [OPTIONS] [IMAGEPATH]
```

### 引数

| 引数 | 説明 |
|---|---|
| `[IMAGEPATH]` | 切り出し元の画像ファイルのパス |

### オプション

| オプション | 説明 |
|---|---|
| `-d, --outdir <OUTDIR>` | 出力先フォルダのパス。省略時は入力画像と同じフォルダに出力 |
| `-o, --outimage <OUTIMAGE>` | 出力ファイル名。省略時は `元ファイル名_サイズ.拡張子` |
| `-p, --position <POSITION>` | 切り出し位置。`start` / `center` / `end` のいずれかを指定 |
| `-s, --size <SIZE>` | 出力する正方形のサイズ（ピクセル）。デフォルトは `1080` |
| `-e, --expand [true\|false]` | 入力画像の短辺が `size` 未満のとき、`size` まで拡大するか否か |
| `-c, --config <CONFIG>` | 設定ファイルのパス。省略時は実行ファイルと同フォルダの `imgsquare.conf` |
| `-h, --help` | ヘルプを表示 |
| `-V, --version` | バージョンを表示 |

### `--position` について

切り出し位置は画像の「長辺方向」で決まります。

- **縦長画像**（高さ > 幅）なら、上・中央・下
- **横長画像**（幅 > 高さ）なら、左・中央・右

| 値 | 縦長 | 横長 |
|---|---|---|
| `start` | 上から | 左から |
| `center` | 中央 | 中央 |
| `end` | 下から | 右から |

### `--expand` について

デフォルトは `false`（拡大しない）。  
フラグだけ渡す（`-e`）と `true` 扱いになります。

```bash
# フラグだけで true になる
imgsquare photo.png -e

# 明示的に指定もできる
imgsquare photo.png -e true
imgsquare photo.png -e false
```

### 設定ファイル (`imgsquare.conf`)

初回起動時に実行ファイルと同じフォルダに `imgsquare.conf` が自動生成されます。  
よく使う設定はここに書いておくと楽です。コマンドライン引数は設定ファイルより優先されます。

```json
{
  "imagepath": null,
  "outdir": "",
  "outimage": "",
  "position": "Center",
  "size": 1080,
  "expand": false
}
```

---

## 使用例

### 基本：exeに画像をドラッグ＆ドロップ

**一番シンプルな使い方はこれ。** exeファイルに対象の画像をD&Dするだけです。

エクスプローラーの「送る」メニューから使いたい場合は、`imgsquare.exe` のショートカットを以下のフォルダに置いておくと便利です。

```
%APPDATA%\Microsoft\Windows\SendTo
```

デフォルト設定では以下の動作になります：

- 切り出し位置：`start`（左端から）
- 出力サイズ：`1080x1080`
- 出力先：**元画像と同じフォルダ**
- 出力ファイル名：`元ファイル名_サイズ.拡張子`

同名ファイルが既に存在する場合はファイル名に連番が付きます。

```
avatar.png       → avatar_1080.png
avatar_1080.png が既にある → avatar_1080 (1).png
avatar_1080 (1).png も既にある → avatar_1080 (2).png
```

### コマンドラインで使う場合

```bash
# デフォルト（左端から 1080x1080 で切り出し）
imgsquare photo.png

# 出力先フォルダと出力ファイル名を指定
imgsquare photo.png -d ./output -o thumb.png

# サイズを 1024 に変えて、足りなければ拡大する
imgsquare photo.png -s 1024 -e

# 中央から切り出したい場合
imgsquare photo.png -p center
```

---

## ライセンス

[MIT](./LICENSE)