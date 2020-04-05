# acc
AtCoderのテストや提出するやつ

## ディレクトリ構成
```
<CONTEST_DIR>/
├── A.cpp
├── B.cpp
├── C.cpp
├── D.cpp
├── E.cpp
├── F.cpp
├── config.toml
└── testcase
    ├── A.toml
    ├── B.toml
    ├── C.toml
    ├── D.toml
    ├── E.toml
    └── F.toml
```

## 使い方

### ユーザ情報設定
以下のコマンドでAtCoderのユーザ名とパスワードを登録
```bash
$ acc config <USERNAME> <PASSWORD>
```
- \<config\_dir\>/acc/userdata.tomlが作成されその中に保存される
---

### コンテストプロジェクト作成
\<config\_dir\>/acc/config.tomlとコマンドオプションなどをもとにコンテストのプロジェクトを作成する．

```bash
$ acc init [OPTION] <DIR_NAME>

ex )
$ acc init -e py -l 3023 abc160
```

基本的にディレクトリ名をコンテスト名として使用するため，ディレクトリと異なるコンテストディレクトリを作成したい場合は，コマンド実行後にコンテストディレクトリ内のconfig.tomlを編集する必要がある．

オプションは以下の通り，なお，オプションが指定されていないときは\<config\_dir\>/acc/config.tomlで指定された値で解決しようとする．

| &nbsp;&nbsp;&nbsp;OPTION&nbsp;&nbsp;&nbsp; |                                                            説明                                                            |
| :----------------------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| -e <br>--extension                         | 最初に作成されるファイルの拡張子を指定する, \<config\_dir\>/acc/template.\<extension\>があればテンプレートとして使用される |
| -l <br> --lang                             | AtCoderで提出するときの言語を指定する(詳細は後述)                                                                          |

-l --lang で指定する値の詳細

1. 新システム

| 使用する言語 | 指定する値 |
| :---: | :---: |
| C (GCC 9.2.1) | 4001 |
| C (Clang 10.0.0) | 4002 |
| C++ (GCC 9.2.1) | 4003 |
| C++ (Clang 10.0.0) | 4004 |
| Java (OpenJDK 11.0.6) | 4005 |
| Python (3.8.2) | 4006 |
| Bash (5.0.11) | 4007 |
| bc (1.07.1) | 4008 |
| Awk (GNU Awk 4.1.4) | 4009 |
| C# (.NET Core 3.1.201) | 4010 |
| C# (Mono-mcs 6.8.0.105) | 4011 |
| C# (Mono-csc 3.5.0) | 4012 |
| Clojure (1.10.1.536) | 4013 |
| Crystal (0.33.0) | 4014 |
| D (DMD 2.091.0) | 4015 |
| D (GDC 9.2.1) | 4016 |
| D (LDC 1.20.1) | 4017 |
| Dart (2.7.2) | 4018 |
| dc (1.4.1) | 4019 |
| Erlang (22.3) | 4020 |
| Elixir (1.10.2) | 4021 |
| F# (.NET Core 3.1.201) | 4022 |
| F# (Mono 10.2.3) | 4023 |
| Forth (gforth 0.7.3) | 4024 |
| Fortran(GNU Fortran 9.2.1) | 4025 |
| Go (1.14.1) | 4026 |
| Haskell (GHC 8.8.3) | 4027 |
| Haxe (4.0.3); js | 4028 |
| Haxe (4.0.3); Java | 4029 |
| JavaScript (Node.js 12.16.1) | 4030 |
| Julia (1.4.0) | 4031 |
| Kotlin (1.3.71) | 4032 |
| Lua (Lua 5.3.5) | 4033 |
| Lua (LuaJIT 2.1.0) | 4034 |
| Dash (0.5.8) | 4035 |
| Nim (1.0.6) | 4036 |
| Objective-C (Clang 10.0.0) | 4037 |
| Common Lisp (SBCL 2.0.3) | 4038 |
| OCaml (4.10.0) | 4039 |
| Octave (5.2.0) | 4040 |
| Pascal (FPC 3.0.4) | 4041 |
| Perl (5.26.1) | 4042 |
| Raku (Rakudo 2020.02.1) | 4043 |
| PHP (7.4.4) | 4044 |
| Prolog (SWI-Prolog 8.0.3) | 4045 |
| PyPy2 (7.3.0) | 4046 |
| PyPy3 (7.3.0) | 4047 |
| Racket (7.6) | 4048 |
| Ruby (2.7.1) | 4049 |
| Rust (1.42.0) | 4050 |
| Scala (2.13.1) | 4051 |
| Java (OpenJDK 1.8.0) | 4052 |
| Scheme (Gauche 0.9.9) | 4053 |
| Standard ML (MLton 20130715) | 4054 |
| Swift (5.2.1) | 4055 |
| Text (cat 8.28) | 4056 |
| TypeScript (3.8) | 4057 |
| Visual Basic (.NET Core 3.1.101) | 4058 |
| Zsh (5.4.2) | 4059 |
| COBOL - Fixed (OpenCOBOL 1.1.0) | 4060 |
| COBOL - Free (OpenCOBOL 1.1.0) | 4061 |
| Brainfuck (bf 20041219) | 4062 |
| Ada2012 (GNAT 9.2.1) | 4063 |
| Unlambda (2.0.0) | 4064 |
| Cython (0.29.16) | 4065 |
| Sed (4.4) | 4066 |
| Vim (8.2.0460) | 4067 |

2. 旧システム

|          使用する言語           | 指定する値 |
| :-----------------------------: | :--------: |
|        C++14 (GCC 5.4.1)        |    3003    |
|     Bash (GNU bash v4.3.11)     |    3001    |
|          C (GCC 5.4.1)          |    3002    |
|         C (Clang 3.8.0)         |    3004    |
|       C++14 (Clang 3.8.0)       |    3005    |
|        C# (Mono 4.6.2.0)        |    3006    |
|         Clojure (1.8.0)         |    3007    |
|    Common Lisp (SBCL 1.1.14)    |    3008    |
|       D (DMD64 v2.070.1)        |    3009    |
|         D (LDC 0.17.0)          |    3010    |
|          D (GDC 4.9.4)          |    3011    |
|    Fortran (gfortran v4.8.4)    |    3012    |
|            Go (1.6)             |    3013    |
|      Haskell (GHC 7.10.3)       |    3014    |
|      Java7 (OpenJDK 1.7.0)      |    3015    |
|      Java8 (OpenJDK 1.8.0)      |    3016    |
|   JavaScript (node.js v5.12)    |    3017    |
|         OCaml (4.02.3)          |    3018    |
|       Pascal (FPC 2.6.2)        |    3019    |
|         Perl (v5.18.2)          |    3020    |
|          PHP (5.6.30)           |    3021    |
|         Python2 (2.7.6)         |    3022    |
|         Python3 (3.4.3)         |    3023    |
|          Ruby (2.3.3)           |    3024    |
|         Scala (2.11.7)          |    3025    |
|     Scheme (Gauche 0.9.3.3)     |    3026    |
|           Text (cat)            |    3027    |
|    Visual Basic (Mono 4.0.1)    |    3028    |
|         C++ (GCC 5.4.1)         |    3029    |
|        C++ (Clang 3.8.0)        |    3030    |
|     Objective-C (GCC 5.3.0)     |    3501    |
|    Objective-C (Clang3.8.0)     |    3502    |
|    Swift (swift-2.2-RELEASE)    |    3503    |
|          Rust (1.15.1)          |    3504    |
|       Sed (GNU sed 4.2.2)       |    3505    |
|        Awk (mawk 1.3.3)         |    3506    |
|     Brainfuck (bf 20041219)     |    3507    |
|  Standard ML (MLton 20100608)   |    3508    |
|          PyPy2 (5.6.0)          |    3509    |
|          PyPy3 (2.4.0)          |    3510    |
|        Crystal (0.20.5)         |    3511    |
|          F# (Mono 4.0)          |    3512    |
|        Unlambda (0.1.3)         |    3513    |
|           Lua (5.3.2)           |    3514    |
|         LuaJIT (2.0.4)          |    3515    |
|       MoonScript (0.5.0)        |    3516    |
|         Ceylon (1.2.1)          |    3517    |
|          Julia (0.5.0)          |    3518    |
|         Octave (4.0.2)          |    3519    |
|          Nim (0.13.0)           |    3520    |
|       TypeScript (2.1.6)        |    3521    |
|   Perl6 (rakudo-star 2016.01)   |    3522    |
|         Kotlin (1.0.0)          |    3523    |
|          PHP7 (7.0.15)          |    3524    |
| COBOL - Fixed (OpenCOBOL 1.1.0) |    3525    |
| COBOL - Free (OpenCOBOL 1.1.0)  |    3526    |

---

### テスト
ソースコードのテストを行う.

```bash
$ acc test <TASK>

ex )
$ acc test A
```

- コンテストディレクトリ内で実行を行う必要がある．
- 初回実行時AtCoderからテストを取得し，プロジェクト内にtestcase/\<TASK\>.tomlとして保存する．
- テストケースを追加したい場合は，testcaseディレクトリ内のファイルを編集することで対応できる

---

### 提出
ソースコードの提出を行う．

```bash
$ acc submit <TASK>

ex )
$ acc submit A
```
---

## 設定
### 詳細

|           項目            |             型              |                              説明                              |
| :------------------------ | :-------------------------- | -------------------------------------------------------------- |
| contest                   | String                      | コンテスト名(コンテストプロジェクト内の設定)                   |
| contest\_task\_name         | String                      | URLのタスク部分のコンテスト名(contestと異なるとき指定)        |
| total\_task               | Integer(符号なし8ビット)    | acc init時に作成されるファイル数を指定(拡張子設定が無いと無効) |
| extension                 | String                      | ファイルの拡張子を指定                                         |
| language\_id              | Integer(符号なし１６ビット) | AtCoderの言語指定                                              |
| test.compiler             | String                      | テスト時に使用するコンパイラを指定                             |
| test.compile\_arg         | String                      | コンパイル時のarg指定                                          |
| test.command              | String                      | 実行するコマンドを指定                                         |
| test.command\_arg         | String                      | コマンドを実行するときのarg指定                                |
| test.tle\_time            | Integer(符号なし１６ビット) | TLEの時間指定[ms]                                              |
| test.print\_wrong\_answer | Boolean                     | WAのときの出力                                                 |

---

### 設定例
\<config\_dir\>/acc/config.tomlの例
\<TASK\>はacc testで指定したものを代入するためのもの
#### C++

```toml
total_task = 6
extension = "cpp"
language_id = 3003

[test]
compiler = "g++"
compile_arg = "<TASK>.cpp -o <TASK>"
command = "./<TASK>"
tle_time = 3000
print_wrong_answer = true
```

#### Python3

```toml
total_task = 6
extension = "py"
language_id = 3023

[test]
command = "python3"
command_arg = "<TASK>.py"
tle_time = 3000
print_wrong_answer = true
```
