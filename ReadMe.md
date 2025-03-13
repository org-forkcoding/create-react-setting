# Create React Setting

이 프로젝트는 주어진 경로에 React 프로젝트의 기본 디렉토리 구조를 생성하는 Rust 프로그램입니다.

## 사용법

```bash
create-react-setting <base-path>
```

또는

```bash
# bashrc나 zshrc에 등록후 사용
cres <base-path>
```

- `<base-path>`: 디렉토리 구조가 생성될 기본 경로를 지정합니다.

## 디렉토리 구조

프로그램은 다음과 같은 디렉토리를 생성합니다:

- `actions`
- `components`
- `providers`
- `lib`
- `animations`
- `hooks`
- `store`

## 설치

Rust가 설치되어 있어야 합니다. [Rust 설치](https://www.rust-lang.org/tools/install)

```bash
git clone <repository-url>
cd create-react-setting
cargo build --release
```

## 실행

빌드 후, 다음 명령어로 프로그램을 실행할 수 있습니다:

```bash
./target/release/create-react-setting <base-path>
```

## 기여

기여를 환영합니다! 이 프로젝트에 기여하려면, 이 저장소를 포크하고 풀 리퀘스트를 제출하세요.
