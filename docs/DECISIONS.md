# 결정 인덱스

- 이 파일은 결정 인덱스입니다. 짧은 결정은 전문을 여기 적고, 긴 결정은 `decisions/D<번호>-<주제>.md`에 정본을 두고 요약·링크·다시 볼 점만 적습니다.
- 번호는 한 번 부여하면 바꾸지 않습니다.

| 번호 | 결정 | 상태 |
|---|---|---|
| 1 | 라이선스는 Apache-2.0를 저장소 최상위 `LICENSE`로 둔다. 소스 헤더 여부는 코드 착수 시 정한다 | 확정 |
| 2 | 원서 접속은 Axum HTTP 서버로 열되 04-4에서 다룬다. 수신 주소는 `snap.jsonc`, 계정은 `credentials.jsonc`, 런타임 주입은 환경 변수. 기본은 로컬 루프백이고 외부 공개는 설정과 인증을 거친다 | 채택 (04-4 착수 시 확정) |
| 3 | 자격 파일 읽기에 `json_comments` 0.2.2(Apache-2.0), `serde_json` 1.0.154(MIT OR Apache-2.0), `thiserror` 2.0.21(MIT OR Apache-2.0), 시험용 dev-dependency로 `tempfile` 3(MIT OR Apache-2.0)을 쓴다. 저장소 라이선스(D1의 Apache-2.0)와 MIT·Apache-2.0 모두 호환된다. 근거: 주석 허용 JSON은 comment stripping 전용 크레이트로 읽고, JSON 객체 표는 serde_json으로 해석한다. 오류 타입은 thiserror, 시험용 임시 폴더는 tempfile로 만든다. 다시 볼 점: `json_comments`가 후행 쉼표를 허용하지 않는지 02-2에서 재확인한다(현재 시험에서 거부 확인) | 채택 |
| 4 | `deny.toml`의 `[licenses] allow`를 OSI·FSF 승인 관례 10종(MIT, Apache-2.0, Apache-2.0 WITH LLVM-exception, ISC, BSD-3-Clause, MIT-0, CC0-1.0, Unicode-3.0, Zlib, Unlicense)으로 채운다. 01 뼈대는 5종만 뒀고 02-1에서 `thiserror` 트리의 `unicode-ident 1.0.27`이 `(MIT OR Apache-2.0) AND Unicode-3.0` 라이선스로 처음 등장해 `cargo deny check`가 거부했다. 근거: 백사 프로젝트가 이미 승인한 allow 목록과 같은 기준(OSI·FSF 승인 관례만). copyleft(LGPL·GPL)와 미승인(CDLA)은 기본 거부. 다시 볼 점: CDLA-Permissive-2.0 예외 항목(백사의 webpki-root-certs 단일 예외)은 필요 시점에 별도 결정으로 연다 | 채택 |
