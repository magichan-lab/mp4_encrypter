# MP4 Encrypter

MP4 Encrypter は、MP4 ファイルの暗号化を行うデスクトップアプリです。

### 基本動作

- 暗号化キーを入力した上で ウインドウにMP4ファイルをドラッグ&ドロップすることでファイルが暗号化されます。

### 対象暗号化方式

- CENC (Common Encryption) 
- 鍵長: 128ビット（16バイト）

### キー入力仕様

- 入力可能文字は 16 進数 (`0-9`, `a-f`, `A-F`) のみで、最大 32 文字までとなります。
- 32 文字未満の場合は 0 埋めして 16 バイトデータとして復号に使用します。

### 出力ファイル

- 出力ファイル名は入力ファイル名に `_enc` を付与した `*_enc.mp4` 形式です。

### 前提

- Rust / Cargo
- FFmpeg 開発用ファイル
  - `include`
  - `lib`
- Windows 向けビルドでは、FFmpeg の共有ライブラリと依存 DLL が実行環境から参照できること

### ビルド時の FFmpeg の配置方法

ビルド時の FFmpeg 探索順は次の通りです。

1. 環境変数 `FFMPEG_DIR` が定義されている場合
   - `FFMPEG_DIR/include`
   - `FFMPEG_DIR/lib`
2. `FFMPEG_DIR` が未定義の場合
   - `third_party/ffmpeg/include`
   - `third_party/ffmpeg/lib`

### セットアップ例

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

### 開発時の確認コマンド

```bash
cargo fmt --check
cargo test
cargo build
```

> `build.rs` は `FFMPEG_DIR` または `third_party/ffmpeg` に有効な FFmpeg の `include` / `lib` が無い場合は失敗します。

## ライセンス

本リポジトリのソースコードは **MIT License** です。詳細は `LICENSE` を参照してください。

## サードパーティライブラリ

このソフトウェアはFFmpegライブラリ（LGPL v2.1以降）を使用しています：
- libavformat
- libavcodec
- libavutil
- libswresample
- libswscale

FFmpegソースコード：
https://ffmpeg.org/

Note：
- 本アプリケーションは外部ライブラリとして **FFmpeg** を **動的リンク** または **静的リンク** で利用します。
FFmpeg 自体は本リポジトリの MIT ライセンスには含まれず、**LGPL-2.1-or-later** の条件に従います。
配布時のクレジットと注意事項は `THIRD_PARTY_NOTICES.md` を参照してください。
- ユーザーはFFmpegを修正版に置き換えてアプリケーションを再リンクすることができます。
ビルド手順はこのリポジトリに記載されています。
