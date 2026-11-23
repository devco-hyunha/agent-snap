# 02-1 명세와 AC

## 형식

### 비밀 값 래퍼
- 타입: `SecretString`. 안에 `String`을 담는다. `Debug` 출력은 가렸음을 알리는 표시 문자열만 낸다.
- 값을 꺼내는 메서드는 하나만 둔다(`expose()`). 값은 전송·저장 필요 지점에서만 쓰고 로그에는 쓰지 않는다.
- `Display` 구현 금지(표시로도 노출 방지). `PartialEq`·`Hash`는 지금 두지 않는다(필요 시점에 02-5에서 결정).

### snap 설정 폴더 경로
- 함수: snap 설정 폴더 경로를 돌려준다. base 경로를 인자로 받아 결정하고, OS의 실제 경로는 OS별 파일에서만 정한다.
- Windows: `%APPDATA%\snap`. Linux: `$XDG_CONFIG_HOME/snap`, 미설정 시 `~/.config/snap`.
- 파일 분리: `mod.rs`(public re-export·테스트 진입), `windows.rs`(windows 구현), `unix.rs`(unix 구현). `mod.rs`에 `#[cfg]` 본문을 두지 않는다.

### 자격 파일
- 파일 이름 `credentials.jsonc`, 위치는 snap 설정 폴더.
- 형식(JSONC, 주석 허용 JSON). 예시(자리는 02-2의 `snap.example.jsonc`가 담당):

    ```jsonc
    {
      // 엔드포인트 이름 → 키
      "api_keys": {
        "primary": "(키 값)",
        "lab ": "(키 값)"
      }
    }
    ```

- 읽기 규칙:
  - 최상위 객체에서 `api_keys`만 읽는다. `api_keys`가 없거나 객체가 아니면 형식 오류.
  - `api_keys`의 값은 문자열만 허용. 문자열이 아니면 형식 오류.
  - 엔드포인트 이름은 앞뒤 공백을 trim한 값으로 찾고, 그 키로 저장한다.
  - 이름이 없으면 키 없음(None)을 돌려준다(정상 경로).
  - 파일이 없으면 파일 없음 오류를 명시적으로 돌려준다(02-2가 없는 파일과 없는 이름을 구분하므로 02-1은 없는 파일도 오류로 잡는다).
  - 주석 허용 JSON만 따른다. 후행 쉼표는 허용하지 않는다.

### 오류·텍스트 배치
- 자격 파일 오류 문구·가리기 표시 문자열은 locale 모듈 키로 둔다(`locale/ko/config.rs`, `locale/en/config.rs` 같은 키 뼈대). 함수 안에 즉석 문자열을 쓰지 않는다.
- 오류 문구는 파일 경로와 원인 요약만 담고, 자격 파일 내용 값과 base_url을 담지 않는다.

## AC
1. `SecretString`의 `Debug` 출력에 평문 키가 나오지 않고 가렸음을 표시함을 테스트로 확인한다.
2. 자격 파일 읽기가 `api_keys`에서 trim한 엔드포인트 이름으로 키를 찾음을 테스트로 확인한다.
3. 자격 파일에 없는 엔드포인트 이름을 요청하면 키 없음(None)을 돌려준다.
4. `api_keys` 누락·값이 문자열이 아닌 경우·파일 없음에서 파일 경로는 밝히고 값은 가린 오류를 돌려주며, 오류 문구가 locale 모듈 키에서 나옴을 확인한다.
5. 설정 폴더 경로 결정이 OS별 파일(`windows.rs`·`unix.rs`)에 있고 `mod.rs`에 `#[cfg]` 본문이 없으며, base 경로를 주입해 테스트하고 OS 실제 경로는 각 OS 시험으로 확인한다.
6. 게이트 3종 통과, 테스트 밖 `unwrap`/`expect`/`panic` 0건, 공개 항목에 한국어 문서 주석.
7. 새 크레이트를 쓴 경우 `docs/DECISIONS.md`에 라이선스 확인 기록이 있고, 리뷰 기록이 `docs/reviews/02/`에 있다.

## 테스트 배치(책임별 파일)
- 래퍼 가리기: `SecretString` Debug 테스트 1 책임 파일.
- 이름 조회: 정상 읽기, trim, 이름 없음(None)을 같은 파일에서 한 책임으로 묶어 확인.
- 거부: `api_keys` 누락, 값 타입 오류, 파일 없음.
- 경로: 주입한 base로 결정 확인(OS별 파일 진입 확인).
시험 본문은 제품 소스에 두지 않고 `crates/snap-core/tests/` 또는 모듈 단위 `#[cfg(test)]`로 책임을 나눠 둔다.