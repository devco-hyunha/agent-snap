# 03 에이전트 루프와 도구

- 상태: planned (본문 정리됨, 세부 plan은 03 세부 작업 착수 시 작성)

## 목적
도구를 써서 실제 코딩 작업을 끝까지 수행하는 에이전트 루프를 만든다.

## 범위(In)
- agent: 시스템 프롬프트, 메시지 기록, 도구 호출, 도구 결과 전달, 반복 상한, 종료 조건.
- 도구 호출 형식은 모델 프로필로 고른다(2026-11-08 사용자 결정 반영): 기본은 OpenAI `tools`/`tool_calls`, 로컬 미지원 모델은 텍스트 프로토콜 폴백, `execute`(코드 모드)는 도구 목록을 한 자리에 몰아 넣는 브리지로 둔다. 상세: [12번 문서](../research/12-tool-calling-forms.md).
- tools(`Tool` 트레이트, 도구당 파일 하나): read_file, write_file, edit_file(정확한 문자열 교체, 일치가 하나가 아니면 이유를 담아 실패), list_dir, glob, grep, run_command.
- 작업 폴더 경계: `..`, 절대 경로, 심볼릭 링크로 작업 폴더 밖에 쓰는 것을 거부한다.
- 승인·실행은 CLI가 주입하는 승인자 인터페이스로 분리한다(이후 자동화 단계 교체 가능). M1 기본 승인자는 명령마다 확인한다.
- 셸/`run_command` 실행 deadline(기본 **600초**, 설정으로 변경 가능).
- 대화형 승인 응답 deadline(기본 **300초**, 만료 시 거부).
- 명령 중단·deadline 만료 시 프로세스 트리 종료(플랫폼별; Windows Job Object 우선).
- 도구 출력이 길면 잘라서 전달한다.
- 작은 로컬 모델에 맞춘 짧은 시스템 프롬프트를 쓴다.
- `agent-snap run "<과제>"` 단발 실행.
- 단발 호출의 작업 명세는 사용자 원문을 보존하고 목표·입력·완료 조건만 담는다.
- 작업 명세의 완료 조건에 검증 명령이 있으면 에이전트가 끝난 뒤 그 명령을 실행해 성공 여부를 확인하고, 실패하면 실패를 드러낸다.
- 모델 선택: `run`은 `--model <이름>`으로 모델을 고른다. 지정하지 않으면 설정의 모델 목록에서 고르게 하고 이어서 실행한다. 입력을 받을 수 없으면 모델 이름 목록을 담은 오류로 끝낸다.
- (AGENTS.md 읽기 추가, 2026-11-08 사용자 승인) 작업 폴더의 `AGENTS.md`를 읽어 프로젝트 지시로 시스템 프롬프트에 조립한다. 03 단발 실행 에이전트가 코딩 한 바퀴에 프로젝트 지시를 반영하는 필수 요소로 두는다. 04 대화형 세션의 AGENTS.md 읽기와 같은 규칙 파일 사용이고, 03은 04 규칙 계층 설계에 앞서 최소 형태만 쓴다.
- 설정·키 위치: 사용자 설정 폴더의 `agent-snap.jsonc`와 자격 파일 `credentials.jsonc`를 읽는다. `AGENT_SNAP_CONFIG_DIR`로 폴더를 바꿀 수 있다. `agent-snap.example.jsonc`의 위치 설명을 맞춘다.
- 비밀 파일 차단: 파일 도구는 사용자 설정 폴더 아래와 작업 폴더(하위 폴더 포함)의 `agent-snap.jsonc`와 `credentials.jsonc`를 읽거나 쓰지 않고 목록·검색 결과로도 내보내지 않는다(DECISIONS D17).
- 도구 호출 인자: 인자 JSON을 해석하고, 깨진 JSON은 복구를 시도하며, 실패하면 모델에 짧은 오류를 돌려준다.

## 비범위(Out)
대화형 화면, 세션 저장, 감시.
- L2·L3 자동화 단계·세션 동안 허용·패턴 자동 허용·Prospero식 기본 무인.
- 실행 격리(AppContainer 등, D9).
- timeout→백그라운드+주기 wake는 03에서 설계만 적거나 세부 작업으로 분리 가능(구현 범위는 세부 작업 착수 시 AC로 확정).

## 조사
- 주제: 언어 모델용 도구 인터페이스 설계(에이전트-컴퓨터 인터페이스), 코드 편집 형식별 성공률, 작은 모델의 도구 호출 안정성.
- opencode, Codex CLI의 편집 도구와 명령 승인 설계를 참고한다.
- Pi 코딩 에이전트(Flue의 기반 하네스)의 짧은 시스템 프롬프트와 최소 도구 구성을 확인해 참고한다. Flue(Apache-2.0)의 함수형 에이전트 선언, 타입이 있는 도구, 샌드박스 분리를 참고한다.

## D22 정책 정본

D22 **정책 정본**은 `docs/decisions/D22-execution-reliability-autonomy.md`이다. 이 기획 문서는 03 범위·AC·세부 작업만 다루며, L1~L3 정의·무인 해석을 여기서 재정의하지 않는다.

## 반영할 연구
- 도구는 최소·구조화하고 실패 시 짧은 오류를 돌려준다(SWE-agent 2405.15793).
- 도구 호출 파서는 JSON 복구와 재시도를 하고, 도구 호출 턴은 낮은 온도로 둔다(2609.07370).
- 편집은 검색-치환을 기본으로 하고, 형식별 실패율은 06에서 잰다(2604.27296).
- Prospero(RESEARCH Prospero 절): 셸·승인 deadline, 프로세스 트리 종료(Job Object). timeout→백그라운드는 후보.
- 실행 신뢰성·승인자 주입·L1 기본(D22).

## AC
1. 모의 서버 시나리오에서 도구 호출 → 결과 전달 → 최종 답 루프가 끝까지 돈다.
2. 작업 폴더 밖 쓰기 거부, run_command 미승인 시 미실행, edit_file 다중 일치 실패를 테스트로 확인한다. 사용자 설정 폴더와 작업 폴더의 `agent-snap.jsonc`, `credentials.jsonc` 파일을 파일 도구가 다루지 않음을 테스트로 확인한다.
3. 실제 엔드포인트로 과제 3개(파일 만들기, 수정, 테스트 실행)를 수행한 기록이 `docs/reports/03-smoke.md`에 있다.
4. 도구 호출을 지원하지 않는 모델이면 이유가 담긴 오류를 낸다. 모델을 지정하지 않으면 목록에서 고른 모델로 실행하고, 입력을 받을 수 없으면 모델 이름 목록을 담은 오류를 낸다. 설정 폴더 위치가 정해진다(테스트, 폴더 환경 정보는 주입).
5. RESEARCH.md에 조사 항목과 적용 위치가 있다. 검증 명령(`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`)을 통과한다.
6. 단발 호출 작업 명세에 사용자 원문이 보존되고 목표·입력·완료 조건만 담김을 테스트로 확인한다.
7. 작업 명세의 완료 조건에 검증 명령이 있으면 에이전트가 끝난 뒤 그 명령을 실행하고, 실패하면 0이 아닌 종료 코드와 실패 이유를 낸다(모의 서버 테스트).
8. `run_command`가 deadline 만료 시 미완료로 끝나고, 자식 프로세스가 테스트에서 남지 않음을 확인한다(플랫폼 가능 범위, 기본값 600초·설정 가능).
9. 승인 응답 deadline 만료 시 명령이 실행되지 않고 거부로 기록됨을 테스트로 확인한다(기본값 300초·설정 가능). 승인자 미주입/비대화형은 즉시 거부.

## 테스트 계획
검증 명령(`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`), 실제 엔드포인트 스모크.

## 문서 영향
- `docs/decisions/D22-execution-reliability-autonomy.md` (D22 정본)
- `docs/DECISIONS.md` D22
- `AGENTS.md` 키 규칙
- `agent-snap.example.jsonc` 위치 설명
- `docs/RESEARCH.md` 작업 03 인덱스, `docs/research/03-*.md` 상세, 자율성 검토 절

## 리스크
로컬 모델의 도구 호출 품질이 낮을 수 있다. 결과를 보고하고, 텍스트 프로토콜 대안은 필요하면 별도 작업으로 기획한다.
- `run_command`로 비밀 파일을 읽는 우회는 이름 규칙으로 다 막을 수 없다. 실행 전 사용자 확인으로 막고 스모크 보고에 한계로 적는다.
- 승인 timeout과 「무인 계속」을 혼동하지 말 것 — M1은 만료=거부 후 그 도구 결과만 모델에 알림, L3가 아님.

## DoD
AC 충족, 검증 통과, 리뷰 PASS.
- 추가(D43): 03 완료 시 "코딩 한 바퀴" 기본 도구가 전부 갖춰진 상태여 한다. [기능 목록](../research/agent-feature-inventory-2026-11-07.md) (a) 항목과 대조한 결과를 `docs/reports/03-smoke.md`에 남긴다. 목록의 후보 중 04 배치 항목을 04 기획에 넘기는지 확인한다.
- 추가(2026-11-08): AGENTS.md 읽기(03 범위)를 03-1에서 구현하고 (a) 대조에 포함한다. 심볼 도구·지시 파일 색인·규칙 계층·메모리는 04 배치 확정(D43·D45)로 03 범위 밖이다.

### 세부 작업 공통 기준
각 세부 작업의 tasks.md는 이 절을 따른다.
- 문서 규격(2026-11-08, PROJECT_RULES 문서 규격 절): 각 세부 plan.md에 `## 작업계획` 표를 두고 열은 `| 작업 | 범위 | 금지 | 산출물 | 검증 명령 |`로 한다(행 이름은 `T<번호> <작업명>`). plan.md·tasks.md에는 `## 작업 파일` 표를 두고 열은 `| 파일 | 작업 | 상태 |`로 한다. 상태는 계획 / 진행 / 완료 / 막힘 중 하나이고, 상세 작업 파일(T-파일)의 상태가 바뀔 때마다 plan.md와 tasks.md의 표를 함께 고친다. 상세 작업 파일은 T1부터 순번을 붙인 `T<번호>-<짧은 이름>.md` 파일 하나이며, 절은 목표·근거·읽을 파일·할 일·검증·완료 조건·상태·라운드 보고로 한다. 상세 작업 파일에는 결정 문서 본문을 복제하지 않고 절 번호와 포인터만 적는다.
- 하네스 전달(2026-11-08, 2026-11-09 문구 정합): 상세 작업 파일의 `읽을 파일`에는 `harness/HARNESS.md`, `AGENTS.md`, `harness/PROJECT_RULES.md` 개발 규칙 절, dev-principles 경로(`.agents/skills/dev-principles/SKILL.md`, Claude Code·Cursor는 `.claude/skills/dev-principles/SKILL.md`)를 반드시 넣는다. `할 일`에는 PROJECT_RULES 64절의 두 단계를 그대로 넣는다: ① "dev-principles 2~11절(SRP 파일·함수 분리, DRY 재사용 사다리) 준수" ② "서로 다른 책임을 한 파일에 몰아넣지 않고 역할별 파일로 분리". `완료 조건`에는 "dev-principles 2~11절 준수 확인과 파일 분기 구조"를 넣는다. 검증 절의 SRP·DRY 확인은 한 기준으로 통일한다: "중복 정의 없음을 grep으로 확인하고 재사용 정본 파일:라인을 보고에 적는다". 위임 프롬프트는 delegation-brief의 포인터 샌드위치(가드레일 6줄 → 작업 포인터 → 자기점검 4줄, 500자 이하)로 보내고, 가드레일 3번에 위 4개 문서 선독이 들어 있다.
- 작업 방식: 한 번 위임하고 한 번 코드 리뷰로 끝낸다(D11).
- 검증 명령: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- 공통 DoD: 그 세부 작업의 AC를 충족한다. 검증 명령이 통과한다. `docs/RESEARCH.md` 적용 위치를 갱신했다. 코드 품질 리뷰(review-harness 체크리스트와 dev-principles 2~11절(`.agents/skills/dev-principles/SKILL.md`, Claude Code와 Cursor는 `.claude/skills/dev-principles/SKILL.md`), 테스트 코드 포함)가 PASS했고 기록이 `docs/reviews/03/`에 있다.

## 세부 작업
세부 작업 하나는 한 번 위임하고 한 번 코드 리뷰로 끝나는 기능 하나다(DECISIONS D11). 자동화 단계 L2·L3는 본 작업 비범위(D22)이다. 아래 순서대로 하나씩 진행한다. 세부 작업마다 plan.md, spec.md, tasks.md를 둔다. (2026-11-08 번호 재정렬: 03-1부터 03-10. 2026-11-09 (a)(b) 이관으로 03-11~13 추가, 스모크 03-10은 표의 마지막 자리 순서 14번. 착수 전 상태는 각 문서 상단에 `planned`로 둔다.)

| 순서 | 번호 | 이름 | 선행 | 문서 |
|---|---|---|---|---|
| 1 | 03-1 | `Tool` 트레이트 설계(도구 등록·결과·승인자 자리) | 02 | [03-01-tool-trait](03-agent-loop-tools/03-01-tool-trait/plan.md) |
| 2 | 03-2 | 에이전트 루프·호출 형식 1안(OpenAI 기본 경로) | 03-1 | [03-02-agent-loop](03-agent-loop-tools/03-02-agent-loop/plan.md) |
| 3 | 03-3 | 파일 도구·grep·glob·list_dir·비밀 파일 차단 | 03-1, 03-2 | [03-03-file-tools](03-agent-loop-tools/03-03-file-tools/plan.md) |
| 4 | 03-4 | 호출 형식 2안 텍스트 프로토콜 폴백(로컬 미지원 모델) | 03-2 | [03-04-form-protocol](03-agent-loop-tools/03-04-form-protocol/plan.md) |
| 5 | 03-5 | AGENTS.md 읽기 조립 | 03-2 | [03-05-agents-md](03-agent-loop-tools/03-05-agents-md/plan.md) |
| 6 | 03-6 | run_command·승인자 주입·승인/실행 deadline | 03-2, 03-3 | [03-06-run-command](03-agent-loop-tools/03-06-run-command/plan.md) |
| 7 | 03-7 | 프로세스 트리 종료(Job Object 등) | 03-6 | [03-07-process-tree](03-agent-loop-tools/03-07-process-tree/plan.md) |
| 8 | 03-8 | 호출 형식 3안 execute 브리지(읽기 전용 도구부터) | 03-3, 03-6 | [03-08-execute-bridge](03-agent-loop-tools/03-08-execute-bridge/plan.md) |
| 9 | 03-9 | (후보) timeout→백그라운드·주기 wake | 03-7 | [03-09-timeout-background](03-agent-loop-tools/03-09-timeout-background/plan.md) |
| 11 | 03-11 | (a)(b) 배치 이관 1차(계획 모드·3단계 승인 규칙·ask_user) | 03-2, 03-6 | [03-11-a-b-tools](03-agent-loop-tools/03-11-a-b-tools/plan.md) |
| 12 | 03-12 | (a)(b) 배치 이관 2차(웹 도구·체크포인트) | 03-11 | [03-12-a-b-tools-2](03-agent-loop-tools/03-12-a-b-tools-2/plan.md) |
| 13 | 03-13 | (a)(b) 배치 이관 3차(심볼 도구·훅·MCP) | 03-12 | [03-13-a-b-tools-3](03-agent-loop-tools/03-13-a-b-tools-3/plan.md) |
| 14 | 03-10 | 단발 run·작업 명세·검증 명령·스모크(3형식·이관 골격 최종 검증) | 03-4, 03-8, 03-6, 03-11~13 | [03-10-run-once-smoke](03-agent-loop-tools/03-10-run-once-smoke/plan.md) |

순서 정정 근거(2026-11-08):
- 03-1(트레이트 설계)이 루프·파일도구·execute의 재료이므로 맨 앞에 둔다.
- 03-4(로컬 AI 폴백)는 스냅 메인 사용처가 로컬 모델이므로 스모크(03-10)보다 앞서 만든다.
- 03-8(execute)은 도구 7개가 갖춰진 뒤(03-3·03-6) 같은 게이트를 지나는 브리지로 붙인다.
- 03-10(스모크)는 3형식(1안·2안·3안)과 (a)(b) 이관 골격(03-11~13)을 다 갖춘 뒤 최종 검증으로 맨 마지막에 둔다.

배치 확정 메모(2026-11-08 기획 재검토):
- ① 병렬 호출(parallel_tool_calls, 한 응답의 여러 호출)은 03-2가 dict 저장으로 순차 실행만 처리하고, 한 응답의 여러 호출 파라미터는 06에서 측정한 뒤 판단한다(후보 D48). 2안 이상의 여러 호출 파싱은 03-4가 정한다.
- ② `tool_call_form` 필드(`openai`·`text`·`auto`, 2026-11-10 D50 확정: 칸 이름은 `tool_call_form`, 선언 우선, `auto`만 감지. 03-2 spec AC 4 첨삭)와 검증 규칙은 03-4 모델 프로필이 만든다. 03-2 T2는 미지원 감지 인터페이스와 4xx 판별만 만든다.
- ③ MCP 확장은 2026-11-09 2차 개정으로 03-13에 배치한다(04 배치 항목에서 03으로 이관).
