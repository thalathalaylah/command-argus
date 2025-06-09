# Command Argus プロジェクト概要

## プロジェクト概要

Command Argusは、コマンドライン操作を効率化するためのデスクトップアプリケーションです。頻繁に使用するコマンドをGUIで管理・実行できる機能を提供します。

### 主な機能
- コマンドの保存・管理（名前、説明、タグ付け）
- GUIからのコマンド実行と結果表示
- コマンド使用統計（使用回数、最終使用日時）
- インクリメンタルサーチによるコマンド検索
- 実行環境のカスタマイズ（作業ディレクトリ、環境変数）

## プロジェクト構造

```
command-argus/
├── command-argus-gui/          # Tauriデスクトップアプリケーション
│   ├── src/                    # React TypeScriptフロントエンド
│   │   ├── App.tsx            # メインアプリケーションコンポーネント
│   │   ├── components/        # UIコンポーネント
│   │   │   ├── CommandForm.tsx   # コマンド作成・編集フォーム
│   │   │   └── CommandList.tsx   # コマンド一覧・実行UI
│   │   └── types.ts           # TypeScript型定義
│   ├── src-tauri/             # Rustバックエンド（Tauriブリッジ）
│   │   ├── src/
│   │   │   ├── main.rs        # エントリーポイント
│   │   │   └── lib.rs         # Tauriコマンド定義
│   │   └── tauri.conf.json   # Tauri設定
│   └── package.json           # Node.js依存関係
│
└── command-argus-logic/        # コアロジック（Rustライブラリ）
    └── src/
        ├── lib.rs             # ライブラリエントリーポイント
        ├── command.rs         # コマンドデータ構造
        ├── executor.rs        # コマンド実行ロジック
        ├── storage.rs         # 永続化層
        └── error.rs           # エラー定義
```

## 技術スタック

### フロントエンド
- **Tauri**: クロスプラットフォームデスクトップアプリフレームワーク
- **React + TypeScript**: UIフレームワーク
- **Tailwind CSS**: スタイリング
- **Vite**: ビルドツール

### バックエンド
- **Rust**: コアロジック実装
- **Serde**: シリアライゼーション
- **UUID**: 一意識別子生成
- **Chrono**: 日時処理
- **Directories**: クロスプラットフォームディレクトリ管理

## アーキテクチャ特徴

1. **クリーンアーキテクチャ**: GUI層とロジック層の明確な分離
2. **型安全性**: TypeScriptとRustによる完全な型付け
3. **クロスプラットフォーム**: Windows、macOS、Linux対応
4. **リアクティブUI**: React状態管理による即座の更新
5. **セキュアな実行**: Rustのプロセスエグゼキューターによる安全なコマンド実行

## データモデル

各コマンドは以下の属性を持ちます：
- **ID**: UUID（一意識別子）
- **名前**: ユーザーフレンドリーな表示名
- **コマンド**: 実行ファイルパス
- **引数**: コマンド引数の配列
- **説明**: オプションの説明文
- **作業ディレクトリ**: 実行時のディレクトリ
- **環境変数**: キー・バリューペア
- **タグ**: 分類用タグの配列
- **タイムスタンプ**: 作成日時、更新日時、最終使用日時
- **使用回数**: 実行回数カウンター

## 開発コマンド

### GUI開発
```bash
cd command-argus-gui
npm install              # 依存関係インストール
npm run dev             # 開発サーバー起動
npm run build           # プロダクションビルド
npm run tauri dev       # Tauriアプリ開発モード
npm run tauri build     # Tauriアプリビルド
```

### ロジック開発
```bash
cd command-argus-logic
cargo build             # ビルド
cargo test              # テスト実行
cargo doc --open        # ドキュメント生成
```

## コミット履歴からの主な変更点

- インクリメンタルサーチ機能の実装
- 長いコマンド出力の表示改善（オーバーフロー対応）
- 削除ボタンの修正
- リリースビルド時のPATH問題の修正