# internal-architecture

この文書は `mp4_encrypter` の内部仕様（アーキテクチャと処理フロー）をまとめたものです。

## 1. 全体構成

本アプリは以下の 4 層で構成されています。

- `presentation`: UI 表示、入力イベント、状態遷移（MVI）
- `application`: ユースケース、ワーカー実行制御
- `domain`: エンティティ、値オブジェクト、エラー、純粋ロジック
- `infrastructure`: FFmpeg / CLI など外部依存の実装

エントリポイントは `src/main.rs` で、`AppRuntime` が `presentation` と `application` の橋渡しを担当します。

## 2. モジュール詳細

### 2.1 presentation 層

主なファイル:

- `state.rs`: 画面状態 `AppModel` / `UiState` / `SessionState`
- `intent.rs`: reducer 入力 `Intent` と副作用 `Effect`
- `reducer.rs`: 状態遷移ロジック（MVI）
- `view.rs`: iced での描画
- `subscription.rs`: FileDropped と Tick の購読
- `message.rs`: UI イベントメッセージ

設計ポイント:

- reducer は「状態更新 + 副作用命令（Effect）の列挙」のみを行う。
- 実際の副作用（ファイル解析、ワーカー起動）は `main.rs` 側の `run_effect` で実行する。
- `AppStatus::Inspecting` 中は進捗バーの代わりにスピナー相当表示を出し、解析待ちを明示する。

### 2.2 application 層

主なファイル:

- `use_cases.rs`
  - `InspectFileUseCase`: MP4 が暗号化済みか判定
  - `ValidateOutputPathUseCase`: 出力先ファイル重複チェック
- `runtime.rs`
  - `DecryptionRuntime`: ワーカー開始/一時停止/再開/中断、イベント回収
- `worker.rs`
  - バックグラウンドスレッドで暗号化処理を実行
  - `WorkerEvent::{Progress, Finished}` を crossbeam-channel で通知

設計ポイント:

- UI スレッドでは重い処理を直接実行しない。
- ファイル解析は `Task::perform` で非同期化。
- 暗号化本処理はワーカースレッドで実行し、UI はイベントをポーリングして反映する。

### 2.3 domain 層

主な要素:

- `entities.rs`
  - `LaunchRequest`, `FileEncryptionState`, `DecryptionProgress`, `DecryptionResult`
- `value_objects.rs`
  - キー正規化・検証
- `errors.rs`
  - `AppError`（UI 表示向けメッセージを提供）
- `services.rs`
  - 出力ファイル名生成などのドメインサービス

設計ポイント:

- `domain` は I/O 非依存。
- 文字列やパスのバリデーションをアプリ全体で再利用可能にする。

### 2.4 infrastructure 層

主なファイル:

- `ffmpeg/repository.rs`
  - `Mp4ProcessingPort` 実装
  - MP4 解析（暗号化マーカー判定）
  - FFmpeg を使った暗号化処理（remux）
- `cli.rs`
  - 起動引数解析（キー / ファイルパス）

設計ポイント:

- FFmpeg C API 呼び出しをこの層に閉じ込める。
- `application` からは `Mp4ProcessingPort` 経由で抽象化して利用する。

## 3. 実行フロー

### 3.1 通常起動

1. `main.rs` の `initialize()` が `CliLaunchArgumentParser` で引数解析
2. `Intent::LaunchParsed` を reducer に渡す
3. 必要に応じて `Effect::InspectFile` または `Effect::StartDecryption` を発行

### 3.2 ドラッグ&ドロップ

1. `subscription.rs` が `Window::FileDropped` を `Message::FileDropped` に変換
2. `update()` が `Intent::FileDropped` を dispatch
3. reducer が `AppStatus::Inspecting` に遷移し `Effect::InspectFile` を返す
4. `run_effect()` が `Task::perform` で解析
5. 完了時 `Message::FileInspectionCompleted` → `Intent::FileInspected`
6. 平文なら暗号化開始、暗号化済みならダイアログ表示

### 3.3 暗号化処理中

1. `Effect::StartDecryption` で `DecryptionRuntime::start_decryption`
2. worker が `WorkerEvent::Progress` を逐次送信
3. `Tick` 時に `drain_worker_events()` で回収し reducer 反映
4. 完了時 `WorkerEvent::Finished` を反映し、`Finished` / `Error` / `Wait` に遷移

## 4. 状態遷移（簡略）

- `Wait` → `Inspecting`: ファイルドロップ直後
- `Inspecting` → `Running`: 解析完了 + 暗号化開始
- `Running` → `Pause`: 実行中の再ドロップ時
- `Pause` → `Running`: 継続選択
- `Running/Pause` → `Wait`: キャンセル完了
- `Running` → `Finished`: 正常終了
- 任意状態 → `Error`: バリデーション/IO/FFmpeg エラー

## 5. 非同期設計の要点

- 解析処理は `Task::perform` で UI スレッド外へオフロード。
- 重い暗号化処理はワーカースレッド（`std::thread::spawn`）で実行。
- UI は Tick + チャネル受信でイベントを取り込むため、描画を止めずに進捗更新できる。

## 6. 既知の注意点

- 型名に `Decryption*` が残っているが、実際の機能は暗号化である。
  - 互換性維持のため現時点では命名を温存している。
- ビルドには FFmpeg の `include/lib` が必須。
