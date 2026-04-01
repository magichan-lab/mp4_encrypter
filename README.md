# MP4 Encrypter

MP4 Encrypter は、MP4 ファイルの暗号化（CENC AES-CTR）を行う Windows 向けデスクトップアプリです。

## 基本動作

1. 暗号化キー（16 進数）を入力する
2. ウィンドウに MP4 ファイルをドラッグ&ドロップする
3. ファイル解析後、暗号化処理が開始される

## 対象暗号化方式

- CENC (Common Encryption)
- 暗号方式: `cenc-aes-ctr`
- 鍵長: 128 ビット（16 バイト）

## キー入力仕様

入力方式は次の 2 種類を選択できます。

### 1) 暗号化キー

- 入力可能文字は 16 進数 (`0-9`, `a-f`, `A-F`) のみです。
- 最大 32 文字まで入力できます（16 バイト）。
- 32 文字未満の場合は 0 埋めして 16 バイトとして暗号化処理に使用します。

### 2) パスフレーズ

- 最大 20 文字まで入力できます。
- 利用可能文字:
  - 英数字: `0-9`, `A-Z`, `a-z`
  - 記号: `! " # $ % & ' ( ) - ^ \\ @ [ ; : ] , . / = ~ | \` { + * } < > ? _`
- 入力パスフレーズの SHA-512 ハッシュを計算し、先頭 16 バイトを暗号化キーとして使用します。

## 出力ファイル

- 出力ファイル名は入力ファイル名に `_enc` を付与した `*_enc.mp4` 形式です。

## 前提

- Rust / Cargo
- FFmpeg 開発用ファイル
  - `include`
  - `lib`
- Windows 向けビルドでは、FFmpeg の共有ライブラリと依存 DLL が実行環境から参照できること

## ビルド時の FFmpeg の配置方法

ビルド時の FFmpeg 探索順は次の通りです。

1. 環境変数 `FFMPEG_DIR` が定義されている場合
   - `FFMPEG_DIR/include`
   - `FFMPEG_DIR/lib`
2. `FFMPEG_DIR` が未定義の場合
   - `third_party/ffmpeg/include`
   - `third_party/ffmpeg/lib`

## セットアップ例

```bash
export FFMPEG_DIR=/opt/ffmpeg
cargo build
```

`FFMPEG_DIR` を利用しない場合は、リポジトリ配下に次の構成で FFmpeg を配置してください。

```text
third_party/
└── ffmpeg/
    ├── include/
    └── lib/
```

## 開発時の確認コマンド

```bash
cargo fmt --check
cargo test
cargo build
```

> `build.rs` は `FFMPEG_DIR` または `third_party/ffmpeg` に有効な FFmpeg の `include` / `lib` が無い場合に失敗します。

## ドキュメント

- 内部仕様: `docs/internal-architecture.md`

## ライセンス

本リポジトリのソースコードは **MIT License** です。詳細は `LICENSE` を参照してください。

## サードパーティライブラリ

このソフトウェアは FFmpeg ライブラリ（LGPL v2.1 以降）を使用しています。

- libavformat
- libavcodec
- libavutil
- libswresample
- libswscale

FFmpeg ソースコード: https://ffmpeg.org/

### 注意

- 本アプリケーションは外部ライブラリとして **FFmpeg** を **動的リンク** または **静的リンク** で利用します。
- FFmpeg 自体は本リポジトリの MIT ライセンスには含まれず、**LGPL-2.1-or-later** の条件に従います。
- 配布時のクレジットと注意事項は `THIRD_PARTY_NOTICES.md` を参照してください。
- ユーザーは FFmpeg を修正版に置き換えてアプリケーションを再リンクできます。
